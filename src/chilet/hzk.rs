use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// HZK 点阵字库的字体大小信息
#[derive(Debug, Clone, Copy)]
struct HzkInfo {
    /// 字符宽度（像素/位）
    pub width: usize,
    /// 字符高度（像素/行数）
    pub height: usize,
    /// 每个字符占用字节数
    pub bytes_per_char: usize,
    /// 每行占用字节数 = ceil(width / 8)
    pub bytes_per_row: usize,
}

impl HzkInfo {
    fn from_filename(name: &str) -> Option<Self> {
        let num: usize = name.strip_prefix("HZK").and_then(|s| s.parse().ok())?;
        match num {
            16 => Some(HzkInfo {
                width: 16,
                height: 16,
                bytes_per_char: 32,
                bytes_per_row: 2,
            }),
            14 => Some(HzkInfo {
                width: 14,
                height: 14,
                bytes_per_char: 28,
                bytes_per_row: 2,
            }),
            12 => Some(HzkInfo {
                width: 12,
                height: 12,
                bytes_per_char: 24,
                bytes_per_row: 2,
            }),
            _ => None,
        }
    }
}

/// 将 Unicode 字符转换为 GB2312 编码后，计算 HZK 字库中的字节偏移。
///
/// 返回文件的字节偏移量。若字符无法编码为 GB2312 或不在汉字范围内，返回 `None`。
fn gb2312_to_offset(ch: char, info: HzkInfo) -> Option<usize> {
    let mut s = [0u8; 4];
    let s = ch.encode_utf8(&mut s);
    let (encoded, _encoding, _had_replacements) = encoding_rs::GBK.encode(s);
    if encoded.len() != 2 {
        return None;
    }
    let high = encoded[0];
    let low = encoded[1];
    if high < 0xA1 || high > 0xF7 || low < 0xA1 || low > 0xFE {
        return None;
    }
    let area = (high - 0xA0) as usize;
    let pos = (low - 0xA0) as usize;
    let offset = ((area - 1) * 94 + (pos - 1)) * info.bytes_per_char;
    Some(offset)
}

/// 从文件中读取指定偏移处的字符点阵数据
fn read_char_data(file: &mut fs::File, offset: usize, len: usize) -> io::Result<Vec<u8>> {
    file.seek(SeekFrom::Start(offset as u64))?;
    let mut buf = vec![0u8; len];
    file.read_exact(&mut buf)?;
    Ok(buf)
}

/// 将点阵数据转换为 ASCII 字符串行
///
/// HZK 字库中每行像素占用 `bytes_per_row` 个字节（大端序，MSB 对应左侧像素）。
/// 对于宽度 < 16 的字号（HZK12/HZK14），有效位左对齐在高位。
/// 位为 1 → 输出前景字符，位为 0 → 输出背景字符。
fn bitmap_to_ascii(data: &[u8], info: HzkInfo, fg_char: char, bg_char: char) -> Vec<String> {
    (0..info.height)
        .map(|row| {
            let offset = row * info.bytes_per_row;
            // 将整行字节合并为 u16（大端序）
            let mut bits: u16 = 0;
            for b in 0..info.bytes_per_row.min(2) {
                bits = (bits << 8) | (data[offset + b] as u16);
            }
            // 左对齐：从 MSB 开始取 width 位（bits 15..(16-width)）
            (0..info.width)
                .map(|col| {
                    if (bits >> (15 - col)) & 1 == 1 {
                        fg_char
                    } else {
                        bg_char
                    }
                })
                .collect::<String>()
        })
        .collect()
}

/// 从 HZK 点阵字库文件中渲染指定文本为 ASCII 图形
///
/// # Arguments
/// * `text` - 要渲染的文本（支持中文，非中文字符以背景字符填充）
/// * `hzk_path` - HZK 字库文件的路径（文件名决定字号，如 HZK16、HZK14、HZK12）
/// * `fg_char` - 前景字符（默认 `'█'`）
/// * `bg_char` - 背景字符（默认 `' '`）
///
/// # Returns
/// 多行 ASCII 字符串，每行对应文本中所有字符的同一像素行。
/// 每个字符之间用一个空格分隔。
pub fn render_hzk(
    text: &str,
    hzk_path: &Path,
    fg_char: char,
    bg_char: char,
) -> io::Result<Vec<String>> {
    let filename = hzk_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "无法解析 HZK 文件名")
        })?;

    let info = HzkInfo::from_filename(filename).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "不支持的 HZK 格式，支持 HZK12/HZK14/HZK16",
        )
    })?;

    let mut file = fs::File::open(hzk_path)?;

    let mut all_rows: Vec<String> = vec![String::new(); info.height];

    for ch in text.chars() {
        let char_rows: Vec<String> = match gb2312_to_offset(ch, info) {
            Some(offset) => match read_char_data(&mut file, offset, info.bytes_per_char) {
                Ok(data) => bitmap_to_ascii(&data, info, fg_char, bg_char),
                Err(_) => vec![bg_char.to_string().repeat(info.width); info.height],
            },
            None => {
                // 非中文字符：填充背景字符
                vec![bg_char.to_string().repeat(info.width); info.height]
            }
        };

        for row in 0..info.height {
            if !all_rows[row].is_empty() {
                all_rows[row].push(' ');
            }
            all_rows[row].push_str(&char_rows[row]);
        }
    }

    Ok(all_rows)
}
