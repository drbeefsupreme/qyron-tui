use py_rpc;
use notcurses::*;
use notcurses::sys::{widgets::*, *};

use crownet::Command;

fn main() -> NcResult<()> {
    let rpc_config = py_rpc::Config::new();
    py_rpc::connect(&rpc_config);

    let nc: &mut Nc = unsafe { Nc::new()? };
    //TODO why do I have to use NcPlane instead of Plane?
    let stdplane: &mut NcPlane = unsafe { nc.stdplane() };
    stdplane.set_fg_rgb(0x40f040);

    let planeopts: NcPlaneOptions = NcPlaneOptions::new_aligned(1, NcAlign::Left, 15, 80);
    let selplane: &mut NcPlane = NcPlane::new_child(stdplane, &planeopts)?;

    let selector = NcSelector::builder()
        .item("Clear", "clears the chyron")
        .item("CAW", "CAW CAW CAW CAW CAW")
        .item("DOPAMINE", "DOPAMINE DOPAMINE DOPAMINE DOPAMINE DOPAMINE")
        .item("Random pixels", "asdfasdhfkalshdg")
        .item("Random shapes", "yes")
        .item("Temperature", "DEBUG: CPU temp")
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
        .finish(selplane)?;

    run_selector(nc, selector, &rpc_config)?;

    selector.destroy()?;

    unsafe { nc.stop()? };

    Ok(())
}

fn send_choice(choice: Command, rpc_config: &py_rpc::Config) {
    match choice {
        Command::Clear => py_rpc::clear(&rpc_config),
        Command::Caw => py_rpc::caw(&rpc_config),
        Command::Dopamine => py_rpc::dopamine(&rpc_config),
        Command::RandomPixels => py_rpc::pixels(&rpc_config),
        Command::RandomShapes => py_rpc::shapes(&rpc_config),
        Command::Temperature => py_rpc::temp(&rpc_config),
        _ => Ok(()),
    };
}

fn run_selector(nc: &mut Nc, selector: &mut NcSelector, rpc_config: &py_rpc::Config) -> NcResult<Command> {
    let mut ni: NcInput = NcInput::new_empty();

    // do not wait for input before first rendering
    nc.render()?;

    loop {
        let keypress: NcReceived = nc.get_blocking(Some(&mut ni))?;

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
                            return Ok(Command::Quit);
                        },
                        // S => down
                        's' | 'S' => {
                            selector.nextitem()?;
                        },
                        // W => up
                        'w' | 'W' => {
                            selector.previtem()?;
                        },
                        _ => (),
                    }
                },
                NcReceived::Key(ev) => match ev {
                    NcKey::Enter => {
                        let choice = selector.selected().ok_or_else(|| NcError::new())?;
                        send_choice(Command::from_str(&choice), &rpc_config);
                    },
                    _ => (),
                },
                _ => (),
            }
        }

        nc.render()?;
    }
}
