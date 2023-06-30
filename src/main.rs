pub mod conf;

fn main() {
    println!("{}", conf::get_conf_from_file("/home/cameron/CLionProjects/multipm/config/multipm.mpmconf"));
    for repo in
    conf::get_repos("/home/cameron/CLionProjects/multipm/config") {
        println!("{}", repo);}
    }