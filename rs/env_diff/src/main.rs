use clap::Parser;
use std::collections::HashSet;
use std::str;
use tools_core::{get_package_info_by_env_name, get_tools_core_version, PackageInfo};

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

fn main() -> anyhow::Result<()> {
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

    let first_set: HashSet<PackageInfo> =
        get_package_info_by_env_name(args.envs.first().unwrap().to_string())?;

    let second_set: HashSet<PackageInfo> =
        get_package_info_by_env_name(args.envs.last().unwrap().to_string())?;

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

    anyhow::Ok(())
}
