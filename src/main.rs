mod log_manager;
mod ui_main;
mod android_check;

fn main() {
    log_manager::init_log();
    android_check::check_jvm();
    log::info!("RizPackageTools开始运行!");
    ui_main::init_ui();
}