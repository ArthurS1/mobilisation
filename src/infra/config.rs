use std::str::FromStr;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use url::Url;

use crate::core::instance_version::*;
use crate::core::category::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/config.graphql",
    response_derives = "Clone"
)]
struct ConfigQuery;

pub enum ConfigFetchError {
    HttpError(Box<reqwest::Error>),
    InstanceVersionParsingError(InstanceVersionParsingError),
    MissingField(String),
}

type Language = String;

#[derive(Default)]
pub struct FetchConfigResponse {
    pub instance_version: InstanceVersion,
    pub categories: Vec<Category>,
    pub languages: Vec<Language>
}

pub async fn fetch_config(
    instance_url: &Url,
    http_client: &reqwest::Client
) -> Result<FetchConfigResponse, ConfigFetchError> {
    let request = ConfigQuery::build_query(config_query::Variables {});
    let response = http_client
        .post(instance_url.to_string())
        .json(&request)
        .send()
        .await
        .map_err(|e| ConfigFetchError::HttpError(Box::new(e)))?
        .json::<Response<config_query::ResponseData>>()
        .await
        .map_err(|e| ConfigFetchError::HttpError(Box::new(e)))?;
    let version: &String = response
        .data
        .as_ref()
        .and_then(|data| data.config.as_ref())
        .and_then(|config| config.version.as_ref())
        .ok_or(ConfigFetchError::MissingField(
            "version.".to_string(),
        ))?;
    let instance_version = InstanceVersion::from_str(version)
        .map_err(|err| ConfigFetchError::InstanceVersionParsingError(err))?;
    let categories = response
        .data
        .as_ref()
        .and_then(|data| data.config.as_ref())
        .and_then(|config| config.event_categories.clone())
        .ok_or(ConfigFetchError::MissingField(
            "categories".to_string(),
        ))?
        .into_iter()
        .collect::<Option<Vec<config_query::ConfigQueryConfigEventCategories>>>()
        .ok_or(ConfigFetchError::MissingField(
            "category.".to_string(),
        ))?
        .into_iter()
        .map(|a| {
            let label = a.label.ok_or(ConfigFetchError::MissingField("label".to_string()))?;
            let id =
                a.id.ok_or(ConfigFetchError::MissingField("id".to_string()))?;

            Ok(Category {
                label: label,
                id: id,
            })
        })
        .collect::<Result<Vec<Category>, ConfigFetchError>>()?;
    let languages = response
        .data
        .as_ref()
        .and_then(|data| data.config.as_ref())
        .and_then(|config| config.languages.clone())
        .map(|e|
            e.into_iter()
            .flatten()
            .collect::<Vec<Language>>()
        )
        .ok_or(ConfigFetchError::MissingField("languages".to_string()))?;
    Ok(FetchConfigResponse {
        instance_version: instance_version,
        categories: categories,
        languages: languages
    })
}
