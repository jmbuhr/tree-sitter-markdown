use tree_sitter_md::*;

fn main() {
    let mut parser = quartoParser::default();
    let filename = std::env::args().nth(1).unwrap();
    let source = std::fs::read(filename).unwrap();
    let mut tree = parser.parse(&source, None).unwrap();
    tree.edit(&tree_sitter::InputEdit {
        start_byte: 0,
        old_end_byte: 1,
        new_end_byte: 0,
        start_position: tree_sitter::Point::new(0, 0),
        old_end_position: tree_sitter::Point::new(0, 1),
        new_end_position: tree_sitter::Point::new(0, 0),
    });
    reparse(&mut parser, &source[1..], tree);
}

fn reparse(parser: &mut quartoParser, source: &[u8], old_tree: quartoTree) {
    parser.parse(source, Some(&old_tree));
}
