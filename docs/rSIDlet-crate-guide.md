# rSIDlet Rust Crate 使用指南

rSIDlet 是一个 Rust 实现的 FIGlet/TOIlet 兼容库，支持传统英文 ASCII 艺术字体（.flf/.tlf）以及中文点阵/矢量字体的渲染，同时提供颜色滤镜、字体下载等实用功能。

## 快速开始

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
rsidlet = "1.1"
```

## Crate 模块总览

| 模块 | 用途 |
|------|------|
| `figfont` | FIGlet/TOIlet 字体加载、解析与渲染 |
| `chilet` | 中文 ASCII 图形渲染（HZK 点阵 + 系统矢量字体） |
| `utils` | 工具函数，包含颜色滤镜与中文检测 |
| `paths` | 字体搜索路径与文件查找 |
| `download` | 从 GitHub 在线字体仓库下载字体 |

---

## 一、`figfont` — FIGlet/TOIlet 字体渲染

### 1.1 `FigFont` trait

所有字体的统一接口，提供通用的加载与渲染方法：

```rust
pub trait FigFont: Sized {
    fn font_type() -> &'static str;
    fn extensions() -> &'static [&'static str];
    fn load<P: AsRef<Path>>(path: P) -> io::Result<Self>;
    fn parse(data: &[u8]) -> io::Result<Self>;
    fn data(&self) -> &FontData;
    fn render(&self, text: &str) -> Vec<String>;
}
```

通过 `load()` 方法加载字体文件，`render()` 方法将文本转换为 ASCII 艺术字符串数组（每行一个元素）。

### 1.2 `FigletFont` — FIGlet 字体 (.flf)

经典 ASCII 艺术字体格式。文件扩展名通常为 `.flf`，使用 `flf2a` 签名。

```rust
use rsidlet::figfont::{FigletFont, FigFont};

let font = FigletFont::load("../fonts/standard.flf")?;
let art = font.render("Hello");
for line in &art {
    println!("{}", line);
}
```

常用方法：

| 方法 | 说明 |
|------|------|
| `standard()` | 使用内置 `standard.flf` 加载标准 figlet 字体 |
| `render(text)` | 将文本渲染为 ASCII 艺术，返回 `Vec<String>` |
| `render_colored(text, filter)` | 渲染文本并应用颜色滤镜 |
| `builtin()` | 返回内置后备字体（无需外部字体文件） |

### 1.3 `ToiletFont` — TOIlet 字体 (.tlf)

TOIlet 字体是 figlet 的扩展格式，使用 `tlf2a` 签名，增加了 Unicode 支持与颜色标记。文件扩展名通常为 `.tlf`。

```rust
use rsidlet::figfont::{ToiletFont, FigFont};

let font = ToiletFont::load("../fonts/future.tlf")?;
let art = font.render("Hello");
for line in &art {
    println!("{}", line);
}
```

常用方法：

| 方法 | 说明 |
|------|------|
| `future()` | 使用内置 `future.tlf` 加载标准 toilet 字体 |
| `render(text)` | 将文本渲染为 ASCII 艺术 |
| `render_colored(text, filter)` | 渲染文本并应用颜色滤镜 |
| `builtin()` | 返回内置后备字体 |
| `has_unicode` | 字体是否支持 Unicode 字符范围映射 |
| `has_color_tags` | 字体是否包含颜色标记 |
| `supports_range(start, end)` | 检查字体是否覆盖特定 Unicode 范围 |
| `color_tag_count()` | 获取颜色标记数量 |
| `full_layout_mode()` | 获取全布局模式值 |

### 1.4 核心数据结构

#### `FontHeader`

字体文件头部参数：

| 字段 | 类型 | 说明 |
|------|------|------|
| `signature` | `String` | 字体签名（`"flf2a"` 或 `"tlf2a"`） |
| `hard_blank` | `char` | 硬空白字符 |
| `height` | `u32` | 字符高度（行数） |
| `baseline` | `u32` | 基线位置 |
| `max_length` | `u32` | 最大字符宽度 |
| `old_layout` | `u32` | 旧布局模式 |
| `comment_lines` | `u32` | 评论行数 |
| `print_direction` | `u32` | 打印方向（0=左到右，1=右到左） |
| `full_layout` | `u32` | 全布局模式 |
| `codetag_count` | `u32` | 代码标记数量 |
| `mappings` | `Vec<CharMapping>` | Unicode 范围到字形索引的映射表 |

#### `Glyph`

单个字符的字形数据：

| 字段 | 类型 | 说明 |
|------|------|------|
| `codepoint` | `u32` | 字符 Unicode 代码点 |
| `lines` | `Vec<String>` | 字形的文本行 |
| `width` | `usize` | 字符宽度 |
| `height` | `usize` | 字符高度 |

关键方法：
- `is_valid()` — 检查字形是否非全空白
- `line(index)` — 获取指定行的内容
- `render_with_hardblank(hard_blank)` — 将硬空白替换为普通空格

#### `FontData`

完整的字体解析结果，聚合了 `FontHeader`、`FontComment` 和 `Vec<Glyph>`，并提供：

```rust
font_data.get_glyph('A');          // 查找字符字形
font_data.char_width('A');         // 获取字符宽度
font_data.render("Hello");         // 渲染文本为 ASCII 艺术
```

### 1.5 `ControlFile` — FIGlet Control 文件解析

Control 文件（`.flc`）用于覆盖字体头部参数和自定义字符映射关系：

```rust
use rsidlet::figfont::control::ControlFile;

