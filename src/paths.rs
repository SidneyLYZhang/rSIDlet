use std::io;
use std::path::{Path, PathBuf};

/// 获取内置字体目录
///
/// 按优先级依次尝试：
/// 1. exe同级的 fonts/（二进制与 fonts 安装在同一目录的场景）
/// 2. exe/../fonts（fonts 在二进制上级目录的场景）
/// 3. cwd/../fonts（开发场景）
/// 4. cwd/fonts（开发场景）
pub fn builtin_font_dir() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let path = parent.join("fonts");
            if path.exists() {
                return Some(path);
            }
        }
        if let Some(parent) = exe.parent() {
            let path = parent.join("..").join("fonts");
            if path.exists() {
                return Some(path);
            }
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        let cwd_parent = cwd.join("..").join("fonts");
        if cwd_parent.exists() {
            return Some(cwd_parent);
        }
        let cwd_fonts = cwd.join("fonts");
        if cwd_fonts.exists() {
            return Some(cwd_fonts);
        }
    }

    None
}

/// 获取扩展字体目录（用户可写，用于下载和安装字体）
///
/// - Linux: `$XDG_DATA_HOME/figlet` 或 `~/.local/share/figlet`
/// - macOS: `~/Library/Application Support/figlet`
/// - Windows: `%USERPROFILE%\fonts`
pub fn extended_font_dir() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        home_dir().map(|h| {
            if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
                PathBuf::from(xdg).join("figlet")
            } else {
                h.join(".local").join("share").join("figlet")
            }
        })
    }
    #[cfg(target_os = "macos")]
    {
        home_dir().map(|h| h.join("Library").join("Application Support").join("figlet"))
    }
    #[cfg(target_os = "windows")]
    {
        home_dir().map(|h| h.join("fonts"))
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

/// 获取系统级 figlet 字体目录（只读，由包管理器安装字体时使用）
fn system_figlet_dirs() -> Vec<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        vec![
            PathBuf::from("/usr/share/figlet"),
            PathBuf::from("/usr/local/share/figlet"),
        ]
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        Vec::new()
    }
}

/// 获取所有字体搜索路径（按优先级排序）
///
/// 返回顺序：
/// 1. 内置 ../fonts 目录
/// 2. 扩展字体目录（用户可写）
/// 3. 系统 figlet 目录（如 /usr/share/figlet，只读）
/// 4. 用户指定目录（通过 `-d` 参数传入）
pub fn font_search_paths(extra_dir: Option<&Path>) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    if let Some(builtin) = builtin_font_dir() {
        paths.push(builtin);
    }

    if let Some(ext) = extended_font_dir() {
        if !paths.contains(&ext) {
            paths.push(ext);
        }
    }

    // 系统级 figlet 目录（只读，由包管理器安装字体时使用）
    for sys_dir in system_figlet_dirs() {
        if sys_dir.exists() && !paths.contains(&sys_dir) {
            paths.push(sys_dir);
        }
    }

    if let Some(extra) = extra_dir {
        let extra = extra.to_path_buf();
        if !paths.contains(&extra) {
            paths.push(extra);
        }
    }

    paths
}

/// 获取系统 TTF/OTF 字体搜索目录
pub fn system_font_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    if let Some(builtin) = builtin_font_dir() {
        dirs.push(builtin);
    }

    #[cfg(target_os = "linux")]
    {
        dirs.push(PathBuf::from("/usr/share/fonts"));
        dirs.push(PathBuf::from("/usr/local/share/fonts"));
        if let Ok(home) = std::env::var("HOME") {
            dirs.push(PathBuf::from(&home).join(".fonts"));
            dirs.push(PathBuf::from(&home).join(".local").join("share").join("fonts"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        dirs.push(PathBuf::from("/System/Library/Fonts"));
        dirs.push(PathBuf::from("/Library/Fonts"));
        if let Ok(home) = std::env::var("HOME") {
            dirs.push(PathBuf::from(&home).join("Library").join("Fonts"));
        }
    }

    #[cfg(target_os = "windows")]
    {
        let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
        dirs.push(PathBuf::from(windir).join("Fonts"));
        if let Some(home) = home_dir() {
            dirs.push(home.join("fonts"));
        }
    }

    dirs
}

/// 在给定目录列表中按名称查找字体文件（.flf / .tlf / .ttf / .otf / .bdf）
///
/// 支持带或不带扩展名的字体名称。优先匹配精确文件名，其次匹配文件名主干。
pub fn find_font_file(name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    let name_lower = name.to_lowercase();
    let has_flf_ext = name_lower.ends_with(".flf") || name_lower.ends_with(".tlf");
    let has_font_ext = name_lower.ends_with(".ttf")
        || name_lower.ends_with(".otf")
        || name_lower.ends_with(".bdf");

    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let fname = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n,
                None => continue,
            };
            let fname_lower = fname.to_lowercase();

            if has_flf_ext || has_font_ext {
                // 精确文件名匹配
                if fname_lower == name_lower {
                    return Some(path);
                }
            } else {
                // 按主干名匹配
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                let ext_lower = ext.to_lowercase();
                let valid_ext = matches!(
                    ext_lower.as_str(),
                    "flf" | "tlf" | "ttf" | "otf" | "bdf"
                );
                if valid_ext {
                    let stem = fname_lower
                        .strip_suffix(&format!(".{}", ext_lower))
                        .unwrap_or(&fname_lower);
                    if stem == name_lower {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// 列出指定目录中所有 flf/tlf 字体文件名
pub fn list_flf_files(dirs: &[PathBuf]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("flf") || ext.eq_ignore_ascii_case("tlf") {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        files.push(name.to_string());
                    }
                }
            }
        }
    }
    files.sort();
    files
}

/// 递归列出系统字体目录中所有 TTF/OTF 字体文件名
pub fn list_system_ttf_files() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for dir in &system_font_dirs() {
        collect_ttf_files(dir, &mut files, 3);
    }
    files.sort();
    files
}

fn collect_ttf_files(dir: &Path, files: &mut Vec<String>, max_depth: usize) {
    if max_depth == 0 || !dir.exists() {
        return;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_ttf_files(&path, files, max_depth - 1);
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            if ext_lower == "ttf" || ext_lower == "otf" || ext_lower == "ttc" {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    files.push(name.to_string());
                }
            }
        }
    }
}

/// 确保扩展字体目录（目录B）存在，不存在时自动创建
///
/// - Linux/macOS：返回 figlet 标准目录（优先已存在的）
/// - Windows：返回 `%USERPROFILE%/fonts`
///
/// 目录不存在时自动创建。此目录用于存放下载的字体文件。
pub fn ensure_extended_font_dir() -> io::Result<PathBuf> {
    if let Some(dir) = extended_font_dir() {
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "无法确定扩展字体目录（目录B）",
        ))
    }
}

/// 获取用户主目录
pub fn home_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .ok()
            .map(PathBuf::from)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
