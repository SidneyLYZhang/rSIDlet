# rSIDlet

[中文](README.md) | English

A Rust implementation of `figlet` with Chinese character support (HZK bitmap fonts / TTF/OTF vector fonts). Provides the `sidlet` CLI tool and a Rust library.

## Installation

### Method 1: Install Script (Recommended)

Download the latest pre-built binary directly from GitHub. The script handles font files and `PATH` configuration automatically.

#### Windows (PowerShell)

```powershell
iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1"
```

Installs to `%LOCALAPPDATA%\Programs\rsidlet\` and adds to user `PATH`. Restart your terminal if the command is not available immediately.

#### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash
```

Installs to `~/.local/bin/`. The script will warn you if the directory is not on `PATH`. Custom install path:

```bash
RSIDLET_INSTALL_DIR=/your/path bash install.sh
```

#### Installing a Specific Version

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash -s v1.1.5

# Windows PowerShell
$ver="v1.1.5"; iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1" -Version $ver
```

### Method 2: Cargo Install

For users with Rust already installed. Cargo does not install font files automatically — run `sidlet --test` after installation to repair.

```bash
cargo install rsidlet
sidlet --test
```

### Method 3: Build from Source

For platforms without pre-built binaries, or if you need the latest development version.

**Prerequisites**: [Rust toolchain](https://rustup.rs/) (stable), [Git](https://git-scm.com/).

```bash
# Clone the repository
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet

# Build the release version
cargo build --release
```

After building, complete the installation:

**1. Copy the binary to a directory on your `PATH`:**

| OS | Command |
|-----|---------|
| Linux / macOS | `cp target/release/sidlet ~/.local/bin/` |
| Windows (PowerShell) | `copy target\release\sidlet.exe %LOCALAPPDATA%\Programs\rsidlet\` |

**2. Copy the `fonts/` directory next to the binary:** The program looks for fonts in the `fonts/` directory alongside the executable — the fonts directory must be copied there.

| OS | Command |
|-----|---------|
| Linux / macOS | `cp -r fonts ~/.local/bin/` |
| Windows (PowerShell) | `xcopy fonts %LOCALAPPDATA%\Programs\rsidlet\fonts\ /E /I` |

> **Tip**: `make package` bundles the compiled binary and fonts into a distributable archive.

### Verifying Installation

```bash
sidlet --version            # Print version number
sidlet --test               # Check font integrity, prints rainbow "It's ready"
sidlet "Hello World"        # English rendering test
sidlet "你好世界"            # Chinese rendering test
```

## CLI Usage

### Basic Rendering

```bash
sidlet "Hello World"                  # Default font (standard)
sidlet -f big.flf "Hello"             # Specific FIGlet/TOIlet font
sidlet "你好世界"                      # Auto-switches to Chinese render mode
sidlet -f simhei.ttf -s 16 "你好世界"  # System vector font
sidlet -w 40 "Long text auto-wrapped" # Limit output width
```

### Color Filters

```bash
sidlet -m rainbow "Rainbow"   # Per-character gradient
sidlet -m fire "Fire"         # Fire gradient
sidlet -m metal "Metal"       # Metallic gradient
sidlet -m red "Red"           # Solid color
```

Supports `none`, `rainbow`, `rainbowline`, `metal`, `fire`, `water`, `random`, plus 8 solid colors. See the [SIDlet manpage](docs/SIDlet-manpage-en.md) for details.

### Custom Foreground / Background

```bash
sidlet --fore "#" --back "." "Hello"
sidlet --fore "■" "你好"
```

### Font Management

```bash
sidlet --install big.flf             # Install font from online repository
sidlet --install myfont.flf -d /path # Install from local file
sidlet --info big.flf                # Show font info
sidlet --list installed              # List installed fonts
sidlet --list letters                # List fonts available for download
sidlet --list font                   # List system vector fonts
sidlet --list colormap               # List available color filters
```

### Advanced Options

```bash
sidlet -C mycontrol.flc -f standard.flf "Hello"   # FIGlet Control file
sidlet -d /custom/fonts -f myfont.flf "Hello"     # Extra font search directory
sidlet -s 14 "你好世界"                             # Bitmap font size (12/14/16)
```

Full options reference: [SIDlet manpage](docs/SIDlet-manpage-en.md).

## Library Usage

```bash
cargo add rsidlet
```

### Basic Rendering

```rust
use rsidlet::figfont;

let data = figfont::load_font_data("standard.flf")?;
let lines = data.render("Hello World");
for line in &lines { println!("{}", line); }
```

### Chinese Rendering

```rust
use rsidlet::chilet;

// HZK bitmap fonts
if let Some(path) = chilet::find_hzk("HZK16") {
    let lines = chilet::render_hzk("你好", &path)?;
    for line in &lines { println!("{}", line); }
}

// System vector font
let lines = chilet::render_vector_font("你好", "SimHei", 32.0)?;
for line in &lines { println!("{}", line); }

// Specific font file
let lines = chilet::render_with_font_file("你好", &font_path, 24.0)?;
for line in &lines { println!("{}", line); }
```

### Color Filters & Concatenation

```rust
use rsidlet::utils;

let filter = utils::parse_filter("rainbow").unwrap();
utils::print_colored(&lines, filter);

let combined = utils::hcat(&left_lines, &right_lines, 4);
```

## Fonts

### Built-in Fonts

| Font File | Type | Description |
|-----------|------|-------------|
| `standard.flf` | FIGlet | Standard font (default) |
| `big.flf` | FIGlet | Large font |
| `phm-shinonome.flf` | FIGlet | PHM shinonome font |
| `future.tlf` | TOIlet | future font |
| `HZK12` | Bitmap | 12×12 Chinese font |
| `HZK14` | Bitmap | 14×14 Chinese font |
| `HZK16` | Bitmap | 16×16 Chinese font |

### Font Search Paths

The program searches for fonts in this priority order (see [paths.rs](src/paths.rs)):

1. `fonts/` alongside the executable
2. `fonts/` one level above the executable
3. Extended font directory (user-writable, `--install` target)
4. System-level figlet directories (`/usr/share/figlet`, etc.)
5. Extra directories specified via `-d`

### Installing More Fonts Online

```bash
sidlet --install [font_name]
```

Fonts sourced from [xero/figlet-fonts](https://github.com/xero/figlet-fonts) and [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts).

### Chinese Fonts

In addition to built-in HZK bitmap fonts, system vector fonts (TTF/OTF) are supported. On Windows, `SimHei` at font size 12 produces good results.

## Documentation

- [SIDlet manpage (full CLI reference)](docs/SIDlet-manpage-en.md)
- [Changelog](CHANGELOG.md)

## Credits

- [FIGlet](http://www.figlet.org/) — The original ASCII art text project
- [TOIlet](https://github.com/cacalabs/toilet) — Enhanced FIGlet with color filter support
- [BitmapFont](https://github.com/aguegu/BitmapFont) — Bitmap Chinese font library
- [xero/figlet-fonts](https://github.com/xero/figlet-fonts) — FIGlet font collection
- [PhMajerus/FIGfonts](https://github.com/PhMajerus/FIGfonts) — Additional FIGlet fonts

## License

[MIT License](LICENSE)
