fn main() {
    cc::Build::new()
        .file("shim.c")
        .define("_POSIX_C_SOURCE", "200112L")
        .define("TB_IMPL", "1")
        .define("TB_LIB_OPTS", "1")
        .define("TB_OPT_TRUECOLOR", "1")
        .compile("shim");
}
