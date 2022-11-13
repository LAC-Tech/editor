#include "termbox2.h"

// Can't be bothered dealing with C Macros in Rust.
void tb_init_truecolor() {
	tb_init();
	tb_set_output_mode(TB_OUTPUT_TRUECOLOR);
}
