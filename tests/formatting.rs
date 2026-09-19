#[test]
fn bytes_are_human_readable() {
    assert_eq!(astrafetch::format_bytes_for_test(1024 * 1024), "1 MiB");
    assert_eq!(astrafetch::format_bytes_for_test(2 * 1024 * 1024 * 1024), "2.0 GiB");
}
