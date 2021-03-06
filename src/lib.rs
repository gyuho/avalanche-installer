pub mod avalanchego;

use std::io::{self, Error, ErrorKind};

use serde::{Deserialize, Serialize};

/// ref. https://github.com/ava-labs/avalanchego/releases
/// ref. https://api.github.com/repos/ava-labs/avalanchego/releases/latest
pub async fn fetch_latest_release(org: &str, repo: &str) -> io::Result<GithubResponse> {
    let ep = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        org, repo
    );
    log::info!("fetching {}", ep);

    let rb = http_manager::get_non_tls(&ep, "").await?;
    let resp: GithubResponse = match serde_json::from_slice(&rb) {
        Ok(p) => p,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("failed to decode {}", e),
            ));
        }
    };
    Ok(resp)
}

/// ref. https://api.github.com/repos/ava-labs/avalanchego/releases/latest
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GithubResponse {
    pub tag_name: String,
    pub assets: Vec<GithubAsset>,
}

/// ref. https://api.github.com/repos/ava-labs/avalanchego/releases/latest
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
}
