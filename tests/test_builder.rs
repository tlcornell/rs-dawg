/// test_builder.rs

extern crate dawg;

use dawg::dawg::DawgBuilder;

#[test]
fn builder_works() {
    let builder = DawgBuilder::new();
    builder
        .add_word("abra")
        .add_word("absol")
        .add_word("crobat")
        .add_word("zubat")
        .build();
}
