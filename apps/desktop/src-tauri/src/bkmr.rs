use serde::{Deserialize, Serialize};
use serde::de;
use std::process::Stdio;

/// Find the bkmr binary by checking common installation paths.
/// macOS .app bundles don't inherit the user's shell PATH, so we can't
/// rely on the OS resolving "bkmr" from PATH alone.
fn bkmr_path() -> String {
    let candidates = [
        "/opt/homebrew/bin/bkmr",
        "/usr/local/bin/bkmr",
    ];
    for &p in &candidates {
        if std::path::Path::new(p).exists() {
            return p.to_string();
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        let local = std::path::Path::new(&home).join(".local/bin/bkmr");
        if local.exists() {
            return local.to_string_lossy().to_string();
        }
        let cargo = std::path::Path::new(&home).join(".cargo/bin/bkmr");
        if cargo.exists() {
            return cargo.to_string_lossy().to_string();
        }
    }
    // Fallback: let the OS resolve via PATH (works in terminal/dev mode)
    "bkmr".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct BkmrBookmark {
    pub id: u64,
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub modified: String,
}

#[derive(Debug, Clone, Deserialize)]
struct BkmrBookmarkRaw {
    pub id: u64,
    pub url: String,
    pub title: String,
    #[serde(deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub modified: String,
}

/// Deserialize tags that bkmr outputs as either a JSON array []
/// or a comma-separated string  (with optional leading/trailing commas).
fn deserialize_tags<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Array(arr) => arr
            .into_iter()
            .map(|v| Ok(v.as_str().unwrap_or_default().to_string()))
            .collect(),
        serde_json::Value::String(s) => Ok(s
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect()),
        _ => Err(de::Error::custom("expected array or string for tags")),
    }
}

impl From<BkmrBookmarkRaw> for BkmrBookmark {
    fn from(raw: BkmrBookmarkRaw) -> Self {
        BkmrBookmark {
            id: raw.id,
            url: raw.url,
            title: raw.title,
            tags: raw.tags,
            description: raw.description,
            modified: raw.modified,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BkmrTag {
    pub name: String,
    pub count: u64,
}

/// Perform a hybrid search (full-text + semantic) via bkmr hsearch.
pub async fn hsearch(query: &str, tags: &[String]) -> Result<Vec<BkmrBookmark>, String> {
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.args(["hsearch", "--json", "--limit", "1000"]);
    if !tags.is_empty() {
        cmd.arg("--tags");
        cmd.arg(tags.join(","));
    }
    cmd.arg(query);
    run_bkmr(cmd).await
}

/// Search bookmarks by tags only (no text query), using bkmr search.
pub async fn search_by_tags(tags: &[String]) -> Result<Vec<BkmrBookmark>, String> {
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.args(["search", "--json", "--limit", "1000"]);
    if !tags.is_empty() {
        cmd.arg("--tags");
        cmd.arg(tags.join(","));
    }
    run_bkmr(cmd).await
}

/// Get all tags with counts.
pub async fn get_tags() -> Result<Vec<BkmrTag>, String> {
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.arg("tags");
    let output = cmd.output().await.map_err(|e| format!("Failed to run bkmr tags: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bkmr tags failed: {stderr}"));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut tags = Vec::new();
    for line in stdout.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() { continue; }
        // Format: "tagname (count)"
        if let Some(paren) = line.rfind(" (") {
            let name = line[..paren].trim().to_string();
            if let Ok(count) = line[paren + 2..].trim_end_matches(')').trim().parse::<u64>() {
                tags.push(BkmrTag { name, count });
            }
        }
    }
    Ok(tags)
}

/// Add a bookmark via bkmr add. Returns the new bookmark ID.
pub async fn add_bookmark(url: &str, title: &str, tags: &[String], description: &str) -> Result<u64, String> {
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.arg("add");
    cmd.arg("--no-color");
    cmd.arg("--no-web");
    cmd.arg("--title");
    cmd.arg(title);
    if !description.is_empty() {
        cmd.arg("--description");
        cmd.arg(description);
    }
    cmd.arg(url);
    if !tags.is_empty() {
        cmd.arg(tags.join(","));
    }

    let output = cmd.output().await.map_err(|e| format!("Failed to run bkmr add: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bkmr add failed: {stderr}"));
    }
    // bkmr add prints: "Added: <title> (ID: <id>)" -- parse the ID
    let stdout = String::from_utf8_lossy(&output.stdout);
    let id_str = stdout.split("ID: ").nth(1).and_then(|s| s.trim_end_matches(')').trim().split_whitespace().next());
    id_str.and_then(|s| s.parse().ok())
        .ok_or_else(|| format!("Could not parse bookmark ID from: {stdout}"))
}

/// Delete bookmarks by IDs. Returns the number of deleted bookmarks.
pub async fn delete_bookmarks(ids: &[u64]) -> Result<u64, String> {
    let ids_str = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.arg("delete");
    cmd.arg("--no-color");
    cmd.arg(&ids_str);
    cmd.stdin(std::process::Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("Failed to run bkmr delete: {e}"))?;
    // bkmr delete prompts interactively; pipe "y" to confirm
    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        let _ = stdin.write_all(b"y\n").await;
    }
    let output = child.wait_with_output().await.map_err(|e| format!("Failed to run bkmr delete: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bkmr delete failed: {stderr}"));
    }
    Ok(ids.len() as u64)
}

