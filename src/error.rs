pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Msg(String),
    Input(String),
    Serde(String),
    Uri(String),
    ServerFn(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn input<T: ToString>(msg: T) -> Self {
        Error::Input(format!("Input Error: {}", msg.to_string()))
    }
}
