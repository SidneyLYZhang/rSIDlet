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

/// 根据名称解析颜色滤镜（委托给 [ColorMap]）。
pub fn parse_filter(name: &str) -> Option<ColorFilter> {
    ColorMap::parse(name).and_then(|cm| cm.to_color_filter())
}

/// 获取所有可用的滤镜名称列表（委托给 [ColorMap]）。
pub fn available_filters() -> Vec<&'static str> {
    ColorMap::available()
        .into_iter()
        .filter(|(_, _, cat)| *cat == "颜色遮蔽")
        .map(|(name, _, _)| name)
        .collect()
}

// ============================================================
// 画布滤镜系统（Canvas Filter System）
// 对标 libcaca 的滤镜管线：crop / rainbow / metal / flip / flop / rotate / border
// ============================================================

/// 画布颜色常量 —— 映射 libcaca 的 CACA_* 到 [AnsiColor]
///
/// `TRANSPARENT` 不在 AnsiColor 中，而是通过 [Canvas::set_color] 的 `bg: Option<AnsiColor>`
/// 参数表示：`None` = 透明（不改变背景色）。
pub mod canvas_color {
    use super::AnsiColor;

    pub const LIGHTBLUE: AnsiColor = AnsiColor::BrightBlue;
    pub const BLUE: AnsiColor = AnsiColor::Blue;
    pub const LIGHTGRAY: AnsiColor = AnsiColor::BrightWhite;
    pub const DARKGRAY: AnsiColor = AnsiColor::BrightBlack;
    pub const LIGHTMAGENTA: AnsiColor = AnsiColor::BrightMagenta;
    pub const LIGHTRED: AnsiColor = AnsiColor::BrightRed;
    pub const YELLOW: AnsiColor = AnsiColor::Yellow;
    pub const LIGHTGREEN: AnsiColor = AnsiColor::BrightGreen;
    pub const LIGHTCYAN: AnsiColor = AnsiColor::BrightCyan;
}

/// 画布操作抽象。
///
/// 实现者只需提供基本的像素读写与几何变换；滤镜系统通过此 trait 与任意画布后端解耦。
pub trait Canvas {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_char(&self, x: u32, y: u32) -> char;
    /// 设置指定单元格的前景色与背景色。`bg` 为 `None` 时表示透明（不改变背景）。
    fn set_color(&mut self, x: u32, y: u32, fg: AnsiColor, bg: Option<AnsiColor>);
    fn put_char(&mut self, x: u32, y: u32, ch: char);
    fn flip(&mut self);
    fn flop(&mut self);
    fn rotate_180(&mut self);
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    /// 设置画布可视边界。(x, y) 可为负值以扩展画布。
    fn set_boundaries(&mut self, x: i32, y: i32, w: u32, h: u32);
}

/// 画布滤镜。
///
/// 使用枚举而非 trait object 以：
/// - 避免堆分配
/// - 支持 `match` 穷尽检查
/// - 实现 `Copy` / `Eq`，便于存入 `Vec` 和比较
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasFilter {
    /// 裁切空白区域
    Crop,
    /// 彩虹色渐变效果
    Rainbow,
    /// 金属色效果
    Metal,
    /// 水平翻转
    Flip,
    /// 垂直翻转
    Flop,
    /// 旋转 180 度
    Rotate180,
    /// 旋转 90 度（逆时针）
    RotateLeft,
    /// 旋转 90 度（顺时针）
    RotateRight,
    /// 用方框字符包围内容
    Border,
}

/// 滤镜解析/应用错误
#[derive(Debug, Clone)]
pub enum FilterError {
    /// 未知滤镜名
    UnknownFilter(String),
    /// 画布尺寸为零
    EmptyCanvas,
}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFilter(name) => write!(f, "未知的滤镜: \"{}\"", name),
            Self::EmptyCanvas => write!(f, "画布尺寸为零，无法应用滤镜"),
        }
    }
}

impl std::error::Error for FilterError {}

/// 滤镜管线上下文。
///
/// 持有画布、已注册的滤镜序列以及用于动画效果的行计数器。
pub struct FilterContext<C: Canvas> {
    pub canvas: C,
    /// 动画行号，每次调用 [apply_filters](Self::apply_filters) 后自增
    pub lines: u32,
    filters: Vec<CanvasFilter>,
}

