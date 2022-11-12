#include <stdint.h>
#include <wchar.h>
#include "termbox2.h"

void term_start() {
	tb_init();
	tb_set_output_mode(TB_OUTPUT_TRUECOLOR);
}

struct tb_event term_get_event() {
	struct tb_event ev;
	tb_poll_event(&ev);
	return ev;
}

void term_print(int x, int y, const char* str) {
	tb_printf(x, y, 0xFFFFFF, 0x000000, str);
}
