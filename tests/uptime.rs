#[test]
fn uptime_formatter_covers_units() {
    assert_eq!(astrafetch::format_uptime_for_test(51), "51s");
    assert_eq!(astrafetch::format_uptime_for_test(23 * 60 + 51), "23m 51s");
    assert_eq!(
        astrafetch::format_uptime_for_test(14 * 3600 + 23 * 60 + 51),
        "14h 23m 51s"
    );
    assert_eq!(
        astrafetch::format_uptime_for_test(3 * 86400 + 14 * 3600 + 23 * 60 + 51),
        "03d 14h 23m 51s"
    );
}
