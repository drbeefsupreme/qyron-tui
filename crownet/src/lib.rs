pub mod selector;
pub mod text_box;

use notcurses::sys::{widgets::*, *};
use std::str::FromStr;
use strum_macros::EnumString;

//  commands to send to LED matrix
#[derive(EnumString)]
pub enum Command {
    Clear,
    ClearText,
    CAW,
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
    GifK,
    GifO,
    GifF,
    GifD,
    GifJ,
    GifS,
    GifB,
    GifT,
    GifA,
    GifH,
    GifP,
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
    pub reader_indicator: &'a mut NcPlane,
    pub selector_indicator: &'a mut NcPlane,
    pub text_layer_selector_indicator: &'a mut NcPlane,
}

impl Planes<'_> {
    pub fn init(stdplane: &mut NcPlane) -> Self {
        let reader_opts = NcPlaneOptions::new_aligned(1, NcAlign::Center, 150, 80);
        let reader_plane: &mut NcPlane = NcPlane::new_child(stdplane, &reader_opts).unwrap();
        reader_plane.set_bg_rgb(0x40f040); //TODO is this doing anything?
        let reader = NcReader::new(reader_plane).unwrap();

        let plane_opts = NcPlaneOptions::new_aligned(15, NcAlign::Left, 15, 80);
        let sel_plane: &mut NcPlane = NcPlane::new_child(stdplane, &plane_opts).unwrap();
        let selector = NcSelector::builder()
            .item("Clear", "clears the chyron")
            .item("ClearText", "clears text")
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
            .item("GifK", "koyaanisqatsi")
            .item("GifO", "obama")
            .item("GifF", "flags")
            .item("GifD", "drugs")
            .item("GifJ", "joker")
            .item("GifS", "spooky")
            .item("GifB", "subgenius")
            .item("GifH", "hank")
            .item("GifP", "bluechew")
            //            .item("GifT", "transparent")
            .item("GifA", "alex jones")
            //        .item("Temperature", "DEBUG: CPU temp")
            .title("CrowNet")
            .secondary("Institute for Advanced Levels")
            .footer("The Too Late Show with Dr. Beelzebub Crow")
            .max_display(10)
            .default_item(1)
            .box_channels(NcChannels::from_rgb(0x20e040, 0x202020))
            .item_channels(
                NcChannels::from_rgb(0xe08040, 0),
                NcChannels::from_rgb(0x80e040, 0),
            )
            .secondary_channels(NcChannels::from_rgb(0xe00040, 0x200000))
            .title_channels(NcChannels::from_rgb(0xffff80, 0x000020))
            .finish(sel_plane)
            .unwrap();

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
            .finish(text_sel_plane)
            .unwrap();

        // Create indicator planes
        let reader_indicator = NcPlane::new_child(
            stdplane,
            &NcPlaneOptions::new_aligned(0, NcAlign::Left, 1, 3),
        )
        .unwrap();
        let selector_indicator = NcPlane::new_child(
            stdplane,
            &NcPlaneOptions::new_aligned(14, NcAlign::Left, 1, 3),
        )
        .unwrap();
        let text_layer_selector_indicator = NcPlane::new_child(
            stdplane,
            &NcPlaneOptions::new_aligned(4, NcAlign::Right, 1, 3),
        )
        .unwrap();

        Self {
            reader,
            selector,
            text_layer_selector: text_sel_selector,
            reader_indicator,
            selector_indicator,
            text_layer_selector_indicator,
        }
    }

    pub fn update_indicator(&mut self, current_plane: &CurrentPlane) {
        // Clear all indicators
        self.reader_indicator.erase();
        self.selector_indicator.erase();
        self.text_layer_selector_indicator.erase();

        // Set the active indicator
        let active_indicator = match current_plane {
            CurrentPlane::TextBox => &mut self.reader_indicator,
            CurrentPlane::Selector => &mut self.selector_indicator,
            CurrentPlane::TextLayerSelector => &mut self.text_layer_selector_indicator,
        };

        active_indicator.set_bg_rgb(0xffff00); // Yellow background
        active_indicator.putstr(">>>"); // Arrow indicator
        active_indicator.render(); // Use render() instead of refresh()
    }
}
