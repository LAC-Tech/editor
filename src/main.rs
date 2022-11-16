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

mod glue {
    #[repr(C)]
    #[derive(PartialEq, Eq, Hash)]
    // Re-declaring tb_event from terbox2
    pub struct TBEvent {
        pub r#type: u8, /* one of TB_EVENT_* constants */
        pub r#mod: u8,  /* bitwise TB_MOD_* constants */
        pub key: u16, /* one of TB_KEY_* constants */
        pub ch: u32,  /* a Unicode code point */
        pub w: i32,    /* resize width */
        pub h: i32,    /* resize height */
        pub x: i32,    /* mouse x */
        pub y: i32    /* mouse y */
    }

    #[repr(C)]
    pub struct Config {
        pub fg: u32,
        pub fg_err: u32,
        pub bg: u32
    }
}

mod inner {
    use std::ffi::{c_int, c_char, CStr};
    use std::mem;
    use crate::glue;

    extern "C" {
        // Termbox2
        fn tb_poll_event(ev: *mut glue::TBEvent) -> c_int;
        fn tb_clear() -> c_int;
        fn tb_present() -> c_int;
        fn tb_shutdown() -> c_int;
        fn tb_print(
            x: c_int, y: c_int, fg: u32, bg: u32, str: *const c_char
        ) -> c_int;
        fn tb_set_cursor(cx: c_int, cy: c_int) -> c_int;

        // Convenience functions to avoid C macros in Rust
        fn tb_init_truecolor();
        pub static tb_key_arrow_left: u16;
        pub static tb_key_arrow_right: u16;
        pub static tb_event_key: u8;
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Cursor(c_int, c_int);

    const CURSOR_ORIGIN: Cursor = Cursor(0, 0);

    impl Cursor {
        fn print(self) {
            unsafe { tb_set_cursor(self.0, self.1); }
        }
    }

    impl std::ops::Add<Cursor> for Cursor {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Self(self.0 + rhs.0, self.1 + rhs.1)
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn can_add_two_cursors_together() {
            let actual = CURSOR_ORIGIN + Cursor(0, 1);
            assert_eq!(actual, Cursor(0, 1));
        }
    }

    #[repr(C)]
    pub struct Term {
        cursor: Cursor,
        config: glue::Config
    }

    impl Term {
        fn new(config: glue::Config) -> Self {
            let cursor = Cursor(0, 0);
            Self {cursor, config}

        }
    }

    #[no_mangle]
    pub extern fn term_start(term: *mut Term, config: glue::Config)  {
        let t = Term::new(config);
        unsafe { 
            term.write(t);
            tb_init_truecolor();
        }
    }

    #[no_mangle]
    pub extern fn term_end() {
        unsafe { tb_shutdown(); }
    }

    #[no_mangle]
    pub extern fn term_get_event() -> glue::TBEvent {
        unsafe { 
            let mut ev = mem::MaybeUninit::<glue::TBEvent>::uninit();
            tb_poll_event(ev.as_mut_ptr());
            return ev.assume_init();
        }
    }

    #[no_mangle]
    pub extern fn term_refresh() {
        unsafe { tb_present(); }
    }

    #[no_mangle]
    pub extern fn term_print(
        term: *const Term, x: c_int, y: c_int, s: *const c_char
    ) -> c_int {
        unsafe {
            let config = &(*term).config;
            tb_print(x, y, config.fg, config.bg, s)
        }
    }

    #[no_mangle]
    pub extern fn term_print_err(
        term: *const Term, x: c_int, y: c_int, s: *const c_char
    ) -> c_int {
        unsafe {
            let config = &(*term).config;
            tb_print(x, y, config.fg_err, config.bg, s)
        }
    }

    #[no_mangle]
    pub extern fn term_open_text_file(
        term: *mut Term, path: *const c_char
    ) {
        let rust_path = unsafe { 
            CStr::from_ptr(path).to_str().expect("valid utf8 sequence")
        };

        let mut buffer: Vec<u8> = vec![];

        if let Ok(file_contents) = std::fs::read_to_string(rust_path) {
            for (i, line) in file_contents.lines().enumerate() {
                buffer.extend_from_slice(line.as_bytes());
                buffer.push(b'\0');

                let cstr = CStr::from_bytes_with_nul(buffer.as_slice())
                    .expect("not a null terminated slice");

                term_print(term, 0, i as std::ffi::c_int, cstr.as_ptr());
                buffer.clear();
            }
        }

        unsafe { 
            (*term).cursor = CURSOR_ORIGIN; 
            (*term).cursor.print();
        }
    }

    #[no_mangle]
    pub extern fn term_move_cursor_left(term: *mut Term) {
        unsafe {
            (*term).cursor = (*term).cursor + Cursor(-1, 0);
            (*term).cursor.print();
        }
    }

    #[no_mangle]
    pub extern fn term_move_cursor_right(term: *mut Term) {
        unsafe {
            (*term).cursor = (*term).cursor + Cursor(1, 0);
            (*term).cursor.print();
        }
    }
}

mod outer {
    use crate::glue;
    use std::collections::HashMap;

    pub const CONFIG: glue::Config = glue::Config {
        fg: 0x00FF00,
        fg_err: 0xFF0000,
        bg: 0x000000
    };
    
    // pub struct KeyBindings {
    //     hash_map: HashMap<glue::TBEvent, fn()>
    // }

    // impl KeyBindings {
    //     pub fn new() -> Self {
    //         Self { hash_map: HashMap::new() }
    //     }

    //     pub fn bind(&mut self, event: glue::TBEvent, action: fn()) {
    //         self.hash_map.insert(event, action);
    //     }

    //     pub fn get(self, event: &glue::TBEvent) -> Option<fn()> {
    //         self.hash_map.get(event).cloned()
    //     }
    // }
}

// Temp function - later scheme will provide the strings
fn cs<S>(s: S) -> std::ffi::CString where S: Into<Vec<u8>> {
    std::ffi::CString::new(s).expect("string should be null terminated")
}

use std::collections::HashMap;

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
                inner::term_move_cursor_left(term);

            } else if event.key == unsafe { inner::tb_key_arrow_right } {
                inner::term_move_cursor_right(term);
            } else {
                break;
            }
        } else {
            break;
        }

        inner::term_refresh();
    }
    
    
    
    inner::term_end();
}
