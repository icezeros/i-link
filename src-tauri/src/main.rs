#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod link;

use glob::glob;
// use std::fs;
// use std::path::Path;
use std::process::Command;

// use regex::Regex;
// use std::process::Command;

// // 递归地获取指定路径下的所有文件，包括子文件
// fn get_all_files(path: &Path, files: &mut Vec<String>) -> io::Result<()> {
//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_dir() {
//             get_all_files(&path, files)?;
//         } else {
//             files.push(path.to_str().unwrap().to_string());
//         }
//     }
//     Ok(())
// }

// use std::fs::{self, DirEntry};

use std::fs::{self, hard_link, soft_link};
use std::io;
use std::path::Path;

fn link_files(source_path: &str, target_path: &str) {
    // 如果目标目录不存在，则创建之
    if !Path::new(target_path).exists() {
        fs::create_dir_all(target_path).expect("无法创建目标目录！");
    }

    // 遍历源目录中的所有文件和文件夹
    let paths = fs::read_dir(source_path).expect("无法读取源目录！");
    for path in paths {
        let source = path.expect("无法获取源目录路径！").path();
        let target_name = source.file_name().expect("文件名无效！").to_str().unwrap();

        let mut target = Path::new(target_path).join(target_name);

        // 如果该文件已经存在于目标目录中，则不需要再次链接它
        if !target.exists() {
            // 如果该路径指向一个文件夹，则递归调用此函数
            if source.is_dir() {
                link_files(source.to_str().unwrap(), target.to_str().unwrap());
            } else {
                // 创建硬链接
                hard_link(&source, &target).expect("无法链接文件！");
            }
        }
    }
}

fn soft_link_all_files(source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    // 如果目标路径不存在，则创建之
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }

    // 遍历源文件夹状态下所有的条目
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?; // 解取结果
        let source_path = entry.path(); // 获取原始路径

        if source_path.is_dir() {
            // 如果这是一个文件夹，递归调用此函数以处理其中的所有文件。
            let dest_subdir = dest_dir.join(entry.file_name());
            soft_link_all_files(&source_path, &dest_subdir)?;
        } else if source_path.is_file() {
            // 如果这是一个文件，请在目标文件夹中创建符号链接
            let dest_path = dest_dir.join(entry.file_name());
            // 使用soft-link来部署源文件到目标目录

            soft_link(&source_path, dest_path)?;
            // #[cfg(target_family = "unix")]
            // std::os::unix::fs::symlink(&source_path, dest_path)?;
            // #[cfg(target_family = "windows")]
            // std::os::windows::fs::symlink_file(&source_path, dest_path)?;
        }
    }
    Ok(())
}

// fn main() { get_all_files("./"); }
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // format!("Hello, {}! You've been greeted from Rust!", name)
    // println!("In file {}", file_path);

    for entry in
        // glob("/Users/huguosen/workspace/ice/tauri-app/**").expect("Failed to read glob pattern")
        glob(name).expect("Failed to read glob pattern")
    {
        // println!("{}", entry);
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

    // let output = Command::new("git").arg("log").arg("--oneline").output()?;
    // ----------------------------------------------------------------
    let source_dir = "/Users/huguosen/workspace/github/ngx-styx/dist/styx";
    let dest_dir = "/Users/huguosen/workspace/ice/tauri-app/node_modules/.tmp/ngx-styx";

    link::hard_link_files(source_dir, dest_dir);

    // let soft_source_dir = "/Users/huguosen/workspace/ice/tauri-app/node_modules/.tmp";
    // let soft_dest_dir = "/Users/huguosen/workspace/ice/tauri-app/node_modules/.tmp2/ngx-styx";
    // soft_link_all_files(soft_source_dir, soft_dest_dir);

    let source_dir = "/Users/huguosen/workspace/ice/tauri-app/node_modules/.tmp/ngx-styx";
    let dest_dir = "/Users/huguosen/workspace/ice/tauri-app/node_modules/.tmp2/ngx-styx";
    link::soft_link_dir(source_dir, dest_dir);
    // if let Err(e) = soft_link_all_files(source_dir, dest_dir) {
    //     panic!("Error occurred while soft-linking files: {}", e);
    // }

    // link::is_symlink_dir(dest_dir, source_dir);

    if link::is_symlink_dir(dest_dir, source_dir) {
        println!("{} is a symlink to {}", dest_dir, source_dir);
    } else {
        println!("{} is not a symlink to {}", dest_dir, source_dir);
    }

    // ----------------------------------------------------------------

    // String::from_utf8(output.stdout)?
    //     .lines()
    //     .for_each(|x| println!("{:?}", x));
    // println!("{:?}", String::from_utf8(output.stdout?));

    let contents = fs::read_to_string("/Users/huguosen/workspace/ice/tauri-app/package.json")
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    contents
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
