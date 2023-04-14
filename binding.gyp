{
  "targets": [
    {
      "target_name": "tree_sitter_quarto_binding",
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "tree-sitter-quarto/src",
        "tree-sitter-quarto-inline/src",
      ],
      "sources": [
        "tree-sitter-quarto/src/parser.c",
        "tree-sitter-quarto/src/scanner.c",
        "tree-sitter-quarto-inline/src/parser.c",
        "tree-sitter-quarto-inline/src/scanner.c",
        "bindings/node/binding.cc"
      ],
      "cflags_c": [
        "-std=c99"
      ]
    }
  ]
}

