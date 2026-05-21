# rSIDlet

[中文](README.md) | English

A Rust implementation of `figlet` with Chinese character support. Chinese rendering is achieved through both bitmap fonts (HZK) and vector fonts (TTF/OTF).

This project provides both a CLI tool (`sidlet`) and a Rust library for integration into other projects.

## Installation

### CLI Tool

```bash
cargo install rsidlet
```

### Library

```bash
cargo add rsidlet
```

## CLI Usage

### Basic Usage

```bash
# Render English text with the default font (standard)
sidlet "Hello World"

# Use a specific figlet/toilet font
sidlet -f big.flf "Hello"

# Render Chinese text (auto-detected, uses bitmap or vector fonts)
sidlet "你好世界"

# Render Chinese with a system vector font
sidlet -f simhei.ttf -s 16 "你好世界"

# Limit output width with automatic line wrapping
sidlet -w 40 "This is a long text"
```

### Color Filters

```bash
# Rainbow (per-character gradient)
sidlet -m rainbow "Rainbow Text"

# Fire effect
sidlet -m fire "Fire Text"

# Metal effect
sidlet -m metal "Metal Text"

# Solid colors (red/green/yellow/blue/magenta/cyan/white/black)
sidlet -m red "Red Text"
```

Available color filters: `none`, `rainbow`, `rainbowline`, `metal`, `fire`, `water`, `random`, plus `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `black`.

### Custom Foreground/Background Characters

```bash
# Use custom characters for foreground and background
sidlet --fore "#" --back "." "Hello"

# Works with Chinese rendering too
sidlet --fore "■" "你好"
```

### Font Management

```bash
# Install figlet/toilet fonts from online repositories
sidlet --install standard.flf
sidlet --install big.flf

# Install from a local file
sidlet --install myfont.flf -d /path/to/fonts

# Show font info
sidlet --info big.flf
sidlet -i

# List installed fonts
sidlet --list installed

# List fonts available for download
sidlet --list letters

# List system vector fonts (TTF/OTF)
sidlet --list font

# List available color filters
sidlet --list colormap

# Check installation status and optionally repair
sidlet --test
```

### Advanced Options

```bash
# Use a figlet control file
sidlet -C mycontrol.flc -f standard.flf "Hello"

# Specify an additional font search directory
sidlet -d /path/to/custom/fonts -f myfont.flf "Hello"

# Set bitmap font size (12/14/16)
sidlet -s 16 "你好世界"
```

## Library Usage

### Basic Rendering

```rust
use rsidlet::figfont;

// Load and render a figlet/toilet font
let data = figfont::load_font_data("standard.flf")?;
let lines = data.render("Hello World");
for line in &lines {
    println!("{}", line);
}
```

### Chinese Rendering

```rust
use rsidlet::chilet;

// Render Chinese text using HZK bitmap fonts
if let Some(path) = chilet::find_hzk("HZK16") {
    let lines = chilet::render_hzk("你好", &path)?;
    for line in &lines { println!("{}", line); }
}

// Render using a system vector font
let lines = chilet::render_vector_font("你好", "SimHei", 32.0)?;
for line in &lines { println!("{}", line); }

// Render using a specific vector font file path
let lines = chilet::render_with_font_file("你好", &font_path, 24.0)?;
for line in &lines { println!("{}", line); }
```

### Color Filters

```rust
use rsidlet::utils;

let filter = utils::parse_filter("rainbow").unwrap();
let lines = vec!["Hello".to_string()];
utils::print_colored(&lines, filter);
```

### Horizontal Concatenation

```rust
use rsidlet::utils;

let combined = utils::hcat(&left_lines, &right_lines, 4);
for line in &combined {
    println!("{}", line);
}
```

## Fonts

### Built-in Fonts

| Font File | Type | Description |
|-----------|------|-------------|
| `standard.flf` | FIGlet | Standard font (default) |
| `big.flf` | FIGlet | Large font |
| `phm-shinonome.flf` | FIGlet | PHM shinonome font |
| `future.tlf` | TOIlet | Future font |
| `HZK12` | BitmapFont | 12x12 bitmap Chinese font |
| `HZK14` | BitmapFont | 14x14 bitmap Chinese font |
| `HZK16` | BitmapFont | 16x16 bitmap Chinese font |

### Installing More Fonts Online

```bash
sidlet --install [font_name]
```

Fonts are downloaded from the following GitHub repositories: [xero/figlet-fonts](https://github.com/xero/figlet-fonts) and [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts).

### Chinese Fonts

In addition to the built-in HZK bitmap fonts, you can also use system-installed vector fonts (TTF/OTF) for rendering. Results vary by font. On Windows, using `SimHei` at font size 12 produces good results.

## Thanks

- [FIGlet](http://www.figlet.org/) — The original ASCII art text project
- [TOIlet](https://github.com/cacalabs/toilet) — Enhanced FIGlet with color filter support
- [BitmapFont](https://github.com/aguegu/BitmapFont) — Bitmap Chinese font library
- [xero/figlet-fonts](https://github.com/xero/figlet-fonts) — FIGlet font collection
- [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts) — Additional FIGlet fonts

## License

[MIT License](LICENSE)
