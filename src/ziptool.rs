use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

// 定义一个函数，接受一个路径参数，返回一个Result类型
pub fn packzip(path: &str, output_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个文件对象，以写入和更新模式打开
    let file = File::create(format!("{}", output_filename))?;
    // 创建一个ZipWriter对象，包装文件对象
    let mut zip = ZipWriter::new(file);
    // 定义一个递归函数，用于遍历目录下的所有文件和子目录，并添加到zip中
    fn add_to_zip(zip: &mut ZipWriter<File>, base: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 获取路径对应的元数据
        let metadata = std::fs::metadata(path)?;
        // 如果是目录，则遍历其下的所有条目，并递归调用自身
        if metadata.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                let name = path.to_str().unwrap();
                add_to_zip(zip, base, name)?;
            }
        } else {
            // 如果是文件，则创建一个相对于基础路径的名称，并添加到zip中
            let name = path.strip_prefix(base).unwrap();
            zip.start_file(name.to_string(), FileOptions::default())?;
            // 读取文件内容，并写入到zip中
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
        Ok(())
    }
    // 调用递归函数，传入zip对象，基础路径和目标路径
    add_to_zip(&mut zip, path, path)?;
    // 完成zip写入操作，并返回结果
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