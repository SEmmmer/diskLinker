extern crate crypto;

use std::io::{Write, Read};
use std::fs::File;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let source_file = String::from("test.PNG");
    let target_file = String::from("new.PNG");
    copy_file(source_file, target_file);
}

fn md5_code(mut file: File) {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut hasher = Md5::new();
    hasher.input(&buffer);
    println!("{}", hasher.result_str());
}

fn copy_file(source_file: String, target_file: String) {
    let source_filepath = String::from("/Volumes/Macintosh HD/Users/yangjinghua/Downloads/disk_linker/");
    let target_filepath = String::from("/Volumes/data_disk/disk_linker/");

    let mut old_file = std::fs::File::open(source_filepath + &source_file).unwrap();
    md5_code(old_file);
    let mut new_file = std::fs::File::create(target_filepath + &target_file).expect("create failed");

    let mut buffer = [0u8];

    loop {
        let bytes = old_file.read(&mut buffer).unwrap();
        new_file.write(&buffer[..bytes]).unwrap();
        if bytes < buffer.len() { break; }
    }
}