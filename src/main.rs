use std::{path::PathBuf, str::FromStr};

use anyhow::*;
use structopt::StructOpt;
use tiem::Tiem;

#[derive(Debug, Clone, StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    subcmd: SubCmd,
}

#[derive(Debug, Clone, StructOpt)]
enum SubCmd {
    Start,
    Stop,
    Status,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    let home = dirs::home_dir().ok_or_else(|| anyhow!("Failed to get home directory."))?;
    let status = home.join("tiem/status.json");
    let log = home.join("tiem/log/");

    let tiem = Tiem::load(&status, &log)?;

    match opts.subcmd {
        SubCmd::Start => {
            tiem.start()?;
        }
        SubCmd::Stop => {
            todo!()
        }
        SubCmd::Status => {
            todo!()
        }
    }
    Ok(())
}
