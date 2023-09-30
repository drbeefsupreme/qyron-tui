use py_rpc;
//use notcurses::*;
use notcurses::sys::{widgets::*, *};
use libnotcurses_sys::c_api::{ncreader, ncreader_write_egc, ncreader_contents, ncreader_destroy,
    notcurses_drop_planes, ncreader_clear};
use std::ffi::{CStr, CString};

use crownet::{Command, Planes, CurrentPlane};

use crownet::text_box::*;

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

fn run_selector(nc: &mut Nc, rpc_config: &py_rpc::Config, stdplane: &mut NcPlane, selector: &mut NcSelector,
    current_plane: &mut CurrentPlane) {
    let mut ni: NcInput = NcInput::new_empty();

    // do not wait for input before first rendering
    nc.render();

    loop {
        match current_plane {
            CurrentPlane::Selector => {
                let keypress: NcReceived = nc.get_blocking(Some(&mut ni)).unwrap();

                if !selector.offer_input(ni) {
                    // do not consider release key: only press
                    if ni.evtype == NcInputType::Release as u32 {
                        continue;
                    }

                    match keypress {
                        NcReceived::Char(ch) => {
                            match ch {
                                // Q => quit
                                'q' | 'Q' => {
                                    send_choice(Command::Quit, &rpc_config, nc);
                                },
                                // S => down
                                's' | 'S' => {
                                    selector.nextitem().unwrap();
                                },
                                // W => up
                                'w' | 'W' => {
                                    selector.previtem().unwrap();
                                },
                                _ => (),
                            }
                        },
                        NcReceived::Key(ev) => match ev {
                            NcKey::Enter => {
                                let choice = selector.selected().ok_or_else(|| NcError::new()).unwrap();
                                send_choice(Command::from_str(&choice), &rpc_config, nc);
                            },
                            NcKey::Home => {
                                *current_plane = CurrentPlane::TextBox;
                            },
                            _ => break,
                        },
                        _ => break,
                    }
                 }
            },
            _ => break,
        }

        nc.render();
    }
}

