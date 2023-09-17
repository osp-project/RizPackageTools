use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};

// 一个函数，用于判断一个字节是否是可读的ASCII字符
pub fn is_readable(byte: u8) -> bool {
    byte >= 32 && byte <= 126
}

// 一个函数，用于从一个二进制文件中读取所有的可读字符串，并返回一个字符串列表
pub fn read_strings_from_file(file_name: &str) -> Vec<String> {
    // 打开文件
    let mut file = File::open(file_name).expect("Failed to open file");
    // 创建一个空的字符串列表
    let mut strings = Vec::new();
    // 创建一个空的缓冲区
    let mut buffer = Vec::new();
    // 循环读取文件中的每个字节
    loop {
        // 读取一个字节
        let mut byte = [0; 1];
        let n = file.read(&mut byte).expect("Failed to read byte");
        // 如果读到了文件末尾，退出循环
        if n == 0 {
            break;
        }
        // 如果字节是可读的，将它添加到缓冲区中
        if is_readable(byte[0]) {
            buffer.push(byte[0]);
        } else {
            // 如果字节不可读，检查缓冲区是否为空
            if !buffer.is_empty() {
                // 如果缓冲区不为空，将它转换为字符串，并添加到字符串列表中
                let string = String::from_utf8(buffer).expect("Invalid UTF-8");
                strings.push(string);
                // 清空缓冲区
                buffer = Vec::new();
            }
        }
    }
    // 返回字符串列表
    strings
}

// 一个函数，用于将一个字符串列表写入到一个二进制文件中，替换原有的可读字符串，并保持文件结构不变
pub fn write_strings_to_file(file_name: &str, strings: Vec<String>) {
    // 打开文件
    let mut file = File::open(file_name).expect("Failed to open file");
    // 创建一个字符串列表的迭代器
    let mut iter = strings.iter();
    // 创建一个空的缓冲区
    let mut buffer = Vec::new();
    // 循环读取文件中的每个字节
    loop {
        // 读取一个字节
        let mut byte = [0; 1];
        let n = file.read(&mut byte).expect("Failed to read byte");
        // 如果读到了文件末尾，退出循环
        if n == 0 {
            break;
        }
        // 如果字节是可读的，将它添加到缓冲区中
        if is_readable(byte[0]) {
            buffer.push(byte[0]);
        } else {
            // 如果字节不可读，检查缓冲区是否为空
            if !buffer.is_empty() {
                // 如果缓冲区不为空，从字符串列表中获取下一个字符串，并将其转换为字节数组
                let next_string = iter.next().expect("Not enough strings");
                let next_bytes = next_string.as_bytes();
                // 检查新旧字符串的长度是否相等
                if buffer.len() == next_bytes.len() {
                    // 如果相等，将新字符串写入到文件中，替换原有的字符串
                    file.seek(SeekFrom::Current(-(buffer.len() as i64))).expect("Failed to seek");
                    file.write_all(next_bytes).expect("Failed to write bytes");
                } else {
                    // 如果不相等，报错并退出程序
                    panic!("String length mismatch");
                }
                // 清空缓冲区
                buffer = Vec::new();
            }
            // 将当前的不可读字节写入到文件中
            file.write_all(&byte).expect("Failed to write byte");
        }
    }
}

pub fn contains_string(v: &Vec<String>, s: String) -> bool {
    // 遍历v中的每个字符串
    for string in v {
        // 判断字符串是否包含s，如果是，将true添加到结果列表中，否则，将false添加到结果列表中
        if string.contains(&s) {
            return true;
        }
    }
    // 返回结果列表
    return false;
}