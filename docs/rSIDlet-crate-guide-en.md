# rSIDlet Rust Crate Usage Guide

rSIDlet is a Rust implementation of a FIGlet/TOIlet-compatible library, supporting both traditional English ASCII art fonts (.flf/.tlf) and Chinese bitmap/vector font rendering, along with color filters, font downloading, and other utility features.

## Quick Start

Add the dependency in `Cargo.toml`:

```toml
[dependencies]
rsidlet = "1.1"
```

## Crate Module Overview

| Module | Purpose |
|--------|---------|
| `figfont` | FIGlet/TOIlet font loading, parsing, and rendering |
| `chilet` | Chinese ASCII art rendering (HZK bitmap + system vector fonts) |
| `utils` | Utility functions, including color filters and Chinese detection |
| `paths` | Font search paths and file lookup |
| `download` | Downloading fonts from online GitHub font repositories |

---

## 1. `figfont` — FIGlet/TOIlet Font Rendering

### 1.1 `FigFont` trait

A unified interface for all fonts, providing common loading and rendering methods:

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

Use `load()` to load a font file, and `render()` to convert text into an array of ASCII art strings (one string per line).

### 1.2 `FigletFont` — FIGlet Font (.flf)

Classic ASCII art font format. File extension is typically `.flf`, using the `flf2a` signature.

```rust
use rsidlet::figfont::{FigletFont, FigFont};

let font = FigletFont::load("../fonts/standard.flf")?;
let art = font.render("Hello");
for line in &art {
    println!("{}", line);
}
```

Common methods:

| Method | Description |
|--------|-------------|
| `standard()` | Load the standard figlet font using the built-in `standard.flf` |
| `render(text)` | Render text as ASCII art, returning `Vec<String>` |
| `render_colored(text, filter)` | Render text and apply a color filter |
| `builtin()` | Return a built-in fallback font (no external font file required) |

### 1.3 `ToiletFont` — TOIlet Font (.tlf)

TOIlet font is an extended figlet format, using the `tlf2a` signature, adding Unicode support and color tags. File extension is typically `.tlf`.

```rust
use rsidlet::figfont::{ToiletFont, FigFont};

let font = ToiletFont::load("../fonts/future.tlf")?;
let art = font.render("Hello");
for line in &art {
    println!("{}", line);
}
```

Common methods:

| Method | Description |
|--------|-------------|
| `future()` | Load the standard toilet font using the built-in `future.tlf` |
| `render(text)` | Render text as ASCII art |
| `render_colored(text, filter)` | Render text and apply a color filter |
| `builtin()` | Return a built-in fallback font |
| `has_unicode` | Whether the font supports Unicode character range mapping |
| `has_color_tags` | Whether the font contains color tags |
| `supports_range(start, end)` | Check if the font covers a specific Unicode range |
| `color_tag_count()` | Get the number of color tags |
| `full_layout_mode()` | Get the full layout mode value |

### 1.4 Core Data Structures

#### `FontHeader`

Font file header parameters:

| Field | Type | Description |
|-------|------|-------------|
| `signature` | `String` | Font signature (`"flf2a"` or `"tlf2a"`) |
| `hard_blank` | `char` | Hard blank character |
| `height` | `u32` | Character height (number of rows) |
| `baseline` | `u32` | Baseline position |
| `max_length` | `u32` | Maximum character width |
| `old_layout` | `u32` | Old layout mode |
| `comment_lines` | `u32` | Number of comment lines |
| `print_direction` | `u32` | Print direction (0=left-to-right, 1=right-to-left) |
| `full_layout` | `u32` | Full layout mode |
| `codetag_count` | `u32` | Code tag count |
| `mappings` | `Vec<CharMapping>` | Unicode range to glyph index mapping table |

#### `Glyph`

Glyph data for a single character:

| Field | Type | Description |
|-------|------|-------------|
| `codepoint` | `u32` | Character Unicode code point |
| `lines` | `Vec<String>` | Text lines of the glyph |
| `width` | `usize` | Character width |
| `height` | `usize` | Character height |

Key methods:
- `is_valid()` — Check whether the glyph is not all whitespace
- `line(index)` — Get the content of a specific line
- `render_with_hardblank(hard_blank)` — Replace hard blanks with regular spaces

#### `FontData`

The complete font parsing result, aggregating `FontHeader`, `FontComment`, and `Vec<Glyph>`, and provides:

```rust
font_data.get_glyph('A');          // Look up a character's glyph
font_data.char_width('A');         // Get a character's width
font_data.render("Hello");         // Render text as ASCII art
```

### 1.5 `ControlFile` — FIGlet Control File Parsing

Control files (`.flc`) are used to override font header parameters and customize character mapping:

