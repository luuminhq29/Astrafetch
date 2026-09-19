use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(
    name = "astrafetch",
    bin_name = "astrafetch",
    version = "1.0.0",
    disable_version_flag = true,
    about = "AstraFetch — Linux System Information Tool",
    after_help = "AstraFetch — Linux System Information Tool\nCreated by Lưu Minh Quang - Astra"
)]
pub struct Cli {
    #[arg(long, action = ArgAction::SetTrue, help = "Realtime dashboard")]
    pub watch: bool,
    #[arg(long, action = ArgAction::SetTrue, help = "Compact one-line output")]
    pub minimal: bool,
    #[arg(long, action = ArgAction::SetTrue, help = "Output machine-readable JSON")]
    pub json: bool,
    #[arg(
        long,
        value_name = "THEME",
        help = "Theme: default, cyber, aurora, matrix, minimal, monochrome"
    )]
    pub theme: Option<String>,
    #[arg(
        long,
        value_name = "LOGO",
        help = "Logo: auto, ubuntu, arch, debian, fedora, linux, custom, none"
    )]
    pub logo: Option<String>,
    #[arg(long, action = ArgAction::SetTrue, help = "Disable animation")]
    pub no_animation: bool,
    #[arg(
        long,
        value_name = "SECONDS",
        help = "Realtime refresh interval in seconds"
    )]
    pub refresh_rate: Option<u64>,
    #[arg(short = 'V', long = "version", action = ArgAction::SetTrue, help = "Show version and author") ]
    pub version_requested: bool,
}
