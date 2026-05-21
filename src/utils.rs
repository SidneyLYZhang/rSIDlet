/// 检测文本中是否包含中文字符
pub fn contains_chinese(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(c,
            '\u{4E00}'..='\u{9FFF}' |
            '\u{3400}'..='\u{4DBF}' |
            '\u{F900}'..='\u{FAFF}'
        )
    })
}

/// ANSI 颜色代码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
    Reset = 0,
}

/// 颜色滤镜类型，对应 toilet 的 filter 参数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFilter {
    /// 无颜色
    None,
    /// 彩虹色（逐字符渐变）
    Rainbow,
    /// 彩虹色（逐行渐变）
    RainbowLine,
    /// 金属色
    Metal,
    /// 火焰色
    Fire,
    /// 水色
    Water,
    /// 随机颜色
    Random,
    /// 纯前景色
    Foreground(AnsiColor),
    /// 渐变（起始色 -> 结束色）
    Gradient { start: AnsiColor, end: AnsiColor },
}

impl ColorFilter {
    /// 将颜色滤镜应用到多行 ASCII 艺术输出上
    pub fn apply(&self, lines: &[String]) -> Vec<String> {
        if lines.is_empty() {
            return lines.to_vec();
        }

        match self {
            ColorFilter::None => lines.to_vec(),
            ColorFilter::Foreground(color) => {
                lines.iter()
                    .map(|line| color_line(line, *color))
                    .collect()
            }
            ColorFilter::Rainbow => apply_rainbow_char(lines),
            ColorFilter::RainbowLine => apply_rainbow_line(lines),
            ColorFilter::Metal => apply_gradient(lines, AnsiColor::BrightBlack, AnsiColor::White),
            ColorFilter::Fire => apply_gradient(lines, AnsiColor::Red, AnsiColor::BrightYellow),
            ColorFilter::Water => apply_gradient(lines, AnsiColor::Blue, AnsiColor::BrightCyan),
            ColorFilter::Random => apply_random(lines),
            ColorFilter::Gradient { start, end } => apply_gradient(lines, *start, *end),
        }
    }

    /// 将颜色滤镜应用到单行文本上
    pub fn apply_to_line(&self, line: &str, line_index: usize, total_lines: usize) -> String {
        match self {
            ColorFilter::None => line.to_string(),
            ColorFilter::Foreground(color) => color_line(line, *color),
            ColorFilter::Rainbow => rainbow_chars(line),
            ColorFilter::RainbowLine => {
                if total_lines > 1 {
                    let ratio = line_index as f32 / (total_lines.saturating_sub(1)) as f32;
                    let color = rainbow_at(ratio);
                    color_line(line, color)
                } else {
                    color_line(line, AnsiColor::Red)
                }
            }
            ColorFilter::Metal => {
                let ratio = if total_lines > 1 {
                    line_index as f32 / (total_lines.saturating_sub(1)) as f32
                } else {
                    0.5
                };
                let color = gradient_color(ratio, AnsiColor::BrightBlack, AnsiColor::White);
                color_line(line, color)
            }
            ColorFilter::Fire => {
                let ratio = if total_lines > 1 {
                    line_index as f32 / (total_lines.saturating_sub(1)) as f32
                } else {
                    0.5
                };
                let color = gradient_color(ratio, AnsiColor::Red, AnsiColor::BrightYellow);
                color_line(line, color)
            }
            ColorFilter::Water => {
                let ratio = if total_lines > 1 {
                    line_index as f32 / (total_lines.saturating_sub(1)) as f32
                } else {
                    0.5
                };
                let color = gradient_color(ratio, AnsiColor::Blue, AnsiColor::BrightCyan);
                color_line(line, color)
            }
            ColorFilter::Random => random_color_line(line),
            ColorFilter::Gradient { start, end } => {
                let ratio = if total_lines > 1 {
                    line_index as f32 / (total_lines.saturating_sub(1)) as f32
                } else {
                    0.5
                };
                let color = gradient_color(ratio, *start, *end);
                color_line(line, color)
            }
        }
    }
}

/// 为整行应用单一颜色
fn color_line(line: &str, color: AnsiColor) -> String {
    if line.is_empty() {
        return line.to_string();
    }
    format!("\x1b[{}m{}\x1b[0m", color as u8, line)
}

/// 逐字符彩虹色
fn rainbow_chars(line: &str) -> String {
    if line.is_empty() {
        return line.to_string();
    }
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len().max(1);
    let mut result = String::new();
    for (i, ch) in chars.iter().enumerate() {
        let color = rainbow_at(i as f32 / (len - 1).max(1) as f32);
        result.push_str(&format!("\x1b[{}m{}\x1b[0m", color as u8, ch));
    }
    result
}

