use py_rpc;
use notcurses::sys::*;

use crownet::{Planes, CurrentPlane};
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
                text_box(nc, &rpc_config, planes.reader, &mut current_plane);
            },
            CurrentPlane::Selector => {
                run_selector(nc, &rpc_config, planes.selector, &mut current_plane);
            }
        }
    }

//    unsafe { nc.stop()? };

//    Ok(())
}
