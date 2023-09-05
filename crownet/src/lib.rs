//  commands to send to LED matrix
pub enum Command {
    Clear,
    Caw,
    Dopamine,
    RandomPixels,
    RandomShapes,
    Temperature,
    Invalid,
    Quit,
    NextGif,
}

impl Command {
    pub fn from_str(nom: &str) -> Command {
        match nom {
            "Clear"         => Command::Clear,
            "CAW"           => Command::Caw,
            "DOPAMINE"      => Command::Dopamine,
            "Random pixels" => Command::RandomPixels,
            "Random shapes" => Command::RandomShapes,
            "Temperature"   => Command::Temperature,
            "Next gif"      => Command::NextGif,
            _               => Command::Invalid,
        }
    }
}
