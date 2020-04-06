use std::io::{Write, Read};

fn main() {
    let source_filepath = String::from("/Volumes/Macintosh HD/Users/yangjinghua/Downloads/disk_linker/");
    let target_filepath = String::from("/Volumes/milk_tea/disk_linker/");

    let source_file = String::from("test.PNG");
    let target_file = String::from("new.PNG");

    let mut file = std::fs::File::open(source_filepath + &source_file).unwrap();

    let mut new_file = std::fs::File::create(target_filepath + &target_file).expect("create failed");

    let mut buffer = [0u8];

    loop {
        let bytes = file.read(&mut buffer).unwrap();
        new_file.write(&buffer[..bytes]).unwrap();
        if bytes < buffer.len() { break; }
    }
}
