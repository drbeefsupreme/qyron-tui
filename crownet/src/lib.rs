pub mod text_box;
pub mod selector;

use py_rpc;
//use notcurses::*;
use notcurses::sys::{widgets::*, *};
use libnotcurses_sys::c_api::{ncreader, ncreader_write_egc, ncreader_contents, ncreader_destroy,
    notcurses_drop_planes, ncreader_clear};
//use std::ffi::{cstr, cstring};

//  commands to send to LED matrix
pub enum Command {
    Clear,
    Caw,
    Dopamine,
    RandomPixels,
    RandomPixelsBG,
    RandomShapes,
    RandomShapesBG,
    Temperature,
    Invalid,
    Quit,
    NextGif,
    NoGif,
    Text,
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
            "Random pixels BG" => Command::RandomShapesBG,
            _               => Command::Invalid,
        }
    }
}

pub enum CurrentPlane {
    Selector,
    TextBox,
}

pub struct Planes<'a> {
    pub reader: &'a mut NcReader,
    pub selector: &'a mut NcSelector,
}

impl Planes<'_> {
    pub fn init(stdplane: &mut NcPlane) -> Self {
        let reader_opts: NcPlaneOptions = NcPlaneOptions::new_aligned(10, NcAlign::Center, 150, 80);
        let reader_plane: &mut NcPlane = NcPlane::new_child(stdplane, &reader_opts).unwrap();
        reader_plane.set_bg_rgb(0x40f040); //TODO is this doing anything?
        let mut reader = NcReader::new(reader_plane).unwrap();

        let plane_opts: NcPlaneOptions = NcPlaneOptions::new_aligned(15, NcAlign::Left, 15, 80);
        let sel_plane: &mut NcPlane = NcPlane::new_child(stdplane, &plane_opts).unwrap();
        let mut selector = NcSelector::builder()
            .item("Clear", "clears the chyron")
            .item("CAW", "CAW CAW CAW CAW CAW")
            .item("DOPAMINE", "DOPAMINE DOPAMINE DOPAMINE DOPAMINE DOPAMINE")
            .item("Random pixels", "asdfasdhfkalshdg")
            .item("Random pixels BG", "skrrt")
            .item("Random shapes", "yes")
            .item("Random shapes BG", "YES")
            .item("Next gif", "bweh")
            .item("No gif", "bye")
    //        .item("Temperature", "DEBUG: CPU temp")
            .title("CrowNet")
            .secondary("Institute for Advanced Levels")
            .footer("The Too Late Show with Dr. Beelzebub Crow")
            .max_display(4)
            .default_item(1)
            .box_channels(NcChannels::from_rgb(0x20e040, 0x202020))
            .item_channels(
                NcChannels::from_rgb(0xe08040, 0),
                NcChannels::from_rgb(0x80e040, 0),
            )
            .secondary_channels(NcChannels::from_rgb(0xe00040, 0x200000))
            .title_channels(NcChannels::from_rgb(0xffff80, 0x000020))
            .finish(sel_plane).unwrap();

        Self {
            reader,
            selector,
        }
    }
}
