use py_rpc;
use notcurses::sys::{widgets::*, *};
use libnotcurses_sys::c_api::{ncreader, ncreader_write_egc, ncreader_contents, ncreader_clear};
use std::ffi::{CStr, CString};
use std::str::FromStr;
use std::borrow::BorrowMut;
use crate::CurrentPlane;
use crate::LayerCommand;

pub fn text_box(nc: &mut Nc, rpc_config: &py_rpc::Config, reader: &mut NcReader,
                selector: &mut NcSelector,
                current_plane: &mut CurrentPlane)
{
    let mut ni: NcInput = NcInput::new_empty();
    unsafe { ncreader_clear(reader) };
    let mut text_vec = Vec::<u8>::new();

    loop {
        match current_plane {
            CurrentPlane::TextBox => {
                let keypress: NcReceived = nc.get_blocking(Some(&mut ni)).expect("keypress");
                match keypress {
                   NcReceived::Char(ch) => {
                       match ch {
                           a => {
                               text_vec.push(a as u8);
                               let print_this = unsafe { CString::new(&[a as u8]).unwrap_unchecked() };
                               unsafe {
                                   ncreader_write_egc(reader as *mut ncreader, print_this.as_ptr());
                               }
                           }
                       }
                   },
                   NcReceived::Key(ev) => match ev {
                       NcKey::Enter => {
                           // let contents = unsafe { ncreader_contents(reader as *mut ncreader) };
                           // let cstr_contents = unsafe { CStr::from_ptr(contents) };
                           // let contents_string = cstr_contents.to_str().unwrap().to_owned();
                           let contents_string = String::from_utf8(text_vec).unwrap();
                           let (layer, speed) = run_layer_selector(nc, rpc_config, selector);
                           send_text(layer, speed, contents_string, &rpc_config);
                           text_vec = Vec::<u8>::new();
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

pub fn run_layer_selector(nc: &mut Nc, rpc_config: &py_rpc::Config, selector: &mut NcSelector)
                          -> (LayerCommand, u32)
{
    let mut ni: NcInput = NcInput::new_empty();
    // default speed is 4
    let mut speed: u32 = 4;

    // do not wait for input before first rendering
    nc.render();

    loop {
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
                            return (LayerCommand::Cancel, 5);
                        },
                        // S => down
                        's' | 'S' => {
                            selector.nextitem().unwrap();
                        },
                        // W => up
                        'w' | 'W' => {
                            selector.previtem().unwrap();
                        },
                        '1' => {
                            speed = 1;
                        },
                        '2' => {
                            speed = 2;
                        },
                        '3' => {
                            speed = 3;
                        },
                        '4' => {
                            speed = 4;
                        },
                        '5' => {
                            speed = 5;
                        },
                        '6' => {
                            speed = 6;
                        },
                        '7' => {
                            speed = 7;
                        },
                        '8' => {
                            speed = 8;
                        },
                        '9' => {
                            speed = 9;
                        },
                        '0' => {
                            speed = 10;
                        },
                        _ => (),
                    }
                },
                NcReceived::Key(ev) => match ev {
                    NcKey::Enter => {
                        let choice = selector.selected().ok_or_else(|| NcError::new()).unwrap();
                        return (LayerCommand::from_str(&choice).unwrap(), speed);
                    },
                    NcKey::Home => {
                        return (LayerCommand::Cancel, 5);
                    },
                    _ => break,
                },
                _ => break,
            }
         }

        nc.render();
    }
    (LayerCommand::Cancel, 5)
}

fn send_text(layer: LayerCommand, speed: u32, text: String, rpc_config: &py_rpc::Config) {
    let mut speed = speed * 10;
    if speed == 10 {
        speed = 2;
    };
    match layer {
        LayerCommand::Layer1 => {
            py_rpc::speed1(&rpc_config, speed);
            py_rpc::text1(&rpc_config, text)
        },
        LayerCommand::Layer2 => {
            py_rpc::speed2(&rpc_config, speed);
            py_rpc::text2(&rpc_config, text)
        },
         LayerCommand::Layer3 => {
            py_rpc::speed3(&rpc_config, speed);
            py_rpc::text3(&rpc_config, text)
        },
        LayerCommand::Layer4 => {
            py_rpc::speed4(&rpc_config, speed);
            py_rpc::text4(&rpc_config, text)
        },
        LayerCommand::Layer5 => {
            py_rpc::speed5(&rpc_config, speed);
            py_rpc::text5(&rpc_config, text)
        },
        _ => Ok(()),
    };
}
