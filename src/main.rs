/**
 * The editor consists of two parts: Inner and Outer.
 * 
 * Inner deals with
 * - writing to the screen
 * - detecting input
 * 
 * Outer deals with
 * - key maps
 * - colorschemes
 * - anything else the user might want to change
 * 
 * The plan is:
 * - inner has a C interface
 * - outer is implemented in scheme and calls inner.
 * 
 * The glue layer are PODs that both layers use to communicate.
 * These naturally have to have a C repr.
 */

use std::mem;

mod glue;
mod inner;

mod outer {
    use crate::glue;

    pub const CONFIG: glue::Config = glue::Config {
        fg: 0x00FF00,
        fg_err: 0xFF0000,
        bg: 0x000000
    };
}

// Temp function - later scheme will provide the strings
fn cs<S>(s: S) -> std::ffi::CString where S: Into<Vec<u8>> {
    std::ffi::CString::new(s).expect("string should be null terminated")
}

fn main() {
    let mut t = mem::MaybeUninit::<inner::Term>::uninit();
    inner::term_start(t.as_mut_ptr(), outer::CONFIG);
    let term = t.as_mut_ptr();

    inner::term_open_text_file(term, cs("./Cargo.toml").as_ptr());
    
    inner::term_refresh();
    
    loop {
        let event = inner::term_get_event();

        if event.r#type == unsafe { inner::tb_event_key } {
            if event.key == unsafe { inner::tb_key_arrow_left } {
                inner::term_move_cursor_move(term, -1, 0);
            } else if event.key == unsafe { inner::tb_key_arrow_right } {
                inner::term_move_cursor_move(term, 1, 0);
            } else if event.key == unsafe { inner::tb_key_arrow_up } {
                inner::term_move_cursor_move(term, 0, -1);
            } else if event.key == unsafe { inner::tb_key_arrow_down } {
                inner::term_move_cursor_move(term, 0, 1);
            }

            else {
                break;
            }
        } else {
            break;
        }

        inner::term_refresh();
    }
    
    
    
    inner::term_end();
}
