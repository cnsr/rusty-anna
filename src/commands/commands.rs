struct Command {
    pub regex: String,
    pub replies: Vec<String>,
}

impl Command {
    fn init(regex: String, replies: Vec<String>) -> Self {
        Self {
            regex: regex, replies: replies
        }
    }
}

struct CommandSet {
    pub commands: Vec<Command>,
}

impl CommandSet {
    fn init(filename: Path) -> Self {
        // TODO: load from a Path filename (supplied in the .env?)
        Self {
            commands: Vec::new(),
        }
    }
}