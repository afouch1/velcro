mod folder_engine;

use folder_engine::*;

use std::env;
use std::collections::HashMap;
use std::string::String;
use std::fs;

use yaml_rust::YamlLoader;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Only take a single argument that is the yaml config file.
    // In the future I might add the option to create the folders somewhere
    // other than the current working directory
    if args.len() < 2 {
        println!("Missing config file. ");
        return;
    }

    let text = fs::read_to_string(&args[1])
        .expect("Unable to read config file");

    // Alert the user a success message or a yaml parsing error
    let message = match YamlLoader::load_from_str(text.as_str()) {
        Ok(docs) => begin(&docs[0]),
        Err(err) => err.to_string()
    };

    println!("{}", message)
}

/// Converts a yaml hash object to the requires HashMap used by the FolderEngine
fn hash_to_map(hash: &Hash) -> HashMap<String, Vec<Folder>> {
    let mut map: HashMap<String, Vec<Folder>> = HashMap::new();

    for (key, value) in hash.iter() {
        if let (Yaml::String(name), Yaml::Array(arr)) = (key, value) {
            map.insert(String::from(name), Folder::from_array(arr));
        };
    };

    map
}

/// Main entry point, given a valid yaml file
fn begin(doc: &yaml_rust::Yaml) -> String {
    let hash = doc.as_hash().expect("Invalid yaml format. Aborting. ");
    let mut map = hash_to_map(hash);

    let folders = map.remove("folders").expect("Missing top level 'folders' item");

    let mut engine = FolderEngine::new(&map);
    match engine.create_folders(&folders) {
        Ok(_) => String::from("Finished!"),
        Err(err) => err.to_string()
    }
}
