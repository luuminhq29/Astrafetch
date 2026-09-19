pub fn no_color() -> bool {
    std::env::var_os("NO_COLOR").is_some() || std::env::var("TERM").ok().as_deref() == Some("dumb")
}
