extern crate yaml_rust;
extern crate anyhow;
extern crate linked_hash_map;
extern crate regex;
extern crate rand;

use lazy_static::__Deref;
use yaml_rust::{YamlLoader, Yaml};
use linked_hash_map::LinkedHashMap;
use std::fs;
use std::path::Path;
use regex::Regex;
use rand::seq::SliceRandom;
use log::{info, warn, error, debug};
use lazy_static::lazy_static;

use crate::commands::hangman::HangmanGame;

lazy_static! {
    pub static ref HANGMAN: HangmanGame = HangmanGame::init().unwrap();
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: Option<String>,
    pub description: Option<String>,
    pub regex: Regex,
    pub replies: Option<Vec<String>>,
    pub execute: Option<String>,
    pub hangman: Option<&'static HangmanGame>,
}

impl Command {
    pub fn init(
        name: Option<String>,
        regex: String,
        description: Option<String>,
        replies: Option<Vec<String>>,
        execute: Option<String>,
        hangman: Option<&'static HangmanGame>
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
            hangman: hangman,
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

    // TODO: implement matcher for executors
    // if a string is returned, reply has to be issued
    pub fn check_against(&self, text: String) -> Option<String> {
        let not_implemented = Some("not implemented".to_string());
        if self.regex.is_match(&text) {
            if self.replies != None {
                return Some(self.get_reply());
            } else {
                // execute script
                match self.execute.clone().unwrap_or_default().as_str() {
                    "hangman.play" => {
                        return not_implemented;
                    },
                    "hangman.score" => {
                        return not_implemented;
                    },
                    _ => {
                        return not_implemented;
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct CommandSet {
    pub commands: Vec<Command>,
    pub help: Regex,
}

impl CommandSet {
    pub async fn init() -> Result<Self, anyhow::Error> {
        let mut initial_commands = Vec::new();
        let help = Regex::new(r"^\.help$").unwrap();
        let initial = Self {
            commands: Vec::new(),
            help: help.clone(),
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
                            // let hangman_instance = HangmanGame::init().await?;
                            for entry in commands.into_hash().unwrap() {
                                let mut raw_command = entry.1.into_hash().unwrap();
                                let mut use_hangman = false;
                                match raw_command.extract_string("execute") {
                                    Some(executor) => {
                                        match &executor[..] {
                                            "hangman.play" | "hangman.score" => {
                                                use_hangman = true;
                                            },
                                            _ => ()
                                        }
                                    }, _ => {}
                                }
                                let parsed_command = Command::init(
                                    entry.0.into_string(),
                                    raw_command.extract_string("regex").unwrap(),
                                    raw_command.extract_string("description"),
                                    raw_command.extract_vec("replies"),
                                    raw_command.extract_string("execute"),
                                    if use_hangman {Some(HANGMAN.deref())} else {None},
                                );
                                info!("Command entry: {:?}", parsed_command);
                                match parsed_command {
                                    Ok(command) => {
                                        initial_commands.push(command);
                                    }, _ => ()
                                }
                            }
                        }
                        Ok(Self {
                            commands: initial_commands,
                            help: help,
                        })
                    },
                    Err(error) => Err(anyhow::Error::new(error))
                }
            },
            false => Ok(initial)
        }
    }
    pub fn check_against_commands(&self, text: String) -> Option<String> {
        if self.help.is_match(&text.clone()) {
            let mut result = String::from("");
            for command in self.commands.clone().into_iter() {
                result += &format!(
                    "[b]{:?}[/b]  [code]{:?}[/code]\n{:?}\n",
                    command.clone().name.unwrap(),
                    command.clone().regex,
                    command.get_description()
                );
            }
            return Some(result);
        }
            
        for command in self.commands.clone().into_iter() {
            info!("checking regex {:?} against '{:#?}'", command.regex, text);
            match command.check_against(text.clone()) {
                Some(result) => {
                    debug!("COMMAND MATCH ON TEXT: {:#?} FOR COMMAND: {:#?}", text, command);
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
        match self.get(&Yaml::from_str(value)) {
            Some (value) => {
                value.to_owned().into_string()
            } _ => None
        }
    }
    fn extract_vec(&mut self, value: &str) -> Option<Vec<String>> {
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