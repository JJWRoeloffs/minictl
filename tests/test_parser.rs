use std::fs;

use minictl::parser;

mod common;

#[test]
fn all_ispl_files_are_valid() {
    for path in common::get_paths_by_ext(&common::get_testdata_dir(), "ispl").unwrap() {
        let contents = fs::read_to_string(path).expect("Files exist");
        let tokens = parser::tokenize(&contents);
        assert!(tokens.iter().all(|(token, _str)| !token.is_err()))
    }
}
