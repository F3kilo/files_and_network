//! Для работы этого кода потребуется:
//! - файл `test_file.txt` в директории с package.
//! - директория `test_dir` в директории с package. Желательно, с содержимым.

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    // Копируем тестовый файл.
    copy_file();

    // Удаляем копию.
    remove_file();

    // Копируем тестовую директорию.
    copy_dir();

    // Удаляем копию.
    remove_dir();

    // Читаем тестовый файл в строку.
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

/// Стандартная библиотека не предоставляет функций для копирования директории.
/// Поэтому, копировать файлы придётся по одному, проходясь по ним в цикле.
fn copy_dir() {
    let path_from = "./test_dir";
    let path_to = "./test_dir_copy";

    // Создаём целевую директорию.
    fs::create_dir(path_to).expect("fail to copy file");

    // Проходим по файлам, которые лежат в исходной директории...
    let dir_entries = fs::read_dir(path_from).expect("fail to read dir");
    for entry in dir_entries {
        let entry = entry.expect("failed to get dir entry");

        // ... если это файл, то копируем его в целевую директорию.
        let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or_default();
        if !is_file {
            continue;
        }

        let file_name = entry.file_name();

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
