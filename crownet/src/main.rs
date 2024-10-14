use notcurses::sys::*;
use py_rpc;

use crownet::selector::*;
use crownet::text_box::*;
use crownet::{CurrentPlane, Planes};

fn main() -> NcResult<()> {
    let rpc_config = py_rpc::Config::new();
    let a = py_rpc::connect(&rpc_config);

    println!("connect: {:?}", a);

    let nc: &mut Nc = unsafe { Nc::new()? };
    let stdplane: &mut NcPlane = unsafe { nc.stdplane() };
    stdplane.set_fg_rgb(0x40f040);

    let mut planes = Planes::init(stdplane);

    let mut current_plane = CurrentPlane::TextBox;
    nc.render();

    loop {
        match current_plane {
            CurrentPlane::TextBox => {
                text_box(
                    nc,
                    &rpc_config,
                    planes.reader,
                    planes.text_layer_selector,
                    &mut current_plane,
                );
            }
            CurrentPlane::Selector => {
                run_selector(nc, &rpc_config, planes.selector, &mut current_plane);
            }
            _ => (),
            //            CurrentPlane::TextLayerSelector => {
            //                run_layer_selector(nc, &rpc_config, planes.text_layer_selector, &mut current_plane);
            //            },
        }
        planes.update_indicator(&current_plane);
        nc.render();
    }

    //    unsafe { nc.stop()? };

    //    Ok(())
}
