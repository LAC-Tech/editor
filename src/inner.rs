use std::ffi::{c_int, c_char, CStr};
use std::mem;
use crate::glue;

mod piece_table {
    #[derive(Clone, Debug)]
    enum Source { Original, Added }


    #[derive(Clone, Debug)]
    struct Descriptor {
        start: usize,
        length: usize,
        source: Source
    }

    pub struct PieceTable<'a> {
        original: &'a str,
        added: String,
        pieces: Vec<&'a str>
    }

    impl<'a> PieceTable<'a> {
        pub fn new(original: &'a str) -> Self {
            Self{original, added: String::new(), pieces: vec![original]}
        }

        pub fn insert(&'a mut self, position: usize, s: &'a str) {
            self.added.push_str(s);

            self.pieces = vec![
                &self.original[0..position -1],
                &self.added[0..s.len()],
                &self.original[position -1..self.original.len()]
            ];
        }

        pub fn delete(&mut self, position: usize) {

        }

        pub fn edited_string(&self) -> String {
            self.pieces.iter()
                .flat_map(|piece| {
                    piece.chars()
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use piece_table::PieceTable;

    #[test]
    fn basics() {
        let mut pt = PieceTable::new(
            "the quick brown fox\njumped over the lazy dog"
        );

        pt.insert(21, "went to the park and\n");

        assert_eq!(
            "the quick brown fox\nwent to the park and\njumped over the lazy dog", &pt.edited_string());

    }
}

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
    pub static tb_key_arrow_up: u16;
    pub static tb_key_arrow_down: u16;
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
pub extern fn term_move_cursor_move(term: *mut Term, x: c_int, y: c_int) {
    unsafe {
        (*term).cursor = (*term).cursor + Cursor(x, y);
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
