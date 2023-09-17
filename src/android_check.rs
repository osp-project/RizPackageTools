use std::process::Command;

pub fn check_jvm() -> bool {
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
        return true;
    } else {
        // Command failed, print the error
        log::info!("Java可能没有正确安装或添加到环境变量");
        log::info!("控制台输出：{}", String::from_utf8_lossy(&output.stderr));
        return false;
    }
}