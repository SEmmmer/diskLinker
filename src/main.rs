extern crate crypto;

use std::io::{Write, Read};
use crypto::md5::Md5;
use crypto::digest::Digest;
// use std::fs::File;

fn main() {
    let source_file = String::from("test.PNG");
    let target_file = String::from("test.PNG");
    if same_file(get_source(source_file.clone()), get_target(target_file.clone())) {
        println!("The file already exist.");
    } else {
        copy_file(source_file, target_file);
    }
}

fn md5_code(file: &mut std::fs::File) -> String {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut hasher = Md5::new();
    hasher.input(&buffer);
    hasher.result_str()
}

fn exists(file: String) -> bool {
    std::fs::metadata(file).is_ok()
}

fn same_file(str1: String, str2: String) -> bool {
    let mut file1 = std::fs::File::open(str1).unwrap();
    if !exists(str2.clone()) { false } else {
        let mut file2 = std::fs::File::open(str2).unwrap();
        if md5_code(&mut file1) == md5_code(&mut file2) { true } else { false }
    }
}

fn copy_file(source_file: String, target_file: String) {
    println!("===== copy start =====");

    let source = get_source(source_file);
    let target = get_target(target_file);

    let mut old_file = std::fs::File::open(source).unwrap();
    let mut new_file = std::fs::File::create(target).expect("create failed");

    let mut buffer = [0u8];

    loop {
        let bytes = old_file.read(&mut buffer).unwrap();
        new_file.write(&buffer[..bytes]).unwrap();
        if bytes < buffer.len() {
            break;
        }
    }
    println!("===== copy over =====");
}

fn get_source(source_file: String) -> String {
    let source_filepath = String::from("/Volumes/Macintosh HD/Users/yangjinghua/Downloads/disk_linker/");
    source_filepath + &source_file
}

fn get_target(target_file: String) -> String {
    let target_filepath = String::from("/Volumes/data_disk/disk_linker/");
    target_filepath + &target_file
}