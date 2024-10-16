use notcurses::sys::{widgets::*, *};
use py_rpc;

use crate::{Command, CurrentPlane};
use std::str::FromStr;

pub fn run_selector(
    nc: &mut Nc,
    rpc_config: &py_rpc::Config,
    selector: &mut NcSelector,
    current_plane: &mut CurrentPlane,
) {
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
                                    send_choice(Command::Quit, &rpc_config);
                                }
                                // S => down
                                's' | 'S' => {
                                    selector.nextitem().unwrap();
                                }
                                // W => up
                                'w' | 'W' => {
                                    selector.previtem().unwrap();
                                }
                                _ => (),
                            }
                        }
                        NcReceived::Key(ev) => match ev {
                            NcKey::Enter => {
                                let choice =
                                    selector.selected().ok_or_else(|| NcError::new()).unwrap();
                                send_choice(Command::from_str(&choice).unwrap(), &rpc_config);
                            }
                            NcKey::Home => {
                                *current_plane = CurrentPlane::TextBox;
                            }
                            _ => break,
                        },
                        _ => break,
                    }
                }
            }
            _ => break,
        }

        nc.render();
    }
}

fn send_choice(choice: Command, rpc_config: &py_rpc::Config) {
    match choice {
        Command::Clear => py_rpc::clear(&rpc_config),
        Command::ClearText => py_rpc::clear_text(&rpc_config),
        Command::CAW => py_rpc::caw(&rpc_config),
        Command::DOPAMINE => py_rpc::dopamine(&rpc_config),
        Command::RandomPixels => py_rpc::pixels(&rpc_config),
        Command::RandomShapes => py_rpc::shapes(&rpc_config),
        Command::Temperature => py_rpc::temp(&rpc_config),
        Command::NextGif => py_rpc::nextGif(&rpc_config),
        Command::NoGif => py_rpc::noGif(&rpc_config),
        Command::EnableGifsLoop => py_rpc::enableGifsLoop(&rpc_config),
        Command::DisableGifsLoop => py_rpc::disableGifsLoop(&rpc_config),
        Command::GifK => py_rpc::gifK(&rpc_config),
        Command::GifO => py_rpc::gifO(&rpc_config),
        Command::GifF => py_rpc::gifF(&rpc_config),
        Command::GifD => py_rpc::gifD(&rpc_config),
        Command::GifJ => py_rpc::gifJ(&rpc_config),
        Command::GifS => py_rpc::gifS(&rpc_config),
        Command::GifB => py_rpc::gifB(&rpc_config),
        Command::GifT => py_rpc::gifT(&rpc_config),
        Command::GifA => py_rpc::gifA(&rpc_config),
        Command::GifH => py_rpc::gifH(&rpc_config),
        Command::GifP => py_rpc::gifP(&rpc_config),
        Command::GifC => py_rpc::gifC(&rpc_config),
        Command::GifM => py_rpc::gifM(&rpc_config),
        Command::ToggleTextMode => py_rpc::toggle_text_mode(&rpc_config),

        Command::RandomShapesBG => py_rpc::shapesBg(&rpc_config),
        Command::RandomPixelsBG => py_rpc::pixelsBg(&rpc_config),
        Command::Text => Ok(()),
        _ => Ok(()),
    };
}
