//  commands to send to LED matrix
pub enum Command {
    Clear,
    Caw,
    Dopamine,
    RandomPixels,
    RandomShapes,
    RandomShapesBG,
    Temperature,
    Invalid,
    Quit,
    NextGif,
    NoGif,
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
            "No gif"        => Command::NoGif,
            "Random shapes BG" => Command::RandomShapesBG,
            _               => Command::Invalid,
        }
    }
}