/// Export all bookmarks as JSON to a file. Returns the file path.
pub async fn export_all(dir: &str) -> Result<String, String> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("bookmarks-{date}.json");
    let path = std::path::Path::new(dir).join(&filename);

    let output = tokio::process::Command::new(bkmr_path())
        .args(["search", "--json", "--limit", "10000"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run bkmr search: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bkmr search failed: {stderr}"));
    }

    // Format JSON nicely and write to file
    let bookmarks: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse bkmr output: {e}"))?;
    let pretty = serde_json::to_string_pretty(&bookmarks)
        .map_err(|e| format!("Failed to format JSON: {e}"))?;
    tokio::fs::write(&path, &pretty)
        .await
        .map_err(|e| format!("Failed to write backup file: {e}"))?;

    Ok(path.to_string_lossy().to_string())
}
/// Load all bookmarks (for local fuzzy search).
pub async fn get_all_bookmarks() -> Result<Vec<BkmrBookmark>, String> {
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.args(["search", "--json", "--limit", "50000"]);
    run_bkmr(cmd).await
}

/// Check if a bookmark exists with the exact URL.
pub async fn check_bookmark(url: &str) -> Result<Option<BkmrBookmark>, String> {
    let bookmarks = get_all_bookmarks().await?;
    Ok(bookmarks.into_iter().find(|b| b.url == url))
}

/// Show a single bookmark by ID.
pub async fn show_bookmark(id: u64) -> Result<Option<BkmrBookmark>, String> {
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.args(["show", "--json", "--no-color", &id.to_string()]);
    let output = cmd.output().await.map_err(|e| format!("Failed to run bkmr show: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bkmr show failed: {stderr}"));
    }
    let raw: Vec<BkmrBookmarkRaw> = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse bkmr output: {e}"))?;
    Ok(raw.into_iter().next().map(Into::into))
}

/// Update a bookmark's title and tags (overwrite mode with --force).
pub async fn update_bookmark(id: u64, title: &str, tags: &[String], description: &str) -> Result<(), String> {
    let mut cmd = tokio::process::Command::new(bkmr_path());
    cmd.arg("update");
    cmd.arg("--no-color");
    cmd.arg(id.to_string());
    cmd.arg("--title");
    cmd.arg(title);
    if !description.is_empty() {
        cmd.arg("--description");
        cmd.arg(description);
    }
    if !tags.is_empty() {
        cmd.arg("--force");
        cmd.arg("--tags");
        cmd.arg(tags.join(","));
    }
    let output = cmd.output().await.map_err(|e| format!("Failed to run bkmr update: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bkmr update failed: {stderr}"));
    }
    Ok(())
}


async fn run_bkmr(mut cmd: tokio::process::Command) -> Result<Vec<BkmrBookmark>, String> {
    let output = cmd.output().await.map_err(|e| format!("Failed to run bkmr: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bkmr command failed: {stderr}"));
    }
    let raw: Vec<BkmrBookmarkRaw> = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse bkmr output: {e}"))?;
    Ok(raw.into_iter().map(Into::into).collect())
}