/// 逐字符彩虹色应用到多行
fn apply_rainbow_char(lines: &[String]) -> Vec<String> {
    let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let max_width = max_width.max(1);
    lines.iter()
        .map(|line| {
            if line.is_empty() {
                return line.clone();
            }
            let chars: Vec<char> = line.chars().collect();
            let mut result = String::new();
            for (i, ch) in chars.iter().enumerate() {
                let color = rainbow_at(i as f32 / (max_width - 1).max(1) as f32);
                result.push_str(&format!("\x1b[{}m{}\x1b[0m", color as u8, ch));
            }
            result
        })
        .collect()
}

/// 逐行彩虹色
fn apply_rainbow_line(lines: &[String]) -> Vec<String> {
    let total = lines.len().max(1);
    lines.iter().enumerate()
        .map(|(i, line)| {
            if line.is_empty() {
                return line.clone();
            }
            let color = rainbow_at(i as f32 / (total - 1).max(1) as f32);
            color_line(line, color)
        })
        .collect()
}

/// 渐变颜色应用
fn apply_gradient(lines: &[String], start: AnsiColor, end: AnsiColor) -> Vec<String> {
    let total = lines.len().max(1);
    lines.iter().enumerate()
        .map(|(i, line)| {
            if line.is_empty() {
                return line.clone();
            }
            let ratio = i as f32 / (total - 1).max(1) as f32;
            let color = gradient_color(ratio, start, end);
            color_line(line, color)
        })
        .collect()
}

/// 随机颜色
fn apply_random(lines: &[String]) -> Vec<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    lines.iter()
        .map(|line| {
            if line.is_empty() {
                return line.clone();
            }
            let mut hasher = DefaultHasher::new();
            line.hash(&mut hasher);
            let hash = hasher.finish();
            let colors = [
                AnsiColor::Red, AnsiColor::Green, AnsiColor::Yellow,
                AnsiColor::Blue, AnsiColor::Magenta, AnsiColor::Cyan,
                AnsiColor::BrightRed, AnsiColor::BrightGreen, AnsiColor::BrightYellow,
                AnsiColor::BrightBlue, AnsiColor::BrightMagenta, AnsiColor::BrightCyan,
            ];
            let color = colors[(hash % colors.len() as u64) as usize];
            color_line(line, color)
        })
        .collect()
}

fn random_color_line(line: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    if line.is_empty() {
        return line.to_string();
    }
    let mut hasher = DefaultHasher::new();
    line.hash(&mut hasher);
    let hash = hasher.finish();
    let colors = [
        AnsiColor::Red, AnsiColor::Green, AnsiColor::Yellow,
        AnsiColor::Blue, AnsiColor::Magenta, AnsiColor::Cyan,
        AnsiColor::BrightRed, AnsiColor::BrightGreen, AnsiColor::BrightYellow,
        AnsiColor::BrightBlue, AnsiColor::BrightMagenta, AnsiColor::BrightCyan,
    ];
    let color = colors[(hash % colors.len() as u64) as usize];
    color_line(line, color)
}

/// 在 [0,1] 位置获取彩虹色
fn rainbow_at(t: f32) -> AnsiColor {
    let t = t.clamp(0.0, 1.0);
    let positions = [
        (0.0, AnsiColor::Red),
        (0.17, AnsiColor::Yellow),
        (0.33, AnsiColor::Green),
        (0.5, AnsiColor::Cyan),
        (0.67, AnsiColor::Blue),
        (0.83, AnsiColor::Magenta),
        (1.0, AnsiColor::Red),
    ];

    for i in 0..positions.len() - 1 {
        if t >= positions[i].0 && t <= positions[i + 1].0 {
            let range = positions[i + 1].0 - positions[i].0;
            let local_t = if range > 0.0 { (t - positions[i].0) / range } else { 0.0 };
            return if local_t < 0.5 {
                positions[i].1
            } else {
                positions[i + 1].1
            };
        }
    }
    AnsiColor::Red
}

/// 在两个颜色之间进行线性插值
fn gradient_color(t: f32, start: AnsiColor, end: AnsiColor) -> AnsiColor {
    let t = t.clamp(0.0, 1.0);
    let start_rgb = ansi_to_rgb(start);
    let end_rgb = ansi_to_rgb(end);

    let r = (start_rgb.0 as f32 * (1.0 - t) + end_rgb.0 as f32 * t) as u8;
    let g = (start_rgb.1 as f32 * (1.0 - t) + end_rgb.1 as f32 * t) as u8;
    let b = (start_rgb.2 as f32 * (1.0 - t) + end_rgb.2 as f32 * t) as u8;

    rgb_to_ansi(r, g, b)
}

