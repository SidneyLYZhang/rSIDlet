use std::fs;
use std::io::{self, BufRead, BufReader, Cursor, Read};
use std::path::Path;

pub mod control;
mod fig;
mod toi;

pub use fig::FigletFont;
pub use toi::ToiletFont;

/// 字符到字形索引的映射参数
#[derive(Debug, Clone)]
pub struct CharMapping {
    /// 起始代码点（包含）
    pub start: u32,
    /// 结束代码点（包含），如果为单个字符则等于 start
    pub end: u32,
    /// 字体文件中字形数据的起始索引
    pub font_index: usize,
}

/// 字体头部信息，figlet 和 toilet 共享相同格式
#[derive(Debug, Clone)]
pub struct FontHeader {
    /// 字体签名（"flf2a" 或 "tlf2a"）
    pub signature: String,
    /// 硬空白字符（用于表示空格的字符）
    pub hard_blank: char,
    /// 字符高度（行数）
    pub height: u32,
    /// 基线位置
    pub baseline: u32,
    /// 最大字符长度
    pub max_length: u32,
    /// 旧布局模式
    pub old_layout: u32,
    /// 评论行数
    pub comment_lines: u32,
    /// 打印方向（0=左到右，1=右到左）
    pub print_direction: u32,
    /// 全布局模式
    pub full_layout: u32,
    /// 代码标记数量
    pub codetag_count: u32,
    /// 字符映射表
    pub mappings: Vec<CharMapping>,
}

impl FontHeader {
    /// 从头部行的第一行解析（签名行）
    fn parse_signature_line(line: &str) -> io::Result<(String, char)> {
        let line = line.trim_end_matches('\n').trim_end_matches('\r');

        if line.starts_with("flf2a") {
            let hard_blank = line.chars().nth(5).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing hard blank in flf header")
            })?;
            Ok(("flf2a".to_string(), hard_blank))
        } else if line.starts_with("tlf2a") {
            let hard_blank = line.chars().nth(5).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing hard blank in tlf header")
            })?;
            Ok(("tlf2a".to_string(), hard_blank))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown font signature: {}", &line[..line.len().min(20)]),
            ))
        }
    }

    /// 解析头部数字参数
    fn parse_header_params(parts: &[&str]) -> io::Result<(u32, u32, u32, u32, u32, u32, u32, u32)> {
        let height = parts.get(0).unwrap_or(&"0").parse().unwrap_or(0);
        let baseline = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
        let max_length = parts.get(2).unwrap_or(&"0").parse().unwrap_or(0);
        let old_layout = parts.get(3).unwrap_or(&"0").parse().unwrap_or(0);
        let comment_lines = parts.get(4).unwrap_or(&"0").parse().unwrap_or(0);
        let print_direction = parts.get(5).unwrap_or(&"0").parse().unwrap_or(0);
        let full_layout = parts.get(6).unwrap_or(&"0").parse().unwrap_or(0);
        let codetag_count = parts.get(7).unwrap_or(&"0").parse().unwrap_or(0);

        if height == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Font height cannot be zero",
            ));
        }

        Ok((height, baseline, max_length, old_layout, comment_lines, print_direction, full_layout, codetag_count))
    }
}

/// 字体评论内容
#[derive(Debug, Clone)]
pub struct FontComment {
    /// 评论文本行
    pub lines: Vec<String>,
}

impl FontComment {
    /// 将评论内容合并为单个字符串
    pub fn content(&self) -> String {
        self.lines.join("\n")
    }
}

/// 单个字符的字形数据
#[derive(Debug, Clone)]
pub struct Glyph {
    /// 字符代码点
    pub codepoint: u32,
    /// 字形的文本行（每行末尾的标记字符已被移除）
    pub lines: Vec<String>,
    /// 字符宽度（最大行长度）
    pub width: usize,
    /// 字符高度（行数）
    pub height: usize,
}

impl Glyph {
    /// 创建空字形的占位符
    pub fn placeholder(codepoint: u32, height: usize) -> Self {
        let lines = vec!["?".repeat(3); height];
        let width = 3;
        Glyph { codepoint, lines, width, height }
    }

    /// 检查该字形是否为有效字符（非全空白）
    pub fn is_valid(&self) -> bool {
        self.lines.iter().any(|line| !line.trim().is_empty())
    }

    /// 获取指定行的内容，越界返回空字符串
    pub fn line(&self, index: usize) -> &str {
        self.lines.get(index).map(|s| s.as_str()).unwrap_or("")
    }

