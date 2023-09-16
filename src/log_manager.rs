use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn init_log() {
    // 创建一个文件输出器，指定文件名和编码器
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} - {m}{n}")))
        .build("log/output.log")
        .unwrap();

    // 创建一个控制台输出器，指定编码器和过滤器
    let console = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}{n}")))
        .build();

    // 创建一个配置对象，添加两个输出器，并设置根日志器的级别和输出器
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("file").appender("console").build(log::LevelFilter::Trace))
        .unwrap();

    // 初始化log4rs
    log4rs::init_config(config).unwrap();

    // 使用log宏记录日志
    log::info!("日志功能初始化完毕!");
}