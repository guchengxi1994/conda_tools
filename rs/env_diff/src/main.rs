use clap::Parser;
use std::collections::HashSet;
use std::process::Command;
use std::str;
use tools_core::{get_tools_core_version, PackageInfo};

const FIXED_LENGTH: usize = 20;

#[derive(Parser, Debug)]
#[command(version=env!("CARGO_PKG_VERSION"), about= "Small tool to compare two conda envs", long_about = None)]
struct Args {
    #[arg(help = "conda envs")]
    envs: Vec<String>,

    #[arg(long, default_value = "false", help = "only shows pypi packages")]
    pip_only: bool,

    #[arg(long, default_value = "false", help = "only shows first differences")]
    first_only: bool,

    #[arg(long, default_value = "false", help = "only shows second differences")]
    second_only: bool,

    #[arg(long, short, default_value = "false", help = "format output")]
    format: bool,
}

fn get_command_output(env: &String) -> anyhow::Result<String> {
    let conda_package_list = Command::new("conda").args(["list", "-n", env]).output()?;
    anyhow::Ok(str::from_utf8(&conda_package_list.stdout)?.to_owned())
}

fn padding_string(s: &str) -> String {
    if s.len() < FIXED_LENGTH {
        let mut _s: String = String::from(s);
        let padding_size = FIXED_LENGTH - s.len();
        " ".repeat(padding_size) + &_s
    } else {
        s[0..FIXED_LENGTH].to_string()
    }
}

fn format_output(p: &PackageInfo, first: bool) {
    if first {
        println!(
            "{}{}{}{}{}{}",
            padding_string(&p.name),
            padding_string(&p.version),
            padding_string(&p.build),
            padding_string(&p.channel),
            padding_string(&p.env_name),
            padding_string(&"")
        )
    } else {
        println!(
            "{}{}{}{}{}{}",
            padding_string(&p.name),
            padding_string(&p.version),
            padding_string(&p.build),
            padding_string(&p.channel),
            padding_string(&""),
            padding_string(&p.env_name)
        )
    }
}

fn main() {
    let args = Args::parse();

    println!(
        "
    
███████╗███╗   ██╗██╗   ██╗    ██████╗ ██╗███████╗███████╗
██╔════╝████╗  ██║██║   ██║    ██╔══██╗██║██╔════╝██╔════╝
█████╗  ██╔██╗ ██║██║   ██║    ██║  ██║██║█████╗  █████╗  
██╔══╝  ██║╚██╗██║╚██╗ ██╔╝    ██║  ██║██║██╔══╝  ██╔══╝  
███████╗██║ ╚████║ ╚████╔╝     ██████╔╝██║██║     ██║     
╚══════╝╚═╝  ╚═══╝  ╚═══╝      ╚═════╝ ╚═╝╚═╝     ╚═╝     
                                                                  
Version {}, core version {}
    ",
        env!("CARGO_PKG_VERSION"),
        get_tools_core_version()
    );

    if args.envs.len() != 2 {
        println!("Error: wrong env list length");
        std::process::exit(1);
    }

    let mut first_set: HashSet<PackageInfo> = HashSet::new();

    if let Some(first) = args.envs.first() {
        let first_pip_list = get_command_output(first);
        match first_pip_list {
            Ok(_s) => {
                let results;
                if cfg!(windows) {
                    results = _s.split("\r\n");
                } else {
                    results = _s.split("\n");
                }

                for i in results.into_iter() {
                    if i.starts_with("#") {
                        continue;
                    }

                    let pi = PackageInfo::from_str(i, first.clone());
                    if let Ok(_pi) = pi {
                        first_set.insert(_pi);
                    }
                }
            }
            Err(_) => {
                std::process::exit(1);
            }
        }
    }

    let mut second_set: HashSet<PackageInfo> = HashSet::new();

    if let Some(second) = args.envs.last() {
        let second_pip_list = get_command_output(second);
        match second_pip_list {
            Ok(_s) => {
                let results;
                if cfg!(windows) {
                    results = _s.split("\r\n");
                } else {
                    results = _s.split("\n");
                }

                for i in results.into_iter() {
                    if i.starts_with("#") {
                        continue;
                    }

                    let pi = PackageInfo::from_str(i, second.clone());
                    if let Ok(_pi) = pi {
                        second_set.insert(_pi);
                    }
                }
            }
            Err(_) => {
                std::process::exit(1);
            }
        }
    }

    if args.format {
        println!(
            "{}{}{}{}{}{}",
            padding_string(&"Name"),
            padding_string(&"Version"),
            padding_string(&"Build"),
            padding_string(&"Channel"),
            padding_string(&"First env"),
            padding_string(&"Second env")
        )
    }

    if args.first_only {
        let diff_set = first_set.difference(&second_set);
        let mut diff_vec = diff_set.into_iter().collect::<Vec<_>>();
        diff_vec.sort();

        for i in diff_vec {
            if args.pip_only {
                if i.channel == "pypi".to_owned() {
                    println!("{}", i);
                }
            } else {
                println!("{}", i);
            }
        }
        std::process::exit(0);
    }

    if args.second_only {
        let diff_set = second_set.difference(&first_set);
        let mut diff_vec = diff_set.into_iter().collect::<Vec<_>>();
        diff_vec.sort();

        for i in diff_vec {
            if args.pip_only {
                if i.channel == "pypi".to_owned() {
                    println!("{}", i);
                }
            } else {
                println!("{}", i);
            }
        }
        std::process::exit(0);
    }

    let mut diff_vec_1 = first_set.difference(&second_set).collect::<Vec<_>>();
    let mut diff_vec_2 = second_set.difference(&first_set).collect::<Vec<_>>();

    diff_vec_1.append(&mut diff_vec_2);
    diff_vec_1.sort();
    for i in diff_vec_1 {
        if args.pip_only {
            if i.channel == "pypi".to_owned() {
                if args.format {
                    format_output(i, i.env_name == args.envs[0]);
                } else {
                    println!("{}", i);
                }
            }
        } else {
            if args.format {
                format_output(i, i.env_name == args.envs[0]);
            } else {
                println!("{}", i);
            }
        }
    }
}
