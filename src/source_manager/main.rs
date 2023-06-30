pub trait Source {
    fn get_name() -> String {return String::new()}
    fn get_custom_functions() -> Vec<String> {return Vec::new()}
    fn install(&self, recipe, config) -> i32;
    fn install_with_files(&self, recipe, config) -> i32;
    fn remove(&self, recipe, config) -> i32;
    fn update_all(&self, recipe, config) -> i32;
    fn update_pkg(&self, recipe, config) -> i32;
    fn update_with_files(&self, recipe, config) -> i32;
}

fn main () {

}