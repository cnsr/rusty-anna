extern crate yaml_rust;
extern crate anyhow;
extern crate linked_hash_map;
extern crate regex;
extern crate rand;

use yaml_rust::{YamlLoader, Yaml};
use linked_hash_map::LinkedHashMap;
use std::fs;
use std::path::Path;
use regex::Regex;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: Option<String>,
    pub description: Option<String>,
    pub regex: Regex,
    pub replies: Option<Vec<String>>,
    pub execute: Option<String>,
}

impl Command {
    pub fn init(
        name: Option<String>,
        regex: String,
        description: Option<String>,
        replies: Option<Vec<String>>,
        execute: Option<String>,
    ) -> Result<Self, anyhow::Error> {
        if replies == None && execute == None {
            let error_text = format!("Command {} does not provide either of replies or executor", regex);
            return Err(anyhow::Error::msg(error_text));
        }
        let compiled_regex = Regex::new(&regex).unwrap();
        // TODO: check for duplicates by regex
        Ok(Self {
            name: name,
            description: description,
            regex: compiled_regex,
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
        // this looks like shit ngl
        return self.replies.clone().unwrap().choose(&mut rand::thread_rng()).unwrap().to_string();
    }

    // TODO: implement checker for matches
    // if a string is returned, reply has to be issued
    pub fn check_against(&self, text: String) -> Option<String> {
        if self.regex.is_match(&text) {
            if self.replies != None {
                return Some(self.get_reply());
            } else {
                // execute script
                return Some(String::from("Requested command execution"));
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct CommandSet {
    pub commands: Vec<Command>,
}

impl CommandSet {
    pub async fn init() -> Result<Self, anyhow::Error> {
        let mut initial_commands = Vec::new();
        let initial = Self {
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
                            let commands = doc.into_hash().unwrap().get(&Yaml::from_str("commands")).unwrap().to_owned();
                            for entry in commands.into_hash().unwrap() {
                                let mut raw_command = entry.1.into_hash().unwrap();
                                let parsed_command = Command::init(
                                    entry.0.into_string(),
                                    raw_command.extract_string("regex").unwrap(),
                                    raw_command.extract_string("description"),
                                    raw_command.extract_vec("replies"),
                                    raw_command.extract_string("execute"),
                                );
                                println!("entry: {:?}", parsed_command);
                                match parsed_command {
                                    Ok(command) => {
                                        initial_commands.push(command);
                                    }, _ => ()
                                }
                            }
                        }
                        Ok(Self {
                            commands: initial_commands,
                        })
                    },
                    Err(error) => Err(anyhow::Error::new(error))
                }
            },
            false => Ok(initial)
        }
    }
    pub fn check_against_commands(&self, text: String) -> Option<String> {
        println!("checking {:#?}", text);
        for command in self.commands.clone().into_iter() {
            match command.check_against(text.clone()) {
                Some(result) => {
                    println!("COMMAND MATCH ON TEXT: {:#?} FOR COMMAND: {:#?}", text, command);
                    return Some(result);
                },
                _ => {}
            }
        }
        None
    }
}

trait ExtractString {
    fn extract_string(&mut self, value: &str) -> Option<String>;
    fn extract_vec(&mut self, value: &str) -> Option<Vec<String>>;
}

impl ExtractString for LinkedHashMap<Yaml, Yaml> {
    fn extract_string(&mut self, value: &str) -> Option<String> {
        // println!("Extracting {:?} from {:?}", value, self);
        match self.get(&Yaml::from_str(value)) {
            Some (value) => {
                value.to_owned().into_string()
            } _ => None
        }
    }
    fn extract_vec(&mut self, value: &str) -> Option<Vec<String>> {
        // println!("Extracting {:?} from {:?}", value, self);
        match self.get(&Yaml::from_str(value)) {
            Some (value) => {
                let mut result = Vec::new();
                for hidden_string in value.to_owned().into_vec().unwrap() {
                    result.push(hidden_string.into_string().unwrap());
                };
                return Some(result);
            } _ => None
        }
    }
}