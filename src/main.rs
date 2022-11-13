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
    use std::ffi::{c_int, c_char};
    use std::mem::MaybeUninit;
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

        // Convenience functions to avoid C macros in Rust
        fn tb_init_truecolor();
    }

    #[no_mangle]
    pub extern fn term_start() {
        unsafe { tb_init_truecolor(); }
    }

    #[no_mangle]
    pub extern fn term_end() {
        unsafe { tb_shutdown(); }
    }

    #[no_mangle]
    pub extern fn term_get_event() -> glue::TBEvent {
        unsafe { 
            let mut ev = MaybeUninit::<glue::TBEvent>::uninit();
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
        config: &glue::Config, x: c_int, y: c_int, s: *const c_char
    ) -> c_int {
        unsafe {
            tb_print(x, y, config.fg, config.bg, s)
        }
    }

    #[no_mangle]
    pub extern fn term_print_err(
        config: &glue::Config, x: c_int, y: c_int, s: *const c_char
    ) -> c_int {
        unsafe {
            tb_print(x, y, config.fg_err, config.bg, s)
        }
    }

    // #[no_mangle]
    // pub fn term_open_text_file<P: AsRef<std::path::Path>>(
    //     self, path: P
    // ) -> Result<(), PrintErr> {
    //     match std::fs::read(path) {
    //         Ok(mut byte_vec) => {
    //             byte_vec.push(b'\0');
    //             self.print(0, 0, &byte_vec)
    //         },
    //         Err(err) => {
    //             let cs = CString::new(err.to_string())
    //                 .expect("error from std lib has internal null bytes"); 
    //             let s = cs.as_bytes_with_nul();
    //             self.print_err(0, 0, s)
    //         }
    //     }
    // }
}

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
    inner::term_start();
    
    inner::term_print(&outer::CONFIG, 0, 0, cs("hello!!!").as_ptr());
    inner::term_refresh();
    inner::term_get_event();

    inner::term_end();
}