let control = ControlFile::load("myfont.flc")?;
// 将 control 中的参数应用到已加载的字体数据上
control.apply_to_font_data(font.data_mut());
```

---

## 二、`chilet` — 中文 ASCII 图形渲染

`chilet` 模块提供中文文本的 ASCII 艺术渲染能力，核心通过 `render()` 函数实现。

### 2.1 统一渲染入口 `render()`

自动根据文本内容（是否包含中文）和请求字号选择合适的渲染路径：

```rust
use rsidlet::chilet::{render, RenderOptions};

let options = RenderOptions::default()
    .with_size(16)
    .chars('█', ' ');

let lines = render("你好世界", &options)?;
for line in &lines {
    println!("{}", line);
}
```

**字体选择逻辑：**

1. **用户指定字体名** → 优先使用指定字体（HZK / 系统矢量字体 / 文件路径）
2. **文本不含中文** → HZK 字库（12/14/16 号）或系统字体遮蔽方案
3. **文本含有中文** → HZK 字库（12/14/16 号）或系统矢量字体遮蔽方案

### 2.2 `RenderOptions` 渲染选项

```rust
pub struct RenderOptions {
    pub font_size: u32,       // 字号（像素），HZK 仅支持 12/14/16
    pub fg_char: char,        // 前景字符，默认 '█'
    pub bg_char: char,        // 背景字符，默认 ' '
    pub threshold: u8,        // 矢量字体二值化阈值（0-255），默认 128
    pub font_name: Option<String>,  // 指定字体名称或路径
}
```

Builder 模式方法：
- `with_size(font_size)` — 设置字号
- `chars(fg, bg)` — 设置前后景字符
- `font(name)` — 指定使用的字体名称

### 2.3 `RenderError` 错误类型

```rust
pub enum RenderError {
    Io(std::io::Error),        // IO 错误
    Vector(VectorError),        // 矢量字体错误
    NoFont(String),             // 找不到合适的字体
}
```

### 2.4 底层渲染函数

#### `render_hzk()` — HZK 点阵字库渲染

直接从 HZK16/HZK14/HZK12 点阵字库解析并绘制：

```rust
use std::path::Path;
use rsidlet::chilet::render_hzk;

let lines = render_hzk("中文", Path::new("fonts/HZK16"), '█', ' ')?;
```

- 支持的字号：12、14、16
- 非中文字符用背景字符填充
- 内部通过 GB2312 编码计算字节偏移，按位输出前景/背景字符

#### `render_with_font()` — 基于 fontdue 的矢量字体渲染

```rust
use fontdue::Font;
use rsidlet::chilet::render_with_font;

let font_data = std::fs::read("C:/Windows/Fonts/simsun.ttc")?;
let font = Font::from_bytes(font_data, fontdue::FontSettings::default())?;
let lines = render_with_font(&font, "你好", 16.0, '█', ' ', 128)?;
```

- `font_size`：像素单位的字体大小（`f32`）
- `threshold`：灰度阈值，coverage > threshold → 前景字符
- 自动根据基线对齐各字符

#### `render_with_font_file()` — 从文件路径直接渲染

```rust
use std::path::Path;
use rsidlet::chilet::render_with_font_file;

let lines = render_with_font_file(
    "你好",
    Path::new("C:/Windows/Fonts/simsun.ttc"),
    20.0,
    '#',
    ' ',
    128,
)?;
```

### 2.5 字体搜索函数

```rust
use rsidlet::chilet::{find_hzk, font_search_paths};

