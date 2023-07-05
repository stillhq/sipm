pub mod conf;
pub mod sourceman {
    pub mod base;
    pub mod sources;
}

fn main() {
    println!("{}", conf::get_conf_from_file("/home/cameron/CLionProjects/multipm/config/multipm.mpmconf"));
    for repo in
    conf::get_repos("/home/cameron/CLionProjects/multipm/config") {
        println!("{}", repo);
    }

    println!("{:?}", sourceman::base::get_metadata_and_source("/home/cameron/CLionProjects/multipm/packages/battlenet.yml"));
}