/// ANSI 颜色到 RGB 的近似映射
fn ansi_to_rgb(color: AnsiColor) -> (u8, u8, u8) {
    match color {
        AnsiColor::Black => (0, 0, 0),
        AnsiColor::Red => (170, 0, 0),
        AnsiColor::Green => (0, 170, 0),
        AnsiColor::Yellow => (170, 170, 0),
        AnsiColor::Blue => (0, 0, 170),
        AnsiColor::Magenta => (170, 0, 170),
        AnsiColor::Cyan => (0, 170, 170),
        AnsiColor::White => (170, 170, 170),
        AnsiColor::BrightBlack => (85, 85, 85),
        AnsiColor::BrightRed => (255, 85, 85),
        AnsiColor::BrightGreen => (85, 255, 85),
        AnsiColor::BrightYellow => (255, 255, 85),
        AnsiColor::BrightBlue => (85, 85, 255),
        AnsiColor::BrightMagenta => (255, 85, 255),
        AnsiColor::BrightCyan => (85, 255, 255),
        AnsiColor::BrightWhite => (255, 255, 255),
        AnsiColor::Reset => (255, 255, 255),
    }
}

/// RGB 到最近 ANSI 颜色的映射
fn rgb_to_ansi(r: u8, g: u8, b: u8) -> AnsiColor {
    let colors = [
        AnsiColor::Black, AnsiColor::Red, AnsiColor::Green, AnsiColor::Yellow,
        AnsiColor::Blue, AnsiColor::Magenta, AnsiColor::Cyan, AnsiColor::White,
        AnsiColor::BrightBlack, AnsiColor::BrightRed, AnsiColor::BrightGreen,
        AnsiColor::BrightYellow, AnsiColor::BrightBlue, AnsiColor::BrightMagenta,
        AnsiColor::BrightCyan, AnsiColor::BrightWhite,
    ];

    let mut best = AnsiColor::White;
    let mut best_dist = u32::MAX;

    for &color in &colors {
        let rgb = ansi_to_rgb(color);
        let dr = (rgb.0 as i32 - r as i32).abs() as u32;
        let dg = (rgb.1 as i32 - g as i32).abs() as u32;
        let db = (rgb.2 as i32 - b as i32).abs() as u32;
        let dist = dr * dr + dg * dg + db * db;

        if dist < best_dist {
            best_dist = dist;
            best = color;
        }
    }

    best
}

/// 将多行字符串打印到终端
pub fn print_lines(lines: &[String]) {
    for line in lines {
        println!("{}", line);
    }
}

/// 将多行字符串使用颜色滤镜打印到终端
pub fn print_colored(lines: &[String], filter: ColorFilter) {
    let colored = filter.apply(lines);
    print_lines(&colored);
}

/// 支持横向拼接两个多行文本块
pub fn hcat(left: &[String], right: &[String], padding: usize) -> Vec<String> {
    let max_lines = left.len().max(right.len());
    let pad = " ".repeat(padding);
    (0..max_lines)
        .map(|i| {
            let l = left.get(i).map(|s| s.as_str()).unwrap_or("");
            let r = right.get(i).map(|s| s.as_str()).unwrap_or("");
            format!("{}{}{}", l, pad, r)
        })
        .collect()
}

/// 根据名称解析颜色滤镜
pub fn parse_filter(name: &str) -> Option<ColorFilter> {
    match name.to_lowercase().as_str() {
        "none" | "" => Some(ColorFilter::None),
        "rainbow" => Some(ColorFilter::Rainbow),
        "rainbowline" | "rainbow_line" => Some(ColorFilter::RainbowLine),
        "metal" => Some(ColorFilter::Metal),
        "fire" => Some(ColorFilter::Fire),
        "water" => Some(ColorFilter::Water),
        "random" => Some(ColorFilter::Random),
        "red" => Some(ColorFilter::Foreground(AnsiColor::Red)),
        "green" => Some(ColorFilter::Foreground(AnsiColor::Green)),
        "yellow" => Some(ColorFilter::Foreground(AnsiColor::Yellow)),
        "blue" => Some(ColorFilter::Foreground(AnsiColor::Blue)),
        "magenta" => Some(ColorFilter::Foreground(AnsiColor::Magenta)),
        "cyan" => Some(ColorFilter::Foreground(AnsiColor::Cyan)),
        "white" => Some(ColorFilter::Foreground(AnsiColor::White)),
        "black" => Some(ColorFilter::Foreground(AnsiColor::Black)),
        _ => None,
    }
}

/// 获取所有可用的滤镜名称列表
pub fn available_filters() -> Vec<&'static str> {
    vec![
        "none", "rainbow", "rainbowline", "metal", "fire", "water", "random",
        "red", "green", "yellow", "blue", "magenta", "cyan", "white", "black",
    ]
}
