use std::alloc::Global;
use std::collections::VecDeque;
use std::io::{BufReader, BufWriter, Read, Write, Seek};
use std::fs::File;

pub struct MetadataFile {
    pub(crate) reader: BufReader<File>,
    pub(crate) string_literal_offset: u32,
    pub(crate) string_literal_count: u32,
    pub(crate) data_info_position: u64,
    pub(crate) string_literal_data_offset: u32,
    pub(crate) string_literal_data_count: u32,
    pub(crate) string_literals: Vec<StringLiteral>,
    pub(crate) str_bytes: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StringLiteral {
    pub(crate) length: u32,
    pub(crate) offset: u32,
}

impl MetadataFile {
    fn new(full_name: &str) -> std::io::Result<Self> {
        let reader = BufReader::new(File::open(full_name)?);
        let mut metadata_file = MetadataFile {
            reader,
            string_literal_offset: 0,
            string_literal_count: 0,
            data_info_position: 0,
            string_literal_data_offset: 0,
            string_literal_data_count: 0,
            string_literals: Vec::new(),
            str_bytes: Vec::new(),
        };
        metadata_file.read_header();
        metadata_file.read_literal();
        metadata_file.read_str_byte();
        println!("基础读取完成");
        Ok(metadata_file)
    }

    pub fn read_header(&mut self) {
        println!("读取头部");
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).unwrap();
        let vansity = u32::from_le_bytes(buf);
        if vansity != 0xFAB11BAF {
            panic!("标志检查不通过");
        }
        self.reader.read_exact(&mut buf).unwrap();
        let _version = i32::from_le_bytes(buf);
        self.reader.read_exact(&mut buf).unwrap();
        self.string_literal_offset = u32::from_le_bytes(buf);
        self.reader.read_exact(&mut buf).unwrap();
        self.string_literal_count = u32::from_le_bytes(buf);
        self.data_info_position = self.reader.stream_position().unwrap();
        self.reader.read_exact(&mut buf).unwrap();
        self.string_literal_data_offset = u32::from_le_bytes(buf);
        self.reader.read_exact(&mut buf).unwrap();
        self.string_literal_data_count = u32::from_le_bytes(buf);
    }

    pub fn read_literal(&mut self) {
        println!("读取Literal");
        // ProgressBar.SetMax((int)stringLiteralCount/8);
        self.reader
            .seek(std::io::SeekFrom::Start(self.string_literal_offset as u64))
            .unwrap();
        for _ in 0..self.string_literal_count / 8 {
            let mut buf = [0u8; 4];
            self.reader.read_exact(&mut buf).unwrap();
            let length = u32::from_le_bytes(buf);
            self.reader.read_exact(&mut buf).unwrap();
            let offset = u32::from_le_bytes(buf);
            self.string_literals.push(StringLiteral { length, offset });
            // ProgressBar.Report();
        }
    }

    pub fn read_str_byte(&mut self) {
        println!("读取字符串的Bytes");
        // ProgressBar.SetMax(stringLiterals.Count);
        for i in 0..self.string_literals.len() {
            self.reader
                .seek(std::io::SeekFrom::Start(
                    (self.string_literal_data_offset + self.string_literals[i].offset) as u64,
                ))
                .unwrap();
            let mut buf = vec![0u8; self.string_literals[i].length as usize];
            self.reader.read_exact(&mut buf).unwrap();
            self.str_bytes.push(buf);
            // ProgressBar.Report();
        }
    }

