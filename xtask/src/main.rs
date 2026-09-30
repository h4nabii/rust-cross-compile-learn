//! 构建后处理：把各目标平台的产物收集、改名到 `dist/` 目录。
//!
//! 通过 `cargo build-all` 运行（见 `.cargo/config.toml` 里的 alias），
//! 内部会先调用 `cargo build-all-only` 生成各 target 的产物。

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 需要收集的目标：目标三元组、产物改名用的 `平台-架构`、是否带 `.exe` 后缀。
const TARGETS: &[(&str, &str, bool)] = &[
    ("x86_64-pc-windows-gnu", "windows-x86_64", true),
    ("aarch64-pc-windows-gnullvm", "windows-arm64", true),
    ("x86_64-unknown-linux-musl", "linux-x86_64", false),
    ("aarch64-unknown-linux-musl", "linux-arm64", false),
];

const BIN_NAME: &str = "mycc";

fn main() -> Result<()> {
    let workspace_root = workspace_root()?;

    build_all(&workspace_root)?;

    let target_dir = workspace_root.join("target");
    let dist_dir = workspace_root.join("dist");

    fs::create_dir_all(&dist_dir)
        .with_context(|| format!("创建目录失败：{}", dist_dir.display()))?;

    for (triple, platform_arch, is_exe) in TARGETS {
        let suffix = if *is_exe { ".exe" } else { "" };
        let src = target_dir
            .join(triple)
            .join("release")
            .join(format!("{BIN_NAME}{suffix}"));
        let dst = dist_dir.join(format!("{BIN_NAME}-{platform_arch}{suffix}"));

        if !src.is_file() {
            bail!(
                "找不到产物：{}（请先运行 `cargo build-all-only`）",
                src.display()
            );
        }

        fs::copy(&src, &dst)
            .with_context(|| format!("复制失败：{} -> {}", src.display(), dst.display()))?;
        println!("已生成 {}", dst.display());
    }

    Ok(())
}

/// 调用 `cargo build-all-only`（zigbuild 全部目标），失败则直接中止后续收集步骤。
fn build_all(workspace_root: &Path) -> Result<()> {
    let status = Command::new("cargo")
        .arg("build-all-only")
        .current_dir(workspace_root)
        .status()
        .context("无法启动 `cargo build-all-only`")?;

    if !status.success() {
        bail!("`cargo build-all-only` 失败，退出码：{status}");
    }
    Ok(())
}

/// 工作区根目录：`xtask` 的上一级目录。
fn workspace_root() -> Result<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .map(Path::to_path_buf)
        .context("无法定位工作区根目录")
}
