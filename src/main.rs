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
 * - outer is implemented in scheme and calls by inner.
 * 
 * The glue layer are PODs that both layers use to communicate.
 * These naturally have to have a C repr.
 */

mod glue {
    #[repr(C)]
    pub struct TBEvent {
        r#type: u8, /* one of TB_EVENT_* constants */
        r#mod: u8,  /* bitwise TB_MOD_* constants */
        key: u16, /* one of TB_KEY_* constants */
        ch: u32,  /* a Unicode code point */
        w: i32,    /* resize width */
        h: i32,    /* resize height */
        x: i32,    /* mouse x */
        y: i32    /* mouse y */
    }

    #[repr(C)]
    pub struct Config {
        pub fg: u32,
        pub fg_err: u32,
        pub bg: u32
    }
}

mod inner {
    use std::{ffi::{c_int, c_char, CStr, FromBytesWithNulError, CString}};
    use crate::glue;

    extern "C" {
        // higher level term stuff
        fn term_start();
        fn term_get_event() -> glue::TBEvent;
    
        // If the C shim does nothing, just use termbox2 directly
        fn tb_clear() -> c_int;
        fn tb_present() -> c_int;
        fn tb_shutdown() -> c_int;
        fn tb_print(x: c_int, y: c_int, fg: u32, bg: u32, str: *const c_char) -> c_int;
    }

    #[derive(Debug)]
    pub enum PrintErr {
        StringNotNullTerminated(String)
    }

    pub struct Term {
        config: glue::Config   
    }

    impl Term {
        pub fn global_start() {
            unsafe { term_start(); }
        }

        pub fn global_end() {
            unsafe { tb_shutdown(); }
        }

        pub fn new(config: glue::Config) -> Self {
            Self {config}
        }

        fn to_cstr(s: &[u8]) -> Result<*const i8, FromBytesWithNulError> {
            CStr::from_bytes_with_nul(s).map(|s| s.as_ptr())
        }

        fn print_with_fg(
            self, fg: u32, x: c_int, y: c_int, s: &[u8]
        ) -> Result<(), PrintErr> {
            Self::to_cstr(s)
            .map_err(|_| {
                let err_msg = std::str::from_utf8(s)
                    .expect("valid utf-8 string")
                    .to_string();

                PrintErr::StringNotNullTerminated(err_msg)
            })
            .map(|c_str| unsafe {
                tb_print(x, y, fg, self.config.bg, c_str);
            })
        }

        pub fn print(
            self, x: c_int, y: c_int, s: &[u8]
        ) -> Result<(), PrintErr> {
            let fg = self.config.fg;
            self.print_with_fg(fg, x, y, s)
        }

        pub fn print_err(
            self, x: c_int, y: c_int, s: &[u8]
        ) -> Result<(), PrintErr> {
            let fg = self.config.fg_err;
            self.print_with_fg(fg, x, y, s)
        }

        pub fn refresh() {
            unsafe { tb_present(); }
        }

        pub fn get_event() -> glue::TBEvent {
            unsafe { term_get_event() }
        }

        pub fn open_text_file<P: AsRef<std::path::Path>>(
            self, path: P
        ) -> Result<(), PrintErr> {

            
            match std::fs::read(path) {
                Ok(mut byte_vec) => {
                    byte_vec.push(b'\0');
                    self.print(0, 0, &byte_vec)
                },
                Err(err) => {
                    let cs = CString::new(err.to_string())
                        .expect("error from std lib has internal null bytes"); 
                    let s = cs.as_bytes_with_nul();
                    self.print_err(0, 0, s)
                }
            }
        }
    }
}

mod outer {
    use crate::glue;

    pub const CONFIG: glue::Config = glue::Config {
        fg: 0x00FF00,
        fg_err: 0xFF0000,
        bg: 0x000000
    };
}

fn main() {
    inner::Term::global_start();
    let term = inner::Term::new(outer::CONFIG);
    term.open_text_file("Cargo.toml");
    inner::Term::refresh();
    inner::Term::get_event();
    
    inner::Term::global_end();
}
