use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub dir: Option<String>,

    #[arg(long)]
    pub dbfilename: Option<String>,
}

