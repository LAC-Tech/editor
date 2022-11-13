#[repr(C)]
#[derive(Clone, Copy)]
struct TBEvent {
    r#type: u8, /* one of TB_EVENT_* constants */
    r#mod: u8,  /* bitwise TB_MOD_* constants */
    key: u16, /* one of TB_KEY_* constants */
    ch: u32,  /* a Unicode code point */
    w: i32,    /* resize width */
    h: i32,    /* resize height */
    x: i32,    /* mouse x */
    y: i32    /* mouse y */
}

use std::ffi::{c_int, c_char, CStr};

extern "C" {
    // higher level term stuff
    fn term_new();
    fn term_wait_for_event() -> TBEvent;

    // If the C shim does nothing, just use termbox2 directly
    fn tb_clear() -> c_int;
    fn tb_present() -> c_int;
    fn tb_shutdown() -> c_int;
    fn tb_print(x: c_int, y: c_int, fg: u32, bg: u32, str: *const c_char) -> c_int;
}

fn main() {
    let cs = CStr::from_bytes_with_nul(b"Merry Christmas!!!\n\0")
        .expect("A single, sentinel, null byte")
        .as_ptr();
    unsafe {
        term_new();
        tb_print(0, 0, 0xFF0000, 0x00FF00, cs);
        tb_present();
        term_wait_for_event();
        tb_clear();
        tb_shutdown();
    }
}
