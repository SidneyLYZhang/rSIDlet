use std::io::{self, BufRead, Write};

use crate::download;
use crate::figfont::{FigFont, FigletFont};
use crate::paths;
use crate::utils;

/// 执行 `--test` 命令
///
/// 1. 检查内置字体目录是否存在
/// 2. 问用户是否需要修复
/// 3. 若一切就绪，用 rainbow 模式输出 "It's ready"
pub fn run() -> io::Result<()> {
    let builtin = paths::builtin_font_dir();
    let extended = paths::extended_font_dir();

    let builtin_ok = builtin.as_ref().map(|p| p.exists()).unwrap_or(false);
    let extended_ok = extended.as_ref().map(|p| p.exists()).unwrap_or(false);

    if !builtin_ok || !extended_ok {
        println!("字体目录检查：");
        if let Some(ref p) = builtin {
            println!(
                "  内置字体目录: {} [{}]",
                p.display(),
                if builtin_ok { "存在" } else { "不存在" }
            );
        }
        if let Some(ref p) = extended {
            println!(
                "  扩展字体目录: {} [{}]",
                p.display(),
                if extended_ok { "存在" } else { "不存在" }
            );
        }

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

    // 加载 standard 字体并输出
    let font = load_standard_font()?;
    let lines = font.render("It's ready");
    utils::print_colored(&lines, utils::ColorFilter::Rainbow);

    Ok(())
}

/// 修复安装：检查并恢复字体目录
fn repair_installation() -> io::Result<()> {
    // 检查内置字体目录
    let builtin = paths::builtin_font_dir();
    if let Some(ref dir) = builtin {
        if !dir.exists() {
            println!("内置字体目录 {} 不存在，无法自动修复。", dir.display());
            println!("请确保 fonts/ 目录与可执行文件在正确的位置。");
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "内置字体目录不存在",
            ));
        }
    }

    // 检查 standard.flf 是否存在
    if let Some(ref dir) = builtin {
        let standard = dir.join("standard.flf");
        if !standard.exists() {
            println!("standard.flf 不存在，正在下载...");
            match download::download_font("standard.flf", dir) {
                Ok(result) => println!("已下载 standard.flf 到 {}", result.file_path.display()),
                Err(e) => {
                    eprintln!("下载失败: {}", e);
                    return Err(e);
                }
            }
        }
    }

    println!("修复完成。");
    Ok(())
}

fn load_standard_font() -> io::Result<FigletFont> {
    let dirs = paths::font_search_paths(None);
    if let Some(path) = paths::find_font_file("standard", &dirs) {
        FigletFont::load(&path)
    } else {
        // 使用内置字体作为后备
        Ok(FigletFont::builtin())
    }
}
