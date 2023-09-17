use native_dialog::FileDialog;
use std::{path::Path, process::Command};
use tokio::time::{sleep, Duration};

slint::include_modules!();

pub async fn init_ui() {
    log::info!("进行UI相关绑定");
    let uimainwindow = UIMainWindow::new().unwrap();
    let mut uimainwindow_weak = uimainwindow.as_weak().unwrap();
    uimainwindow.set_show_edit_package_ui_group(false);
    uimainwindow.set_show_package_select_ui_group(true);
    uimainwindow.on_select_package_btn_click(move || {
        let dialog = FileDialog::new().add_filter("ALL Supported Format", &["apk", "ipa"]);
        //.add_filter("Any file (Maybe can't work)", &["*"]);

        let result = dialog.show_open_single_file();
        match result {
            Ok(Some(path)) => {
                let path = path.to_string_lossy().into_owned();
                log::info!("用户选择的包体路径：{}", path);
                uimainwindow_weak.set_package_selected_path_text(path.into());
            }
            Ok(None) => {
                log::info!("用户并没有选择任何文件");
            }
            Err(error) => {
                log::error!("FilePicker出错，详细信息：{}", error);
                let _ = msgbox::create(
                    "出现错误",
                    ("在尝试打开文件选取器时遇到问题，错误信息：\n".to_string()
                        + error.to_string().as_str())
                    .as_str(),
                    msgbox::IconType::Error,
                );
            }
        }
    });

    uimainwindow_weak = uimainwindow.as_weak().unwrap(); //Make it again

    uimainwindow.on_start_edit_btn_click(move || {
        log::info!("开始进行路径检查");
        if uimainwindow_weak.get_package_selected_path_text().to_string() == "等待选择..." {
            log::info!("用户未选择文件");
            let _ = msgbox::create("未选择文件", "请先选择一个游戏包体文件再继续，可以是IPA也可以是APK", msgbox::IconType::Info);
            return;
        }
        else{
            let binding = uimainwindow_weak.get_package_selected_path_text().to_string();
            let selected_path = Path::new(&binding);
            if !(selected_path.exists() && selected_path.is_file()) {
                log::info!("用户选择的文件路径无效");
                let _ = msgbox::create("无效的文件路径", "请确保路径是指向一个存在的文件，而非一个路径或不存在或无效的文件", msgbox::IconType::Error);
                return;
            }
            log::info!("路径检查通过，开始解包");

            let selected_package_path_str = uimainwindow_weak.get_package_selected_path_text().to_string();
            let selected_package_extension = crate::tools::get_extension_from_filename(&selected_package_path_str.as_str()).unwrap();
            let mut selected_package_metadata_path_str = "";
            let mut can_countine = false;

            if selected_package_extension != "ipa" &&  selected_package_extension != "apk"{
                msgbox::create("解包出错", ("不支持此扩展名：".to_string() + selected_package_extension).as_str(), msgbox::IconType::Error).unwrap();
            }
            else if selected_package_extension == "ipa"{
                crate::ziptool::depackzip(&selected_package_path_str, "cache/ipaUnZip").unwrap();
                selected_package_metadata_path_str = "cache/ipaUnZip/Payload/Rizline.app/Data/Managed/Metadata/global-metadata.dat";
                can_countine = true;
            }
            else if selected_package_extension == "apk"{
                if !crate::android_check::check_jvm(){
                    msgbox::create("Java错误", "在程序启动时我们已经提醒过您了，在未安装Java或未将Java添加到环境变量的情况下无法实现安卓包体相关功能，而你选择了一个apk文件，因此不可能继续往下进行操作。\n如果你已经安装了Java8或更高版本，却还是出现此报错，请尝试重启电脑。", msgbox::IconType::Error).unwrap();
                }
                else{
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

                    let mut cmd = Command::new("java");
                    log::info!("目标APK路径: {}",("".to_string() + &selected_package_path_str.as_str() + "").as_str());
                    cmd.args(["-jar", format!("{}/apktool.jar", currentdir).as_str(), "d", ("".to_string() + &selected_package_path_str.as_str() + "").as_str(), "-o", "cache/ApkDecoding", "-f"]);
                    let output = cmd.output().expect("Failed to execute command");
                    if output.status.success() {
                        let output_str = String::from_utf8_lossy(&output.stderr).to_string();
                        log::info!("状态success，尝试开始下一步");
                        log::info!("APK解包完毕");
                        can_countine = true;
                        selected_package_metadata_path_str = "cache/ApkDecoding/assets/bin/Data/Managed/Metadata/global-metadata.dat";
                    } else {
                        let output_str = String::from_utf8_lossy(&output.stderr).to_string();
                        log::error!("在解包apk时遇到问题，控制台输出：{}", &output_str);
                        msgbox::create("错误",("在解包apk时遇到问题，控制台输出：".to_string() + &output_str).as_str() ,msgbox::IconType::Error).unwrap();
                    }
                }
            }

            if !can_countine{
                uimainwindow_weak.set_showSearchingTips_PackageSelectUIGroup(false);
            }

            if can_countine {
                let target_strings_json: crate::structs::TargetStrings = serde_json::from_str(std::fs::read_to_string("./target_strings.json").unwrap().as_str()).unwrap();

                let thread_handle = async {
                    let need_contains: Vec<String> = vec![
                        target_strings_json.area_test_target,
                        target_strings_json.area_verify_target,
                        target_strings_json.aes256_key_target,
                        target_strings_json.aes256_iv_target,
                        target_strings_json.server_host_target,
                        target_strings_json.game_config_address_target,
                        target_strings_json.xsolla_purchase_address_target,
                        target_strings_json.rsa_public_key_target
                    ];

                    let strings_test_meta = crate::metadata_tools::read_strings_from_file(selected_package_metadata_path_str);

                    std::fs::write("check_ok.cache", "ok").unwrap();

                    for cont_str in &need_contains{
                        if !(crate::metadata_tools::contains_string(&strings_test_meta, cont_str.to_string())){
                            std::fs::remove_file("check_ok.cache").unwrap();
                            std::fs::write("check_error.cache", "error: something not found").unwrap();
                            log::warn!("对应项 {} 未能找到", &cont_str);
                            let _ = msgbox::create("无法在metadata中找到对应项", ("无法在metadata中找到应有项 \"".to_string() + &cont_str + "\" 可能是target_strings.json的内容不正确或你正在使用过时版本的游戏包体，又或是游戏文件被加密").as_str(), msgbox::IconType::Error).unwrap();
                            break;
                        }
                        else{
                            log::info!("对应项 {} 已成功找到", &cont_str);
                        }
                    }
                };

                tokio::spawn(thread_handle);

                log::info!("Metadata搜索线程已创建");
            }
        }
    });

    uimainwindow.on_testbtn_logall_strings_click(move || {
        msgbox::create("废弃功能", "此功能已废弃", msgbox::IconType::Info).unwrap();
        /*
        let strings = crate::metadata_tools::read_strings_from_file(filename);
        let s = format!("{:?}", strings);
        log::info!("{}", s);
        */
    });

    log::info!("尝试进入循环检查");

    uimainwindow_weak = uimainwindow.as_weak().unwrap();

    let uimainwindow_weak_box = Box::new(uimainwindow_weak);

    let uimainwindow_weak_ptr = Box::leak(uimainwindow_weak_box);

    let uimainwindow_weak_ptr = uimainwindow_weak_ptr as *mut UIMainWindow;
    // 将裸指针转换为整数值
    let uimainwindow_weak_addr = uimainwindow_weak_ptr as usize;

    log::info!(
        "已获取UIMainWindow_Weak的内存地址：{}",
        uimainwindow_weak_addr
    );

    tokio::spawn(loop_check_fn(uimainwindow_weak_addr));

    log::info!("LoopCheck线程已创建");

    log::info!("尝试显示UI");
    uimainwindow.run().unwrap(); //运行UI
}