```rust
use rsidlet::figfont::control::ControlFile;

let control = ControlFile::load("myfont.flc")?;
// Apply the control parameters to a loaded font's data
control.apply_to_font_data(font.data_mut());
```

---

## 2. `chilet` — Chinese ASCII Art Rendering

The `chilet` module provides Chinese text ASCII art rendering capabilities, primarily through the `render()` function.

### 2.1 Unified Rendering Entry Point `render()`

Automatically selects the appropriate rendering path based on text content (whether it contains Chinese) and the requested font size:

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

**Font selection logic:**

1. **User-specified font name** → Use the specified font first (HZK / system vector font / file path)
2. **Text without Chinese** → HZK font (sizes 12/14/16) or system vector font fallback
3. **Text with Chinese** → HZK font (sizes 12/14/16) or system vector font fallback

### 2.2 `RenderOptions` Rendering Options

```rust
pub struct RenderOptions {
    pub font_size: u32,       // Font size (pixels), HZK only supports 12/14/16
    pub fg_char: char,        // Foreground character, default '█'
    pub bg_char: char,        // Background character, default ' '
    pub threshold: u8,        // Vector font binarization threshold (0-255), default 128
    pub font_name: Option<String>,  // Specified font name or path
}
```

Builder pattern methods:
- `with_size(font_size)` — Set font size
- `chars(fg, bg)` — Set foreground/background characters
- `font(name)` — Specify the font name to use

### 2.3 `RenderError` Error Types

```rust
pub enum RenderError {
    Io(std::io::Error),        // IO error
    Vector(VectorError),        // Vector font error
    NoFont(String),             // No suitable font found
}
```

### 2.4 Low-Level Rendering Functions

#### `render_hzk()` — HZK Bitmap Font Rendering

Directly parses and renders from HZK16/HZK14/HZK12 bitmap font files:

```rust
use std::path::Path;
use rsidlet::chilet::render_hzk;

let lines = render_hzk("中文", Path::new("fonts/HZK16"), '█', ' ')?;
```

- Supported sizes: 12, 14, 16
- Non-Chinese characters are filled with the background character
- Internally computes byte offsets via GB2312 encoding and outputs foreground/background characters per bit

#### `render_with_font()` — fontdue-Based Vector Font Rendering

```rust
use fontdue::Font;
use rsidlet::chilet::render_with_font;

let font_data = std::fs::read("C:/Windows/Fonts/simsun.ttc")?;
let font = Font::from_bytes(font_data, fontdue::FontSettings::default())?;
let lines = render_with_font(&font, "你好", 16.0, '█', ' ', 128)?;
```

- `font_size`: Font size in pixels (`f32`)
- `threshold`: Grayscale threshold; coverage > threshold → foreground character
- Characters are automatically aligned by baseline

#### `render_with_font_file()` — Render Directly from a File Path

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

### 2.5 Font Search Functions

```rust
use rsidlet::chilet::{find_hzk, font_search_paths};

// Look up HZK font files
if let Some(path) = find_hzk("HZK16") {
    println!("Found HZK16 at: {:?}", path);
}

// Get all font search paths
let paths = font_search_paths();
```

---

## 3. `utils` — Utility Module

### 3.1 `ColorFilter` — Color Filters

Supports various ANSI color effects, similar to toilet's filter parameter:

```rust
use rsidlet::utils::ColorFilter;

let lines = font.render("Hello");
let colored = ColorFilter::Rainbow.apply(&lines);
for line in &colored {
    println!("{}", line);
}
```

All filter variants:

| Variant | Description |
|---------|-------------|
| `None` | No color |
| `Rainbow` | Per-character rainbow gradient |
| `RainbowLine` | Per-line rainbow gradient |
| `Metal` | Metal gradient (black → bright white) |
| `Fire` | Fire gradient (red → bright yellow) |
| `Water` | Water gradient (blue → bright cyan) |
| `Random` | Hash-based random color |
| `Foreground(AnsiColor)` | Solid foreground color |
| `Gradient { start, end }` | Custom two-color gradient |

Two application methods:

- `apply(&self, lines: &[String]) -> Vec<String>` — Apply to multiple lines
- `apply_to_line(&self, line, line_index, total_lines) -> String` — Apply to a single line

### 3.2 `AnsiColor` — ANSI Color Enum

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

### 3.3 `contains_chinese()` — Chinese Character Detection

```rust
use rsidlet::utils::contains_chinese;

assert!(contains_chinese("你好世界"));
assert!(!contains_chinese("Hello"));
```

Detection range covers: CJK Unified Ideographs (U+4E00-U+9FFF), CJK Extension A (U+3400-U+4DBF), and CJK Compatibility Ideographs (U+F900-U+FAFF).

