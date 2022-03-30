use std::string::String;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::process::exit;

use yaml_rust::Yaml;
use yaml_rust::yaml::{Hash, Array};

/// Typealias for a Folder containing sub-folders
type NamedFolder = (String, Vec<Folder>);

/// Represents the two types of "folders" that are given via the yaml config file
pub enum Folder {
    /// A folder that contains nothing
    Final(String),
    /// A folder that contains sub-folders
    TopLevel(NamedFolder)
}

impl Folder {
    /// Creates a top-level folder
    fn from_string(name: &String) -> Folder {
        Folder::Final(String::from(name))
    }

    /// Constructs a Vec of Folders from a yaml Array
    pub fn from_array(arr: &Array) -> Vec<Folder> {
        let value: Vec<Folder> = arr.iter()
            .map(Folder::from)
            .filter(|a| a.is_some())
            .map(|x| x.unwrap()).collect();

        value
    }

    /// Constructs a Folder from a yaml Hash object
    fn from_hash(hash: &Hash) -> Option<Folder> {
        // Per config standards, a yaml hash should only contain a single key/value pair
        // of string/array, so we return the first (and only) key/value pair
        for (key, value) in hash.iter() {
            return match (key, value) {
                (Yaml::String(name), Yaml::Array(arr)) =>
                    Some(Folder::TopLevel((String::from(name), Folder::from_array(arr)))),
                // If we have the string as the key, but an invalid value, alert the user and abort
                (Yaml::String(name), _) => {
                    println!("folder '{}' is not a list. Aborting...", name);
                    exit(1);
                },
                _ => None
            };
        }

        return None;
    }

    /// Converts a yaml object to a folder
    fn from(yaml: &Yaml) -> Option<Folder> {
        match yaml {
            Yaml::String(name) => Some(Folder::from_string(name)),
            Yaml::Hash(hash) => Folder::from_hash(hash),
            _ => None
        }
    }
}

/// Handles the folder creation process
pub struct FolderEngine<'a> {
    /// The current sub-folder path
    current: Vec<String>,

    /// The named folder groups
    pub groups: &'a HashMap<String, Vec<Folder>>
}

impl FolderEngine<'_> {
    /// Constructs a new FolderEngine
    ///
    /// ## arguments
    ///
    /// * `groups` - HashMap containing the named folder groups and folders they represent
    pub fn new(groups: &HashMap<String, Vec<Folder>>) -> FolderEngine {
        FolderEngine {
            current: vec![],
            groups
        }
    }

    /// Creates a single directory.
    fn create_dir(&self, name: &String) -> Result<()> {
        let folder = if self.current.is_empty() {
            format!("./{}", name)
        } else {
            format!("./{}/{}", self.current.join("/"), name)
        };
        println!("Creating folder: {}", folder);
        fs::create_dir(Path::new(&folder))
    }


    /// Handles a single folder with no subfolders
    fn handle_name(&mut self, name: &String) -> Result<()> {
        if let Some(group) = self.groups.get(name) {
            self.create_folders(group)
        } else {
            self.create_dir(name)
        }
    }

    /// Recursively create folders that contain sub-folders
    fn handle_named_folder(&mut self, named_folder: &NamedFolder) -> Result<()> {
        let (name, folder) = named_folder;
        self.create_dir(name)?;
        self.current.push(String::from(name));
        self.create_folders(folder)?;
        self.current.pop();
        Ok(())
    }

    /// Main function to start the folder-creation process
    pub fn create_folders(&mut self, folders: &Vec<Folder>) -> Result<()> {
        for folder in folders {
            match folder {
                Folder::Final(name) => self.handle_name(name)?,
                Folder::TopLevel(named_folder) => self.handle_named_folder(named_folder)?
            }
        }

        Ok(())
    }
}