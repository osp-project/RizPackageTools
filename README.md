# RizPackageTools
针对某款竖屏音乐游戏的改包工具

## 使用
1. 从[Releases](/Releases)页下载最新的编译后版本
2. 从[这里](https://github.com/osp-project/RizPackageTools/blob/master/target_strings.json)下载`target_settings.json`或自己编写
3. 将`target_settings.json`放置到RizPackageTools的二进制文件运行目录下以便其读取
4. Enjoy it!

## 编译
1. 安装`rust`和`cargo`

2. 切换到 `Nightly` 版编译器：`rustup default nightly`（必须，由于某些功能只允许在Nightly版编译器中启用）

3. 然后运行`install_env.bat`再运行`build.bat`最后能在build文件夹下看到输出，当然你直接`cargo run`也不是不行

如果要切换回stable编译器，使用 `rustup default stable`

## 配置要求
|  配件名称   | 最低配置  | 推荐配置 |
|  ----  | ----  | ---- |
| 操作系统  | Rust支持的目标操作系统 | Rust支持的目标操作系统|
| 架构  | Rust支持的目标处理器架构 | Rust支持的目标处理器架构|
| CPU | 奔腾4或更高等级处理器 | 10代i3或更高等级处理器|
| 内存 | 8G或更高运行内存 | 16G或更高运行内存 |
| 硬盘 | 机械硬盘 | 固态硬盘 |
| 桌面环境 | 对于Linux，请安装GTK-3 | 对于Linux，请安装GTK-3，最好也安上QT |

## 开源软件声明
`RizPackageTools`的诞生离不开伟大的开源软件，为了实现一些功能，我们引用了以下开源软件的源代码或二进制文件，又或是参考了其设计思路：

[iBotPeaches/Apktool](https://github.com/iBotPeaches/Apktool) - A tool for reverse engineering Android apk files 

[JeremieCHN/MetaDataStringEditor](https://github.com/JeremieCHN/MetaDataStringEditor) - Modify string in file global-metadata.dat 
