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
    }

    #[derive(Clone, Copy)]
    struct Cursor(c_int, c_int);

    impl Cursor {
        fn print(self) {
            
            unsafe { tb_set_cursor(self.0, self.1); }
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

        fn move_cursor(&mut self, p: Cursor) {
            self.cursor = p;
            self.cursor.print();
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
        term: *const Term, path: *const c_char
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
    
    pub struct KeyBindings {
        hash_map: HashMap<glue::TBEvent, fn()>
    }

    impl KeyBindings {
        pub fn new() -> Self {
            Self { hash_map: HashMap::new() }
        }

        pub fn bind(&mut self, event: glue::TBEvent, action: fn()) {
            self.hash_map.insert(event, action);
        }

        pub fn get(self, event: &glue::TBEvent) -> Option<fn()> {
            self.hash_map.get(event).cloned()
        }
    }
}

// Temp function - later scheme will provide the strings
fn cs<S>(s: S) -> std::ffi::CString where S: Into<Vec<u8>> {
    std::ffi::CString::new(s).expect("string should be null terminated")
}

fn main() {
    let mut term = mem::MaybeUninit::<inner::Term>::uninit();
    inner::term_start(term.as_mut_ptr(), outer::CONFIG);
    let term_ptr = term.as_mut_ptr();

    inner::term_open_text_file(term_ptr, cs("./Cargo.toml").as_ptr());
    
    inner::term_refresh();
    inner::term_get_event();

    inner::term_end();
}
