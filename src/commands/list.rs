use std::io;

use crate::download;
use crate::paths;
use crate::utils;

/// 执行 `--list` 命令
///
/// 支持子命令：
/// - `font`：列出系统已安装的文字字体
/// - `colormap`：列出可用的颜色遮蔽名称
/// - `installed`：列出当前目录下已有的字体文件名
/// - `letters`：列出可下载但未安装的字体文件名
pub fn run(subcommand: &str, extra_dir: Option<&std::path::Path>) -> io::Result<()> {
    match subcommand {
        "font" => list_system_fonts(extra_dir),
        "colormap" => list_colormaps(),
        "installed" => list_installed(extra_dir),
        "letters" => list_downloadable(extra_dir),
        _ => {
            eprintln!("未知的列表类型: {}", subcommand);
            eprintln!("可用选项: font, colormap, installed, letters");
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("未知的列表类型: {}", subcommand),
            ))
        }
    }
}

/// 列出系统已安装的矢量字体（TTF/OTF）
fn list_system_fonts(_extra_dir: Option<&std::path::Path>) -> io::Result<()> {
    let files = paths::list_system_ttf_files();

    if files.is_empty() {
        println!("未找到任何系统字体。");
        return Ok(());
    }

    println!("系统字体（TTF/OTF）：");
    for (i, f) in files.iter().enumerate() {
        println!("  {}. {}", i + 1, f);
    }
    println!("共 {} 个字体", files.len());

    Ok(())
}

/// 列出可用的颜色蒙版名称（统一画布滤镜与颜色遮蔽）
fn list_colormaps() -> io::Result<()> {
    let maps = utils::ColorMap::available();

    let canvas_filters: Vec<_> = maps.iter().filter(|(_, _, cat)| *cat == "画布滤镜").collect();
    let color_masks: Vec<_> = maps.iter().filter(|(_, _, cat)| *cat == "颜色遮蔽").collect();

    println!("可用的颜色蒙版（画布滤镜 + 颜色遮蔽）：\n");

    if !canvas_filters.is_empty() {
        println!("【画布滤镜】（名称冲突时优先级更高）");
        for (i, (name, _, _)) in canvas_filters.iter().enumerate() {
            println!("  {}. {}", i + 1, name);
        }
        println!();
    }

    if !color_masks.is_empty() {
        println!("【颜色遮蔽】");
        for (i, (name, _, _)) in color_masks.iter().enumerate() {
            println!("  {}. {}", i + 1, name);
        }
    }

    println!(
        "\n共 {} 个可用选项（{} 个画布滤镜 + {} 个颜色遮蔽）",
        maps.len(),
        canvas_filters.len(),
        color_masks.len()
    );
    Ok(())
}

/// 列出当前目录下已有的字体文件名
fn list_installed(extra_dir: Option<&std::path::Path>) -> io::Result<()> {
    let dirs = paths::font_search_paths(extra_dir);

    for dir in &dirs {
        if dir.exists() {
            println!("目录: {}", dir.display());
            let files = paths::list_flf_files(&[dir.clone()]);
            if files.is_empty() {
                println!("  （无字体文件）");
            } else {
                for f in &files {
                    println!("  {}", f);
                }
            }
            println!();
        }
    }

    Ok(())
}

/// 列出可下载但未安装的字体
fn list_downloadable(extra_dir: Option<&std::path::Path>) -> io::Result<()> {
    // 获取在线可用字体列表
    let online = match download::list_available_online() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("获取在线字体列表失败: {}", e);
            return Err(e);
        }
    };

    // 获取本地已安装字体
    let dirs = paths::font_search_paths(extra_dir);
    let local = paths::list_flf_files(&dirs);
    let local_lower: Vec<String> = local.iter().map(|s| s.to_lowercase()).collect();

    // 筛选未安装的字体
    let downloadable: Vec<&String> = online
        .iter()
        .filter(|name| {
            let lower = name.to_lowercase();
            !local_lower.contains(&lower)
        })
        .collect();

    if downloadable.is_empty() {
        println!("所有在线字体均已安装。");
    } else {
        // 用 `;` 横向连接输出
        let names: Vec<String> = downloadable.iter().map(|s| s.to_string()).collect();
        println!("{}", names.join(";"));
    }

    Ok(())
}
