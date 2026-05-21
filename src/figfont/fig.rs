use std::io;

use super::{parse_font_base, FontData, FigFont, Glyph};
use crate::utils::ColorFilter;

/// Figlet 字体 (.flf)
///
/// Figlet 字体是经典的 ASCII 艺术字体格式，
/// 文件扩展名通常为 `.flf`。
#[derive(Debug, Clone)]
pub struct FigletFont {
    data: FontData,
}

impl FigletFont {
    /// 创建新的 FigletFont（内部使用）
    pub(crate) fn new(data: FontData) -> Self {
        FigletFont { data }
    }

    /// 将字体转换为原始数据（用于统一处理）
    pub(crate) fn into_data(self) -> FontData {
        self.data
    }

    /// 使用默认的 standard.flf 字体文件加载
    ///
    /// 从 `../fonts/standard.flf` 路径加载标准 figlet 字体。
    /// 这是 figlet 工具的经典默认字体。
    ///
    /// # Examples
    /// ```no_run
    /// use rsidlet::figfont::FigletFont;
    /// use rsidlet::figfont::FigFont;
    ///
    /// let font = FigletFont::standard().expect("Failed to load standard font");
    /// let art = font.render("Hello");
    /// for line in &art {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn standard() -> io::Result<Self> {
        Self::load("../fonts/standard.flf")
    }

    /// 渲染文本并应用颜色滤镜
    ///
    /// 将文本转换为 ASCII 艺术字，然后应用颜色滤镜输出。
    ///
    /// # Examples
    /// ```no_run
    /// use rsidlet::figfont::FigletFont;
    /// use rsidlet::utils::ColorFilter;
    ///
    /// let font = FigletFont::standard().unwrap();
    /// let colored = font.render_colored("Hello", ColorFilter::Rainbow);
    /// for line in &colored {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn render_colored(&self, text: &str, filter: ColorFilter) -> Vec<String> {
        let lines = self.render(text);
        filter.apply(&lines)
    }

    /// 使用内置的占位符字体（当外部字体加载失败时使用）
    ///
    /// 提供一个简单的内置字体，确保即使没有外部字体文件也能工作。
    pub fn builtin() -> Self {
        let data = create_builtin_standard();
        FigletFont { data }
    }
}

impl FigFont for FigletFont {
    fn font_type() -> &'static str {
        "figlet"
    }

    fn extensions() -> &'static [&'static str] {
        &["flf"]
    }

    fn parse(data: &[u8]) -> io::Result<Self> {
        let cursor = io::Cursor::new(data);
        let font_data = parse_font_base(cursor)?;
        Ok(FigletFont::new(font_data))
    }

    fn data(&self) -> &FontData {
        &self.data
    }
}

/// 内置标准字体数据（简化版，作为后备）
fn create_builtin_standard() -> FontData {
    use super::{FontComment, FontHeader};

    let glyphs = create_builtin_glyphs();

    FontData {
        header: FontHeader {
            signature: "flf2a".to_string(),
            hard_blank: '$',
            height: 7,
            baseline: 6,
            max_length: 11,
            old_layout: 0,
            comment_lines: 1,
            print_direction: 0,
            full_layout: 0,
            codetag_count: 0,
            mappings: Vec::new(),
        },
        comment: FontComment {
            lines: vec!["Builtin standard font".to_string()],
        },
        glyphs,
    }
}

