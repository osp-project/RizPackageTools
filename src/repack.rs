use std::process::Command;

pub fn repack_now(is_android: bool){
    let mut currentdir = "".to_string();

    match std::env::current_exe() {
        Ok(exe_path) => {
            log::info!("此程序的文件地址: {}", exe_path.display());
            match exe_path.parent() {
                Some(dir_path) => {log::info!("此程序的路径: {}", dir_path.display()); currentdir = dir_path.display().to_string() + "/"},
                None => {log::error!("指定的文件地址没有路径");msgbox::create("错误", "请不要把这个程序放在根目录运行...这可真是活久见", msgbox::IconType::Error).unwrap()},
            }
        }
        Err(e) => {log::error!("无法获取当前程序的文件地址: {}", e);msgbox::create("错误", "无法获取此程序的文件地址", msgbox::IconType::Error).unwrap()},
    };

    if is_android{
        let mut cmd = Command::new("java");
        cmd.args(["-jar", format!("{}/apktool.jar", currentdir).as_str(), "b", "cache/ApkDecoding", "-o", "Output_Package_Android.apk", "-f"]);
        let output = cmd.output().expect("Failed to execute command");
        if output.status.success() {

        } 
        else {
            let output_str = String::from_utf8_lossy(&output.stderr).to_string();
            log::error!("在打包apk时遇到问题，控制台输出：{}", &output_str);
            msgbox::create("错误",("在打包apk时遇到问题，控制台输出：".to_string() + &output_str).as_str() ,msgbox::IconType::Error).unwrap();
        }
    }
    else{
        log::info!("包体为ios，开始压缩Payload为zip");
        crate::ziptool::packzip("cache/ipaUnZip/", "Output_Package_iOS.ipa").unwrap();
    }
}