    pub fn write_to_new_file(&mut self, file_name: &str) {
        let mut writer = BufWriter::new(File::create(file_name).unwrap());

        // 先全部复制过去
        let mut reader_copy = self.reader.get_ref().try_clone().unwrap();
        reader_copy.seek(std::io::SeekFrom::Start(0)).unwrap();
        std::io::copy(&mut reader_copy, &mut writer).unwrap();

        //更新Literal
        println!("更新Literal");
        // ProgressBar.SetMax(stringLiterals.Count);
        writer
            .seek(std::io::SeekFrom::Start(self.string_literal_offset as u64))
            .unwrap();
        let mut count = 0;
        for i in 0..self.string_literals.len() {
            let mut literal = self.string_literals[i];
            literal.offset = count;
            literal.length = self.str_bytes[i].len() as u32;
            writer.write_all(&literal.length.to_le_bytes()).unwrap();
            writer.write_all(&literal.offset.to_le_bytes()).unwrap();
            count += literal.length;
            // ProgressBar.Report();
        }
        //进行一次对齐，不确定是否一定需要，但是Unity是做了，所以还是补上为好
        let tmp = (self.string_literal_data_offset + count) % 4;
        if tmp != 0 {
            count += 4 - tmp;
        }

        // 检查是否够空间放置
        if count > self.string_literal_data_count {
            // 检查数据区后面还有没有别的数据，没有就可以直接延长数据区
            if self.string_literal_data_offset + self.string_literal_data_count < writer.stream_position().unwrap() as u32 {
                // 原有空间不够放，也不能直接延长，所以整体挪到文件尾
                let mut reader_copy = self.reader.get_ref().try_clone().unwrap();
                reader_copy.seek(std::io::SeekFrom::Start(
                    (self.string_literal_data_offset + self.string_literal_data_count) as u64,
                )).unwrap();
                let mut queue = VecDeque::new();
                loop {
                    let mut buf = [0u8; 4096];
                    let len = reader_copy.read(&mut buf).unwrap();
                    if len == 0 {
                        break;
                    }
                    queue.extend(buf[..len].iter().copied());
                }
                writer.seek(std::io::SeekFrom::End(0)).unwrap();
                while let Some(byte) = queue.pop_front() {
                    writer.write_all(&[byte]).unwrap();
                }
                self.string_literal_data_offset = writer.stream_position().unwrap() as u32;
            }
        }
        self.string_literal_data_count = count;

        //写入string
        println!("更新String");
        // ProgressBar.SetMax(strBytes.Count);
        writer
            .seek(std::io::SeekFrom::Start(self.string_literal_data_offset as u64))
            .unwrap();
        for i in 0..self.str_bytes.len() {
            writer.write_all(&self.str_bytes[i]).unwrap();
            // ProgressBar.Report();
        }

        //更新头部
        println!("更新头部");
        writer
            .seek(std::io::SeekFrom::Start(self.data_info_position))
            .unwrap();
        writer
            .write_all(&self.string_literal_data_offset.to_le_bytes())
            .unwrap();
        writer
            .write_all(&self.string_literal_data_count.to_le_bytes())
            .unwrap();
        println!("更新完成");
    }
}

pub fn get_metadata_impl(file_name: &str) -> MetadataFile {
    // 创建一个MetadataFile对象，调用new方法打开文件
    let mut metadata_file = MetadataFile::new(file_name).unwrap();

    metadata_file
}

// 从文件中读取字符串，返回一个字符串向量
pub fn read_strings_from_file(metadata_file: &mut MetadataFile) -> Vec<String> {
    // 创建一个空的字符串向量
    let mut strings = Vec::new();
    // 遍历元数据文件中的字符串字节向量
    for str_bytes in &metadata_file.str_bytes {
        // 将字节向量转换为字符串，如果转换失败，返回错误信息
        let string = String::from_utf8(str_bytes.to_vec()).unwrap_or_else(|e| format!("Invalid UTF-8 sequence: {}", e));
        // 将字符串添加到字符串向量中
        strings.push(string);
    }

    // 返回字符串向量
    return strings;
}

// 将字符串向量写入文件
pub fn write_strings_to_file(file_name: &str, metadata_file: &mut MetadataFile ,strings: Vec<String>) {

    metadata_file.str_bytes.clear();

    for string in &strings {
        // 将字符串转换为字节向量，如果转换失败，返回错误信息
        let str_bytes = string.to_owned().into_bytes();
        metadata_file.str_bytes.push(str_bytes);
    }

    metadata_file.write_to_new_file(file_name);
}


pub fn convert_vecstring_to_vecglobalvecu8global(v: Vec<String>) -> Vec<Vec<u8, Global>, Global> {
    let (ptr, len, cap) = v.into_raw_parts();
    let mut u: Vec<Vec<u8, Global>, Global> = Vec::with_capacity_in(cap, Global);
    for i in 0..len {
        let s = unsafe { ptr.add(i).read() };
        let b = s.into_bytes();
        u.push(b);
    }
    unsafe { std::ptr::drop_in_place(ptr) };
    u
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

pub fn replace_strings(vec: &mut Vec<String>, old: &str, new: &str) {
    // 遍历Vec中的每个String
    for s in vec.iter_mut() {
        // 如果String包含old，就用new替换它
        *s = s.replace(old, new);
    }
}