fn create_builtin_glyphs() -> Vec<Glyph> {
    let mut glyphs = Vec::new();

    // 空格 (32)
    glyphs.push(Glyph {
        codepoint: 32,
        lines: vec!["".into(); 7],
        width: 0,
        height: 7,
    });

    // 填充 33-64 的基础字符
    for cp in 33..=64 {
        let ch = std::char::from_u32(cp).unwrap_or('?');
        glyphs.push(Glyph {
            codepoint: cp,
            lines: vec![format!("{}", ch); 7],
            width: 1,
            height: 7,
        });
    }

    // 大写字母 A-Z (65-90)
    let uppercase: Vec<(u32, Vec<&str>)> = vec![
        (65, vec![
            "  __ _  ",
            " / _` | ",
            "| (_| | ",
            " \\__,_| ",
            "        ",
            "        ",
            "        ",
        ]),
        (66, vec![
            " _      ",
            "| |__   ",
            "| '_ \\  ",
            "| |_) | ",
            "|_.__/  ",
            "        ",
            "        ",
        ]),
        (67, vec![
            "  ___  ",
            " / __| ",
            "| (__  ",
            " \\___| ",
            "       ",
            "       ",
            "       ",
        ]),
        (68, vec![
            " ___   ",
            "|   \\  ",
            "| |) | ",
            "|___/  ",
            "       ",
            "       ",
            "       ",
        ]),
        (69, vec![
            " _____ ",
            "| ____|",
            "| __|  ",
            "|___|  ",
            "       ",
            "       ",
            "       ",
        ]),
        (70, vec![
            " _____ ",
            "|  ___|",
            "| |_   ",
            "|_|    ",
            "       ",
            "       ",
            "       ",
        ]),
        (71, vec![
            "  __ _ ",
            " / _` |",
            "| (_| |",
            " \\__, |",
            "  __/ |",
            " |___/ ",
            "       ",
        ]),
        (72, vec![
            " _   _ ",
            "| | | |",
            "| |_| |",
            " \\___/ ",
            "       ",
            "       ",
            "       ",
        ]),
        (73, vec![
            " ___ ",
            "|_ _|",
            " | | ",
            "|___|",
            "     ",
            "     ",
            "     ",
        ]),
        (74, vec![
            "    _ ",
            "   | |",
            " _ | |",
            "(_)|_|",
            "      ",
            "      ",
            "      ",
        ]),
        (75, vec![
            " _  __",
            "| |/ /",
            "| ' < ",
            "|_|\\_\\",
            "      ",
            "      ",
            "      ",
        ]),
        (76, vec![
            " _     ",
            "| |    ",
            "| |__  ",
            "|____| ",
            "       ",
            "       ",
            "       ",
        ]),
        (77, vec![
            " __  __ ",
            "|  \\/  |",
            "| |\\/| |",
            "|_|  |_|",
            "        ",
            "        ",
            "        ",
        ]),
        (78, vec![
            " _   _ ",
            "| \\ | |",
            "|  \\| |",
            "|_| \\_|",
            "       ",
            "       ",
            "       ",
        ]),
        (79, vec![
            "  ___  ",
            " / _ \\ ",
            "| (_) |",
            " \\___/ ",
            "       ",
            "       ",
            "       ",
        ]),
        (80, vec![
            " ____  ",
            "|  _ \\ ",
            "| |_) |",
            "|  __/ ",
            "|_|    ",
            "       ",
            "       ",
        ]),
        (81, vec![
            "  ___  ",
            " / _ \\ ",
            "| | | |",
            " \\__\\_\\",
            "       ",
            "       ",
            "       ",
        ]),
        (82, vec![
            " ____  ",
            "|  _ \\ ",
            "| |_) |",
            "| _ <  ",
            "|_| \\__\\",
            "       ",
            "       ",
        ]),
        (83, vec![
            " ____  ",
            "/ ___| ",
            "\\___ \\ ",
            " ___) |",
            "|____/ ",
            "       ",
            "       ",
        ]),
        (84, vec![
            " _____ ",
            "|_   _|",
            "  | |  ",
            "  |_|  ",
            "       ",
            "       ",
            "       ",
        ]),
        (85, vec![
            " _   _ ",
            "| | | |",
            "| |_| |",
            " \\___,_|",
            "       ",
            "       ",
            "       ",
        ]),
        (86, vec![
            "__     __",
            "\\ \\   / /",
            " \\ \\ / / ",
            "  \\ V /  ",
            "   \\_/   ",
            "         ",
            "         ",
        ]),
        (87, vec![
            "__        __",
            "\\ \\      / /",
            " \\ \\ /\\ / / ",
            "  \\ V  V /  ",
            "   \\_/\\_/   ",
            "            ",
            "            ",
        ]),
        (88, vec![
            "__  __",
            "\\ \\/ /",
            " \\  / ",
            " /  \\ ",
            "/_/\\_\\",
            "      ",
            "      ",
        ]),
        (89, vec![
            "__   __",
            "\\ \\ / /",
            " \\ V / ",
            "  | |  ",
            "  |_|  ",
            "       ",
            "       ",
        ]),
        (90, vec![
            " _____",
            "|_   /",
            "  / / ",
            " /___|",
            "      ",
            "      ",
            "      ",
        ]),
    ];

    for (cp, lines) in uppercase {
        let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        glyphs.push(Glyph {
            codepoint: cp,
            lines,
            width,
            height: 7,
        });
    }

    // 填充 91-96
    for cp in 91..=96 {
        let ch = std::char::from_u32(cp).unwrap_or('?');
        glyphs.push(Glyph {
            codepoint: cp,
            lines: vec![format!("{}", ch); 7],
            width: 1,
            height: 7,
        });
    }

    // 小写字母 a-z (97-122) - 使用大写作为后备
    for cp in 97..=122 {
        let upper_cp = cp - 32;
        if let Some(g) = glyphs.iter().find(|g| g.codepoint == upper_cp) {
            let mut cloned = g.clone();
            cloned.codepoint = cp;
            glyphs.push(cloned);
        } else {
            let ch = std::char::from_u32(cp).unwrap_or('?');
            glyphs.push(Glyph {
                codepoint: cp,
                lines: vec![format!("{}", ch); 7],
                width: 1,
                height: 7,
            });
        }
    }

    // 填充 123-126
    for cp in 123..=126 {
        let ch = std::char::from_u32(cp).unwrap_or('?');
        glyphs.push(Glyph {
            codepoint: cp,
            lines: vec![format!("{}", ch); 7],
            width: 1,
            height: 7,
        });
    }

    glyphs
}
