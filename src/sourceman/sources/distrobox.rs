#[derive(Debug)]
pub struct RecipeMetadata {
    name: String,
    description: String,
    author: String,
    package_author: String,
    package_version: f64,
    license: String,
    url: String,
    categories: Vec<String>,
    source_type: String,
    can_auto_update: bool,
    arch: String,
}

pub struct ContainerRecipe {
    container_id: String,
    container_name: String,
}

pub struct DistroboxSource;
// impl Source for DistroboxSource {
//     const NAME: String = String::from("distrobox");

//     fn initialize(&self, _: Config) -> i32 {
//         return 0;
//     }
//     fn install(&self, _: Recipe, _: Config) -> i32 {
//         todo!()
//     }
//     fn install_local_file(&self, _: Recipe, _: Config) -> i32 {
//         todo!()
//     }
//     fn install_with_files(&self, _: Recipe, _: Config) -> i32 {
//         todo!()
//     }
//     fn remove(&self, _: Recipe, _: Config) -> i32 {
//         todo!()
//     }
//     fn update_all(&self, _: Recipe, _: Config) -> i32 {
//         todo!()
//     }
//     fn update_pkg(&self, _: Recipe, _: Config) -> i32 {
//         todo!()
//     }
//     fn update_with_files(&self, _: Recipe, _: Config) -> i32 {
//         todo!()
//     }
//     fn validate_recipe(&self, _: Recipe) -> i32 {
//         todo!()
//     }
//     fn generate_recipe(&self, metadata: RecipeMetadata, source_data: dyn Any) -> (i32, Recipe) {
//         todo!()
//     }
//     fn interactive_recipe_generator(&self) -> (i32, dyn Any) {
//         todo!()
//     }
//     fn sync_repo_cache(&self, _: Config) -> i32 {
//         return 0;
//     }
// }
