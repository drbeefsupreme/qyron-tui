use py_rpc;
use notcurses::*;
use notcurses::sys::{widgets::*, *};

fn main() -> NcResult<()> {
    let nc: &mut Nc = unsafe { Nc::new()? };
    //TODO why do I have to use NcPlane instead of Plane?
    let stdplane: &mut NcPlane = unsafe { nc.stdplane() };
    stdplane.set_fg_rgb(0x40f040);

    let planeopts: NcPlaneOptions = NcPlaneOptions::new_aligned(1, NcAlign::Left, 15, 80);
    let selplane: &mut NcPlane = NcPlane::new_child(stdplane, &planeopts)?;

    let selector = NcSelector::builder()
        .item("PARODY", "IN MINECRAFT, IN MINECRAFT, NOT LEGALLY ACTIONABLE, TOTALLY MAKE BELIEVE")
        .item("DOPAMINE", "DOPAMINE DOPAMINE DOPAMINE DOPAMINE DOPAMINE")
        .title("CrowNet")
        .secondary("Institute for Advanced Levels")
        .footer("CAW CAW CAW")
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

    let selected: String = run_selector(nc, selector)?;

    selector.destroy()?;

    unsafe { nc.stop()? };

    println!("{}", selected);

    Ok(())
}

fn run_selector(nc: &mut Nc, selector: &mut NcSelector) -> NcResult<String> {
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
                            return selector.selected().ok_or_else(|| NcError::new());
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
                        return selector.selected().ok_or_else(|| NcError::new());
                    },
                    _ => (),
                },
                _ => (),
            }
        }

        nc.render()?;
    }
}
