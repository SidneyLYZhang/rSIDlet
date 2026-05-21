use std::io;
use std::path::Path;

use crate::download;
use crate::paths;

/// 执行 `--install` 命令
///
/// 从 GitHub 下载指定字体文件并安装到字体目录。
pub fn run(font_name: Option<&str>, extra_dir: Option<&Path>) -> io::Result<()> {
    let name = font_name.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "请指定要安装的字体名称")
    })?;

    let dirs = paths::font_search_paths(extra_dir);

    // 检查字体是否已安装
    if let Some(existing) = paths::find_font_file(name, &dirs) {
        println!("字体已安装: {}", existing.display());
        return Ok(());
    }

    // 确定目标安装目录
    let dest_dir = if let Some(dir) = extra_dir {
        dir.to_path_buf()
    } else if let Some(ext) = paths::extended_font_dir() {
        ext
    } else if let Some(builtin) = paths::builtin_font_dir() {
        builtin
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "没有可用的字体安装目录",
        ));
    };

    println!("正在下载字体: {} ...", name);
    match download::download_font(name, &dest_dir) {
        Ok(result) => {
            println!("安装成功: {}", result.file_path.display());
            println!("来源: {}", result.source_url);
        }
        Err(e) => {
            eprintln!("下载失败: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
