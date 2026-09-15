//! Manual project paths are interpreted on the owning engine's platform.

use std::path::PathBuf;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpacePath {
    pub path: String,
    pub exists: bool,
    pub git_detected: bool,
}

pub fn prepare(path: &str, create_if_missing: bool) -> anyhow::Result<SpacePath> {
    anyhow::ensure!(
        !path.is_empty() && !path.contains('\0'),
        "enter a folder path"
    );
    #[cfg(not(windows))]
    anyhow::ensure!(
        !path.contains('\\') && path.as_bytes().get(1) != Some(&b':'),
        "Windows paths are not supported on this engine"
    );
    let path = if path == "~" {
        crate::repos::home_dir()
    } else if let Some(relative) = path.strip_prefix("~/").or_else(|| path.strip_prefix("~\\")) {
        crate::repos::home_dir().join(relative)
    } else {
        PathBuf::from(path)
    };
    anyhow::ensure!(
        path.is_absolute(),
        "enter an absolute folder path for this engine"
    );
    let exists = match std::fs::metadata(&path) {
        Ok(metadata) => {
            anyhow::ensure!(metadata.is_dir(), "the path is a file, not a folder");
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if create_if_missing {
                std::fs::create_dir_all(&path)?;
                true
            } else {
                false
            }
        }
        Err(error) => return Err(error.into()),
    };
    Ok(SpacePath {
        git_detected: exists && path.join(".git").exists(),
        path: path.to_string_lossy().into_owned(),
        exists,
    })
}