---

## 4. `paths` — Paths and Font Lookup

### 4.1 Font Search Paths

```rust
use rsidlet::paths::{font_search_paths, system_font_dirs, builtin_font_dir, extended_font_dir};

// All font search paths (sorted by priority)
let paths = font_search_paths(Some(Path::new("/extra/fonts")));

// System TTF/OTF font directories
let dirs = system_font_dirs();

// Built-in font directory
if let Some(dir) = builtin_font_dir() { /* ... */ }

// Extended font directory (user-writable, for downloading and installing fonts)
if let Some(dir) = extended_font_dir() { /* ... */ }
```

Search priority:
1. Built-in `fonts/` directory
2. Extended font directory (`%USERPROFILE%/fonts`, etc.)
3. System figlet directories (`/usr/share/figlet`)
4. User-specified extra directory (passed via `-d` flag)

### 4.2 Font File Lookup

```rust
use rsidlet::paths::find_font_file;

let dirs = font_search_paths(None);
if let Some(path) = find_font_file("standard", &dirs) {
    println!("Found at: {:?}", path);
}
```

Supports .flf / .tlf / .ttf / .otf / .bdf formats, compatible with font names with or without extensions.

### 4.3 Font Listing

```rust
use rsidlet::paths::{list_flf_files, list_system_ttf_files};

let flf_fonts = list_flf_files(&font_search_paths(None));
println!("Available figlet fonts: {:?}", flf_fonts);

let ttf_fonts = list_system_ttf_files();
println!("System TTF fonts: {:?}", ttf_fonts);
```

### 4.4 Ensure Extended Font Directory Exists

```rust
use rsidlet::paths::ensure_extended_font_dir;

let dir = ensure_extended_font_dir()?;
println!("Extended font dir: {:?}", dir);
```

Platform default paths:
- **Windows**: `%USERPROFILE%/fonts`
- **Linux**: `$XDG_DATA_HOME/figlet` or `~/.local/share/figlet`
- **macOS**: `~/Library/Application Support/figlet`

---

## 5. `download` — Online Font Downloading

Automatically downloads font files from GitHub font repositories, with a built-in retry mechanism (default 3 retries).

### 5.1 Download FIGlet/TOIlet Fonts

```rust
use std::path::Path;
use rsidlet::download::download_font;

let result = download_font("standard", Path::new("./fonts"))?;
println!("Downloaded to: {:?} from {}", result.file_path, result.source_url);
```

Automatically appends .flf/.tlf extensions, trying multiple GitHub repositories in sequence:
- [xero/figlet-fonts](https://github.com/xero/figlet-fonts)
- [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts)
- [aguegu/BitmapFont](https://github.com/aguegu/BitmapFont)

### 5.2 Download Arbitrary Files (HZK, etc.)

```rust
use rsidlet::download::download_file;

let result = download_file("HZK16", Path::new("./fonts"))?;
```

No automatic extension completion; suitable for HZK and other extensionless files.

### 5.3 Get Online Font List

```rust
use rsidlet::download::list_available_online;

let fonts = list_available_online()?;
println!("Online fonts: {:?}", fonts);
```

Returns a list of all available .flf/.tlf font filenames.

### 5.4 Download Font Raw Data

```rust
use rsidlet::download::download_font_data;

let (data, filename) = download_font_data("standard")?;
// data: Vec<u8> font file binary data
// filename: the actual downloaded file name
```

`DownloadResult` struct:

```rust
pub struct DownloadResult {
    pub file_path: PathBuf,    // Local save path
    pub source_url: String,    // Download source URL
}
```

---

## 6. Complete Usage Examples

### Example 1: FIGlet Font Rendering + Rainbow Colors

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

### Example 2: Chinese Text Rendering

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

### Example 3: Chinese Rendering with a Specified Vector Font

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

### Example 4: Load TOIlet Font + Metal Gradient

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

### Example 5: Download and Load a Font

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

### Example 6: Get Font Information

```rust
use rsidlet::figfont::{FigletFont, FigFont};

fn main() -> std::io::Result<()> {
    let font = FigletFont::standard()?;
    let data = font.data();

    println!("Signature: {}", data.header.signature);
    println!("Height: {}", data.header.height);
    println!("Baseline: {}", data.header.baseline);
    println!("Hard blank: '{}'", data.header.hard_blank);
    println!("Comment: {}", data.comment.content());
    println!("Glyph count: {}", data.glyphs.len());

    if let Some(glyph) = data.get_glyph('A') {
        println!("Width of 'A': {}", glyph.width);
        println!("Height of 'A': {}", glyph.height);
    }

    Ok(())
}
```
