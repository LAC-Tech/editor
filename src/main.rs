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
 * - outer is implemented in scheme and called by inner.
 * 
 * The glue layer are PODs that both layers use to communicate.
 * These naturally have to have a C represntation
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
    use std::ffi::{c_int, c_char, CStr};
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

        fn to_cstr(s: &str) -> *const i8 {
            CStr::from_bytes_with_nul(s.as_bytes())
                .expect("A single, sentinel, null byte")
                .as_ptr()
        }

        pub fn print(self, x: c_int, y: c_int, s: &str) {
            let c_str = Self::to_cstr(s);
            unsafe {
                tb_print(x, y, self.config.fg, self.config.bg, c_str);
            }
        }

        pub fn refresh() {
            unsafe { tb_present(); }
        }

        pub fn get_event() -> glue::TBEvent {
            unsafe { term_get_event() }
        }

        // fn open_text_file<P: AsRef<std::path::Path>>(path: P) {
//     match std::fs::read_to_string(path) {
//         Ok(s) => tb_print(0, 0, config::FG, config::BG, s),
//         Err(s) => tb_print(0, 0, config::FG_ERR, config::BG, s)
//     };
// )
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
    term.print(0, 0, "Merry Christmas!!!\n\0");
    inner::Term::refresh();
    inner::Term::get_event();
    inner::Term::global_end();
    
}