impl<C: Canvas> FilterContext<C> {
    pub fn new(canvas: C) -> Self {
        Self { canvas, lines: 0, filters: Vec::new() }
    }

    /// 从 `':'` 分隔的字符串解析并注册滤镜。
    ///
    /// 支持 `"rotate"` 作为 `"180"` 的别名以保持向后兼容。
    /// 空段（如 `"flip::flop"`）会被安全跳过。
    pub fn add_filters(&mut self, spec: &str) -> Result<(), FilterError> {
        for name in spec.split(':') {
            let name = name.trim();
            if name.is_empty() {
                continue;
            }
            let filter = CanvasFilter::parse(name)?;
            self.filters.push(filter);
        }
        Ok(())
    }

    /// 按顺序应用所有已注册的滤镜。
    pub fn apply_filters(&mut self) -> Result<(), FilterError> {
        if self.canvas.width() == 0 || self.canvas.height() == 0 {
            return Err(FilterError::EmptyCanvas);
        }
        // 拷贝一份避免同时持有 &self.filters 和 &mut self
        let filters: Vec<CanvasFilter> = self.filters.iter().copied().collect();
        for filter in filters {
            filter.execute(self);
        }
        self.lines += 1;
        Ok(())
    }
}

impl CanvasFilter {
    /// 从名称解析滤镜。
    fn parse(name: &str) -> Result<Self, FilterError> {
        match name {
            "crop" => Ok(Self::Crop),
            "rainbow" => Ok(Self::Rainbow),
            "metal" => Ok(Self::Metal),
            "flip" => Ok(Self::Flip),
            "flop" => Ok(Self::Flop),
            "rotate" | "180" => Ok(Self::Rotate180), // "rotate" 向后兼容 C 版本
            "left" => Ok(Self::RotateLeft),
            "right" => Ok(Self::RotateRight),
            "border" => Ok(Self::Border),
            other => Err(FilterError::UnknownFilter(other.to_string())),
        }
    }

    /// 滤镜对应的命令行名称。
    pub fn name(self) -> &'static str {
        match self {
            Self::Crop => "crop",
            Self::Rainbow => "rainbow",
            Self::Metal => "metal",
            Self::Flip => "flip",
            Self::Flop => "flop",
            Self::Rotate180 => "180",
            Self::RotateLeft => "left",
            Self::RotateRight => "right",
            Self::Border => "border",
        }
    }

    /// 滤镜的中文描述。
    pub fn description(self) -> &'static str {
        match self {
            Self::Crop => "裁切未使用的空白区域",
            Self::Rainbow => "添加彩虹色渐变效果",
            Self::Metal => "添加金属色效果",
            Self::Flip => "水平翻转",
            Self::Flop => "垂直翻转",
            Self::Rotate180 => "旋转 180 度",
            Self::RotateLeft => "逆时针旋转 90 度",
            Self::RotateRight => "顺时针旋转 90 度",
            Self::Border => "用方框字符包围内容",
        }
    }

    /// 所有可用滤镜及其描述。
    pub fn available() -> Vec<(Self, &'static str)> {
        vec![
            (Self::Crop, Self::Crop.description()),
            (Self::Rainbow, Self::Rainbow.description()),
            (Self::Metal, Self::Metal.description()),
            (Self::Flip, Self::Flip.description()),
            (Self::Flop, Self::Flop.description()),
            (Self::Rotate180, Self::Rotate180.description()),
            (Self::RotateLeft, Self::RotateLeft.description()),
            (Self::RotateRight, Self::RotateRight.description()),
            (Self::Border, Self::Border.description()),
        ]
    }

    /// 执行滤镜（内部方法，由 [FilterContext::apply_filters] 调用）。
    fn execute<C: Canvas>(self, ctx: &mut FilterContext<C>) {
        match self {
            Self::Crop => apply_crop(ctx),
            Self::Rainbow => apply_rainbow(ctx),
            Self::Metal => apply_metal(ctx),
            Self::Flip => ctx.canvas.flip(),
            Self::Flop => ctx.canvas.flop(),
            Self::Rotate180 => ctx.canvas.rotate_180(),
            Self::RotateLeft => ctx.canvas.rotate_left(),
            Self::RotateRight => ctx.canvas.rotate_right(),
            Self::Border => apply_border(ctx),
        }
    }
}

