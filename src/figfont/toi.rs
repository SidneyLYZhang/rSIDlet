use std::io;

use super::{parse_font_base, FontData, FigFont, Glyph};
use crate::utils::ColorFilter;

/// Toilet 字体 (.tlf)
///
/// Toilet 字体是 figlet 格式的扩展，增加了对 Unicode、
/// 颜色标记和滤镜的支持。文件扩展名通常为 `.tlf`。
///
/// Toilet 字体使用 `tlf2a` 签名代替 figlet 的 `flf2a`，
/// 其余头部格式与 figlet 完全相同。
#[derive(Debug, Clone)]
pub struct ToiletFont {
    data: FontData,
    /// Toilet 特有的扩展标记
    pub has_unicode: bool,
    pub has_color_tags: bool,
}

impl ToiletFont {
    /// 创建新的 ToiletFont（内部使用）
    pub(crate) fn new(data: FontData) -> Self {
        let has_unicode = data.header.codetag_count > 0;
        let has_color_tags = data.header.full_layout > 0;

        ToiletFont {
            data,
            has_unicode,
            has_color_tags,
        }
    }

    /// 将字体转换为原始数据（用于统一处理）
    pub(crate) fn into_data(self) -> FontData {
        self.data
    }

    /// 使用默认的 future.tlf 字体文件加载
    ///
    /// 从 `../fonts/future.tlf` 路径加载标准 toilet 字体。
    /// 这是 toilet 工具的经典默认字体之一。
    ///
    /// # Examples
    /// ```no_run
    /// use rsidlet::figfont::ToiletFont;
    /// use rsidlet::figfont::FigFont;
    ///
    /// let font = ToiletFont::future().expect("Failed to load future font");
    /// let art = font.render("Hello");
    /// for line in &art {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn future() -> io::Result<Self> {
        Self::load("../fonts/future.tlf")
    }

    /// 渲染文本并应用颜色滤镜
    ///
    /// 将文本转换为 ASCII 艺术字，然后应用颜色滤镜输出。
    /// 这是 toilet 风格的核心功能之一。
    ///
    /// # Examples
    /// ```no_run
    /// use rsidlet::figfont::ToiletFont;
    /// use rsidlet::utils::ColorFilter;
    ///
    /// let font = ToiletFont::future().unwrap();
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
    pub fn builtin() -> Self {
        let data = create_builtin_future();
        ToiletFont::new(data)
    }

    /// 检查字体是否支持特定 Unicode 字符范围
    pub fn supports_range(&self, start: u32, end: u32) -> bool {
        (start..=end).all(|cp| self.data.glyphs.iter().any(|g| g.codepoint == cp))
    }

    /// 获取字体的颜色标记数量
    pub fn color_tag_count(&self) -> u32 {
        self.data.header.codetag_count
    }

    /// 获取字体的全布局模式
    pub fn full_layout_mode(&self) -> u32 {
        self.data.header.full_layout
    }
}

impl FigFont for ToiletFont {
    fn font_type() -> &'static str {
        "toilet"
    }

    fn extensions() -> &'static [&'static str] {
        &["tlf"]
    }

    fn parse(data: &[u8]) -> io::Result<Self> {
        let cursor = io::Cursor::new(data);
        let font_data = parse_font_base(cursor)?;

        // 验证 toilet 签名
        if !font_data.header.signature.starts_with("tlf") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expected toilet font (tlf2a), got: {}", font_data.header.signature),
            ));
        }

        Ok(ToiletFont::new(font_data))
    }

    fn data(&self) -> &FontData {
        &self.data
    }
}

