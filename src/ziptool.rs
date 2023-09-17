use std::fs::File;
use std::io::{BufReader, BufWriter};
use zip::{ZipArchive, ZipWriter};

// 定义一个函数用于压缩一个目录中的所有文件和子目录
pub fn packzip(source_dir: &str, zip_file: &str) -> Result<(), zip::result::ZipError> {
    // 打开 ZIP 文件并创建一个缓冲写入器
    let file = File::create(zip_file)?;
    let writer = BufWriter::new(file);

    // 从写入器中创建一个 ZIP 写入对象
    let mut zip = ZipWriter::new(writer);

    // 遍历源目录中的所有文件和子目录
    for entry in walkdir::WalkDir::new(source_dir) {
        // 获取目录项
        let entry = entry.unwrap();

        // 获取文件或目录的路径
        let path = entry.path();

        // 获取文件或目录的名称
        let name = path.strip_prefix(source_dir).unwrap();

        // 判断文件或目录的类型
        if path.is_file() {
            // 如果是文件，就添加到 ZIP 中
            zip.start_file(name.to_string_lossy(), zip::write::FileOptions::default())?;
            let mut file = File::open(path)?;
            std::io::copy(&mut file, &mut zip)?;
        } else if name.as_os_str().len() != 0 {
            // 如果是目录，就添加到 ZIP 中
            zip.add_directory(name.to_string_lossy(), zip::write::FileOptions::default())?;
        }
    }

    // 结束 ZIP 写入并返回结果
    zip.finish()?;
    Ok(())
}

// 定义一个函数用于解压一个 ZIP 文件中的所有文件和目录
pub fn depackzip(zip_file: &str, target_dir: &str) -> Result<(), zip::result::ZipError> {
    // 打开 ZIP 文件并创建一个缓冲读取器
    let file = File::open(zip_file)?;
    let reader = BufReader::new(file);

    // 从读取器中创建一个 ZIP 归档对象
    let mut archive = ZipArchive::new(reader)?;

    archive.extract(target_dir)?;

    // 返回结果
    Ok(())
}