//! Для работы этого кода потребуется:
//! - файл `test_file.txt` в директории с package.
//! - директория `test_dir` в директории с package. Желательно, с содержимым.

use std::path::Path;
use std::{fs, io};

fn main() {
    // Копируем тестовый файл.
    copy_file();

    // Удаляем копию.
    remove_file();

    // Копируем тестовую директорию.
    copy_dir_all("./test_dir", "./test_dir_copy").unwrap();

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
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // Создаём целевую директорию.
    fs::create_dir_all(&dst)?;

    // Проходим по содержимому оригинальной директории.
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            // Если попалась директория, вызываем copy_dir_all уже для неё.
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            // Если не директорая (файл или symlink), то копируем объект.
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
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
