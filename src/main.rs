use std::io;
use std::path::PathBuf;

use clap::Parser;
use rsidlet::commands;
use rsidlet::figfont;
use rsidlet::paths;
use rsidlet::utils;

#[derive(Parser)]
#[command(
    name = "sidlet",
    version,
    about = "A Rust implementation of figlet -- 将文本转换为 ASCII 艺术字",
    long_about = "sidlet 将文本转换为 ASCII 艺术字，支持传统 figlet/toilet 字体、中文点阵字体和矢量字体。",
    after_help = "示例:\n  sidlet \"Hello World\"\n  sidlet -f big.flf \"Hello\"\n  sidlet --install standard.flf\n  sidlet --info big.flf\n  sidlet --list font\n  sidlet --list colormap\n  sidlet --list installed\n  sidlet --list letters\n  sidlet --test"
)]
struct Cli {
    /// 要绘制成 ASCII 艺术的文本
    #[arg(num_args = 0.., allow_hyphen_values = true)]
    message: Vec<String>,

    /// 指定本地字体目录
    #[arg(short = 'd', long = "directory")]
    font_directory: Option<PathBuf>,

    /// 指定字体文件或字体名（默认使用 standard.flf）
    #[arg(short = 'f', long = "font")]
    font: Option<String>,

    /// 最大输出宽度（列数），超宽时自动换行
    #[arg(short = 'w', long = "width")]
    output_width: Option<usize>,

    /// Figlet control 文件路径
    #[arg(short = 'C', long = "control")]
    control_file: Option<PathBuf>,

    /// 颜色遮蔽名称
    #[arg(short = 'm', long = "maskcolor")]
    mask_color: Option<String>,

    /// Chilet 点阵字体大小（默认 12）
    #[arg(short = 's', long = "size", default_value = "12")]
    font_size: f32,

    /// Chilet 模式前景字符（默认 █）
    #[arg(long = "fore", default_value = "█")]
    foreground: String,

    /// Chilet 模式背景字符（默认空格）
    #[arg(long = "back", default_value = " ")]
    background: String,

    /// 安装新字体 [fontfile/fontname]
    #[arg(long = "install", num_args = 0..=1, default_missing_value = "")]
    install: Option<String>,

    /// 显示字体信息 [-i] [fontfile/fontname]
    #[arg(short = 'i', long = "info", num_args = 0..=1, default_missing_value = "")]
    info: Option<String>,

    /// 列出字体/颜色遮蔽/已安装/可下载 (font|colormap|installed|letters)
    #[arg(long = "list")]
    list: Option<String>,

    /// 测试安装状态
    #[arg(long = "test", default_value_t = false)]
    test: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> io::Result<()> {
    // 模式分发
    if cli.test {
        return commands::test::run();
    }

    if cli.install.is_some() {
        let font_name = cli.install.as_deref().filter(|s| !s.is_empty());
        return commands::install::run(font_name, cli.font_directory.as_deref());
    }

    if cli.info.is_some() {
        let font_name = cli.info.as_deref().filter(|s| !s.is_empty());
        return commands::info::run(font_name, cli.font_directory.as_deref());
    }

    if let Some(ref list_type) = cli.list {
        return commands::list::run(list_type, cli.font_directory.as_deref());
    }

    // 基础绘制模式
    let message = if cli.message.is_empty() {
        eprintln!("请提供要绘制的文本消息。使用 --help 查看帮助。");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "缺少文本消息",
        ));
    } else {
        cli.message.join(" ")
    };

    draw(&message, &cli)
}

/// 基础绘制管线
fn draw(message: &str, cli: &Cli) -> io::Result<()> {
    let dirs = paths::font_search_paths(cli.font_directory.as_deref());
    let font_name = cli.font.as_deref().unwrap_or("standard.flf");

    let has_chinese = utils::contains_chinese(message);
    let font_lower = font_name.to_lowercase();
    let is_vector_font = font_lower.ends_with(".ttf")
        || font_lower.ends_with(".otf")
        || font_lower.ends_with(".bdf");

    let lines = if has_chinese || is_vector_font {
        render_chilet(message, font_name, &dirs, cli)?
    } else {
        render_figlet(message, font_name, &dirs, cli)?
    };

    // 应用色彩滤镜
    if let Some(ref color) = cli.mask_color {
        if let Some(filter) = utils::parse_filter(color) {
            utils::print_colored(&lines, filter);
        } else {
            eprintln!("警告: 未知的颜色遮蔽 '{}'", color);
            utils::print_colored(&lines, utils::ColorFilter::None);
        }
    } else {
        utils::print_colored(&lines, utils::ColorFilter::None);
    }

    Ok(())
}

