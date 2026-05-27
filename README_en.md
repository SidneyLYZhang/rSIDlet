# rSIDlet

[中文](README.md) | English

A Rust implementation of `figlet` with Chinese character support. Chinese rendering is achieved through both bitmap fonts (HZK) and vector fonts (TTF/OTF).

This project provides both a CLI tool (`sidlet`) and a Rust library for integration into other projects.

## Installation

### Method 1: Install Script (Recommended)

Run the install script directly from GitHub to automatically download the latest pre-built binary and complete the installation. The script handles font files, PATH configuration, and other setup tasks automatically.

#### Windows (PowerShell)

Run the following command in PowerShell:

```powershell
iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1"
```

The install script will automatically:
- Download the latest release for your platform
- Install `sidlet.exe` to `%LOCALAPPDATA%\Programs\rsidlet\`
- Copy font files to the installation directory
- Add the installation directory to your user `PATH` environment variable

> **Tip**: If the command is not available immediately after installation, restart your terminal.

#### Linux / macOS (bash)

Run the following command in your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash
```

The install script will automatically:
- Detect your operating system and architecture
- Download the corresponding latest release
- Install `sidlet` to `~/.local/bin/`
- Copy font files to the installation directory

> **Note**: If `~/.local/bin` is not on your `PATH`, the script will prompt you to add it. You can also specify a custom install directory via an environment variable:
> ```bash
> RSIDLET_INSTALL_DIR=/your/custom/path bash install.sh
> ```

#### Installing a Specific Version

To install a specific version, pass the version tag as a parameter:

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.sh | bash -s v1.1.0

# Windows PowerShell
$ver="v1.1.0"; iwr -Uri "https://raw.githubusercontent.com/SidneyLYZhang/rSIDlet/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; & "$env:TEMP\install.ps1" -Version $ver
```

### Method 2: Build from Source

If pre-built binaries are not available for your platform, or if you need the latest development version, you can build from source.

#### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable channel)
- [Git](https://git-scm.com/)

#### Steps

**1. Clone the Repository**

```bash
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet
```

**2. Build and Install**

```bash
# Build the release version
cargo build --release

# Install to system
make install
```

`make install` will automatically, depending on your operating system:
- Copy the compiled binary to the installation directory
- Copy the `fonts/` folder

> **Manual Installation**: If you do not use `make install`, you need to manually copy `target/release/sidlet` (`target\release\sidlet.exe` on Windows) to a directory on your `PATH`, and ensure the `fonts/` folder is located in the same directory as the executable or its parent directory.

**3. Fonts Folder Notes**

The fonts folder (`fonts/`) contains the following required files:

| File | Description |
|------|-------------|
| `standard.flf` | Default FIGlet font |
| `big.flf` | Large FIGlet font |
| `phm-shinonome.flf` | PHM shinonome font |
| `future.tlf` | TOIlet future font |
| `HZK12` / `HZK14` / `HZK16` | Bitmap Chinese fonts |

At runtime, the program searches for fonts in the following priority order:
1. `fonts/` directory at the same level as or one level above the executable
2. User extended font directory (see [Font Search Paths](docs/SIDlet-manpage-en.md))
3. System-level figlet directories (`/usr/share/figlet`, etc.)

If you move the executable to a different location, make sure the `fonts/` folder is present in the same directory, or run `sidlet --test` to automatically repair the font configuration.

#### OS-Specific Build Instructions

<details>
<summary><b>Linux</b></summary>

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install build dependencies (Ubuntu/Debian example)
sudo apt install build-essential pkg-config

# Clone and build
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet
cargo build --release
make install
```
</details>

<details>
<summary><b>macOS</b></summary>

```bash
# Install Xcode Command Line Tools (if not already installed)
xcode-select --install

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet
cargo build --release
make install
```
</details>

<details>
<summary><b>Windows</b></summary>

```powershell
# Install Rust toolchain (download rustup-init.exe from https://rustup.rs/)

# Clone the repository
git clone https://github.com/SidneyLYZhang/rSIDlet.git
cd rSIDlet

# Build
cargo build --release

# Install
make install
```

> On Windows, you can also install via Cargo and then repair fonts with `sidlet --test`:
> ```powershell
> cargo install rsidlet
> sidlet --test
> ```
</details>

### Verifying the Installation

After installation, run the following command to verify the installation was successful:

```bash
sidlet --version
```

If installed correctly, this will output the current version number.

Run the self-check command to ensure all font files are intact:

```bash
sidlet --test
```

If everything is in order, you will see `It's ready` displayed in rainbow colors.

Test the rendering functionality:

```bash
# English rendering
sidlet "Hello World"

# Chinese rendering
sidlet "你好世界"
```

### Library

To use in a Rust project, add `rsidlet` as a dependency:

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
