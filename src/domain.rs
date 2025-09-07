use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::{num::ParseIntError, str::FromStr};
use url::Url;

/////////////////////////////////This is the domain part////////////////////////
#[derive(Debug)]
pub struct Category {
    id: String,
    label: String,
}

#[derive(Debug)]
pub struct Event {
    title: String,
    picture_url: Url,
}

#[derive(Debug)]
pub struct InstanceVersion {
    major: i32,
    minor: i32,
    patch: i32,
}

pub enum InstanceVersionParsingError {
    ParseIntError(ParseIntError),
    LengthError(),
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
                _ => Err(InstanceVersionParsingError::LengthError()),
            })
    }
}

///////////////////////////////Infrastructure part///////////////////////////////

#[derive(Debug)]
pub enum EventsFetchError {
    HttpError(Box<reqwest::Error>),
    FailedToParseTotalErr,
    FailedToParseEvents,
    CouldNotCollect,
    Other(String)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/events.graphql"
)]
struct SearchEventsQuery;

pub async fn fetch_events(
    http_client: &reqwest::Client,
    graphql_url: &str,
) -> Result<(Vec<Event>, i64), EventsFetchError> {
    let request_body = SearchEventsQuery::build_query(search_events_query::Variables {});
    let res = http_client
        .post(graphql_url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| EventsFetchError::HttpError(Box::new(e)))?;
    let response_body: Response<search_events_query::ResponseData> = res
        .json()
        .await
        .map_err(|e| EventsFetchError::HttpError(Box::new(e)))?;
    let total_events_fetched = response_body
        .data
        .as_ref()
        .and_then(|data| data.search_events.as_ref())
        .map(|search_events| search_events.total)
        .ok_or(EventsFetchError::FailedToParseTotalErr)?;
    let events = response_body
        .data.ok_or(EventsFetchError::Other("No data field".to_string()))
        .and_then(|data| data.search_events.ok_or(EventsFetchError::Other("No search_events field.".to_string())))
        .and_then(|search_events| {
            search_events
                .elements
                .into_iter()
                .collect::<Option<Vec<_>>>()
                .ok_or(EventsFetchError::Other("Collecting to an option returned None.".to_string()))
        })
        .and_then(|events| {
            events
                .into_iter()
                .map(|e| match e {
                    search_events_query::SearchEventsQuerySearchEventsElements {
                        title: Some(t),
                        on: _,
                        picture:
                            Some(search_events_query::SearchEventsQuerySearchEventsElementsPicture {
                                url: Some(u),
                            }),
                    } => Url::from_str(u.as_str()).map_err(|_| EventsFetchError::Other("Not a valid URL".to_string())).and_then(|e| {
                        Ok(Event {
                            title: t,
                            picture_url: e,
                        })
                    }),
                    search_events_query::SearchEventsQuerySearchEventsElements {
                        title: Some(t),
                        on: _,
                        picture: _
                    } => Err(EventsFetchError::Other(format!("Structure of event not expected: title: {}", t).to_string())),
                    _ => Err(EventsFetchError::Other("Structure of event not expected".to_string())),
                })
                .map(|e| e.map_err(|err| {
                  println!("Error: {:?}", err);
                  err
                }))
                .filter(|e| e.is_ok())
                .collect::<Result<Vec<Event>, EventsFetchError>>()
        })?;
    Ok((events, total_events_fetched))
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/config.graphql"
)]
struct ConfigQuery;

pub enum ConfigFetchError {
    HttpError(Box<reqwest::Error>),
    InstanceVersionParsingError(InstanceVersionParsingError),
    OtherError(String),
}

pub async fn fetch_config(
    graphql_url: &str,
) -> Result<(InstanceVersion, Vec<Category>), ConfigFetchError> {
    // This can be moved later
    let client = reqwest::Client::new();

    // The GraphQL query itself
    let request_body = ConfigQuery::build_query(config_query::Variables {});
    let res = client
        .post(graphql_url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| ConfigFetchError::HttpError(Box::new(e)))?;
    let response_body: Response<config_query::ResponseData> = res
        .json()
        .await
        .map_err(|e| ConfigFetchError::HttpError(Box::new(e)))?;

    // Extracting domain objects
    let version: &String = response_body
        .data
        .as_ref()
        .and_then(|data| data.config.as_ref())
        .and_then(|config| config.version.as_ref())
        .ok_or(ConfigFetchError::OtherError(
            "Failed to get version.".to_string(),
        ))?;
    let instance_version = InstanceVersion::from_str(version)
        .map_err(|err| ConfigFetchError::InstanceVersionParsingError(err))?;
    let categories = response_body
        .data
        .and_then(|data| data.config)
        .and_then(|config| config.event_categories)
        .ok_or(ConfigFetchError::OtherError(
            "Failed to get categories.".to_string(),
        ))?
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(ConfigFetchError::OtherError(
            "A category is null.".to_string(),
        ))?
        .into_iter()
        .map(|a| {
            let label = a.label.ok_or(ConfigFetchError::OtherError(String::from(
                "label undefined",
            )))?;
            let id =
                a.id.ok_or(ConfigFetchError::OtherError(String::from("id undefined")))?;

            Ok(Category {
                label: label,
                id: id,
            })
        })
        .collect::<Result<Vec<Category>, ConfigFetchError>>()?;
    Ok((instance_version, categories))
}

////////////////////////////////Testing/////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_query() {
        let client = reqwest::Client::new();
        let query_result = fetch_events(&client, "https://mobilizon.fr/api").await;
        match query_result {
            Ok(events) => {
                println!("{:?}", events);
                assert!(true);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!();
            },
        }
    }
}
