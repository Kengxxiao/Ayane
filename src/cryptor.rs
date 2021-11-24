use std::{ops::{Deref, DerefMut}, pin::Pin};
use actix_http::{Error, Payload, Response, http::{HeaderValue, StatusCode}};
use actix_web::{FromRequest, HttpRequest, Responder, web::BytesMut};
use aes::Aes256;
use block_modes::{Cbc, block_padding::Pkcs7};
use futures::{FutureExt, StreamExt, future::{LocalBoxFuture, ok}};
use futures_util::future::Ready;
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use serde::{Serialize, de::DeserializeOwned};
use block_modes::BlockMode;
use serde_json::json;

use crate::database;

pub struct GlobalSerde<T>(pub T, pub i32, pub Option<String>);

impl<T> GlobalSerde<T> {
    // pub fn into_inner(self) -> T {
    //     self.0
    // }
    pub fn success(data: T) -> Self {
        GlobalSerde(data, 1, None)
    }
}

pub fn get_fail_resp(error_message: &str, err_code: i32) -> GlobalSerde<serde_json::Value> {
    GlobalSerde(default_server_error(error_message), err_code, None)
}

impl<T> Deref for GlobalSerde<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for GlobalSerde<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

impl<T: Serialize> Responder for GlobalSerde<T> {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let msgpack_enc = !req.query_string().contains("format=json");
        let request_id = req.headers().get("request-id").unwrap_or(&HeaderValue::from_static("")).to_str().unwrap().to_string();
        // println!("check rid: {}", request_id);
        let (new_req, new_sid, sid, vid) = database::update_request_id_and_sid(request_id);
        let body_2 = &serde_json::json!({
            "data_headers": {
                "short_udid": sid,
                "viewer_id": vid,
                "sid": new_sid,
                "servertime": chrono::Utc::now().timestamp(),
                "result_code": &self.1,
                "request_id": new_req
            },
            "data": &self.0
        });
        let body = match msgpack_enc {
            false => serde_json::to_string(body_2).unwrap(),
            true => {
                let key : String = thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect();
                let encryptor = Aes256Cbc::new_var(key.as_bytes(), "ha4nBYA2APUD6Uv1".as_bytes()).unwrap();
                let mut encrypted = encryptor.encrypt_vec(&rmp_serde::to_vec(&body_2).unwrap());
                encrypted.append(&mut key.into_bytes());
                base64::encode(encrypted)
            }
        };
        ok(Response::build(StatusCode::OK)
            .content_type(if msgpack_enc {"application/x-msgpack"} else {"application/json"})
            .body(body))
    }
}

pub struct GloablSerdeBody<T> {
    fut: Option<LocalBoxFuture<'static, Result<T, Error>>>,
    stream: Option<Payload>,
    msgpack_enc: bool
}

impl<T> GloablSerdeBody<T> where T: DeserializeOwned + 'static {
    pub fn new(req: &HttpRequest, payload: &mut Payload) -> Self{
        GloablSerdeBody {
            fut: None,
            stream: Some(payload.take()),
            msgpack_enc: !req.query_string().contains("format=json")
        }
    }
}

fn default_server_error<T>(error_message: &str) -> T where T: DeserializeOwned + 'static {
    //println!("{} {}", error_message, err_code);
    serde_json::from_value(json!({
        "server_error": {
            "status": 3,
            "title": "错误提示",
            "message": format!("{}\\n回到标题界面。", error_message)
        }
    })).unwrap()
}

pub fn decrypt_base64_encrypted(enc: String) -> i64 {
    let b = match base64::decode(enc.as_str()) {
        Ok(s) => s,
        Err(_) => return enc.parse::<i64>().unwrap()
    };
    let len = b.len();
    let decryptor = Aes256Cbc::new_var(&b[len - 32..len], "ha4nBYA2APUD6Uv1".as_bytes()).unwrap();
    match &decryptor.decrypt_vec(&b[..len-32]) {
        Ok(data) => {
            let c = String::from_utf8(data.to_owned()).unwrap();
            c.parse::<i64>().unwrap_or_default()
        },
        Err(e) =>  {
            println!("{}", e);
            0
        }
    }
}

impl<T> futures::Future for GloablSerdeBody<T> where T: DeserializeOwned + 'static {
    type Output = Result<T, Error>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(ref mut fut) = self.fut {
            return Pin::new(fut).poll(cx);
        }
        let mut stream = self.stream.take().unwrap();
        let msenc = self.msgpack_enc;
        self.fut = Some(
            async move {
                let mut body = BytesMut::new();
                while let Some(item) = stream.next().await {
                    let chunk = item?;
                    body.extend_from_slice(&chunk);
                }
                match msenc {
                    true => {
                        let len = body.len();
                        if len < 32 {
                            return Ok(default_server_error("无法解析的请求"));
                        }
                        let decryptor = Aes256Cbc::new_var(&body[len - 32..len], "ha4nBYA2APUD6Uv1".as_bytes()).unwrap();
                        match &decryptor.decrypt_vec(&body[..len-32]) {
                            Ok(data) => {
                                match rmp_serde::from_read_ref(data) {
                                    Ok(datas) => Ok(datas),
                                    Err(err) => Ok(default_server_error(&err.to_string()))
                                }
                            },
                            Err(err) => Ok(default_server_error(&err.to_string()))
                        }
                        
                    },
                    false => match serde_json::from_slice::<T>(&body) {
                        Ok(data) => Ok(data),
                        Err(err) => Ok(default_server_error(&err.to_string()))
                    }
                }
            }.boxed_local(),
        );
        self.poll(cx)
    }
}

impl<T> FromRequest for GlobalSerde<T> where T: DeserializeOwned + 'static, {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let request_id = req.headers().get("request-id").unwrap_or(&HeaderValue::from_static("")).to_str().unwrap().to_string();
        let sid = req.headers().get("sid").unwrap_or(&HeaderValue::from_static("")).to_str().unwrap().to_string();
        let login_phase = !req.to_owned().path().contains("tool/sdk_login");

        GloablSerdeBody::new(req, payload).map(move |res| match res {
            Err(e) => Ok(GlobalSerde(default_server_error(&e.to_string()), 9997, None)),
            Ok(data) => {
                if login_phase {
                    let (rid, sid2, _, _) = database::get_request_id_and_sid(&request_id);
                    if request_id != rid {
                        return Ok(GlobalSerde(default_server_error("来自非法响应的请求"), 9999, None))
                    }
                    if sid2 != sid {
                        return Ok(GlobalSerde(default_server_error("未接续的请求"), 9998, None))
                    }
                }
                Ok(GlobalSerde(data, 1, Some(request_id)))
            }
        }).boxed_local()
    }
}