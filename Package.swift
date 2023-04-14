// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "TreeSitterquarto",
    platforms: [.macOS(.v10_13), .iOS(.v11)],
    products: [
        .library(name: "TreeSitterquarto", targets: ["TreeSitterquarto", "TreeSitterquartoInline"]),
    ],
    dependencies: [],
    targets: [
        .target(name: "TreeSitterquarto",
                path: "tree-sitter-quarto",
                exclude: [
                    "corpus",
                    "grammar.js",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.c",
                ],
                resources: [
                    .copy("queries")
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")]),
        .target(name: "TreeSitterquartoInline",
                path: "tree-sitter-quarto-inline",
                exclude: [
                    "corpus",
                    "grammar.js",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.c",
                ],
                resources: [
                    .copy("queries")
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")])
    ]
)
