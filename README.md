# AYANE
使用Rust实现的，简易编写的公主连结国服服务端。  
名称取自《公主连结ReDive》中的角色[北条绫音](https://mzh.moegirl.org.cn/zh-hans/%E5%8C%97%E6%9D%A1%E7%BB%AB%E9%9F%B3)。  
本项目仅供学习交流，严禁用于商业途径，请于获取的24小时内删除。  

## 如何使用？
使用前请准备好MySQL环境。  
将[struct.sql](https://github.com/Kengxxiao/Ayane/blob/master/struct.sql)导入您的MySQL服务端，之后修改[本行](https://github.com/Kengxxiao/Ayane/blob/ddec768ca5150ca82d087507e0267bf3b69c84dd/src/database.rs#L25)，将链接指向您配置好的MySQL服务端。  
使用Cargo编译项目，并运行在指定设备上。  
同时，本项目提供了一个简易的配置页面用于查询和调整内容，见[html](https://github.com/Kengxxiao/Ayane/blob/master/arena_log_search.html)。

## 已实现的功能
- [x] 人物等级与装备基本系统
- [x] 公会战
- [x] 战斗竞技场

## 未实现或存在缺陷的功能
- [ ] 公主竞技场
- [ ] 游戏内直接进行⭐6才能开花
- [ ] 公会战战斗日志分析
