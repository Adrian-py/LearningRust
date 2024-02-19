pub enum Command {
    View,
    Add,
    Remove,
    Exit,
    Unknown,
}

impl Command {
    // To add more commands, add another case here
    pub fn from(input: u8) -> Command {
        match input {
            1 => Command::View,
            2 => Command::Add,
            3 => Command::Remove,
            4 => Command::Exit,
            _ => Command::Unknown,
        }
    }
}
