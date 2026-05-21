use std::collections::HashMap;
use std::io;
use std::path::Path;

/// Control 文件解析结果
#[derive(Debug, Clone, Default)]
pub struct ControlFile {
    /// 注释行
    pub comments: Vec<String>,
    /// 字体签名（如 "flf2a"）
    pub signature: Option<String>,
    /// 硬空白字符
    pub hard_blank: Option<char>,
    /// 字符高度
    pub height: Option<u32>,
    /// 基线位置
    pub baseline: Option<u32>,
    /// 最大字符长度
    pub max_length: Option<u32>,
    /// 旧布局模式
    pub old_layout: Option<u32>,
    /// 全布局模式
    pub full_layout: Option<u32>,
    /// 代码标记数量
    pub codetag_count: Option<u32>,
    /// 打印方向
    pub print_direction: Option<u32>,
    /// 评论行数
    pub comment_lines: Option<u32>,
    /// 字符映射：字符 -> 字形名称
    pub char_mappings: HashMap<char, String>,
    /// Unicode 范围映射
    pub unicode_ranges: Vec<UnicodeRange>,
}

/// Unicode 范围映射
#[derive(Debug, Clone)]
pub struct UnicodeRange {
    pub start: u32,
    pub end: u32,
    pub font_index: usize,
}

impl ControlFile {
    /// 从文件路径加载并解析 control 文件
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// 从字符串解析 control 文件
    pub fn parse(content: &str) -> io::Result<Self> {
        let mut control = ControlFile::default();
        let mut in_comment = true;

        for line in content.lines() {
            let trimmed = line.trim();

            // 跳过空行
            if trimmed.is_empty() {
                in_comment = false;
                continue;
            }

            // 注释行
            if trimmed.starts_with('#') && in_comment {
                control.comments.push(trimmed[1..].trim().to_string());
                continue;
            }

            in_comment = false;

            // 解析签名行
            if trimmed.starts_with("flf2a") || trimmed.starts_with("tlf2a") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                control.signature = Some(parts[0].to_string());

                // 硬空白字符
                if parts[0].len() > 5 {
                    control.hard_blank = parts[0].chars().nth(5);
                }

                // 头部参数
                if parts.len() > 1 {
                    control.height = parts.get(1).and_then(|s| s.parse().ok());
                }
                if parts.len() > 2 {
                    control.baseline = parts.get(2).and_then(|s| s.parse().ok());
                }
                if parts.len() > 3 {
                    control.max_length = parts.get(3).and_then(|s| s.parse().ok());
                }
                if parts.len() > 4 {
                    control.old_layout = parts.get(4).and_then(|s| s.parse().ok());
                }
                if parts.len() > 5 {
                    control.comment_lines = parts.get(5).and_then(|s| s.parse().ok());
                }
                if parts.len() > 6 {
                    control.print_direction = parts.get(6).and_then(|s| s.parse().ok());
                }
                if parts.len() > 7 {
                    control.full_layout = parts.get(7).and_then(|s| s.parse().ok());
                }
                if parts.len() > 8 {
                    control.codetag_count = parts.get(8).and_then(|s| s.parse().ok());
                }
                continue;
            }

            // 解析标签: tag value
            if let Some((tag, value)) = parse_tag_line(trimmed) {
                match tag {
                    "baseline" => control.baseline = value.parse().ok(),
                    "height" => control.height = value.parse().ok(),
                    "maxlen" | "max_length" => control.max_length = value.parse().ok(),
                    "oldlayout" | "old_layout" => control.old_layout = value.parse().ok(),
                    "fulllayout" | "full_layout" => control.full_layout = value.parse().ok(),
                    "codetag_count" => control.codetag_count = value.parse().ok(),
                    "printdir" | "print_direction" => control.print_direction = value.parse().ok(),
                    "hardblank" | "hard_blank" => {
                        control.hard_blank = value.chars().next();
                    }
                    "char" => {
                        // 格式: char <code> <glyph_name>
                        let parts: Vec<&str> = value.splitn(2, ' ').collect();
                        if parts.len() == 2 {
                            if let Ok(code) = u32::from_str_radix(parts[0].trim_start_matches("0x"), 16) {
                                if let Some(ch) = char::from_u32(code) {
                                    control.char_mappings.insert(ch, parts[1].to_string());
                                }
                            }
                        }
                    }
                    _ => {
                        // 未知标签，忽略
                    }
                }
                continue;
            }

            // 尝试解析 Unicode 范围映射
            if let Some(range) = parse_unicode_range(trimmed) {
                control.unicode_ranges.push(range);
            }
        }

