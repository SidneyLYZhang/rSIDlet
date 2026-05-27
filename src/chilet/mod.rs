//! 中文点阵 ASCII 图形生成模块
//!
//! 提供两种核心能力：
//!
//! 1. **HZK 点阵字库直读**：直接读取经典 HZK 点阵字库（HZK16/HZK14/HZK12），
//!    解析后按前后景字符绘制输出。
//!
//! 2. **系统字体遮蔽方案**：使用 TTF/OTF 矢量字体，将文字渲染为灰度位图，
//!    再按阈值二值化，映射为前景/背景字符，实现"遮蔽对齐"输出。
//!
//! 通过 [`render`] 函数提供统一入口，自动根据文本内容（是否含中文）和
//! 请求字号选择合适的渲染路径。

mod hzk;
mod vector;

use std::path::{Path, PathBuf};

pub use hzk::render_hzk;
pub use vector::{render_with_font, render_with_font_file, VectorError};

// ============================================================
// 渲染选项
// ============================================================

/// 渲染选项配置
pub struct RenderOptions {
    /// 请求的字号（像素）。对 HZK 字库仅支持 12/14/16；
    /// 其他字号自动回退到系统字体遮蔽方案。
    pub font_size: u32,
    /// 前景字符（默认 `'█'`）
    pub fg_char: char,
    /// 背景字符（默认 `' '`）
    pub bg_char: char,
    /// 系统字体二值化阈值（0-255），默认 128
    pub threshold: u8,
    /// 欲使用的系统字体名称或路径（可选）。
    /// 若指定且可找到，优先使用该系统字体进行遮蔽渲染。
    pub font_name: Option<String>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            font_size: 16,
            fg_char: '█',
            bg_char: ' ',
            threshold: 128,
            font_name: None,
        }
    }
}

impl RenderOptions {
    /// 设置请求的字号
    pub fn with_size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }

    /// 设置前景/背景字符
    pub fn chars(mut self, fg: char, bg: char) -> Self {
        self.fg_char = fg;
        self.bg_char = bg;
        self
    }

    /// 设置系统字体名称
    pub fn font(mut self, name: &str) -> Self {
        self.font_name = Some(name.to_string());
        self
    }
}

// ============================================================
// 统一渲染入口
// ============================================================

/// 渲染错误类型
#[derive(Debug)]
pub enum RenderError {
    Io(std::io::Error),
    Vector(VectorError),
    /// 找不到合适的字体
    NoFont(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::Io(e) => write!(f, "IO 错误: {}", e),
            RenderError::Vector(e) => write!(f, "矢量字体错误: {}", e),
            RenderError::NoFont(s) => write!(f, "未找到可用字体: {}", s),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<std::io::Error> for RenderError {
    fn from(e: std::io::Error) -> Self {
        RenderError::Io(e)
    }
}

impl From<VectorError> for RenderError {
    fn from(e: VectorError) -> Self {
        RenderError::Vector(e)
    }
}

/// 统一渲染入口：自动选择 HZK 点阵字库或系统字体遮蔽方案
///
/// # 字体选择逻辑
///
/// 1. 若文本**不含中文**：
///    - 若指定了系统字体名 → 使用系统字体遮蔽方案
///    - 若请求的字号为 12/14/16 且 HZK 字库存在 → 使用 HZK
///    - 否则 → 尝试查找默认系统字体进行遮蔽渲染
///
/// 2. 若文本**含有中文**：
///    - 若请求的字号为 12/14/16 且 HZK 字库存在 → 使用 HZK
///    - 否则 → 使用系统字体遮蔽方案（不允许不经遮蔽直接输出系统字体字符）
///
/// # Arguments
/// * `text` - 要渲染的文本
/// * `options` - 渲染选项（字号、前后景字符、阈值、字体名等）
///
/// # Returns
/// 多行 ASCII 字符串
pub fn render(text: &str, options: &RenderOptions) -> Result<Vec<String>, RenderError> {
    let hzk_size = options.font_size;

    // 如果用户指定了字体名称，先尝试解析
    if let Some(ref font_name) = options.font_name {
        // 检查是否为 HZK 字体名
        if font_name.starts_with("HZK") || font_name.starts_with("hzk") {
            if let Some(hzk_path) = find_hzk(&font_name.to_uppercase()) {
                return Ok(render_hzk(text, &hzk_path, options.fg_char, options.bg_char)?);
            }
            return Err(RenderError::NoFont(format!("HZK 字库未找到: {}", font_name)));
        }

        // 尝试作为系统字体名查找
        if let Some(font_path) = find_vector_font(font_name) {
            return Ok(render_with_font_file(
                text,
                &font_path,
                options.font_size as f32,
                options.fg_char,
                options.bg_char,
                options.threshold,
            )?);
        }

        // 尝试作为字体文件路径
        let path = Path::new(font_name);
        if path.exists() {
            return Ok(render_with_font_file(
                text,
                path,
                options.font_size as f32,
                options.fg_char,
                options.bg_char,
                options.threshold,
            )?);
        }

        return Err(RenderError::NoFont(format!("未找到字体: {}", font_name)));
    }

    // 无指定字体名 → 自动选择

    // HZK 字号 + HZK 可用 → 优先使用 HZK
    if matches!(hzk_size, 12 | 14 | 16) {
        let hzk_name = format!("HZK{}", hzk_size);
        if let Some(hzk_path) = find_hzk(&hzk_name) {
            return Ok(render_hzk(text, &hzk_path, options.fg_char, options.bg_char)?);
        }
    }

    // 回退到系统字体遮蔽方案
    render_with_system_font_fallback(text, options)
}

/// 使用系统字体遮蔽方案渲染（自动查找可用字体）
fn render_with_system_font_fallback(
    text: &str,
    options: &RenderOptions,
) -> Result<Vec<String>, RenderError> {
    // 按优先级尝试常见的系统中文字体
    let fallback_fonts = [
        // Windows
        "simsun.ttc",
        "simhei.ttf",
        "msyh.ttc",
        "mingliu.ttc",
        // macOS
        "PingFang.ttc",
        "Heiti SC",
        "STHeiti",
        // Linux
        "NotoSansCJK",
        "wqy-microhei.ttc",
        "DroidSansFallbackFull.ttf",
    ];

    for name in &fallback_fonts {
        if let Some(font_path) = find_vector_font(name) {
            return Ok(render_with_font_file(
                text,
                &font_path,
                options.font_size as f32,
                options.fg_char,
                options.bg_char,
                options.threshold,
            )?);
        }
    }

    Err(RenderError::NoFont(
        "未找到可用的系统字体，请通过 --font 参数指定字体文件路径或安装中文字体".to_string(),
    ))
}

// ============================================================
// 字体搜索
// ============================================================

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

/// 按字体名称在系统中查找矢量字体文件（TTF/OTF/TTC）
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
    let has_ext = search_name.ends_with(".ttf")
        || search_name.ends_with(".otf")
        || search_name.ends_with(".ttc");

    for dir in system_font_dirs() {
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
                let stem = fname_lower
                    .rsplitn(2, '.')
                    .nth(1)
                    .unwrap_or(&fname_lower);
                stem == name_lower
            };
            if matched {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let ext_lower = ext.to_lowercase();
                if ext_lower == "ttf" || ext_lower == "otf" || ext_lower == "ttc" {
                    return Ok(path);
                }
            }
        }
    }

    Err(std::io::Error::from(std::io::ErrorKind::NotFound))
}
