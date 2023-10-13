use py_rpc;
use notcurses::sys::{widgets::*, *};
use libnotcurses_sys::c_api::{ncreader, ncreader_write_egc, ncreader_contents, ncreader_clear};
use std::ffi::{CStr, CString};
use std::str::FromStr;
use crate::CurrentPlane;
use crate::LayerCommand;

//TODO make a selector for which layer to write on
pub fn text_box(nc: &mut Nc, rpc_config: &py_rpc::Config, reader: &mut NcReader,
                selector: &mut NcSelector,
                current_plane: &mut CurrentPlane)
{
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
                           let layer = run_layer_selector(nc, rpc_config, selector);
                           send_text(layer, contents_string, &rpc_config);
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
                          -> LayerCommand
{
    let mut ni: NcInput = NcInput::new_empty();

    // do not wait for input before first rendering
    nc.render();

    loop {
//        match current_plane {
//            CurrentPlane::Selector => {

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
                                    return LayerCommand::Cancel;
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
                                return LayerCommand::from_str(&choice).unwrap();
                            },
                            NcKey::Home => {
                                return LayerCommand::Cancel;
                            },
                            _ => break,
                        },
                        _ => break,
                    }
                 }
//            },
//            _ => break,
//        }

        nc.render();
    }
    LayerCommand::Cancel
}

fn send_text(layer: LayerCommand, text: String, rpc_config: &py_rpc::Config) {
    match layer {
        LayerCommand::Layer1 => py_rpc::text1(&rpc_config, text),
        LayerCommand::Layer2 => py_rpc::text2(&rpc_config, text),
        LayerCommand::Layer3 => py_rpc::text3(&rpc_config, text),
        LayerCommand::Layer4 => py_rpc::text4(&rpc_config, text),
        LayerCommand::Layer5 => py_rpc::text5(&rpc_config, text),
        _ => Ok(()),
    };
}
