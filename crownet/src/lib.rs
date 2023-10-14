pub mod text_box;
pub mod selector;

use notcurses::sys::{widgets::*, *};
use std::str::FromStr;
use strum_macros::EnumString;

//  commands to send to LED matrix
#[derive(EnumString)]
pub enum Command {
    Clear,
    Caw,
    DOPAMINE,
    RandomPixels,
    RandomPixelsBG,
    RandomShapes,
    RandomShapesBG,
    Temperature,
    Invalid,
    Quit,
    NextGif,
    NoGif,
    EnableGifsLoop,
    DisableGifsLoop,
    Text,
}

#[derive(EnumString)]
pub enum LayerCommand {
    Layer1,
    Layer2,
    Layer3,
    Layer4,
    Layer5,
    Cancel,
}

// impl Command {
//     pub fn from_str(nom: &str) -> Command {
//         match nom {
//             "Clear"         => Command::Clear,
//             "CAW"           => Command::Caw,
//             "DOPAMINE"      => Command::Dopamine,
//             "Random pixels" => Command::RandomPixels,
//             "Random shapes" => Command::RandomShapes,
//             "Temperature"   => Command::Temperature,
//             "Next gif"      => Command::NextGif,
//             "No gif"        => Command::NoGif,
//             "Random shapes BG" => Command::RandomShapesBG,
//             "Random pixels BG" => Command::RandomShapesBG,
//             _               => Command::Invalid,
//         }
//     }
// }

pub enum CurrentPlane {
    Selector,
    TextLayerSelector,
    TextBox,
}

pub struct Planes<'a> {
    pub reader: &'a mut NcReader,
    pub selector: &'a mut NcSelector,
    pub text_layer_selector: &'a mut NcSelector,
}

impl Planes<'_> {
    pub fn init(stdplane: &mut NcPlane) -> Self {
        let reader_opts = NcPlaneOptions::new_aligned(10, NcAlign::Center, 150, 80);
        let reader_plane: &mut NcPlane = NcPlane::new_child(stdplane, &reader_opts).unwrap();
        reader_plane.set_bg_rgb(0x40f040); //TODO is this doing anything?
        let reader = NcReader::new(reader_plane).unwrap();

        let plane_opts = NcPlaneOptions::new_aligned(15, NcAlign::Left, 15, 80);
        let sel_plane: &mut NcPlane = NcPlane::new_child(stdplane, &plane_opts).unwrap();
        let selector = NcSelector::builder()
            .item("Clear", "clears the chyron")
            .item("CAW", "CAW CAW CAW CAW CAW")
            .item("DOPAMINE", "DOPAMINE DOPAMINE DOPAMINE DOPAMINE DOPAMINE")
            .item("RandomPixels", "asdfasdhfkalshdg")
            .item("RandomPixelsBG", "skrrt")
            .item("RandomShapes", "yes")
            .item("RandomShapesBG", "YES")
            .item("NextGif", "bweh")
            .item("NoGif", "bye")
            .item("EnableGifsLoop", "cycle")
            .item("DisableGifsLoop", "nocycle")
    //        .item("Temperature", "DEBUG: CPU temp")
            .title("CrowNet")
            .secondary("Institute for Advanced Levels")
            .footer("The Too Late Show with Dr. Beelzebub Crow")
            .max_display(7)
            .default_item(1)
            .box_channels(NcChannels::from_rgb(0x20e040, 0x202020))
            .item_channels(
                NcChannels::from_rgb(0xe08040, 0),
                NcChannels::from_rgb(0x80e040, 0),
            )
            .secondary_channels(NcChannels::from_rgb(0xe00040, 0x200000))
            .title_channels(NcChannels::from_rgb(0xffff80, 0x000020))
            .finish(sel_plane).unwrap();

        let text_sel_opts = NcPlaneOptions::new_aligned(5, NcAlign::Right, 150, 80);
        let text_sel_plane = NcPlane::new_child(stdplane, &text_sel_opts).unwrap();
        let text_sel_selector = NcSelector::builder()
            .item("Layer1", "layer 1")
            .item("Layer2", "layer 2")
            .item("Layer3", "layer 3")
            .item("Layer4", "layer 4")
            .item("Layer5", "layer 5")
            .title("Select a layer")
            .max_display(5)
            .default_item(1)
             .box_channels(NcChannels::from_rgb(0x20e040, 0x202020))
            .item_channels(
                NcChannels::from_rgb(0xe08040, 0),
                NcChannels::from_rgb(0x80e040, 0),
            )
            .secondary_channels(NcChannels::from_rgb(0xe00040, 0x200000))
            .title_channels(NcChannels::from_rgb(0xffff80, 0x000020))
            .finish(text_sel_plane).unwrap();

        Self {
            reader,
            selector,
            text_layer_selector: text_sel_selector,
        }
    }
}
