use py_rpc;
use notcurses::sys::{widgets::*, *};
use libnotcurses_sys::c_api::{ncreader, ncreader_write_egc, ncreader_contents, ncreader_clear};
use std::ffi::{CStr, CString};
use crate::CurrentPlane;

//TODO make a selector for which layer to write on
pub fn text_box(nc: &mut Nc, rpc_config: &py_rpc::Config, reader: &mut NcReader,
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
