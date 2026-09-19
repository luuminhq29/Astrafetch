#[test]
fn project_metadata_is_stable() {
    assert_eq!(env!("CARGO_PKG_NAME"), "astrafetch");
    assert_eq!(env!("CARGO_PKG_VERSION"), "1.0.0");
    assert_eq!(env!("CARGO_PKG_AUTHORS"), "Lưu Minh Quang - Astra");
}