    /// 将硬空白字符替换为普通空格
    pub fn render_with_hardblank(&self, hard_blank: char) -> Vec<String> {
        self.lines.iter()
            .map(|line| line.replace(hard_blank, " "))
            .collect()
    }
}

/// 字体解析后的完整数据结构
#[derive(Debug, Clone)]
pub struct FontData {
    pub header: FontHeader,
    pub comment: FontComment,
    pub glyphs: Vec<Glyph>,
}

impl FontData {
    /// 查找指定字符的字形
    pub fn get_glyph(&self, ch: char) -> Option<&Glyph> {
        let codepoint = ch as u32;
        self.glyphs.iter().find(|g| g.codepoint == codepoint)
    }

    /// 获取字符宽度，如果字符不存在返回0
    pub fn char_width(&self, ch: char) -> usize {
        self.get_glyph(ch).map(|g| g.width).unwrap_or(0)
    }

    /// 将文本转换为 ASCII 艺术行
    pub fn render(&self, text: &str) -> Vec<String> {
        let height = self.header.height as usize;
        let mut result = vec![String::new(); height];

        for ch in text.chars() {
            let glyph = self.get_glyph(ch)
                .or_else(|| self.get_glyph('?'))
                .cloned()
                .unwrap_or_else(|| Glyph::placeholder(ch as u32, height));

            let rendered = glyph.render_with_hardblank(self.header.hard_blank);

            for (i, line) in rendered.iter().enumerate() {
                if i < result.len() {
                    result[i].push_str(line);
                }
            }
        }

        result
    }
}

/// 统一的字体读取 trait
pub trait FigFont: Sized {
    /// 字体类型名称
    fn font_type() -> &'static str;

    /// 支持的文件扩展名
    fn extensions() -> &'static [&'static str];

    /// 从路径加载字体，自动处理 zip 压缩
    fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();

        if is_zip_file(path) {
            Self::load_from_zip(path)
        } else {
            let data = fs::read(path)?;
            Self::parse(&data)
        }
    }

    /// 从 zip 文件中加载字体
    fn load_from_zip<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file_in_zip = archive.by_index(i)?;
            let name = file_in_zip.name();

            let ext = Path::new(name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            if Self::extensions().contains(&ext.as_str()) {
                let mut data = Vec::new();
                file_in_zip.read_to_end(&mut data)?;
                return Self::parse(&data);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No {} font file found in zip archive", Self::font_type()),
        ))
    }

    /// 从二进制数据解析字体
    fn parse(data: &[u8]) -> io::Result<Self>;

    /// 获取字体数据引用
    fn data(&self) -> &FontData;

    /// 渲染文本为 ASCII 艺术
    fn render(&self, text: &str) -> Vec<String> {
        self.data().render(text)
    }

    /// 获取字体头部信息
    fn header(&self) -> &FontHeader {
        &self.data().header
    }

    /// 获取字体评论
    fn comment(&self) -> &FontComment {
        &self.data().comment
    }

    /// 获取指定字符的字形
    fn get_glyph(&self, ch: char) -> Option<&Glyph> {
        self.data().get_glyph(ch)
    }
}

/// 从路径自动检测并加载字体（不区分 figlet/toilet，返回统一的 FontData）
pub fn load_font_data<P: AsRef<Path>>(path: P) -> io::Result<FontData> {
    let path = path.as_ref();

    let data = if is_zip_file(path) {
        read_from_zip_any(path)?
    } else {
        fs::read(path)?
    };

    // 根据签名自动判断格式
    let reader = BufReader::new(Cursor::new(&data));
    let first_line = reader.lines().next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty font file"))??;

    if first_line.starts_with("tlf2a") {
        ToiletFont::parse(&data).map(|f| f.into_data())
    } else {
        // 默认按 figlet 格式解析
        FigletFont::parse(&data).map(|f| f.into_data())
    }
}

