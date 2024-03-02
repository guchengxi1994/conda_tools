use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;
use tools_core::{get_package_info_by_env_name, get_tools_core_version, PackageInfo};
use walkdir::WalkDir;

const IMPORT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)import\s+([\w\.]+)(?:\s+as\s+\w+)?").unwrap());

const FROM_IMPORT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)from\s+([\w\.]+)(?:\s+import\s+\w+)?").unwrap());

fn extract_module_names(code: &str) -> String {
    let caps = FROM_IMPORT_REGEX.captures_iter(code).collect::<Vec<_>>();
    if caps.len() != 0 {
        let s = caps.first().unwrap()[1].to_string();
        if s.contains(".") {
            return s
                .split(".")
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string();
        }
        return s;
    }

    let caps = IMPORT_REGEX.captures_iter(code).collect::<Vec<_>>();
    if caps.len() != 0 {
        let s = caps.first().unwrap()[1].to_string();
        if s.contains(".") {
            return s
                .split(".")
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string();
        }
        return s;
    }

    "".to_string()
}

#[derive(Parser, Debug)]
#[command(version=env!("CARGO_PKG_VERSION"), about= "Small tool get all pip requestments in project", long_about = None)]
struct Args {
    #[arg(long, short, help = "project path", default_value = "./")]
    path: String,

    #[arg(long, short, help = "conda env name", default_value = "base")]
    conda_name: String,

    #[arg(long, help = "save requirements file path", default_value = "None")]
    save: String,
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

    let set: HashSet<PackageInfo> = get_package_info_by_env_name(args.conda_name.to_string())?;

    for entry in WalkDir::new(/*for test*/ r"D:\github_repo\object_gan")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(s) = entry.path().to_str() {
            if s.ends_with(".py") {
                let f = File::open(s)?;
                let reader = io::BufReader::new(f);

                for (index, line) in reader.lines().enumerate() {
                    let line = line?;
                    let module = extract_module_names(&line);
                    if module != "".to_owned() {
                        println!("{}  {}  {}", index, line, module);
                    }
                }
            }
        }
    }

    println!("END");
    anyhow::Ok(())
}

mod tests {
    #[test]
    fn test_regex() {
        let example_codes = vec![
            "import os",
            "    import sys",
            "from math import sqrt",
            "from collections import defaultdict",
            "from collections.aaa import bbb",
            "import cv2.imread as imread",
            "",
        ];

        for code in example_codes {
            let result = crate::extract_module_names(code);

            println!(
                "In the code \"{}\", the extracted module name is: {:?}",
                code, result
            );
        }
    }
}
