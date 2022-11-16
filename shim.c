#include "termbox2.h"

// Can't be bothered dealing with C Macros in Rust.
void tb_init_truecolor() {
	tb_init();
	tb_set_output_mode(TB_OUTPUT_TRUECOLOR);
}

const uint16_t tb_key_arrow_left = TB_KEY_ARROW_LEFT;
const uint16_t tb_key_arrow_right = TB_KEY_ARROW_RIGHT;
const uint16_t tb_key_arrow_up = TB_KEY_ARROW_UP;
const uint16_t tb_key_arrow_down = TB_KEY_ARROW_DOWN;

const uint8_t tb_event_key = TB_EVENT_KEY;
