use clap::Parser;
use hyper::{header::SET_COOKIE, HeaderMap};

#[derive(Clone, Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Which port is server running on
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    /// Box margin in pixel
    #[arg(long, default_value_t = 0)]
    pub margin: usize,

    /// Box margin in pixel
    #[arg(long, default_value_t = 12)]
    pub padding: usize,

    /// Box width in pixel
    #[arg(long, default_value_t = 400)]
    pub width: usize,

    /// Box height in pixel
    #[arg(long, default_value_t = 80)]
    pub height: usize,

    /// Box border radius in pixel
    #[arg(long, default_value_t = 6)]
    pub border_radius: usize,

    /// Box background color in CSS string value
    #[arg(long, default_value_t = String::from("#0004"))]
    pub background: String,

    /// Text color in CSS string value
    #[arg(long, default_value_t = String::from("#EEE"))]
    pub foreground: String,

    /// Text fontsize in pixel
    #[arg(long, default_value_t = 36)]
    pub fontsize: usize,

    /// How long the text will stay visible until it disappear
    #[arg(long, default_value_t = 2000)]
    pub stay_duration: usize,

    /// Text disappear transition duration
    #[arg(long, default_value_t = 500)]
    pub fade_duration: usize,
}

impl Config {
    pub fn to_cookie_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let cookies = vec![
            format!("port={}", self.port),
            format!("margin={}", self.margin),
            format!("padding={}", self.padding),
            format!("width={}", self.width),
            format!("height={}", self.height),
            format!("border_radius={}", self.border_radius),
            format!("background={}", self.background),
            format!("foreground={}", self.foreground),
            format!("fontsize={}", self.fontsize),
            format!("stay_duration={}", self.stay_duration),
            format!("fade_duration={}", self.fade_duration),
        ];

        for cookie in cookies.iter() {
            headers.append(SET_COOKIE, cookie.parse().unwrap());
        }

        headers
    }
}
