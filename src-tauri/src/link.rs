use std::fs::{self, hard_link, soft_link};
// use std::io;
use std::path::Path;

pub fn hard_link_files(source_path: &str, target_path: &str) {
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
                hard_link_files(source.to_str().unwrap(), target.to_str().unwrap());
            } else {
                // 创建硬链接
                hard_link(&source, &target).expect("无法链接文件！");
            }
        }
    }
}

pub fn soft_link_dir(src_dir: &str, target_dir: &str) -> std::io::Result<()> {
    // 检查创建链接的目标文件夹是否已经存在软链接

    let target_dir_path = std::path::Path::new(target_dir);
    if target_dir_path.exists() && target_dir_path.symlink_metadata()?.file_type().is_symlink() {
        // 如果已存在软链接则删除掉重新创建
        fs::remove_file(target_dir_path)?;
    }

    if !target_dir_path.exists() {
        fs::create_dir_all(target_dir).expect("无法创建目标目录！");
    }

    // soft_link(&src_dir, &target_dir)?;

    // 遍历源文件夹中的所有子文件夹并创建软链接到目标文件夹下面
    for dir_entry in fs::read_dir(src_dir)? {
        let entry = dir_entry?;
        let entry_type = entry.file_type()?;

        // 只对文件夹进行链接操作
        // if entry_type.is_dir() {
        let source_dir_path = entry.path();
        let target_dir_path = std::path::Path::new(&target_dir).join(entry.file_name());
        soft_link(&source_dir_path, &target_dir_path)?;
        // }
    }

    Ok(())
}

pub fn is_symlink_dir(link_path: &str, target_path: &str) -> bool {
    let link_meta = match fs::metadata(link_path) {
        Ok(meta) => meta,
        Err(_) => return false, // Link path not found or not accessible
    };
    println!("result 1 ======");
    println!("{} ========", link_meta.file_type().is_symlink());

    if !link_meta.file_type().is_symlink() {
        // Link path is not a symlink. Return false.
        return false;
    }
    println!("result 2 ======");

    let link_target = match fs::read_link(link_path) {
        Ok(target) => target,
        Err(_) => return false, // Error reading symlink target
    };
    println!("result 3 ======");

    let abs_target_path = Path::new(target_path)
        .canonicalize()
        .unwrap_or_else(|_| panic!("Error getting absolute path for {}", target_path));
    let abs_link_target = link_target.canonicalize().unwrap_or_else(|_| {
        panic!(
            "Error getting absolute path for symlink target: {:?}",
            link_target
        )
    });
    println!("result 4 ======");

    abs_target_path == abs_link_target
}
