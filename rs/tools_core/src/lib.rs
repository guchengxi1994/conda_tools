use core::hash::Hash;
use std::{collections::HashSet, fmt::Display, process::Command};

pub fn get_tools_core_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[derive(Ord)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub build: String,
    pub channel: String,
    pub env_name: String,
}

impl PartialEq for PackageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.version == other.version
            && self.build == other.build
            && self.channel == other.channel
    }
}

impl Display for PackageInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(name {}, version {},build {}, channel {},env_name {})",
            self.name, self.version, self.build, self.channel, self.env_name
        )
    }
}

impl Eq for PackageInfo {}

impl Hash for PackageInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.version.hash(state);
        self.build.hash(state);
        self.channel.hash(state);
        self.env_name.hash(state);
    }
}

impl PartialOrd for PackageInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl PackageInfo {
    pub fn from_str(s: &str, env_name: String) -> anyhow::Result<Self> {
        let mut res = s.split(" ").collect::<Vec<_>>();
        res.retain(|&x| x != "");
        if res.len() == 3 {
            return anyhow::Ok(PackageInfo {
                name: res[0].to_string(),
                version: res[1].to_string(),
                build: res[2].to_string(),
                channel: "".to_string(),
                env_name,
            });
        }
        if res.len() == 4 {
            return anyhow::Ok(PackageInfo {
                name: res[0].to_string(),
                version: res[1].to_string(),
                build: res[2].to_string(),
                channel: res[3].to_string(),
                env_name,
            });
        }

        anyhow::bail!("cannot convert")
    }
}

fn get_command_output(env: &String) -> anyhow::Result<String> {
    let conda_package_list = Command::new("conda").args(["list", "-n", env]).output()?;
    anyhow::Ok(std::str::from_utf8(&conda_package_list.stdout)?.to_owned())
}

pub fn get_package_info_by_env_name(env_name: String) -> anyhow::Result<HashSet<PackageInfo>> {
    let mut pip_set: HashSet<PackageInfo> = HashSet::new();
    let output = get_command_output(&env_name)?;

    let results;
    if cfg!(windows) {
        results = output.split("\r\n");
    } else {
        results = output.split("\n");
    }

    for i in results.into_iter() {
        if i.starts_with("#") {
            continue;
        }

        let pi = PackageInfo::from_str(i, env_name.clone());
        if let Ok(_pi) = pi {
            pip_set.insert(_pi);
        }
    }

    anyhow::Ok(pip_set)
}
