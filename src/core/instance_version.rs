use std::num::ParseIntError;
use std::str::FromStr;

pub enum InstanceVersionParsingError {
    ParseIntError(ParseIntError),
    ParseError(String),
}

#[derive(Debug, Default)]
pub struct InstanceVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl FromStr for InstanceVersion {
    type Err = InstanceVersionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('.')
            .map(|e| i32::from_str(e).map_err(|e| InstanceVersionParsingError::ParseIntError(e)))
            .collect::<Result<Vec<_>, InstanceVersionParsingError>>()
            .and_then(|e| match e[..] {
                [major, minor, patch] => Ok(InstanceVersion {
                    major: major,
                    minor: minor,
                    patch: patch,
                }),
                _ => Err(InstanceVersionParsingError::ParseError(s.to_string())),
            })
    }
}
