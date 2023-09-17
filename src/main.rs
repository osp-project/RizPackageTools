mod log_manager;
mod ui_main;
mod android_check;
mod metadata_tools;
mod structs;
mod tools;
mod ziptool;

#[tokio::main]
async fn main() {
    log_manager::init_log();
    if !android_check::check_jvm(){
        msgbox::create("Java错误", ("您似乎未安装Java或未将已安装的Java添加到环境变量，RizPackageTools本身不需要Java，但由于实现安卓包体修改相关功能需要引用到使用Java编写的apktool，因此您必须安装Java并将其添加到环境变量中。\n如果可以，请您从https://www.oracle.com/java/technologies/downloads/#jdk20-windows下载并安装最新版本的Java\n".to_string()).as_str(), msgbox::IconType::Error).unwrap();
    }
    log::info!("RizPackageTools开始运行!");
    ui_main::init_ui().await;
}