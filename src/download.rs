use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// 懒加载的 HTTP Agent，自动从 HTTP_PROXY/HTTPS_PROXY/NO_PROXY 环境变量读取代理设置
static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| ureq::AgentBuilder::new().build());

/// GitHub 字体仓库配置
const FONT_REPOS: &[FontRepo] = &[
    FontRepo {
        owner: "xero",
        repo: "figlet-fonts",
        branch: "master",
        subdir: "",
    },
    FontRepo {
        owner: "PhMajerus",
        repo: "FIGfonts",
        branch: "main",
        subdir: "fonts",
    },
    FontRepo {
        owner: "aguegu",
        repo: "BitmapFont",
        branch: "master",
        subdir: "font",
    },
];

/// 默认下载重试次数
const MAX_RETRIES: u32 = 3;

struct FontRepo {
    owner: &'static str,
    repo: &'static str,
    branch: &'static str,
    subdir: &'static str,
}

/// 下载结果
#[derive(Debug)]
pub struct DownloadResult {
    pub file_path: PathBuf,
    pub source_url: String,
}

/// 从 GitHub 仓库下载字体文件到目标目录（支持重试）
///
/// 依次尝试所有 GitHub 仓库，找到后下载到 `dest_dir`。
/// 自动补全 .flf/.tlf 扩展名。
pub fn download_font(name: &str, dest_dir: &Path) -> io::Result<DownloadResult> {
    let candidates = resolve_candidates(name);

    for repo in FONT_REPOS {
        for candidate in &candidates {
            let url = build_raw_url(repo, candidate);

            match try_download_with_retry(&url, candidate, dest_dir, MAX_RETRIES) {
                Ok(result) => return Ok(result),
                Err(_) => continue,
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("无法从任何 GitHub 仓库下载字体: {}", name),
    ))
}

/// 以精确文件名从 GitHub 仓库下载文件到目标目录（支持重试）
///
/// 不自动补全扩展名，适用于 HZK 等无扩展名的文件。
/// 依次尝试所有 GitHub 仓库。
pub fn download_file(name: &str, dest_dir: &Path) -> io::Result<DownloadResult> {
    for repo in FONT_REPOS {
        let url = build_raw_url(repo, name);

        match try_download_with_retry(&url, name, dest_dir, MAX_RETRIES) {
            Ok(result) => return Ok(result),
            Err(_) => continue,
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("无法从任何 GitHub 仓库下载文件: {}", name),
    ))
}

/// 下载字体文件的原始数据（支持重试）
pub fn download_font_data(name: &str) -> io::Result<(Vec<u8>, String)> {
    let candidates = resolve_candidates(name);

    for repo in FONT_REPOS {
        for candidate in &candidates {
            let url = build_raw_url(repo, candidate);

            match try_download_data_with_retry(&url, MAX_RETRIES) {
                Ok(data) => return Ok((data, candidate.clone())),
                Err(_) => continue,
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("无法从任何 GitHub 仓库下载字体: {}", name),
    ))
}

/// 获取两个 GitHub 仓库中可用的字体文件列表
pub fn list_available_online() -> io::Result<Vec<String>> {
    let mut all_files: Vec<String> = Vec::new();

    for repo in FONT_REPOS {
        let api_url = if repo.subdir.is_empty() {
            format!(
                "https://api.github.com/repos/{}/{}/contents/",
                repo.owner, repo.repo
            )
        } else {
            format!(
                "https://api.github.com/repos/{}/{}/contents/{}",
                repo.owner, repo.repo, repo.subdir
            )
        };

        if let Ok(files) = fetch_repo_file_list(&api_url) {
            for f in files {
                let lower = f.to_lowercase();
                if lower.ends_with(".flf") || lower.ends_with(".tlf") {
                    if !all_files.contains(&f) {
                        all_files.push(f);
                    }
                }
            }
        }
    }

    if all_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "无法获取在线字体列表（GitHub API 可能限流）",
        ));
    }

    all_files.sort();
    Ok(all_files)
}

/// 根据仓库配置和文件名构建 raw.githubusercontent.com 直链 URL
fn build_raw_url(repo: &FontRepo, filename: &str) -> String {
    if repo.subdir.is_empty() {
        format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            repo.owner, repo.repo, repo.branch, filename
        )
    } else {
        format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}/{}",
            repo.owner, repo.repo, repo.branch, repo.subdir, filename
        )
    }
}

