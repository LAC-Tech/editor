#include "termbox2.h"

// Can't be bothered dealing with C Macros in Rust.
void tb_init_truecolor() {
	tb_init();
	tb_set_output_mode(TB_OUTPUT_TRUECOLOR);
}

struct tb_event tb_key(uint16_t k) {
	struct tb_event result;
	result.key = k;
	result.type = TB_EVENT_KEY;
	result.mod = 0;
	return result;
}

const uint16_t tb_key_arrow_left = TB_KEY_ARROW_LEFT;
const uint16_t tb_key_arrow_right = TB_KEY_ARROW_RIGHT;