// ---- 滤镜实现 ----

/// 裁切画布到非空字符的包围盒。
fn apply_crop<C: Canvas>(ctx: &mut FilterContext<C>) {
    let w = ctx.canvas.width();
    let h = ctx.canvas.height();
    let mut xmin = w;
    let mut xmax = 0u32;
    let mut ymin = h;
    let mut ymax = 0u32;

    for y in 0..h {
        for x in 0..w {
            if ctx.canvas.get_char(x, y) != ' ' {
                if x < xmin {
                    xmin = x;
                }
                if x > xmax {
                    xmax = x;
                }
                if y < ymin {
                    ymin = y;
                }
                if y > ymax {
                    ymax = y;
                }
            }
        }
    }

    if xmax >= xmin && ymax >= ymin {
        ctx.canvas
            .set_boundaries(xmin as i32, ymin as i32, xmax - xmin + 1, ymax - ymin + 1);
    }
}

/// 彩虹色：对非空字符按 `(x/2 + y + lines) % 6` 循环着色。
fn apply_rainbow<C: Canvas>(ctx: &mut FilterContext<C>) {
    use canvas_color::*;

    let palette = [LIGHTMAGENTA, LIGHTRED, YELLOW, LIGHTGREEN, LIGHTCYAN, LIGHTBLUE];
    let w = ctx.canvas.width();
    let h = ctx.canvas.height();

    for y in 0..h {
        for x in 0..w {
            let ch = ctx.canvas.get_char(x, y);
            if ch != ' ' {
                let idx = ((x / 2 + y + ctx.lines) % 6) as usize;
                ctx.canvas.set_color(x, y, palette[idx], None);
                ctx.canvas.put_char(x, y, ch);
            }
        }
    }
}

/// 金属色：对非空字符按 `(lines + y + x/8) / 2 % 4` 循环着色。
fn apply_metal<C: Canvas>(ctx: &mut FilterContext<C>) {
    use canvas_color::*;

    let palette = [LIGHTBLUE, BLUE, LIGHTGRAY, DARKGRAY];
    let w = ctx.canvas.width();
    let h = ctx.canvas.height();

    for y in 0..h {
        for x in 0..w {
            let ch = ctx.canvas.get_char(x, y);
            if ch == ' ' {
                continue;
            }
            let idx = (((ctx.lines + y + x / 8) / 2) % 4) as usize;
            ctx.canvas.set_color(x, y, palette[idx], None);
            ctx.canvas.put_char(x, y, ch);
        }
    }
}

/// 边框：将画布四周各扩展 1 格，并用 Unicode 方框字符绘制边框。
fn apply_border<C: Canvas>(ctx: &mut FilterContext<C>) {
    let w = ctx.canvas.width() as i32;
    let h = ctx.canvas.height() as i32;

    // 扩展边界，内容自动偏移到 (1,1) 起始
    ctx.canvas
        .set_boundaries(-1, -1, (w + 2) as u32, (h + 2) as u32);

    let new_w = ctx.canvas.width();
    let new_h = ctx.canvas.height();

    for y in 0..new_h {
        for x in 0..new_w {
            let ch = if y == 0 {
                if x == 0 {
                    '┌'
                } else if x == new_w - 1 {
                    '┐'
                } else {
                    '─'
                }
            } else if y == new_h - 1 {
                if x == 0 {
                    '└'
                } else if x == new_w - 1 {
                    '┘'
                } else {
                    '─'
                }
            } else if x == 0 || x == new_w - 1 {
                '│'
            } else {
                continue;
            };
            ctx.canvas.put_char(x, y, ch);
        }
    }
}

// ============================================================
// 统一颜色蒙版系统（Color Map）
// 整合画布滤镜（CanvasFilter）与颜色遮蔽（ColorFilter），
// 提供统一的名称解析和应用入口。
// 命名冲突时：画布滤镜优先级高于颜色遮蔽。
// ============================================================