/// Chilet 模式渲染（中文/矢量字体）
fn render_chilet(
    message: &str,
    font_name: &str,
    dirs: &[PathBuf],
    cli: &Cli,
) -> io::Result<Vec<String>> {
    let font_lower = font_name.to_lowercase();
    let is_vector = font_lower.ends_with(".ttf") || font_lower.ends_with(".otf");

    if is_vector {
        // 使用矢量字体渲染
        let font_path = paths::find_font_file(font_name, dirs).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("未找到矢量字体: {}", font_name))
        })?;

        let lines = rsidlet::chilet::render_with_font_file(message, &font_path, cli.font_size)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        return Ok(apply_charset(lines, &cli.foreground, &cli.background));
    }

    // 使用 HZK 点阵字体渲染中文
    let hzk_size = cli.font_size as u32;
    let hzk_name = match hzk_size {
        14 => "HZK14",
        16 => "HZK16",
        _ => "HZK12",
    };

    let hzk_path = rsidlet::chilet::find_hzk(hzk_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("未找到 HZK 中文字库: {}（字体目录可能不完整）", hzk_name),
        )
    })?;

    let lines = rsidlet::chilet::render_hzk(message, &hzk_path)?;
    Ok(apply_charset(lines, &cli.foreground, &cli.background))
}

/// Figlet/Toilet 模式渲染
fn render_figlet(
    message: &str,
    font_name: &str,
    dirs: &[PathBuf],
    cli: &Cli,
) -> io::Result<Vec<String>> {
    let font_path = paths::find_font_file(font_name, dirs).ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, format!("未找到字体: {}", font_name))
    })?;

    let mut data = figfont::load_font_data(&font_path)?;

    // 加载并应用 control 文件（如果指定）
    if let Some(ref control_path) = cli.control_file {
        let control = figfont::control::ControlFile::load(control_path)?;
        control.apply_to_font_data(&mut data);
    }

    let lines = data.render(message);

    // 宽度换行
    if let Some(max_width) = cli.output_width {
        Ok(wrap_figlet_lines(&data, message, max_width))
    } else {
        Ok(lines)
    }
}

/// 对 figlet 渲染结果进行宽度换行
fn wrap_figlet_lines(data: &figfont::FontData, message: &str, max_width: usize) -> Vec<String> {
    let lines = data.render(message);
    let current_max = lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);

    if current_max <= max_width {
        return lines;
    }

    // 将文本按能容纳的最大字符数分段，分别渲染后纵向拼接
    let chars: Vec<char> = message.chars().collect();
    let mut result = Vec::new();
    let mut start = 0;

    while start < chars.len() {
        // 二分查找能容纳的最大字符数
        let mut lo = 1;
        let mut hi = chars.len() - start;
        let mut best = 1;

        while lo <= hi {
            let mid = (lo + hi) / 2;
            let chunk: String = chars[start..start + mid].iter().collect();
            let chunk_lines = data.render(&chunk);
            let w = chunk_lines
                .iter()
                .map(|l| l.chars().count())
                .max()
                .unwrap_or(0);
            if w <= max_width {
                best = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        let chunk: String = chars[start..start + best].iter().collect();
        let chunk_lines = data.render(&chunk);
        result.extend(chunk_lines);
        // 段间空行
        if start + best < chars.len() {
            result.push(String::new());
        }
        start += best;
    }

    result
}

/// 对已经渲染的 ASCII 行进行字符替换（chilet 模式的 fore/back 自定义）
fn apply_charset(lines: Vec<String>, fore: &str, back: &str) -> Vec<String> {
    if fore == "█" && back == " " {
        return lines;
    }
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '█' => fore.chars().next().unwrap_or('█'),
                    ' ' => back.chars().next().unwrap_or(' '),
                    other => other,
                })
                .collect::<String>()
        })
        .collect()
}
