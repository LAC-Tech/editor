fn main() {
    cc::Build::new()
        .file("shim.c")
        .define("TB_IMPL", "1")
        .define("TB_LIB_OPTS", "1")
        .define("TB_OPT_TRUECOLOR", "1")
        .compile("shim");
}