/// 统一的颜色蒙版枚举，涵盖所有画布滤镜与颜色遮蔽效果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMap {
    // ---- 画布滤镜（名称冲突时优先级更高）----
    Crop,
    /// 彩虹色（画布滤镜版本，优先级高于颜色遮蔽同名项）
    Rainbow,
    /// 金属色（画布滤镜版本，优先级高于颜色遮蔽同名项）
    Metal,
    Flip,
    Flop,
    Rotate180,
    RotateLeft,
    RotateRight,
    Border,
    // ---- 颜色遮蔽 ----
    None,
    RainbowLine,
    Fire,
    Water,
    Random,
    Foreground(AnsiColor),
    Gradient { start: AnsiColor, end: AnsiColor },
}

impl ColorMap {
    /// 从名称解析颜色蒙版。
    /// 对于同时存在画布滤镜和颜色遮蔽的名称（如 "rainbow"、"metal"），
    /// 画布滤镜优先。
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            // 画布滤镜名称
            "crop" => Some(Self::Crop),
            "rainbow" => Some(Self::Rainbow),
            "metal" => Some(Self::Metal),
            "flip" => Some(Self::Flip),
            "flop" => Some(Self::Flop),
            "180" | "rotate" => Some(Self::Rotate180),
            "left" => Some(Self::RotateLeft),
            "right" => Some(Self::RotateRight),
            "border" => Some(Self::Border),
            // 颜色遮蔽名称
            "none" | "" => Some(Self::None),
            "rainbowline" | "rainbow_line" => Some(Self::RainbowLine),
            "fire" => Some(Self::Fire),
            "water" => Some(Self::Water),
            "random" => Some(Self::Random),
            "red" => Some(Self::Foreground(AnsiColor::Red)),
            "green" => Some(Self::Foreground(AnsiColor::Green)),
            "yellow" => Some(Self::Foreground(AnsiColor::Yellow)),
            "blue" => Some(Self::Foreground(AnsiColor::Blue)),
            "magenta" => Some(Self::Foreground(AnsiColor::Magenta)),
            "cyan" => Some(Self::Foreground(AnsiColor::Cyan)),
            "white" => Some(Self::Foreground(AnsiColor::White)),
            "black" => Some(Self::Foreground(AnsiColor::Black)),
            _ => None,
        }
    }

    /// 返回该变体的规范名称。
    pub fn name(self) -> &'static str {
        match self {
            Self::Crop => "crop",
            Self::Rainbow => "rainbow",
            Self::Metal => "metal",
            Self::Flip => "flip",
            Self::Flop => "flop",
            Self::Rotate180 => "180",
            Self::RotateLeft => "left",
            Self::RotateRight => "right",
            Self::Border => "border",
            Self::None => "none",
            Self::RainbowLine => "rainbowline",
            Self::Fire => "fire",
            Self::Water => "water",
            Self::Random => "random",
            Self::Foreground(AnsiColor::Red) => "red",
            Self::Foreground(AnsiColor::Green) => "green",
            Self::Foreground(AnsiColor::Yellow) => "yellow",
            Self::Foreground(AnsiColor::Blue) => "blue",
            Self::Foreground(AnsiColor::Magenta) => "magenta",
            Self::Foreground(AnsiColor::Cyan) => "cyan",
            Self::Foreground(AnsiColor::White) => "white",
            Self::Foreground(AnsiColor::Black) => "black",
            Self::Foreground(_) => "custom",
            Self::Gradient { .. } => "gradient",
        }
    }

    /// 返回该变体所属类别。
    pub fn category(self) -> &'static str {
        match self {
            Self::Crop | Self::Rainbow | Self::Metal | Self::Flip | Self::Flop
            | Self::Rotate180 | Self::RotateLeft | Self::RotateRight | Self::Border => {
                "画布滤镜"
            }
            Self::None | Self::RainbowLine | Self::Fire | Self::Water | Self::Random
            | Self::Foreground(_) | Self::Gradient { .. } => {
                "颜色遮蔽"
            }
        }
    }

    /// 统一应用颜色蒙版到文本行。
    ///
    /// 所有变体（画布滤镜几何变换、画布滤镜颜色效果、颜色遮蔽）均直接作用于
    /// 文本行，无需区分调用路径。画布滤镜的颜色效果使用画布色板；颜色遮蔽维持
    /// 原有配色。
    pub fn apply(&self, lines: &[String]) -> Vec<String> {
        if lines.is_empty() {
            return lines.to_vec();
        }
        match self {
            // 几何变换（画布滤镜）
            Self::Crop => crop_text(lines),
            Self::Flip => flip_text(lines),
            Self::Flop => flop_text(lines),
            Self::Rotate180 => rotate180_text(lines),
            Self::RotateLeft => rotate_left_text(lines),
            Self::RotateRight => rotate_right_text(lines),
            Self::Border => border_text(lines),
            // 画布滤镜颜色效果（使用画布色板）
            Self::Rainbow => canvas_rainbow_text(lines),
            Self::Metal => canvas_metal_text(lines),
            // 颜色遮蔽
            Self::None => lines.to_vec(),
            Self::RainbowLine => ColorFilter::RainbowLine.apply(lines),
            Self::Fire => ColorFilter::Fire.apply(lines),
            Self::Water => ColorFilter::Water.apply(lines),
            Self::Random => ColorFilter::Random.apply(lines),
            Self::Foreground(c) => ColorFilter::Foreground(*c).apply(lines),
            Self::Gradient { start, end } => ColorFilter::Gradient { start: *start, end: *end }.apply(lines),
        }
    }

    /// 转换为 [ColorFilter]。画布滤镜中仅颜色效果（Rainbow/Metal）有等效映射，
    /// 纯几何变换（Crop/Flip 等）返回 `None`。
    pub fn to_color_filter(self) -> Option<ColorFilter> {
        match self {
            Self::Rainbow => Some(ColorFilter::Rainbow),
            Self::Metal => Some(ColorFilter::Metal),
            Self::Crop | Self::Flip | Self::Flop | Self::Rotate180
            | Self::RotateLeft | Self::RotateRight | Self::Border => None,
            Self::None => Some(ColorFilter::None),
            Self::RainbowLine => Some(ColorFilter::RainbowLine),
            Self::Fire => Some(ColorFilter::Fire),
            Self::Water => Some(ColorFilter::Water),
            Self::Random => Some(ColorFilter::Random),
            Self::Foreground(c) => Some(ColorFilter::Foreground(c)),
            Self::Gradient { start, end } => Some(ColorFilter::Gradient { start, end }),
        }
    }

    /// 转换为 [CanvasFilter]。颜色遮蔽变体返回 `None`。
    pub fn to_canvas_filter(self) -> Option<CanvasFilter> {
        match self {
            Self::Crop => Some(CanvasFilter::Crop),
            Self::Rainbow => Some(CanvasFilter::Rainbow),
            Self::Metal => Some(CanvasFilter::Metal),
            Self::Flip => Some(CanvasFilter::Flip),
            Self::Flop => Some(CanvasFilter::Flop),
            Self::Rotate180 => Some(CanvasFilter::Rotate180),
            Self::RotateLeft => Some(CanvasFilter::RotateLeft),
            Self::RotateRight => Some(CanvasFilter::RotateRight),
            Self::Border => Some(CanvasFilter::Border),
            Self::None | Self::RainbowLine | Self::Fire | Self::Water | Self::Random
            | Self::Foreground(_) | Self::Gradient { .. } => None,
        }
    }

    /// 返回所有可用名称及其类别，画布滤镜在前。
    pub fn available() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("crop", "crop", "画布滤镜"),
            ("rainbow", "rainbow", "画布滤镜"),
            ("metal", "metal", "画布滤镜"),
            ("flip", "flip", "画布滤镜"),
            ("flop", "flop", "画布滤镜"),
            ("180", "180", "画布滤镜"),
            ("left", "left", "画布滤镜"),
            ("right", "right", "画布滤镜"),
            ("border", "border", "画布滤镜"),
            ("none", "none", "颜色遮蔽"),
            ("rainbowline", "rainbowline", "颜色遮蔽"),
            ("fire", "fire", "颜色遮蔽"),
            ("water", "water", "颜色遮蔽"),
            ("random", "random", "颜色遮蔽"),
            ("red", "red", "颜色遮蔽"),
            ("green", "green", "颜色遮蔽"),
            ("yellow", "yellow", "颜色遮蔽"),
            ("blue", "blue", "颜色遮蔽"),
            ("magenta", "magenta", "颜色遮蔽"),
            ("cyan", "cyan", "颜色遮蔽"),
            ("white", "white", "颜色遮蔽"),
            ("black", "black", "颜色遮蔽"),
        ]
    }
}

