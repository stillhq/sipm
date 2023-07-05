use crate::sourceman::base as Base;

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
    arch: String
}
impl fmt::Debug for RecipeMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nDescription: {}\nAuthor: {}\n\
            Package Author: {}\nPackage Version: {}\nLicense: {}\n\
            URL: {}\nCategories: {:?}\nSource Type: {}\nArch: {}\nCan Auto Update: {}\n",
               self.name, self.description, self.author,
               self.package_author, self.package_version, self.license,
               self.url, self.categories, self.source_type, self.arch, self.can_auto_update
        )
    }
}

pub struct DistroboxSource;
impl Base::Source for DistroboxSource {
    const NAME: String = String::from("distrobox");

    fn initialize(&self, _: Config) -> i32 { return 0 }
    fn install(&self, _: Recipe, _: Config) -> i32;
    fn install_local_file(&self, _: Recipe, _: Config) -> i32;
    fn install_with_files(&self, _: Recipe, _: Config) -> i32;
    fn remove(&self, _: Recipe, _: Config) -> i32;
    fn update_all(&self, _: Recipe, _: Config) -> i32;
    fn update_pkg(&self, _: Recipe, _: Config) -> i32;
    fn update_with_files(&self, _: Recipe, _: Config) -> i32;
    fn validate_recipe(&self, _: Recipe) -> i32;
    fn generate_recipe(&self, metadata: RecipeMetadata, source_data: dyn Any) -> (i32, Recipe);
    fn interactive_recipe_generator(&self) -> (i32, dyn Any);
    fn sync_repo_cache(&self, _: Config) -> i32 { return 0 }
}