/// 带重试的下载：尝试从 URL 下载文件并保存到目标目录
fn try_download_with_retry(
    url: &str,
    filename: &str,
    dest_dir: &Path,
    max_retries: u32,
) -> io::Result<DownloadResult> {
    let mut last_err = None;
    for attempt in 0..max_retries {
        match try_download(url, filename, dest_dir) {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_err = Some(e);
                if attempt < max_retries - 1 {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    }
    Err(last_err.unwrap_or_else(|| {
        io::Error::other("下载失败：已达最大重试次数")
    }))
}

/// 带重试的数据下载：尝试从 URL 下载数据
fn try_download_data_with_retry(url: &str, max_retries: u32) -> io::Result<Vec<u8>> {
    let mut last_err = None;
    for attempt in 0..max_retries {
        match try_download_data(url) {
            Ok(data) => return Ok(data),
            Err(e) => {
                last_err = Some(e);
                if attempt < max_retries - 1 {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    }
    Err(last_err.unwrap_or_else(|| {
        io::Error::other("下载失败：已达最大重试次数")
    }))
}

/// 尝试从 URL 下载并保存到目标目录
fn try_download(url: &str, filename: &str, dest_dir: &Path) -> io::Result<DownloadResult> {
    let data = try_download_data(url)?;

    std::fs::create_dir_all(dest_dir)?;
    let file_path = dest_dir.join(filename);
    std::fs::write(&file_path, &data)?;

    Ok(DownloadResult {
        file_path,
        source_url: url.to_string(),
    })
}

/// 尝试从 URL 下载数据
fn try_download_data(url: &str) -> io::Result<Vec<u8>> {
    let resp = AGENT.get(url)
        .set("User-Agent", "rsidlet/0.1.0")
        .call()
        .map_err(|e| {
            io::Error::new(io::ErrorKind::ConnectionRefused, format!("下载失败: {}", e))
        })?;

    let mut data = Vec::new();
    resp.into_reader().read_to_end(&mut data)?;
    Ok(data)
}

/// 获取 GitHub 仓库目录中的文件列表
fn fetch_repo_file_list(api_url: &str) -> io::Result<Vec<String>> {
    let resp = AGENT.get(api_url)
        .set("User-Agent", "rsidlet/0.1.0")
        .set("Accept", "application/vnd.github.v3+json")
        .call()
        .map_err(|e| {
            io::Error::new(io::ErrorKind::ConnectionRefused, format!("API 请求失败: {}", e))
        })?;

    let body = resp.into_string().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("读取响应失败: {}", e))
    })?;

    // GitHub API 可能返回 prettified 或 minified JSON
    // 需要在整个 body 中搜索所有 "name" 字段，而非逐行查找第一个
    let mut names = Vec::new();
    let mut pos = 0usize;
    while pos < body.len() {
        let haystack = &body[pos..];
        let start = match haystack.find("\"name\"") {
            Some(s) => s,
            None => break,
        };
        let abs = pos + start;
        // 找到 ": " 后面的引号
        let after_key = match body[abs..].find(':') {
            Some(c) => abs + c + 1,
            None => {
                pos = abs + 6;
                continue;
            }
        };
        let val = &body[after_key..];
        let q1 = match val.find('"') {
            Some(q) => q,
            None => {
                pos = abs + 6;
                continue;
            }
        };
        let q2 = match val[q1 + 1..].find('"') {
            Some(q) => q,
            None => {
                pos = abs + 6;
                continue;
            }
        };
        let name = &val[q1 + 1..q1 + 1 + q2];
        names.push(name.to_string());
        pos = after_key + q1 + 1 + q2 + 1;
    }

    Ok(names)
}

/// 根据字体名生成候选文件名列表
fn resolve_candidates(name: &str) -> Vec<String> {
    let name_lower = name.to_lowercase();

    // 如果已经带扩展名，直接使用
    if name_lower.ends_with(".flf") || name_lower.ends_with(".tlf") {
        return vec![name.to_string(), name_lower];
    }

    // 尝试添加常见扩展名
    vec![
        format!("{}.flf", name),
        format!("{}.tlf", name),
        format!("{}.FLF", name),
        format!("{}.TLF", name),
    ]
}
