use fontdue::Font;
use std::path::Path;

/// 矢量字体渲染的错误类型
#[derive(Debug)]
pub enum VectorError {
    Io(std::io::Error),
    Font(String),
}

impl std::fmt::Display for VectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorError::Io(e) => write!(f, "IO 错误: {}", e),
            VectorError::Font(s) => write!(f, "字体错误: {}", s),
        }
    }
}

impl std::error::Error for VectorError {}

impl From<std::io::Error> for VectorError {
    fn from(e: std::io::Error) -> Self {
        VectorError::Io(e)
    }
}

/// 使用字体数据将文本渲染为 ASCII 点阵图形（"系统字体遮蔽方案"）
///
/// 将文本用矢量字体渲染为灰度位图，再按阈值二值化：
/// coverage > threshold → 前景字符，否则 → 背景字符。
///
/// # Arguments
/// * `font` - fontdue 字体对象
/// * `text` - 要渲染的文本
/// * `font_size` - 字体大小（像素）
/// * `fg_char` - 前景字符
/// * `bg_char` - 背景字符
/// * `threshold` - 二值化阈值（0-255），默认 128
///
/// # Returns
/// 多行 ASCII 字符串。每行对应一行像素，所有字符按基线对齐排列。
pub fn render_with_font(
    font: &Font,
    text: &str,
    font_size: f32,
    fg_char: char,
    bg_char: char,
    threshold: u8,
) -> Result<Vec<String>, VectorError> {
    let line_metrics = font
        .horizontal_line_metrics(font_size)
        .ok_or_else(|| VectorError::Font("无法获取行度量信息".to_string()))?;

    let ascent = line_metrics.ascent.ceil() as isize;
    let descent = line_metrics.descent.floor() as isize;
    let line_height = (ascent - descent) as usize;
    let total_height = line_height + line_metrics.line_gap.ceil() as usize;

    struct GlyphData {
        metrics: fontdue::Metrics,
        bitmap: Vec<u8>,
    }

    let mut glyphs: Vec<GlyphData> = Vec::new();
    let mut total_width: usize = 0;

    for ch in text.chars() {
        let (metrics, bitmap) = font.rasterize(ch, font_size);
        total_width += metrics.advance_width.ceil() as usize;
        glyphs.push(GlyphData {
            metrics,
            bitmap: bitmap.into_iter().collect(),
        });
    }

    if glyphs.is_empty() {
        return Ok(vec![String::new(); total_height]);
    }

    // 用 char 画布（支持多字节 UTF-8 字符作为前后景）
    let mut rows: Vec<Vec<char>> = vec![vec![bg_char; total_width.max(1)]; total_height];

    let mut x: usize = 0;
    for g in &glyphs {
        let m = &g.metrics;
        let top_row = ascent - m.ymin as isize - m.height as isize;
        for br in 0..m.height {
            let canvas_row = top_row + br as isize;
            if canvas_row < 0 || canvas_row >= total_height as isize {
                continue;
            }
            let cr = canvas_row as usize;
            for bc in 0..m.width {
                let px = x + bc;
                if px >= total_width {
                    break;
                }
                let coverage = g.bitmap[br * m.width + bc];
                rows[cr][px] = if coverage > threshold { fg_char } else { bg_char };
            }
        }
        x += m.advance_width.ceil() as usize;
    }

    // 去除尾部空白行
    let last_non_empty = rows
        .iter()
        .rposition(|row| row.iter().any(|&c| c != bg_char));

    let rendered_rows: Vec<String> = rows
        .iter()
        .take(last_non_empty.map_or(0, |i| i + 1))
        .map(|row| {
            // 去除尾部空白列
            let last_col = row.iter().rposition(|&c| c != bg_char);
            match last_col {
                Some(pos) => row[..=pos].iter().collect::<String>(),
                None => String::new(),
            }
        })
        .collect();

    Ok(rendered_rows)
}

/// 从字体文件路径加载并渲染文本
///
/// # Arguments
/// * `text` - 要渲染的文本
/// * `font_path` - TTF/OTF 字体文件路径
/// * `font_size` - 字体大小（像素）
/// * `fg_char` - 前景字符
/// * `bg_char` - 背景字符
/// * `threshold` - 二值化阈值（0-255）
///
/// # Returns
/// 多行 ASCII 字符串
pub fn render_with_font_file(
    text: &str,
    font_path: &Path,
    font_size: f32,
    fg_char: char,
    bg_char: char,
    threshold: u8,
) -> Result<Vec<String>, VectorError> {
    let font_data = std::fs::read(font_path)?;
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default())
        .map_err(|e| VectorError::Font(format!("字体加载失败: {}", e)))?;
    render_with_font(&font, text, font_size, fg_char, bg_char, threshold)
}
