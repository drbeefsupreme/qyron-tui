use py_rpc;
//use notcurses::*;
use notcurses::sys::{widgets::*, *};
use libnotcurses_sys::c_api::{ncreader, ncreader_write_egc, ncreader_contents, ncreader_destroy,
    notcurses_drop_planes, ncreader_clear};
use std::ffi::{CStr, CString};

use crownet::Command;

static mut bweh: bool = true;

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

//TODO make a selector for which layer to write on
fn text_box(nc: &mut Nc, rpc_config: &py_rpc::Config, stdplane: &mut NcPlane, reader: &mut NcReader,
    current_plane: &mut CurrentPlane) {
    let mut ni: NcInput = NcInput::new_empty();
    unsafe { ncreader_clear(reader) };

    loop {
        match current_plane {
            CurrentPlane::TextBox => {
                let keypress: NcReceived = nc.get_blocking(Some(&mut ni)).expect("keypress");
                match keypress {
                   NcReceived::Char(ch) => {
                       match ch {
                           a => {
                               let mut vec = Vec::<u8>::new();
                               vec.push(a as u8);
                               let print_this = unsafe { CString::from_vec_unchecked(vec) };
                               unsafe {
                                   ncreader_write_egc(reader as *mut ncreader, print_this.as_ptr() as *const u8);
                               }
                           }
                       }
                   },
                   NcReceived::Key(ev) => match ev {
                       NcKey::Enter => {
                           let contents = unsafe { ncreader_contents(reader as *mut ncreader) };
                           let cstr_contents = unsafe { CStr::from_ptr(contents) };
                           let contents_string = cstr_contents.to_str().unwrap().to_owned();
                           send_text(&rpc_config, contents_string);
                       },
                       NcKey::Home => {
                           *current_plane = CurrentPlane::Selector;
                           break;
                       },
                       _ => break,
                   },
                _ => (),
               }
            },
            _ => break,
        }
        nc.render();
    }
}

fn send_text(rpc_config: &py_rpc::Config, text: String) {
    py_rpc::text(&rpc_config, text);
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

enum CurrentPlane {
    Selector,
    TextBox,
}

struct Planes<'a> {
    reader: &'a mut NcReader,
    selector: &'a mut NcSelector,
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
