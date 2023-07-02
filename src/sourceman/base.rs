use std::fmt::Debug;
use crate::conf::Config as Config;
use std::{fmt, fs};
use std::hash::Hash;
use std::io::Read;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub fn get_source_and_metadata (file_path: &str) -> (RecipeMetadata, Yaml) {
    let mut file = fs::File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let recipe = YamlLoader::load_from_str(&contents).unwrap();
    let metadata = &recipe[0]["metadata"];

    let categories_vec = metadata["categories"].as_vec().unwrap();
    let categories = categories_vec.iter().map(|x| x.as_str().unwrap().to_string()).collect(); // Convert categories to Vec<String> from YAML

    let recipe_metadata = RecipeMetadata {
        name: metadata["name"].as_str().unwrap().to_string(),
        description: metadata["description"].as_str().unwrap().to_string(),
        author: metadata["author"].as_str().unwrap().to_string(),
        package_author: metadata["package_author"].as_str().unwrap().to_string(),
        package_version: metadata["package_version"].as_f64().unwrap(),
        license: metadata["license"].as_str().unwrap().to_string(),
        url: metadata["url"].as_str().unwrap().to_string(),
        categories,
        source_type: metadata["type"].as_str().unwrap().to_string(),
        arch: metadata["arch"].as_str().unwrap().to_string(),
      };

    let source_data = &recipe[0][recipe_metadata.source_type.as_str()];
    (recipe_metadata, source_data.clone())
}

pub trait SourceRecipe {
    // fn prase_yaml (&self, file: &str) -> RecipeMetadata {
    // }
}

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
    arch: String
}
impl fmt::Debug for RecipeMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nDescription: {}\nAuthor: {}\n\
            Package Author: {}\nPackage Version: {}\nLicense: {}\n\
            URL: {}\nCategories: {:?}\nSource Type: {}\nArch: {}\n",
               self.name, self.description, self.author,
               self.package_author, self.package_version, self.license,
               self.url, self.categories, self.source_type, self.arch
        )
    }
}

pub trait Source {
    const NAME: String;
    const CUSTOM_FUNCTIONS: Vec<String>;
    const SEPARATE_FROM_MULTIPM_REPO: bool;

    fn initialize(&self, _: Config) -> i32 { return 0 }
    fn install(&self, _: dyn SourceRecipe, _: Config) -> i32;
    fn install_with_files(&self, _: dyn SourceRecipe, _: Config) -> i32;
    fn remove(&self, _: dyn SourceRecipe, _: Config) -> i32;
    fn update_all(&self, _: dyn SourceRecipe, _: Config) -> i32;
    fn update_pkg(&self, _: dyn SourceRecipe, _: Config) -> i32;
    fn update_with_files(&self, _: dyn SourceRecipe, _: Config) -> i32;
    fn validate_recipe(&self, _: dyn SourceRecipe) -> i32;
    fn get_repo_cache(&self, _: Config) -> i32 { return 0 }
}