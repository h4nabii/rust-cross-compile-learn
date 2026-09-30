use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Hello) => println!("helloworld"),
        None => println!("请指定一个子命令（试试 hello）"),
    }
}