/// 内置 future 风格字体数据（简化版，作为后备）
fn create_builtin_future() -> FontData {
    use super::{FontComment, FontHeader};

    let mut glyphs = Vec::new();

    // 空格 (32)
    glyphs.push(Glyph {
        codepoint: 32,
        lines: vec!["".into(); 6],
        width: 0,
        height: 6,
    });

    // 填充 33-64
    for cp in 33..=64 {
        let ch = std::char::from_u32(cp).unwrap_or('?');
        glyphs.push(Glyph {
            codepoint: cp,
            lines: vec![format!("{}", ch); 6],
            width: 1,
            height: 6,
        });
    }

    // Future 风格的大写字母 A-Z (65-90)
    let uppercase: Vec<(u32, Vec<&str>)> = vec![
        (65, vec![
            "  ____   ___  ",
            " / __ \\/   | ",
            "/ /_/ / /| | ",
            "| _, _/ ___ | ",
            "/_/ |/_/  |_| ",
            "              ",
        ]),
        (66, vec![
            " ____  ____  ",
            "| __ )| __ ) ",
            "|  _ \\|  _ \\ ",
            "| |_) | |_) |",
            "|____/|____/ ",
            "             ",
        ]),
        (67, vec![
            "  ________  ",
            " / ____/ /  ",
            "| |   / /   ",
            "| |  / /___ ",
            "|_| /_____/ ",
            "            ",
        ]),
        (68, vec![
            " ____  _____ ",
            "|  _ \\|__  / ",
            "| | | | /_ <  ",
            "| |_| |__/ /  ",
            "|____/____/   ",
            "              ",
        ]),
        (69, vec![
            " ______ _____ ",
            "|  ____| ____|",
            "| |__  | |__   ",
            "|  __| |___ \\  ",
            "| |____ ___) | ",
            "|______|____/  ",
        ]),
        (70, vec![
            " ______ __    ",
            "|  ____/_/    ",
            "| |__ _ _ __  ",
            "|  __| | '_ \\ ",
            "| |  | | | | |",
            "|_|  |_|_| |_|",
        ]),
        (71, vec![
            "  _________   ",
            " / ___/ __ \\  ",
            "| |  | |  | | ",
            "| |__| |__| | ",
            " \\____\\____/  ",
            "              ",
        ]),
        (72, vec![
            " _    ___  ",
            "| |  /   | ",
            "| | / /| | ",
            "| |/ ___ | ",
            "|_/_/  |_| ",
            "           ",
        ]),
        (73, vec![
            " _____ ",
            "|_   _|",
            "  | |  ",
            " _| |_  ",
            "|_____| ",
            "        ",
        ]),
        (74, vec![
            "    _____ ",
            "   |___ / ",
            "     |_ \\ ",
            "    ___) |",
            "   |____/ ",
            "          ",
        ]),
        (75, vec![
            " _  __________",
            "| |/ /__  /__ /",
            "| ' <  / / /_ \\",
            "| . \\ / /___) |",
            "|_|\\_/____/___/",
            "               ",
        ]),
        (76, vec![
            " _       ",
            "| |      ",
            "| |      ",
            "| |____  ",
            "|______| ",
            "         ",
        ]),
        (77, vec![
            " __  _______  __",
            "|  \\/  / __ \\/ /",
            "| |\\/ / / / / / ",
            "| |  / /_/ /_/ /  ",
            "|_|  \\____/___/   ",
            "                 ",
        ]),
        (78, vec![
            " _   ________ ",
            "| | / /  _/ / ",
            "| |/ // // /  ",
            "|  /// // /___ ",
            "|_/___/_____/ ",
            "              ",
        ]),
        (79, vec![
            "  ____  _____ ",
            " / __ \\/__  / ",
            "| |  | | /_ <  ",
            "| |__| |__/ /  ",
            " \\____/____/   ",
            "              ",
        ]),
        (80, vec![
            " ____  _____ ",
            "|  _ \\|__  / ",
            "| |_) | /_ <  ",
            "|  __/___/ /  ",
            "|_|   /____/   ",
            "              ",
        ]),
        (81, vec![
            "  ____  ___   ",
            " / __ \\/   |  ",
            "| |  | / /| | ",
            "| |__/// ___ | ",
            " \\____/_/  |_|",
            "              ",
        ]),
        (82, vec![
            " ____  ____  ",
            "|  _ \\|  _ \\ ",
            "| |_) | |_) |",
            "|  _ <|  __/ ",
            "|_| \\ \\__|   ",
            "    \\/       ",
        ]),
        (83, vec![
            "  _______  ",
            " / __/ _ \\ ",
            "| _// // / ",
            "|___/____/  ",
            "            ",
            "            ",
        ]),
        (84, vec![
            " _________   ",
            "|__  /__  |  ",
            "  / /  / /   ",
            " / /__/ /    ",
            "/______/     ",
            "             ",
        ]),
        (85, vec![
            " __  _______ ",
            "|  \\/  / __ \\",
            "| |\\/ / /_/ /",
            "| |  | | _, _/",
            "|_|  |_|/_/   ",
            "              ",
        ]),
        (86, vec![
            "__    ___________",
            "\\ \\  / /__  /__  /",
            " \\ \\/ /  / /  / / ",
            "  \\  /  / /  / /   ",
            "   \\/  /_/  /_/    ",
            "                   ",
        ]),
        (87, vec![
            " _      ____________",
            "| | /| / /__  /__  /",
            "| |/ |/ /  / /  / / ",
            "|__/|__/  / /  / /   ",
            "         /_/  /_/    ",
            "                    ",
        ]),
        (88, vec![
            "__  _________",
            "\\ \\/ /__  /_ /",
            " \\  /  / /(_)",
            " /  \\ / / _   ",
            "/_/\\_/___(_)  ",
            "              ",
        ]),
        (89, vec![
            "__   ________",
            "\\ \\ / /__  / ",
            " \\ V /  / /  ",
            "  | |  / /   ",
            "  |_| /_/    ",
            "             ",
        ]),
        (90, vec![
            " _____/__  / ",
            "|__  / /_ <  ",
            "  / /___/ /  ",
            " /_/_____/   ",
            "             ",
            "             ",
        ]),
    ];

    for (cp, lines) in uppercase {
        let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        glyphs.push(Glyph {
            codepoint: cp,
            lines,
            width,
            height: 6,
        });
    }

    // 填充 91-96
    for cp in 91..=96 {
        let ch = std::char::from_u32(cp).unwrap_or('?');
        glyphs.push(Glyph {
            codepoint: cp,
            lines: vec![format!("{}", ch); 6],
            width: 1,
            height: 6,
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
                lines: vec![format!("{}", ch); 6],
                width: 1,
                height: 6,
            });
        }
    }

    // 填充 123-126
    for cp in 123..=126 {
        let ch = std::char::from_u32(cp).unwrap_or('?');
        glyphs.push(Glyph {
            codepoint: cp,
            lines: vec![format!("{}", ch); 6],
            width: 1,
            height: 6,
        });
    }

    FontData {
        header: FontHeader {
            signature: "tlf2a".to_string(),
            hard_blank: '$',
            height: 6,
            baseline: 5,
            max_length: 15,
            old_layout: 0,
            comment_lines: 1,
            print_direction: 0,
            full_layout: 1,
            codetag_count: 0,
            mappings: Vec::new(),
        },
        comment: FontComment {
            lines: vec!["Builtin future font (toilet style)".to_string()],
        },
        glyphs,
    }
}
