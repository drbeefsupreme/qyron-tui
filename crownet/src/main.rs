use py_rpc;
use notcurses::sys::{widgets::*, *};
use libnotcurses_sys::c_api::{ncreader, ncreader_write_egc, ncreader_contents, ncreader_destroy,
    notcurses_drop_planes, ncreader_clear};
use std::ffi::{CStr, CString};

use crownet::{Command, Planes, CurrentPlane};

use crownet::text_box::*;
use crownet::selector::*;

fn main() -> NcResult<()> {
    let rpc_config = py_rpc::Config::new();
    let a = py_rpc::connect(&rpc_config);

    println!("connect: {:?}", a);

    let nc: &mut Nc = unsafe { Nc::new()? };
    let stdplane: &mut NcPlane = unsafe { nc.stdplane() };
    stdplane.set_fg_rgb(0x40f040);

    let planes = Planes::init(stdplane);

    let mut current_plane = CurrentPlane::TextBox;

    loop {
        match current_plane {
            CurrentPlane::TextBox => {
                text_box(nc, &rpc_config, stdplane, planes.reader, &mut current_plane);
            },
            CurrentPlane::Selector => {
                run_selector(nc, &rpc_config, stdplane, planes.selector, &mut current_plane);
            }
        }
    }

    unsafe { nc.stop()? };

    Ok(())
}

fn send_choice(choice: Command, rpc_config: &py_rpc::Config, nc: &mut Nc) {
    match choice {
        Command::Clear => py_rpc::clear(&rpc_config),
        Command::Caw => py_rpc::caw(&rpc_config),
        Command::Dopamine => py_rpc::dopamine(&rpc_config),
        Command::RandomPixels => py_rpc::pixels(&rpc_config),
        Command::RandomShapes => py_rpc::shapes(&rpc_config),
        Command::Temperature => py_rpc::temp(&rpc_config),
        Command::NextGif     => py_rpc::nextGif(&rpc_config),
        Command::NoGif       => py_rpc::noGif(&rpc_config),
        Command::RandomShapesBG => py_rpc::shapesBg(&rpc_config),
        Command::RandomPixelsBG => py_rpc::pixelsBg(&rpc_config),
        Command::Text => Ok(()),
        _ => Ok(()),
    };
}
