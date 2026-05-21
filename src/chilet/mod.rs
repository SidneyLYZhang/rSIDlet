//! 中文点阵 ASCII 图形生成模块
//!
//! 提供两种核心能力：
//!
//! 1. **矢量字体渲染**：使用 TTF/OTF 矢量字体，将文字渲染为二值图像，
//!    再逐像素转为 ASCII 字符（`█` 表示前景，空格表示背景）。
//!    支持按字体名称搜索系统字体，或直接指定字体文件路径。
//!
//! 2. **点阵字库直读**：直接读取经典 HZK 点阵字库（HZK16/HZK14/HZK12），
//!    解析后输出对应的 ASCII 图形。
//!
//! # 示例
//!
//! ```ignore
//! use rsidlet::chilet;
//!
//! // 用 HZK16 渲染中文
//! if let Some(path) = chilet::find_hzk("HZK16") {
//!     let lines = chilet::render_hzk("你好", &path).unwrap();
//!     for line in &lines { println!("{}", line); }
//! }
//!
//! // 用矢量字体渲染
//! let lines = chilet::render_vector_font("你好", "SimHei", 32.0).unwrap();
//! for line in &lines { println!("{}", line); }
//! ```

mod hzk;
mod vector;

use std::path::{Path, PathBuf};

pub use hzk::render_hzk;
pub use vector::{render_with_font, render_with_font_file, VectorError};

/// 获取字体搜索路径列表（委托到 paths 模块）
pub fn font_search_paths() -> Vec<PathBuf> {
    crate::paths::font_search_paths(None)
}

/// 获取系统字体搜索目录（委托到 paths 模块）
pub fn system_font_dirs() -> Vec<PathBuf> {
    crate::paths::system_font_dirs()
}

/// 在搜索路径中查找 HZK 点阵字库文件
///
/// HZK 字库只能从 `../fonts` 或平台额外字体目录中搜索。
/// 也可直接通过 [`render_hzk`] 指定具体路径加载。
///
/// # Arguments
/// * `name` - HZK 字库名称，必须以 `HZK` 开头（如 `HZK16`、`HZK14`、`HZK12`）
///
/// # Returns
/// 找到的文件路径，或 `None`
pub fn find_hzk(name: &str) -> Option<PathBuf> {
    if !name.starts_with("HZK") {
        return None;
    }
    for dir in font_search_paths() {
        let path = dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// 按字体名称在系统中查找矢量字体文件（TTF/OTF）
///
/// 搜索范围包括系统字体目录和 `../fonts` 目录。
/// 支持带或不带扩展名的字体名称匹配。
///
/// # Arguments
/// * `name` - 字体名称，如 `SimHei`、`simhei.ttf`、`Arial`
///
/// # Returns
/// 找到的字体文件路径，或 `None`
pub fn find_vector_font(name: &str) -> Option<PathBuf> {
    let search_name = name.to_lowercase();
    let has_ext = search_name.ends_with(".ttf") || search_name.ends_with(".otf");

    for dir in system_font_dirs() {
        // 递归搜索（最多 3 层）
        if let Ok(found) = search_font_in_dir(&dir, &search_name, has_ext, 3) {
            return Some(found);
        }
    }
    None
}

/// 递归搜索目录中的字体文件
fn search_font_in_dir(
    dir: &Path,
    name_lower: &str,
    has_ext: bool,
    max_depth: usize,
) -> std::io::Result<PathBuf> {
    if max_depth == 0 || !dir.exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Err(std::io::Error::from(std::io::ErrorKind::NotFound)),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Ok(found) = search_font_in_dir(&path, name_lower, has_ext, max_depth - 1) {
                return Ok(found);
            }
        } else if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
            let fname_lower = fname.to_lowercase();
            let matched = if has_ext {
                fname_lower == name_lower
            } else {
                let stem = fname_lower.rsplitn(2, '.').nth(1).unwrap_or(&fname_lower);
                stem == name_lower
            };
            if matched {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let ext_lower = ext.to_lowercase();
                if ext_lower == "ttf" || ext_lower == "otf" {
                    return Ok(path);
                }
            }
        }
    }

    Err(std::io::Error::from(std::io::ErrorKind::NotFound))
}

/// 使用矢量字体按名称渲染文本为 ASCII 图形
///
/// 先通过 [`find_vector_font`] 搜索字体文件，再加载并渲染。
///
/// # Arguments
/// * `text` - 要渲染的文本
/// * `font_name` - 字体名称（如 `SimHei`、`Arial`）
/// * `font_size` - 字体大小（像素）
///
/// # Returns
/// 多行 ASCII 字符串，失败时返回错误
pub fn render_vector_font(
    text: &str,
    font_name: &str,
    font_size: f32,
) -> Result<Vec<String>, VectorError> {
    let font_path = find_vector_font(font_name)
        .ok_or_else(|| VectorError::Font(format!("未找到字体: {}", font_name)))?;
    render_with_font_file(text, &font_path, font_size)
}
