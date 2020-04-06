use std::io::{Write, Read};

fn main() {
    let filepath = String::from("/Volumes/Macintosh HD/Users/yangjinghua/Downloads/disk_linker/test.PNG");

    let mut file = std::fs::File::open(filepath).unwrap();

    let mut new_file = std::fs::File::create("data.PNG").expect("create failed");

    println!("文件打开成功：{:?}", file);

    let mut buffer = [0u8];

    loop {
        let bytes = file.read(&mut buffer).unwrap();
        new_file.write(&buffer[..bytes]).unwrap();
        if bytes < buffer.len() { break; }
    }
}
