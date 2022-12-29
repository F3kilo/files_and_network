use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    copy_file();
    remove_file();
    copy_dir();
    remove_dir();
    read_file();
}

fn copy_file() {
    let path_from = "./test_file.txt";
    let path_to = "./test_file_copy.txt";
    fs::copy(path_from, path_to).expect("fail to copy file");
}

fn remove_file() {
    let path = "./test_file_copy.txt";
    fs::remove_file(path).expect("fail to remove file");
}

fn copy_dir() {
    let path_from = "./test_dir";
    let path_to = "./test_dir_copy";
    fs::create_dir(path_to).expect("fail to copy file");

    let dir_entries = fs::read_dir(path_from).expect("fail to read dir");
    for entry in dir_entries {
        let file_name = entry.expect("failed to get dir entry").file_name();

        let mut path_from = PathBuf::from_str(path_from).expect("bad path");
        path_from.push(file_name.clone());

        let mut path_to = PathBuf::from_str(path_to).expect("bad path");
        path_to.push(file_name);

        fs::copy(path_from, path_to).expect("fail to copy file");
    }
}

fn remove_dir() {
    let path = "./test_dir_copy";
    fs::remove_dir_all(path).expect("fail to remove directory");
}

fn read_file() {
    let path = "./test_file.txt";
    let content = fs::read_to_string(path).expect("fail to read file");
    println!("file content: {content}");
}