use std::io;

use crate::paths;

/// 执行 `--info` 命令
///
/// - 若指定字体名：提取评论信息并打印，同时用该字体画出字体名
/// - 若未指定字体：输出字体目录路径信息
pub fn run(font_name: Option<&str>, extra_dir: Option<&std::path::Path>) -> io::Result<()> {
    match font_name {
        Some(name) => show_font_info(name, extra_dir),
        None => show_directory_info(extra_dir),
    }
}

fn show_font_info(name: &str, extra_dir: Option<&std::path::Path>) -> io::Result<()> {
    let dirs = paths::font_search_paths(extra_dir);

    let font_path = paths::find_font_file(name, &dirs).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("未找到字体: {}", name),
        )
    })?;

    let data = crate::figfont::load_font_data(&font_path)?;

    // 打印字体信息
    println!("字体文件: {}", font_path.display());
    println!("签名: {}", data.header.signature);
    println!("高度: {} 行", data.header.height);
    println!("基线: {}", data.header.baseline);
    println!("最大字符宽度: {}", data.header.max_length);
    println!("旧布局: {}", data.header.old_layout);
    println!("全布局: {}", data.header.full_layout);
    println!("代码标记数: {}", data.header.codetag_count);
    println!("硬空白字符: '{}'", data.header.hard_blank);

    // 打印评论
    let comment_content = data.comment.content();
    if !comment_content.is_empty() {
        println!();
        println!("评论信息:");
        println!("{}", comment_content);
    }

    // 用该字体画出字体名
    println!();
    println!("字体名 ASCII 图形:");
    let display_name = font_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(name);
    let lines = data.render(display_name);
    for line in &lines {
        println!("{}", line);
    }

    Ok(())
}

fn show_directory_info(extra_dir: Option<&std::path::Path>) -> io::Result<()> {
    let dirs = paths::font_search_paths(extra_dir);

    println!("字体搜索目录（按优先级）：");
    for (i, dir) in dirs.iter().enumerate() {
        let status = if dir.exists() { "存在" } else { "不存在" };
        println!("  {}. {} [{}]", i + 1, dir.display(), status);
    }

    if let Some(builtin) = paths::builtin_font_dir() {
        println!();
        println!("内置字体目录: {}", builtin.display());
    }

    if let Some(ext) = paths::extended_font_dir() {
        println!("扩展系统目录: {}", ext.display());
    }

    Ok(())
}
