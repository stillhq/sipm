pub mod conf;
pub mod sourceman {
    pub mod base; pub use base as srcman_base;
}

fn main() {
    println!("{}", conf::get_conf_from_file("/home/cameron/CLionProjects/multipm/config/multipm.mpmconf"));
    for repo in
    conf::get_repos("/home/cameron/CLionProjects/multipm/config") {
        println!("{}", repo);
    }

    println!("{:?}", sourceman::base::get_source_and_metadata("/home/cameron/CLionProjects/multipm/packages/battlenet.yml"));
}