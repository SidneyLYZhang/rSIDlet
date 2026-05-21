use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// HZK 点阵字库的字体大小信息
#[derive(Debug, Clone, Copy)]
struct HzkInfo {
    /// 字符宽度（像素）
    pub width: usize,
    /// 字符高度（像素）
    pub height: usize,
    /// 每个字符占用字节数
    pub bytes_per_char: usize,
}

impl HzkInfo {
    fn from_filename(name: &str) -> Option<Self> {
        // 从 "HZK16"、"HZK14"、"HZK12" 中提取数字
        let num: usize = name
            .strip_prefix("HZK")
            .and_then(|s| s.parse().ok())?;
        match num {
            16 => Some(HzkInfo { width: 16, height: 16, bytes_per_char: 32 }),
            14 => Some(HzkInfo { width: 14, height: 14, bytes_per_char: 28 }),
            12 => Some(HzkInfo { width: 12, height: 12, bytes_per_char: 24 }),
            _ => None,
        }
    }
}

/// 将 Unicode 字符转换为 GB2312 编码后，计算 HZK 字库中的字节偏移。
///
/// 返回文件的字节偏移量。若字符无法编码为 GB2312 或不在汉字范围内，返回 `None`。
fn gb2312_to_offset(ch: char, info: HzkInfo) -> Option<usize> {
    // 用 GBK 编码（兼容 GB2312），获取 GB2312 双字节码
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
/// 每两个字节代表一行像素（大端序），MSB 对应左侧像素。
/// 位为 1 → 输出 `█`（前景），位为 0 → 输出空格（背景）。
fn bitmap_to_ascii(data: &[u8], info: HzkInfo) -> Vec<String> {
    let bytes_per_row = 2; // HZK 格式每行固定 2 字节
    (0..info.height)
        .map(|row| {
            let offset = row * bytes_per_row;
            let byte0 = data[offset];
            let byte1 = data[offset + 1];
            let bits = ((byte0 as u16) << 8) | (byte1 as u16);
            (0..info.width)
                .map(|col| {
                    if (bits >> (15 - col)) & 1 == 1 { '█' } else { ' ' }
                })
                .collect::<String>()
        })
        .collect()
}

/// 从 HZK 点阵字库文件中渲染指定文本为 ASCII 图形
///
/// # Arguments
/// * `text` - 要渲染的中文文本
/// * `hzk_path` - HZK 字库文件的路径
///
/// # Returns
/// 多行 ASCII 字符串，每行对应文本中所有字符的同一像素行。
/// 每个字符之间用一个空格分隔。
pub fn render_hzk(text: &str, hzk_path: &Path) -> io::Result<Vec<String>> {
    let filename = hzk_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "无法解析 HZK 文件名"))?;

    let info = HzkInfo::from_filename(filename)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "不支持的 HZK 格式，支持 HZK12/HZK14/HZK16"))?;

    let mut file = fs::File::open(hzk_path)?;

    let mut all_rows: Vec<String> = vec![String::new(); info.height];

    for ch in text.chars() {
        // 非中文字符用空白替代
        let char_rows: Vec<String> = match gb2312_to_offset(ch, info) {
            Some(offset) => {
                match read_char_data(&mut file, offset, info.bytes_per_char) {
                    Ok(data) => bitmap_to_ascii(&data, info),
                    Err(_) => vec![" ".repeat(info.width); info.height],
                }
            }
            None => vec![" ".repeat(info.width); info.height],
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
