use std::env;
use std::fs;

fn main() {
    let mut source_dir = String::from("/Volumes/Macintosh HD/Users/yangjinghua/Downloads/disk_linker");
    let target_dir = "/Volumes/milk_tea/disk_linker";
    let mut filename = source_dir.push_str("/test.PNG");

    // println!("{}", filename);

    let file = std::fs::File::open("/Volumes/Macintosh HD/Users/yangjinghua/Downloads/disk_linker/test.PNG").unwrap();
    println!("文件打开成功：{:?}", file);
}