/// 内部函数：解析字体基础数据（头部 + 评论 + 字形），两种字体通用
pub(crate) fn parse_font_base<R: Read>(reader: R) -> io::Result<FontData> {
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    // 读取签名行
    buf_reader.read_line(&mut line)?;
    if line.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty font file"));
    }

    let (signature, hard_blank) = FontHeader::parse_signature_line(&line)?;

    // 解析头部参数：签名后的部分
    let sig_end = if line.starts_with("flf2a") || line.starts_with("tlf2a") {
        6 // "flf2aX" 或 "tlf2aX" 其中 X 是 hard_blank
    } else {
        0
    };

    let param_str = &line[sig_end..];
    let params: Vec<&str> = param_str.split_whitespace().collect();

    let (height, baseline, max_length, old_layout, comment_lines, print_direction, full_layout, codetag_count) =
        FontHeader::parse_header_params(&params)?;

    // 读取评论行
    let mut comment_lines_vec = Vec::new();
    for _ in 0..comment_lines {
        let mut comment_line = String::new();
        if buf_reader.read_line(&mut comment_line)? == 0 {
            break;
        }
        comment_lines_vec.push(comment_line.trim_end_matches('\n').trim_end_matches('\r').to_string());
    }

    // 读取字符映射表（如果存在）
    let mappings = Vec::new();

    // 解析必需字符（32-126）+ 额外字符
    let mut glyphs = Vec::new();
    let mut current_codepoint: u32 = 32; // 从空格开始

    // 首先读取 95 个必需字符（32-126）
    for expected_cp in 32..=126 {
        match read_glyph(&mut buf_reader, expected_cp, height as usize) {
            Ok(glyph) => {
                current_codepoint = expected_cp + 1;
                glyphs.push(glyph);
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                // 如果必需字符不完整，用占位符填充
                glyphs.push(Glyph::placeholder(expected_cp, height as usize));
                current_codepoint = expected_cp + 1;
            }
            Err(e) => return Err(e),
        }
    }

    // 读取额外字符直到 EOF
    loop {
        match read_glyph(&mut buf_reader, current_codepoint, height as usize) {
            Ok(glyph) => {
                glyphs.push(glyph);
                current_codepoint += 1;
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }

    let header = FontHeader {
        signature,
        hard_blank,
        height,
        baseline,
        max_length,
        old_layout,
        comment_lines,
        print_direction,
        full_layout,
        codetag_count,
        mappings,
    };

    let comment = FontComment {
        lines: comment_lines_vec,
    };

    Ok(FontData { header, comment, glyphs })
}

/// 从读取器中读取单个字符的字形
fn read_glyph<R: BufRead>(reader: &mut R, codepoint: u32, height: usize) -> io::Result<Glyph> {
    let mut lines = Vec::with_capacity(height);
    let mut max_width = 0;

    for i in 0..height {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("EOF at glyph {codepoint}, line {i}/{height}"),
            ));
        }

        // 移除行尾的结束标记（通常是最后一个重复的非空格字符）
        let line = line.trim_end_matches('\n').trim_end_matches('\r');
        let line = strip_endmark(line);

        max_width = max_width.max(line.chars().count());
        lines.push(line.to_string());
    }

    Ok(Glyph {
        codepoint,
        lines,
        width: max_width,
        height,
    })
}

/// 移除字形行末尾的结束标记字符
/// 标记字符是行末重复的最后一个非空格字符
fn strip_endmark(line: &str) -> &str {
    if line.is_empty() {
        return line;
    }

    // 找到行尾的第一个非空格字符
    let trimmed = line.trim_end();
    if trimmed.is_empty() {
        return "";
    }

    // 获取最后一个字符
    let last_char = trimmed.chars().last().unwrap();

    // 检查是否是结束标记（在原始行中，末尾有这个字符作为标记）
    // 标准做法：移除行尾的重复标记字符
    let mut end_pos = trimmed.len();
    for (i, ch) in trimmed.char_indices().rev() {
        if ch == last_char && i == end_pos - last_char.len_utf8() {
            end_pos = i;
        } else {
            break;
        }
    }

    // 如果只剩一个标记字符，保留它（这是正常内容）
    // 如果有多个连续的相同字符在末尾，移除最后一个
    if end_pos < trimmed.len() && end_pos > 0 {
        &trimmed[..end_pos]
    } else {
        trimmed
    }
}

/// 检查文件是否为 zip 压缩文件
fn is_zip_file<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(data) = fs::read(&path) {
        data.starts_with(b"PK\x03\x04")
    } else {
        false
    }
}

/// 从 zip 文件中读取第一个找到的字体文件数据（不区分格式）
fn read_from_zip_any<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let file = fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    // 优先寻找 .flf 和 .tlf 文件
    for ext in &["flf", "tlf"] {
        for i in 0..archive.len() {
            let name = archive.by_index(i)?.name().to_string();
            if name.to_lowercase().ends_with(&format!(".{}", ext)) {
                let mut data = Vec::new();
                let mut file_in_zip = archive.by_index(i)?;
                file_in_zip.read_to_end(&mut data)?;
                return Ok(data);
            }
        }
    }

    // 如果没有找到特定扩展名，返回第一个文件
    if archive.len() > 0 {
        let mut file_in_zip = archive.by_index(0)?;
        let mut data = Vec::new();
        file_in_zip.read_to_end(&mut data)?;
        Ok(data)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No files found in zip archive",
        ))
    }
}
