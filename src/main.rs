mod conf;

fn main() {;
    for repo in
    conf::get_repos("/home/cameron/CLionProjects/multipm/config") {
        println!("{}", repo);}
    }