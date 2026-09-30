//! 配置文件的定位、读取与解析。

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// 配置文件名。
///
/// 固定放在「当前工作目录」下。需要改名时只改这一处。
pub const CONFIG_FILE_NAME: &str = "mycc.config.json";

/// 配置文件的内容。
///
/// 按实际的 JSON 结构增删字段即可；多余的字段会被忽略，缺失的字段
/// 因为有 `#[serde(default)]` 会取默认值，所以不会报错。
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// 示例字段，替换成你自己的字段。
    #[serde(default)]
    pub name: Option<String>,
}

impl Config {
    /// 配置文件的完整路径：`<当前工作目录>/mycc.config.json`
    pub fn path() -> Result<PathBuf> {
        let dir = std::env::current_dir().context("无法获取当前工作目录")?;
        Ok(dir.join(CONFIG_FILE_NAME))
    }

    /// 读取并解析配置。
    ///
    /// 文件不存在时返回 `Ok(None)`，适合「配置可选」的场景。
    pub fn load_opt() -> Result<Option<Self>> {
        let path = Self::path()?;
        if !path.is_file() {
            return Ok(None);
        }
        Self::from_file(&path).map(Some)
    }

    /// 从指定路径读取并解析。
    pub fn from_file(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("读取配置文件失败：{}", path.display()))?;
        let config = serde_json::from_str(&text)
            .with_context(|| format!("解析配置文件失败：{}", path.display()))?;
        Ok(config)
    }
}