        Ok(control)
    }

    /// 将 control 文件的效果应用到字体数据上
    pub fn apply_to_font_data(&self, data: &mut crate::figfont::FontData) {
        if let Some(h) = self.height {
            data.header.height = h;
        }
        if let Some(b) = self.baseline {
            data.header.baseline = b;
        }
        if let Some(m) = self.max_length {
            data.header.max_length = m;
        }
        if let Some(o) = self.old_layout {
            data.header.old_layout = o;
        }
        if let Some(f) = self.full_layout {
            data.header.full_layout = f;
        }
        if let Some(c) = self.codetag_count {
            data.header.codetag_count = c;
        }
        if let Some(p) = self.print_direction {
            data.header.print_direction = p;
        }
        if let Some(hb) = self.hard_blank {
            data.header.hard_blank = hb;
        }
    }
}

/// 解析 "tag value" 格式行
fn parse_tag_line(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    // 跳过注释和空行
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    // 查找标签和值之间的分隔
    if let Some(pos) = line.find(|c: char| c.is_whitespace()) {
        let tag = &line[..pos];
        let value = line[pos..].trim();
        if !tag.is_empty() && !value.is_empty() {
            return Some((tag, value));
        }
    }

    None
}

/// 尝试解析 Unicode 范围映射行
/// 格式: 0xXXXX-0xYYYY <n>
fn parse_unicode_range(line: &str) -> Option<UnicodeRange> {
    let line = line.trim();
    if !line.starts_with("0x") && !line.starts_with("0X") {
        return None;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let range_part = parts[0];
    let index_part = parts[1];

    if let Some(dash) = range_part.find('-') {
        let start_str = &range_part[..dash].trim();
        let end_str = range_part[dash + 1..].trim();

        if let (Ok(start), Ok(end), Ok(index)) = (
            u32::from_str_radix(start_str.trim_start_matches("0x").trim_start_matches("0X"), 16),
            u32::from_str_radix(end_str.trim_start_matches("0x").trim_start_matches("0X"), 16),
            index_part.parse::<usize>(),
        ) {
            return Some(UnicodeRange {
                start,
                end,
                font_index: index,
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_control() {
        let content = "# Test control file\nflf2a$ 8 7 15 0 1 0 0 0\n";
        let control = ControlFile::parse(content).unwrap();
        assert_eq!(control.signature.as_deref(), Some("flf2a$"));
        assert_eq!(control.hard_blank, Some('$'));
        assert_eq!(control.height, Some(8));
        assert_eq!(control.baseline, Some(7));
    }

    #[test]
    fn test_parse_tag_lines() {
        let content = "flf2a$\nbaseline 5\nheight 10\nmaxlen 20\n";
        let control = ControlFile::parse(content).unwrap();
        assert_eq!(control.baseline, Some(5));
        assert_eq!(control.height, Some(10));
        assert_eq!(control.max_length, Some(20));
    }

    #[test]
    fn test_parse_char_mapping() {
        let content = "flf2a$\nchar 0x41 A\nchar 0x42 B\n";
        let control = ControlFile::parse(content).unwrap();
        assert_eq!(control.char_mappings.get(&'A'), Some(&"A".to_string()));
        assert_eq!(control.char_mappings.get(&'B'), Some(&"B".to_string()));
    }
}