// 查找 HZK 字库文件
if let Some(path) = find_hzk("HZK16") {
    println!("Found HZK16 at: {:?}", path);
}

// 获取所有字体搜索路径
let paths = font_search_paths();
```

---

## 三、`utils` — 工具模块

### 3.1 `ColorFilter` — 颜色滤镜

支持多种 ANSI 颜色效果，类似 toilet 的 filter 参数：

```rust
use rsidlet::utils::ColorFilter;

let lines = font.render("Hello");
let colored = ColorFilter::Rainbow.apply(&lines);
for line in &colored {
    println!("{}", line);
}
```

所有滤镜变体：

| 变体 | 说明 |
|------|------|
| `None` | 无颜色 |
| `Rainbow` | 逐字符彩虹渐变 |
| `RainbowLine` | 逐行彩虹渐变 |
| `Metal` | 金属渐变（黑→亮白） |
| `Fire` | 火焰渐变（红→亮黄） |
| `Water` | 水纹渐变（蓝→亮青） |
| `Random` | 基于哈希的随机颜色 |
| `Foreground(AnsiColor)` | 纯色前景 |
| `Gradient { start, end }` | 自定义双色渐变 |

两个应用方法：

- `apply(&self, lines: &[String]) -> Vec<String>` — 应用到多行
- `apply_to_line(&self, line, line_index, total_lines) -> String` — 应用到单行

### 3.2 `AnsiColor` — ANSI 颜色枚举

```rust
pub enum AnsiColor {
    Black = 30,     Red = 31,       Green = 32,     Yellow = 33,
    Blue = 34,      Magenta = 35,   Cyan = 36,      White = 37,
    BrightBlack = 90,   BrightRed = 91,   BrightGreen = 92,
    BrightYellow = 93,  BrightBlue = 94,  BrightMagenta = 95,
    BrightCyan = 96,    BrightWhite = 97,
    Reset = 0,
}
```

### 3.3 `contains_chinese()` — 中文检测

```rust
use rsidlet::utils::contains_chinese;

assert!(contains_chinese("你好世界"));
assert!(!contains_chinese("Hello"));
```

检测范围覆盖：基本汉字区（U+4E00-U+9FFF）、扩展 A 区（U+3400-U+4DBF）以及兼容汉字区（U+F900-U+FAFF）。

---

## 四、`paths` — 路径与字体查找

### 4.1 字体搜索路径

```rust
use rsidlet::paths::{font_search_paths, system_font_dirs, builtin_font_dir, extended_font_dir};

// 所有字体搜索路径（按优先级排序）
let paths = font_search_paths(Some(Path::new("/extra/fonts")));

// 系统 TTF/OTF 字体目录
let dirs = system_font_dirs();

// 内置字体目录
if let Some(dir) = builtin_font_dir() { /* ... */ }

// 扩展字体目录（用户可写，用于下载安装字体）
if let Some(dir) = extended_font_dir() { /* ... */ }
```

搜索优先级：
1. 内置 `fonts/` 目录
2. 扩展字体目录（`%USERPROFILE%/fonts` 等）
3. 系统 figlet 目录（`/usr/share/figlet`）
4. 用户指定额外目录（`-d` 参数传入）

### 4.2 字体文件查找

```rust
use rsidlet::paths::find_font_file;

let dirs = font_search_paths(None);
if let Some(path) = find_font_file("standard", &dirs) {
    println!("Found at: {:?}", path);
}
```

支持 .flf / .tlf / .ttf / .otf / .bdf 格式，兼容带/不带扩展名的字体名称。

### 4.3 字体列表

```rust
use rsidlet::paths::{list_flf_files, list_system_ttf_files};

let flf_fonts = list_flf_files(&font_search_paths(None));
println!("Available figlet fonts: {:?}", flf_fonts);

let ttf_fonts = list_system_ttf_files();
println!("System TTF fonts: {:?}", ttf_fonts);
```

### 4.4 确保扩展字体目录存在

```rust
use rsidlet::paths::ensure_extended_font_dir;

