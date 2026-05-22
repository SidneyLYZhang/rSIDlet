use std::io::{self, BufRead, Write};

use crate::download;
use crate::figfont::{FigFont, FigletFont};
use crate::paths;
use crate::utils;

/// 执行 `--test` 命令
///
/// 1. 检查字体目录（目录A 和 目录B）是否存在
/// 2. 若有目录缺失，询问用户是否修复
/// 3. 修复完成后（或一切就绪），用 rainbow 模式输出 "It's ready"
pub fn run() -> io::Result<()> {
    let builtin = paths::builtin_font_dir();
    let extended = paths::extended_font_dir();

    let builtin_ok = builtin.as_ref().map(|p| p.exists()).unwrap_or(false);
    let extended_ok = extended.as_ref().map(|p| p.exists()).unwrap_or(false);

    if !builtin_ok || !extended_ok {
        println!("字体目录检查：");
        println!(
            "  目录A (内置): {} [{}]",
            builtin.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "未找到".to_string()),
            if builtin_ok { "存在" } else { "不存在" }
        );
        println!(
            "  目录B (扩展): {} [{}]",
            extended.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "未找到".to_string()),
            if extended_ok { "存在" } else { "不存在" }
        );

        println!();
        print!("是否需要修复安装？（y/n）：");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input == "y" || input == "yes" {
            repair_installation()?;
        } else {
            println!("已取消。");
            return Ok(());
        }
    }

    let font = load_standard_font()?;
    let lines = font.render("It's ready");
    utils::print_colored(&lines, utils::ColorFilter::Rainbow);

    Ok(())
}

/// 修复安装：
/// 1. 确保目录B存在（不存在时自动创建；目录A 仅检查不创建）
/// 2. 检查并下载缺失的必要字体文件到目录B
fn repair_installation() -> io::Result<()> {
    // Step 1: 确保目录B存在
    let dest_dir = paths::ensure_extended_font_dir()?;
    println!("目录B (扩展) 已就绪: {}", dest_dir.display());

    // Step 2: 检查并下载缺失的必要字体文件
    let search_dirs = paths::font_search_paths(None);

    // FIGlet/TOIlet 字体：来自 xero/figlet-fonts 和 PhMajerus/FIGfonts
    let required_flf = [
        ("big.flf", "big.flf"),
        ("future.tlf", "future.tlf"),
        ("standard.flf", "standard.flf"),
        ("phm-shinonome.flf", "phm-shinonome.flf"),
    ];

    for (filename, search_name) in &required_flf {
        if paths::find_font_file(search_name, &search_dirs).is_some() {
            continue;
        }
        println!("正在下载 {} ...", filename);
        match download::download_font(filename, &dest_dir) {
            Ok(result) => println!("  已下载: {}", result.file_path.display()),
            Err(e) => eprintln!("  下载 {} 失败: {}", filename, e),
        }
    }

    // HZK 中文字库：来自 aguegu/BitmapFont
    let required_hzk = ["HZK12", "HZK14", "HZK16"];

    for hzk in &required_hzk {
        if hzk_exists_in_dirs(hzk, &search_dirs) {
            continue;
        }
        println!("正在下载 {} ...", hzk);
        match download::download_file(hzk, &dest_dir) {
            Ok(result) => println!("  已下载: {}", result.file_path.display()),
            Err(e) => eprintln!("  下载 {} 失败: {}", hzk, e),
        }
    }

    println!("修复完成。");
    Ok(())
}

/// 检查 HZK 文件是否在任一搜索目录中存在
fn hzk_exists_in_dirs(name: &str, dirs: &[std::path::PathBuf]) -> bool {
    for dir in dirs {
        if dir.join(name).exists() {
            return true;
        }
    }
    false
}

fn load_standard_font() -> io::Result<FigletFont> {
    let dirs = paths::font_search_paths(None);
    if let Some(path) = paths::find_font_file("standard", &dirs) {
        FigletFont::load(&path)
    } else {
        Ok(FigletFont::builtin())
    }
}
