use std::iter::FromIterator;

use anyhow::Result;
use rand::seq::SliceRandom;

use log::{info, warn, error, debug};

struct Score {
    name: String,
    id: String,
    score: u16,

}

trait OperateScores {
    fn list_hiscores(&mut self) -> String;
    fn sort_hiscores(&mut self);
    fn update_hiscore(&mut self, id: String);
    fn write_to_file(&mut self);
    fn read_from_file(&mut self);
}

impl OperateScores for Vec<Score> {
    fn list_hiscores(&mut self) -> String {
        if self.len() < 1 {
            return String::from("No hiscores to show.");
        }
        String::from("Not yet implemented.")
    }
    fn sort_hiscores(&mut self) {
        self.sort_by_key(|score| score.score);
    }

    fn update_hiscore(&mut self, id: String) {
        for score in self {
            if score.id == id && score.score != u16::MAX {
                score.score += 1;
            }
        }
    }

    fn write_to_file(&mut self) {
        ()
    }

    fn read_from_file(&mut self) {
        ()
    }
}

struct HangmanGame {
    word: String,
    guessed: Vec<char>,
    words: Vec<String>,
    hiscores: Vec<Score>,
}

impl HangmanGame {
    async fn init() -> Result<Self, anyhow::Error> {
        let mut game = Self {
            word: String::from("initial"),
            guessed: vec!(),
            words: vec!(),
            hiscores: vec!(),
        };
        game.load_words().await?;
        game.assign_new_word();
        game.load_hiscores().await?;

        Ok(game)
    }

    async fn load_hiscores(&mut self) -> Result<Vec<Score>, anyhow::Error> {
        Ok(vec!())
    }

    async fn load_words(&mut self) -> Result<Vec<Score>, anyhow::Error> {
        Ok(vec!())
    }

    // will only assign a new word if there are more than one word
    fn assign_new_word(&mut self) {
        let new_word = self.words.choose(&mut rand::thread_rng()).unwrap().to_string();
        if new_word != self.word {
            self.word = new_word;
        } else {
            if self.words.len() > 1 {
                self.assign_new_word();
            }
        }
        self.guessed.clear();
    }

    fn display_word(&self) -> String {
        let mut result: Vec<char> = vec!();
        for word_char in self.word.chars() {
            if !self.guessed.contains(&word_char) {
                result.push('_');
            } else {
                result.push(word_char);
            }
        }
        result.into_iter().collect()
    }

    fn check_win(&mut self) -> bool {
        // TODO: add implementation
        false
    }

    pub fn make_a_guess(&mut self, letter: char, user: String) -> String {
        if self.guessed.contains(&letter) {
            return String::from("Letter has already been guessed.");
        }
        let word_as_chars = Vec::from_iter(self.word.chars());
        let mut result: String;
        if word_as_chars.contains(&letter) {
            if self.check_win() {
                result = String::from("You won! Full word:\n");
                result += &self.display_word();


                self.hiscores.update_hiscore(user);
                self.assign_new_word();

                return result;
            } else {
                result = String::from("Yep.\n");
            }
        } else {
            result = String::from("Nop.\n");
        }
        self.guessed.push(letter);
        result += &self.display_word();
        return result;
    }
}