// ---- 文本级几何变换（ColorMap 内部使用）----

/// 裁切文本行中的空白区域（四周）
fn crop_text(lines: &[String]) -> Vec<String> {
    let h = lines.len();
    let max_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    if max_w == 0 {
        return vec![];
    }

    let mut xmin = max_w;
    let mut xmax = 0usize;
    let mut ymin = h;
    let mut ymax = 0usize;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != ' ' {
                xmin = xmin.min(x);
                xmax = xmax.max(x);
                ymin = ymin.min(y);
                ymax = ymax.max(y);
            }
        }
    }

    if xmax < xmin || ymax < ymin {
        return vec![];
    }

    lines[ymin..=ymax]
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let end = (xmax + 1).min(chars.len());
            if xmin >= chars.len() {
                String::new()
            } else {
                chars[xmin..end].iter().collect()
            }
        })
        .collect()
}

/// 水平翻转
fn flip_text(lines: &[String]) -> Vec<String> {
    lines.iter().map(|l| l.chars().rev().collect()).collect()
}

/// 垂直翻转
fn flop_text(lines: &[String]) -> Vec<String> {
    lines.iter().rev().cloned().collect()
}

/// 旋转 180 度
fn rotate180_text(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .rev()
        .map(|l| l.chars().rev().collect())
        .collect()
}

