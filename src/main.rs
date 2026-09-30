mod config;

use clap::{Parser, Subcommand};
use config::{CONFIG_FILE_NAME, Config};

#[derive(Parser)]
#[command(version, about = "跨平台构建辅助工具")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 输出 Hello World
    Hello,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 可执行文件同目录下的配置文件；不存在则为 None
    let config = Config::load_opt()?;

    match cli.command {
        Some(Commands::Hello) => {
            println!("helloworld");
            match &config {
                Some(cfg) => println!(
                    "已加载 {CONFIG_FILE_NAME}：name = {}",
                    cfg.name.as_deref().unwrap_or("<未设置>")
                ),
                None => println!("未找到 {CONFIG_FILE_NAME}，跳过配置"),
            }
        }
        None => println!("请指定一个子命令（试试 hello）"),
    }

    Ok(())
}
