use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = 0x10000)]
    pub mem: usize,

    pub file: String,
}
