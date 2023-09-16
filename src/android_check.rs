use std::process::Command;

pub fn check_jvm() {
    // Create a Command instance with "java" as the program name
    let mut cmd = Command::new("java");
    // Add "--version" as the argument
    cmd.arg("--version");
    // Execute the command and get the output
    let output = cmd.output().expect("Failed to execute command");
    // Check the status code
    if output.status.success() {
        // Command executed successfully, print the output
        log::info!("检测到Java已正确安装！");
        log::info!("Java版本信息：{}", String::from_utf8_lossy(&output.stdout));
    } else {
        // Command failed, print the error
        log::info!("Java可能没有正确安装或添加到环境变量，准备弹窗");
        log::info!("控制台输出：{}", String::from_utf8_lossy(&output.stderr));
        let _ = msgbox::create("Java错误", ("您似乎未安装Java或未将已安装的Java添加到环境变量，RizPackageTools本身不需要Java，但由于实现安卓包体修改相关功能需要引用到使用Java编写的apktool，因此您必须安装Java并将其添加到环境变量中。\n控制台输出信息：\n".to_string() + std::str::from_utf8(String::from_utf8_lossy(&output.stderr).as_bytes()).unwrap()).as_str(), msgbox::IconType::Error);
    }
}