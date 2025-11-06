use bytes::Bytes;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use serde;
use std::{fmt::Debug, str::FromStr};
use url::Url;
/// The GraphQL dependency will look for the UUID type in caps
use uuid::Uuid as UUID;

use crate::core::event::Event;

/// Errors when querying the events
pub enum EventsFetchError {
    HttpError(Box<reqwest::Error>),
    MissingField(String),
    MissingTotalResultsField,
}

/// Errors in one specific event already that has been already retrieved
pub enum EventDecodeError {
    InvalidPictureUrl(String),
    UnexpectedStructureOfEvent(UUID, String),
    EventWithNoId,
}

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/events.graphql",
    response_derives = "Debug"
)]
struct SearchEventsQuery;

pub async fn fetch_events(
    http_client: &reqwest::Client,
    graphql_url: &str,
) -> Result<(Vec<Result<Event, EventDecodeError>>, i64), EventsFetchError> {
    let time_now = chrono::Utc::now();
    let request = SearchEventsQuery::build_query(search_events_query::Variables {
        begins_on: time_now,
    });
    let response = http_client
        .post(graphql_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| EventsFetchError::HttpError(Box::new(e)))?
        .json::<Response<search_events_query::ResponseData>>()
        .await
        .map_err(|e| EventsFetchError::HttpError(Box::new(e)))?;
    let total_events_fetched = response
        .data
        .as_ref()
        .and_then(|data| data.search_events.as_ref())
        .map(|search_events| search_events.total)
        .ok_or(EventsFetchError::MissingTotalResultsField)?;
    let raw_events = response
        .data
        .ok_or(EventsFetchError::MissingField("data".to_string()))
        .and_then(|data| {
            data.search_events
                .ok_or(EventsFetchError::MissingField("search_events".to_string()))
        })
        .and_then(|search_events| {
            search_events
                .elements
                .into_iter()
                .collect::<Option<Vec<_>>>()
                .ok_or(EventsFetchError::MissingField("event".to_string()))
        })?;
    let events = raw_events
        .into_iter()
        .map(|e| match e {
            search_events_query::SearchEventsQuerySearchEventsElements {
                uuid: Some(id),
                title: Some(title),
                on: _,
                begins_on: Some(begins_on),
                ends_on: Some(ends_on),
                picture:
                    Some(search_events_query::SearchEventsQuerySearchEventsElementsPicture {
                        url: Some(url),
                    }),
            } => Url::from_str(url.as_str())
                .map_err(|_| EventDecodeError::InvalidPictureUrl(url))
                .and_then(|picture_url| {
                    Ok(Event {
                        id,
                        title,
                        picture_url: Some(picture_url),
                        begins_on: crate::core::event::DateTime::new(begins_on),
                        ends_on: crate::core::event::DateTime::new(ends_on),
                    })
                }),
            failed_structure @ search_events_query::SearchEventsQuerySearchEventsElements {
                uuid: Some(id),
                title: _,
                on: _,
                begins_on: _,
                ends_on: _,
                picture: _,
            } => Err(EventDecodeError::UnexpectedStructureOfEvent(
                id,
                format!("{:?}", failed_structure),
            )),
            _ => Err(EventDecodeError::EventWithNoId),
        })
        .collect::<Vec<Result<Event, EventDecodeError>>>();
    Ok((events, total_events_fetched))
}

pub enum EventPictureFetchError {
    HttpError(Box<reqwest::Error>),
}

pub async fn fetch_event_picture(
    http_client: &reqwest::Client,
    picture_url: &url::Url,
) -> Result<Bytes, EventPictureFetchError> {
    http_client
        .get(picture_url.to_string())
        .send()
        .await
        .map_err(|e| EventPictureFetchError::HttpError(Box::new(e)))?
        .bytes()
        .await
        .map_err(|e| EventPictureFetchError::HttpError(Box::new(e)))
}
