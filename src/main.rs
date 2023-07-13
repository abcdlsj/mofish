use clap::Parser;

mod crawl;
mod server;

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
    let args = Args::parse();

    if args.server {
        server::start(args.port.unwrap_or(3001)).await;
    } else {
        crawl::output_to(args.output)
    }
}