pub async fn loop_check_fn(uimainwindow_weak_addr: usize) {
    // 使用 move 关键字
    log::info!("开始循环检查");
    loop {
        sleep(Duration::from_millis(1000)).await;
        //log::info!("Try Check");
        if (Path::new("check_error.cache")).exists() && (Path::new("check_error.cache")).is_file() {
            std::process::exit(114514);
        }
        if (Path::new("check_ok.cache")).exists() && (Path::new("check_ok.cache")).is_file() {
            std::fs::remove_file("check_ok.cache").unwrap();
            break;
        }
    }

    unsafe {
        // 在函数中使用 slice_from_raw_parts 将内存地址转换为切片
        let uimainwindow_weak_slice = std::slice::from_raw_parts_mut(
            uimainwindow_weak_addr as *mut UIMainWindow,
            std::mem::size_of::<UIMainWindow>(),
        );

        // 使用 as_mut_ptr 转换为可变指针
        let uimainwindow_weak_ptr = uimainwindow_weak_slice.as_mut_ptr();

        // 使用 as_mut 转换为可变引用
        let uimainwindow_weak_ref = uimainwindow_weak_ptr.as_mut().unwrap();

        uimainwindow_weak_ref.set_show_package_select_ui_group(false); // 使用引用
        uimainwindow_weak_ref.set_show_edit_package_ui_group(true);

        let target_strings_json: crate::structs::TargetStrings = serde_json::from_str(
            std::fs::read_to_string("./target_strings.json")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        uimainwindow_weak_ref
            .set_rsa_public_key_text(target_strings_json.rsa_public_key_target.into());
        uimainwindow_weak_ref.set_area_test_text(target_strings_json.area_test_target.into());
        uimainwindow_weak_ref.set_area_verify_text(target_strings_json.area_verify_target.into());
        uimainwindow_weak_ref.set_aes256_key_text(target_strings_json.aes256_key_target.into());
        uimainwindow_weak_ref.set_aes256_iv_text(target_strings_json.aes256_iv_target.into());
        uimainwindow_weak_ref
            .set_game_config_address_text(target_strings_json.game_config_address_target.into());
        uimainwindow_weak_ref.set_server_host_text(target_strings_json.server_host_target.into());
        uimainwindow_weak_ref
            .set_xsolla_purchase_address(target_strings_json.xsolla_purchase_address_target.into());
    }
}
