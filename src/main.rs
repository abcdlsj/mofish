use clap::Parser;
use pretty_env_logger;
#[macro_use]
extern crate log;

mod crawl;
mod readability;
mod server;
mod storage;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "mofish.json")]
    output: String,

    #[clap(short, long, default_value = "false")]
    server: bool,

    #[clap(short, long, default_value = "3001")]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    match storage::init_storage() {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to init storage: {}", e);
            return;
        }
    }

    let args = Args::parse();

    if args.server {
        server::start(args.port.unwrap_or(3001)).await;
    } else {
        crawl::output_to(args.output)
    }
}
