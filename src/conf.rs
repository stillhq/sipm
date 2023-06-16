use std::{fmt, fs};
use configparser::ini::Ini;

pub struct Config {
    enabled_sources: Vec<String>,
    repositories: Vec<Repo>,
    local_install: bool,
    ignore_gpg: bool,
    ignore_mirrors: bool,
    default_local_install: bool,
    container_directory: String,
    cache_directory: String
}

pub struct Repo {
    pub repo_id: String,
    pub url: String,
    pub mirrorlist: String,
    pub gpg: String,
    pub gpg_check: bool,
    pub enabled: bool
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "Repo ID: {}\nURL: {}\nMirrorlist: {}\n\
            GPG: {}\nGPG Check: {}\nEnabled: {}\n",
            self.repo_id, self.url, self.mirrorlist,
            self.gpg, self.gpg_check, self.enabled
        )
    }
}

pub fn get_repos(directory: &str) -> Vec<Repo> {
    // Read the config file
    // Parse the config file
    // Return a vector of Repo structs
    let mut repos: Vec<Repo> = Vec::new();
    for file in fs::read_dir(&directory.to_string()).expect(&format!("{} {}", &directory, "doesn't exist").to_string()) {
        let filename = file.unwrap().path().display().to_string().as_str();
        if filename.ends_with(".mpmrepo") {
            for repo in get_repos_from_file(filename) {
                repos.push(repo);
            }
        }
    }
    return repos;
}

pub fn get_repos_from_file(file: &str) -> Vec<Repo> {
    let mut repo_list: Vec<Repo> = Vec::new();
    let mut repo_ini = Ini::new();
    let repo_map = repo_ini.load(file).unwrap();
    for (repo, props) in repo_map.iter() {
        let mut gpg = String::from("");
        let mut gpg_check = false;
        if let Some(gpg_check) = repo_ini.getbool(&repo, "gpgcheck").unwrap_or(Option::None) {
            if gpg_check {
                gpg = props.get("gpg").expect(
                    format!("Missing gpg key for {}", &repo.as_str()).as_str())
                    .clone().unwrap();
            }
        }
        repo_list.push(Repo {
            repo_id: repo.clone(),
            url: props.get("url").expect(
                format!("Missing url for {}", &repo.as_str()).as_str())
                .clone().unwrap(),
            mirrorlist: props.get("url")
                .unwrap_or(&Option::Some(String::from("")))
                .clone().unwrap(),
            gpg,
            gpg_check,
            enabled: repo_ini.getbool(&repo, "enabled").unwrap_or(Option::None).unwrap_or(true)
        });
    }
    return repo_list;
}