let dir = ensure_extended_font_dir()?;
println!("Extended font dir: {:?}", dir);
```

平台默认路径：
- **Windows**: `%USERPROFILE%/fonts`
- **Linux**: `$XDG_DATA_HOME/figlet` 或 `~/.local/share/figlet`
- **macOS**: `~/Library/Application Support/figlet`

---

## 五、`download` — 在线字体下载

从 GitHub 字体仓库自动下载字体文件，内置重试机制（默认 3 次）。

### 5.1 下载 FIGlet/TOIlet 字体

```rust
use std::path::Path;
use rsidlet::download::download_font;

let result = download_font("standard", Path::new("./fonts"))?;
println!("Downloaded to: {:?} from {}", result.file_path, result.source_url);
```

自动补全 .flf/.tlf 扩展名，依次尝试多个 GitHub 仓库：
- [xero/figlet-fonts](https://github.com/xero/figlet-fonts)
- [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)
- [aguegu/BitmapFont](https://github.com/aguegu/BitmapFont)

### 5.2 下载任意文件（HZK 等）

```rust
use rsidlet::download::download_file;

let result = download_file("HZK16", Path::new("./fonts"))?;
```

不自动补全扩展名，适用于 HZK 等无扩展名的文件。

### 5.3 获取在线字体列表

```rust
use rsidlet::download::list_available_online;

let fonts = list_available_online()?;
println!("Online fonts: {:?}", fonts);
```

返回所有可用的 .flf/.tlf 字体文件名列表。

### 5.4 下载字体原数据

```rust
use rsidlet::download::download_font_data;

let (data, filename) = download_font_data("standard")?;
// data: Vec<u8> 字体文件二进制数据
// filename: 实际下载的文件名
```

`DownloadResult` 结构体：

```rust
pub struct DownloadResult {
    pub file_path: PathBuf,    // 本地保存路径
    pub source_url: String,    // 下载来源 URL
}
```

---

## 六、完整使用示例

### 示例一：FIGlet 字体渲染 + 彩虹色

```rust
use rsidlet::figfont::{FigletFont, FigFont};
use rsidlet::utils::ColorFilter;

fn main() -> std::io::Result<()> {
    let font = FigletFont::standard()?;
    let colored = font.render_colored("Hello, Rust!", ColorFilter::Rainbow);
    for line in &colored {
        println!("{}", line);
    }
    Ok(())
}
```

### 示例二：中文文本渲染

```rust
use rsidlet::chilet::{render, RenderOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = RenderOptions::default()
        .with_size(16)
        .chars('#', ' ');

    let lines = render("你好，世界！", &options)?;
    for line in &lines {
        println!("{}", line);
    }
    Ok(())
}
```

### 示例三：指定矢量字体渲染中文

```rust
use rsidlet::chilet::{render, RenderOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = RenderOptions::default()
        .with_size(24)
        .chars('@', '.')
        .font("simsun.ttc");

    let lines = render("中文渲染", &options)?;
    for line in &lines {
        println!("{}", line);
    }
    Ok(())
}
```

### 示例四：加载 TOIlet 字体 + 金属渐变

```rust
use rsidlet::figfont::{ToiletFont, FigFont};
use rsidlet::utils::ColorFilter;

fn main() -> std::io::Result<()> {
    let font = ToiletFont::future()?;
    let colored = font.render_colored("TOIlet", ColorFilter::Metal);
    for line in &colored {
        println!("{}", line);
    }
    Ok(())
}
```

### 示例五：下载并加载字体

```rust
use std::path::Path;
use rsidlet::download::download_font;
use rsidlet::paths::ensure_extended_font_dir;
use rsidlet::figfont::{FigletFont, FigFont};

fn main() -> std::io::Result<()> {
    let dest = ensure_extended_font_dir()?;
    let result = download_font("big", &dest)?;
    let font = FigletFont::load(&result.file_path)?;
    let art = font.render("BIG");
    for line in &art {
        println!("{}", line);
    }
    Ok(())
}
```

### 示例六：获取字体信息

```rust
use rsidlet::figfont::{FigletFont, FigFont};

fn main() -> std::io::Result<()> {
    let font = FigletFont::standard()?;
    let data = font.data();

    println!("签名: {}", data.header.signature);
    println!("高度: {}", data.header.height);
    println!("基线: {}", data.header.baseline);
    println!("硬空白字符: '{}'", data.header.hard_blank);
    println!("评论: {}", data.comment.content());
    println!("字形数量: {}", data.glyphs.len());

    if let Some(glyph) = data.get_glyph('A') {
        println!("'A' 的宽度: {}", glyph.width);
        println!("'A' 的高度: {}", glyph.height);
    }

    Ok(())
}
```