/// 逆时针旋转 90 度
fn rotate_left_text(lines: &[String]) -> Vec<String> {
    let h = lines.len();
    let max_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    if max_w == 0 || h == 0 {
        return vec![];
    }
    let padded: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_w, ' ');
            chars
        })
        .collect();
    (0..max_w)
        .map(|i| (0..h).rev().map(|j| padded[j][i]).collect())
        .collect()
}

/// 顺时针旋转 90 度
fn rotate_right_text(lines: &[String]) -> Vec<String> {
    let h = lines.len();
    let max_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    if max_w == 0 || h == 0 {
        return vec![];
    }
    let padded: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_w, ' ');
            chars
        })
        .collect();
    (0..max_w)
        .rev()
        .map(|i| (0..h).map(|j| padded[j][i]).collect())
        .collect()
}

/// 添加方框边框
fn border_text(lines: &[String]) -> Vec<String> {
    if lines.is_empty() {
        return vec![String::new(); 0];
    }
    let max_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let mut result = Vec::with_capacity(lines.len() + 2);

    let top: String = std::iter::once('┌')
        .chain(std::iter::repeat('─').take(max_w))
        .chain(std::iter::once('┐'))
        .collect();
    result.push(top);

    for line in lines {
        let pad = max_w.saturating_sub(line.chars().count());
        result.push(format!("│{}{}│", line, " ".repeat(pad)));
    }

    let bottom: String = std::iter::once('└')
        .chain(std::iter::repeat('─').take(max_w))
        .chain(std::iter::once('┘'))
        .collect();
    result.push(bottom);

    result
}

/// 画布彩虹色（使用 canvas_color 色板）
fn canvas_rainbow_text(lines: &[String]) -> Vec<String> {
    use canvas_color::*;
    let palette = [LIGHTMAGENTA, LIGHTRED, YELLOW, LIGHTGREEN, LIGHTCYAN, LIGHTBLUE];
    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let mut result = String::new();
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    let idx = ((x / 2 + y) % 6) as usize;
                    result.push_str(&format!("\x1b[{}m{}\x1b[0m", palette[idx] as u8, ch));
                } else {
                    result.push(ch);
                }
            }
            result
        })
        .collect()
}

/// 画布金属色（使用 canvas_color 色板）
fn canvas_metal_text(lines: &[String]) -> Vec<String> {
    use canvas_color::*;
    let palette = [LIGHTBLUE, BLUE, LIGHTGRAY, DARKGRAY];
    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let mut result = String::new();
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    let idx = (((y + x / 8) / 2) % 4) as usize;
                    result.push_str(&format!("\x1b[{}m{}\x1b[0m", palette[idx] as u8, ch));
                } else {
                    result.push(ch);
                }
            }
            result
        })
        .collect()
}
