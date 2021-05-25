extern crate yaml_rust;
extern crate anyhow;

use yaml_rust::YamlLoader;
use std::fs;
use std::path::Path;

pub struct Command {
    pub description: Option<String>,
    pub regex: String,
    pub replies: Option<Vec<String>>,
    pub execute: Option<String>,
}

impl Command {
    pub fn init(
        regex: String,
        description: Option<String>,
        replies: Option<Vec<String>>,
        execute: Option<String>,
    ) -> Result<Self, anyhow::Error> {
        if replies == None && execute == None {
            let error_text = format!("Command {} does not provide either of replies or executor", regex);
            return Err(anyhow::Error::msg(error_text));
        }
        Ok(Self {
            description: description,
            regex: regex,
            replies: replies,
            execute: execute,
        })
    }

    pub fn get_description(&self) -> String {
        match &self.description {
            Some(desc) => desc.to_owned(),
            None => String::from("No description specified")
        }
    }

    pub fn get_reply(&self) -> String {
        String::from("Not Implemented")
    }
}

pub struct CommandSet {
    pub commands: Vec<Command>,
}

impl CommandSet {
    pub async fn init() -> Result<Self, anyhow::Error> {
        let mut empty = Self {
            commands: Vec::new(),
        };
        // TODO: load from a Path filename (supplied in the .env?)
        match Path::new("commands.yml").exists() {
            true => {
                let file = fs::read_to_string("commands.yml")?.to_string();
                let docs = YamlLoader::load_from_str(&file);
                match docs {
                    Ok(yamls) => {
                        for doc in yamls {
                            println!("doc {:#?}", doc)
                        }
                        Ok(empty)
                    },
                    // TODO: consider returning empty?
                    Err(error) => Err(anyhow::Error::new(error))
                }
            },
            false => Ok(empty)
        }
    }
}