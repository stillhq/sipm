use configparser::ini::Ini;
use std::{fmt, fs};

pub struct Config {
    enabled_sources: Vec<String>,
    repositories: Vec<Repo>,
    ignore_gpg: bool,
    ignore_mirrors: bool,
    local_install: bool,
    repo_directory: String,
    root_directory: String,
    user_directory: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, // "Enabled Sources: {:?}\nRepositories: {:?}\n\
            "Ignore GPG: {}\nIgnore Mirrors: {}\n\
            Root Directory: {}\nUser Directory: {}\nRepo Directory {}",
            // self.enabled_sources, self.repositories,
            self.ignore_gpg,
            self.ignore_mirrors,
            self.root_directory,
            self.user_directory,
            self.repo_directory
        )
    }
}

pub(crate) fn get_conf_from_file(file: &str) -> Config {
    let mut conf_ini = Ini::new();
    let conf_map = conf_ini.load(file).unwrap();
    let settings = conf_map.get("multipm").unwrap();

    // TODO: Implement sources
    let sources = Vec::new();
    // TODO: Local install != is root
    let local_install = false;

    let ignore_mirrors = conf_ini
        .getbool("multipm", "ignore_mirrors")
        .unwrap_or(Option::from(false))
        .unwrap();
    let ignore_gpg = conf_ini
        .getbool("multipm", "ignore_gpg")
        .unwrap_or(Option::from(false))
        .unwrap();

    let root_directory = settings
        .get("root_directory")
        .expect("Missing root_directory key in config file")
        .clone()
        .unwrap();
    let user_directory = settings
        .get("user_directory")
        .expect("Missing user_directory key in config file")
        .clone()
        .unwrap();
    let repo_directory = settings
        .get("repo_directory")
        .expect("Missing repo_directory key in config file")
        .clone()
        .unwrap();
    let repo_list = get_repos(repo_directory.as_str());

    Config {
        enabled_sources: sources,
        repositories: repo_list,
        ignore_gpg,
        ignore_mirrors,
        local_install,
        repo_directory,
        root_directory,
        user_directory,
    }
}

pub struct Repo {
    pub repo_id: String,
    pub url: String,
    pub mirrorlist: String,
    pub gpg: String,
    pub gpg_check: bool,
    pub enabled: bool,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Repo ID: {}\nURL: {}\nMirrorlist: {}\n\
            GPG: {}\nGPG Check: {}\nEnabled: {}\n",
            self.repo_id, self.url, self.mirrorlist, self.gpg, self.gpg_check, self.enabled
        )
    }
}

pub fn get_repos(directory: &str) -> Vec<Repo> {
    // Read the config file
    // Parse the config file
    // Return a vector of Repo structs
    let mut repos: Vec<Repo> = Vec::new();
    for file in fs::read_dir(directory)
        .expect(&format!("repo directory {} doesn't exist", &directory).to_string())
    {
        let filename = file.unwrap().path().display().to_string();
        if filename.ends_with(".mpmrepo") {
            for repo in get_repos_from_file(filename.as_str()) {
                repos.push(repo);
            }
        }
    }
    repos
}

pub fn get_repos_from_file(file: &str) -> Vec<Repo> {
    let mut repo_list: Vec<Repo> = Vec::new();
    let mut repo_ini = Ini::new();
    let repo_map = repo_ini.load(file).unwrap();
    for (repo, props) in repo_map.iter() {
        let mut gpg = String::from("");
        let gpg_check = false;
        if let Some(gpg_check) = repo_ini.getbool(repo, "gpgcheck").unwrap_or(Option::None) {
            if gpg_check {
                gpg = props
                    .get("gpg")
                    .expect(format!("Missing gpg key for {}", &repo.as_str()).as_str())
                    .clone()
                    .unwrap();
            }
        }
        repo_list.push(Repo {
            repo_id: repo.clone(),
            url: props
                .get("url")
                .expect(format!("Missing url for {}", &repo.as_str()).as_str())
                .clone()
                .unwrap(),
            mirrorlist: props
                .get("url")
                .unwrap_or(&Option::Some(String::from("")))
                .clone()
                .unwrap(),
            gpg,
            gpg_check,
            enabled: repo_ini
                .getbool(repo, "enabled")
                .unwrap_or(Option::None)
                .unwrap_or(true),
        });
    }
    repo_list
}
