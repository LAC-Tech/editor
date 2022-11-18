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
