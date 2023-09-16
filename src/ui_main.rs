use native_dialog::FileDialog;

slint::include_modules!();

pub fn init_ui() {
    log::info!("进行UI相关绑定");
    let uimainwindow = UIMainWindow::new().unwrap();
    let uimainwindow_weak = uimainwindow.as_weak().unwrap();
    uimainwindow.set_show_edit_package_ui_group(false);
    uimainwindow.set_show_package_select_ui_group(true);
    uimainwindow.on_select_package_btn_click(move || {
        let dialog = FileDialog::new()
            .add_filter("ALL Supported Format", &["apk", "ipa"])
            .add_filter("Any file (Maybe can't work)", &["*"]);

        let result = dialog.show_open_single_file();
        match result {
            Ok(Some(path)) => {
                let path = path.to_string_lossy().into_owned();
                log::info!("用户选择的包体路径：{}",path);
                uimainwindow_weak.set_package_selected_path_text(path.into());
            }
            Ok(None) => {
                log::info!("用户并没有选择任何文件");
            }
            Err(error) => {
                log::error!("FilePicker出错，详细信息：{}",error);
                let _ = msgbox::create("出现错误", ("在尝试打开文件选取器时遇到问题，错误信息：\n".to_string() + error.to_string().as_str()).as_str(), msgbox::IconType::Error);
            }
        }
    });

    log::info!("尝试显示UI");
    uimainwindow.run().unwrap(); //运行UI
}
