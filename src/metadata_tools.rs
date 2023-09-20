// 引入标准库中的文件操作模块
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

// 定义一个结构体，用于存储字符串的信息
#[derive(Debug, PartialEq, Clone, Copy)]
struct StringInfo {
    // 字符串在数据区的偏移量
    offset: u32,
    // 字符串的长度
    length: u32,
}

pub fn read_strings_from_file(file_name: &str) -> Vec<String> {
    // 创建一个空的字符串向量，用于存储结果
    let mut strings = Vec::new();
    // 打开文件，如果失败则返回错误信息
    let mut file = File::open(file_name).expect("Failed to open file");
    // 跳过文件的前8个字节，因为它们是文件的标识符，不是字符串信息
    file.seek(SeekFrom::Start(8)).expect("Failed to seek to string info");
    // 读取文件的4个字节，作为字符串的数量
    let mut num_bytes = [0; 4];
    file.read_exact(&mut num_bytes).expect("Failed to read number of strings");
    // 将字节数组转换为无符号32位整数
    let num_strings = u32::from_le_bytes(num_bytes);
    // 创建一个空的字符串信息向量，用于存储每个字符串的信息
    let mut string_infos = Vec::new();
    // 遍历每个字符串
    for _ in 0..num_strings {
        // 读取文件的8个字节，作为字符串的偏移量和长度
        let mut info_bytes = [0; 8];
        file.read_exact(&mut info_bytes).expect("Failed to read string info");
        // 将字节数组切片为两部分，分别转换为无符号32位整数
        let offset = u32::from_le_bytes(info_bytes[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(info_bytes[4..8].try_into().unwrap());
        // 创建一个字符串信息结构体，并添加到向量中
        let string_info = StringInfo { offset, length };
        string_infos.push(string_info);
    }
    // 遍历每个字符串信息
    for string_info in string_infos {
        // 根据偏移量和长度，从文件中读取对应的字节
        let mut string_bytes = vec![0; string_info.length as usize];
        file.seek(SeekFrom::Start(string_info.offset as u64))
            .expect("Failed to seek to string offset");
        let readexact_match = file.read_exact(&mut string_bytes);
        match readexact_match{
            Err(err) => {
                log::warn!("读取metadata中的指定字节时遭遇非致命错误：{}",err.to_string());
            }
            Ok(_) => {},
        }
        // 将字节向量转换为字符串，并添加到结果向量中
        let string = String::from_utf8_lossy(&string_bytes).to_string();
        /* 
        match string{
            Ok(string) => {
                strings.push(string);
            }
            Err(err) => {
                log::warn!("读取metadata中的字符串时遭遇非致命错误：{}",err.to_string());
            }
        }
        */
        strings.push(string);
    }
    // 返回结果向量
    strings
}

pub fn write_strings_to_file(file_name: &str, strings: Vec<String>) {
    // 打开文件，如果失败则返回错误信息
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_name)
        .expect("Failed to open file");
    // 获取文件的元数据，如果失败则返回错误信息
    let metadata = file.metadata().expect("Failed to get file metadata");
    // 获取文件的长度
    let file_len = metadata.len();
    // 跳过文件的前8个字节，因为它们是文件的标识符，不是字符串信息
    file.seek(SeekFrom::Start(8)).expect("Failed to seek to string info");
    // 读取文件的4个字节，作为字符串的数量
    let mut num_bytes = [0; 4];
    file.read_exact(&mut num_bytes).expect("Failed to read number of strings");
    // 将字节数组转换为无符号32位整数，并与输入的字符串向量的长度比较，如果不相等则返回错误信息
    let num_strings = u32::from_le_bytes(num_bytes);
    if num_strings != strings.len() as u32 {
        panic!("The number of input strings does not match the number of strings in the file");
    }
    // 创建一个空的字符串信息向量，用于存储每个字符串的信息
    let mut string_infos = Vec::new();
    // 遍历每个字符串
    for _ in 0..num_strings {
        // 读取文件的8个字节，作为字符串的偏移量和长度
        let mut info_bytes = [0; 8];
        file.read_exact(&mut info_bytes).expect("Failed to read string info");
        // 将字节数组切片为两部分，分别转换为无符号32位整数
        let offset = u32::from_le_bytes(info_bytes[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(info_bytes[4..8].try_into().unwrap());
        // 创建一个字符串信息结构体，并添加到向量中
        let string_info = StringInfo { offset, length };
        string_infos.push(string_info);
    }
    // 计算数据区的起始位置和结束位置
    let data_start = 12 + num_strings * 8;
    let data_end = string_infos.last().unwrap().offset + string_infos.last().unwrap().length;
    // 计算数据区的原始长度和修改后的长度
    let data_len = data_end - data_start;
    let new_data_len: u32 = strings.iter().map(|s| s.len() as u32).sum();
    // 如果修改后的长度小于等于原始长度，则直接覆盖写入
    if new_data_len <= data_len {
        // 遍历每个字符串和对应的信息
        for (string, string_info) in strings.iter().zip(string_infos.iter()) {
            // 将字符串转换为字节向量，并获取其长度
            let string_bytes = string.as_bytes();
            let string_len = string_bytes.len() as u32;
            // 根据偏移量，将字节向量写入文件
            file.seek(SeekFrom::Start(string_info.offset as u64))
                .expect("Failed to seek to string offset");
            file.write_all(string_bytes)
                .expect("Failed to write string bytes");
            // 更新字符串信息中的长度，并将其转换为字节数组
            let mut new_string_info = *string_info;
            new_string_info.length = string_len;
            let new_info_bytes = new_string_info.offset.to_le_bytes()
                .iter()
                .chain(new_string_info.length.to_le_bytes().iter())
                .copied()
                .collect::<Vec<u8>>();
            // 根据索引，将字节数组写入文件
            let index = 12 + (string_infos.iter().position(|s| s == string_info).unwrap() as u32) * 8;
            file.seek(SeekFrom::Start(index as u64))
                .expect("Failed to seek to string info");
            file.write_all(&new_info_bytes)
                .expect("Failed to write string info");
        }
    } else {
        // 如果修改后的长度大于原始长度，则写入到文件尾
        // 遍历每个字符串和对应的信息
        for (string, string_info) in strings.iter().zip(string_infos.iter()) {
            // 将字符串转换为字节向量，并获取其长度
            let string_bytes = string.as_bytes();
            let string_len = string_bytes.len() as u32;
            // 将文件指针移动到文件尾，并获取其位置，作为新的偏移量
            let new_offset = file.seek(SeekFrom::End(0)).expect("Failed to seek to end of file");
            // 将字节向量写入文件尾
            file.write_all(string_bytes)
                .expect("Failed to write string bytes");
            // 更新字符串信息中的偏移量和长度，并将其转换为字节数组
            let mut new_string_info = *string_info;
            new_string_info.offset = new_offset as u32;
            new_string_info.length = string_len;
            let new_info_bytes = new_string_info.offset.to_le_bytes()
                .iter()
                .chain(new_string_info.length.to_le_bytes().iter())
                .copied()
                .collect::<Vec<u8>>();
            // 根据索引，将字节数组写入文件
            let index = 12 + (string_infos.iter().position(|s| s == string_info).unwrap() as u32) * 8;
            file.seek(SeekFrom::Start(index as u64))
                .expect("Failed to seek to string info");
            file.write_all(&new_info_bytes)
                .expect("Failed to write string info");
        }
    }
}


// 定义一个函数，接受一个Vec<String>和两个String作为参数
pub fn replace_strings(vec: &mut Vec<String>, old: &str, new: &str) {
    // 遍历Vec中的每个String
    for s in vec.iter_mut() {
        // 如果String包含old，就用new替换它
        *s = s.replace(old, new);
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