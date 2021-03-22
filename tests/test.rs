use std::path::PathBuf;

fn get_test_root() -> PathBuf {
    let mut root = std::path::PathBuf::new();
    root.push(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root
}

fn get_test_dir(toplevel: &'static str) -> PathBuf {
    let mut r = get_test_root();
    r.push(toplevel);
    r
}

fn tstring<'a>(segments: &[&'static str]) -> String {
    let mut buf = get_test_root();
    for segment in segments {
        buf.push(segment)
    }
    buf.to_str().unwrap().to_string()
}

#[test]
fn works_on_flat_dir() {
    let expected = vec![
        tstring(&["one_level", "1"]),
        tstring(&["one_level", "2"]),
        tstring(&["one_level", "3"]),
    ];
    let mut result: Vec<String> = lsr::get_dir_files(get_test_dir("one_level")).unwrap();
    result.sort();
    assert_eq!(expected, result)
}

#[test]
fn works_on_nested_dir() {
    let expected = vec![
        tstring(&["many_level", "1"]),
        tstring(&["many_level", "2"]),
        tstring(&["many_level", "deep", "3"]),
        tstring(&["many_level", "deep", "4"]),
        tstring(&["many_level", "deep", "deep", "5"]),
        tstring(&["many_level", "deep", "deep", "6"]),
        tstring(&["many_level", "deep", "deep", "7"]),
        tstring(&["many_level", "deep", "deep", "deep", "10"]),
        tstring(&["many_level", "deep", "deep", "deep", "8"]),
        tstring(&["many_level", "deep", "deep", "deep", "9"]),
    ];
    let mut result: Vec<String> = lsr::get_dir_files(get_test_dir("many_level")).unwrap();
    result.sort();
    assert_eq!(expected, result)
}
