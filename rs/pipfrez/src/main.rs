use std::{
    fs::File,
    io::{self, BufRead},
};

use clap::Parser;
use tools_core::get_tools_core_version;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version=env!("CARGO_PKG_VERSION"), about= "Small tool get all pip requestments in project", long_about = None)]
struct Args {
    #[arg(long, short, help = "project path", default_value = "./")]
    path: String,

    #[arg(long, short, help = "conda env name", default_value = "base")]
    conda_name: String,
}

fn main() -> anyhow::Result<()> {
    println!(
        "
    
██████╗ ██╗██████╗ ███████╗██████╗ ███████╗███████╗
██╔══██╗██║██╔══██╗██╔════╝██╔══██╗██╔════╝╚══███╔╝
██████╔╝██║██████╔╝█████╗  ██████╔╝█████╗    ███╔╝ 
██╔═══╝ ██║██╔═══╝ ██╔══╝  ██╔══██╗██╔══╝   ███╔╝  
██║     ██║██║     ██║     ██║  ██║███████╗███████╗
╚═╝     ╚═╝╚═╝     ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝  
                                                                  
Version {}, core version {}
    ",
        env!("CARGO_PKG_VERSION"),
        get_tools_core_version()
    );

    let args = Args::parse();

    for entry in WalkDir::new(/*for test*/ r"D:\github_repo\object_gan")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(s) = entry.path().to_str() {
            if s.ends_with(".py") {
                let f = File::open(s)?;
                let reader = io::BufReader::new(f);

                for line in reader.lines() {
                    let line = line?;
                    println!("{}", line);
                }
            }
        }
    }

    println!("END");
    anyhow::Ok(())
}
