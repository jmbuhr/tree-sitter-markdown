fn main() {
    let src_dir_block = std::path::Path::new("tree-sitter-quarto/src");
    let src_dir_inline = std::path::Path::new("tree-sitter-quarto-inline/src");

    let mut c_config = cc::Build::new();
    c_config.include(&src_dir_block);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir_block.join("parser.c");
    c_config.file(&parser_path);
    c_config.compile("parser_block");
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let mut c_config = cc::Build::new();
    c_config.include(&src_dir_inline);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir_inline.join("parser.c");
    c_config.file(&parser_path);
    c_config.compile("parser_inline");
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let mut c_config = cc::Build::new();
    c_config.cpp(false);
    c_config.include(&src_dir_block);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");
    let scanner_path = src_dir_block.join("scanner.c");
    c_config.file(&scanner_path);
    c_config.compile("scanner_block");
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    let mut c_config = cc::Build::new();
    c_config.cpp(false);
    c_config.include(&src_dir_inline);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");
    let scanner_path = src_dir_inline.join("scanner.c");
    c_config.file(&scanner_path);
    c_config.compile("scanner_inline");
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
}
