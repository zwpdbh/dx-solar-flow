use std::env;
use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::path::MAIN_SEPARATOR;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub const PROTOCOL_SEPARATOR: &str = "://";

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Uri {
    uri: String,
    protocol: Protocol,
}

impl FromStr for Uri {
    type Err = crate::Error;

    fn from_str(uri_str: &str) -> crate::Result<Self> {
        Uri::parse_str(uri_str)
    }
}

impl Uri {
    pub fn separator(&self) -> String {
        let sep = self.protocol.separator();
        sep.to_string()
    }

    pub fn for_test(uri: &str) -> Self {
        Self::from_str(uri).unwrap()
    }

    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.uri).extension().and_then(OsStr::to_str)
    }

    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    fn _path(&self) -> &Path {
        Path::new(&self.uri[self.protocol.as_str().len() + PROTOCOL_SEPARATOR.len()..])
    }

    pub fn as_path(&self) -> PathBuf {
        self.path()
    }

    pub fn path(&self) -> PathBuf {
        let p = &self.uri[self.protocol.as_str().len() + PROTOCOL_SEPARATOR.len()..];
        match self.protocol {
            Protocol::File | Protocol::Ram => {
                let sub_path = p
                    .split(self.protocol.separator())
                    .collect::<Vec<&str>>()
                    .join(self.separator().as_str())
                    .to_string();
                if self.is_dir() {
                    PathBuf::from(format!("{}{}", sub_path, self.separator().as_str()))
                } else {
                    PathBuf::from(sub_path)
                }
            }
        }
    }

    pub fn is_dir(&self) -> bool {
        match self.protocol() {
            Protocol::File => self._path().is_dir(),
            Protocol::Ram => self.extension().is_none(),
        }
    }

    /// Attempts to construct a [`Uri`] from a string.
    /// A `file://` protocol is assumed if not specified.
    /// File URIs are resolved (normalized) relative to the current working directory
    /// unless an absolute path is specified.
    /// Handles special characters such as `~`, `.`, `..`.
    fn parse_str(uri_str: &str) -> crate::Result<Self> {
        if uri_str.is_empty() {
            return Err(crate::Error::Uri("URI cannot be empty".to_string()));
        }
        let (protocol, mut path) = match uri_str.split_once(PROTOCOL_SEPARATOR) {
            None => (Protocol::File, uri_str.to_string()),
            Some((protocol, path)) => (Protocol::from_str(protocol)?, path.to_string()),
        };
        if protocol == Protocol::File {
            if path.starts_with('~') {
                // We only accept `~` (alias to the home directory) and `~/path/to/something`.
                // If there is something following the `~` that is not `/`, we bail.
                if path.len() > 1 && !path.starts_with("~/") {
                    return Err(crate::Error::Uri(
                        "failed to normalize URI: tilde expansion is only partially supported"
                            .to_string(),
                    ));
                }

                let home_dir_path = home::home_dir()
                    .ok_or(crate::Error::Uri(
                        "failed to normalize URI: could not resolve home directory".to_string(),
                    ))?
                    .to_string_lossy()
                    .to_string();

                path.replace_range(0..1, &home_dir_path);
            }
            if Path::new(&path).is_relative() {
                let current_dir =
                    env::current_dir().map_err(|e| crate::Error::Uri(format!("{e:?}")))?;
                path = current_dir.join(path).to_string_lossy().to_string();
            }
            path = normalize_path(Path::new(&path))
                .to_string_lossy()
                .to_string();
        }

        Ok(Self {
            uri: format!("{protocol}{PROTOCOL_SEPARATOR}{path}"),
            protocol,
        })
    }
}

/// Normalizes a path by resolving the components like (., ..).
/// This helper does the same thing as `Path::canonicalize`.
/// It only differs from `Path::canonicalize` by not checking file existence
/// during resolution.
/// <https://github.com/rust-lang/cargo/blob/fede83ccf973457de319ba6fa0e36ead454d2e20/src/cargo/util/paths.rs#L61>
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut resulting_path_buf =
        if let Some(component @ Component::Prefix(_)) = components.peek().cloned() {
            components.next();
            PathBuf::from(component.as_os_str())
        } else {
            PathBuf::new()
        };

    for component in components {
        match component {
            Component::Prefix(_) => unreachable!(),
            Component::RootDir => {
                resulting_path_buf.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                resulting_path_buf.pop();
            }
            Component::Normal(inner_component) => {
                resulting_path_buf.push(inner_component);
            }
        }
    }
    resulting_path_buf
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Protocol {
    File = 1,
    Ram = 2,
}

impl Protocol {
    pub fn as_str(&self) -> &str {
        match &self {
            Protocol::File => "file",
            Protocol::Ram => "ram",
        }
    }

    pub fn separator(&self) -> char {
        match &self {
            Protocol::File => MAIN_SEPARATOR,
            Protocol::Ram => '/',
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl FromStr for Protocol {
    type Err = crate::Error;

    fn from_str(protocol: &str) -> crate::Result<Self> {
        match protocol {
            "file" => Ok(Protocol::File),
            "ram" => Ok(Protocol::Ram),
            _ => Err(crate::Error::Uri(format!("Unknown protocol: {protocol}"))),
        }
    }
}
