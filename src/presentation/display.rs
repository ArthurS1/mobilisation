use std::fmt::Display;

use crate::core::instance_version::InstanceVersionParsingError;
use crate::infra::config::ConfigFetchError;
use crate::infra::events::{EventDecodeError, EventsFetchError};

impl Display for ConfigFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigFetchError::MissingField(name) => {
                write!(f, "The field {} is missing from the json.", name)
            }
            ConfigFetchError::HttpError(err) => write!(f, "Http error : {}", err),
            ConfigFetchError::InstanceVersionParsingError(err) => {
                write!(f, "Instance version could not be parsed: {}", err)
            }
        }
    }
}

impl Display for InstanceVersionParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceVersionParsingError::ParseError(original) => {
                write!(
                    f,
                    "The instance version '{}' was not properly formatted.",
                    original
                )
            }
            InstanceVersionParsingError::ParseIntError(parse_int_error) => {
                write!(f, "Failed to parse an integer : {}", parse_int_error)
            }
        }
    }
}

impl Display for EventsFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventsFetchError::HttpError(err) => write!(f, "Http error : {}", err),
            EventsFetchError::MissingTotalResultsField => write!(f, "Missing total number of events in response."),
            EventsFetchError::MissingField(name) => {
                write!(f, "The field {} is missing from the json.", name)
            }
        }
    }
}

impl Display for EventDecodeError {

  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      EventDecodeError::EventWithNoId => write!(f, "Event with no id."),
      EventDecodeError::InvalidPictureUrl(url) => write!(f, "Invalid picture url {}.", url),
      EventDecodeError::UnexpectedStructureOfEvent(id, debug) => write!(f, "Unexpected structure of event with id: {}\nEvent:\n{}\n", id, debug)
    }
  }

}

