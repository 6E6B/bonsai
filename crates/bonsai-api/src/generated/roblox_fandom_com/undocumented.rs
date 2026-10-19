//! `roblox.fandom.com` undocumented — generated from `specs/undocumented/roblox.fandom.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "roblox.fandom.com";

/// `ActionError`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActionError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}

/// `ParsedCategory`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ParsedCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

/// `Parsed`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Parsed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<ParsedCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displaytitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `SiteGeneral`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SiteGeneral {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainpage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitename: Option<String>,
}

/// `SearchHit`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchHit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `SearchInfo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalhits: Option<i64>,
}

/// `QueryResult`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QueryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<SiteGeneral>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<SearchHit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchinfo: Option<SearchInfo>,
}

/// `ActionResult`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ActionError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Parsed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<QueryResult>,
}

/// Query, header and form inputs for [`action_api`].
#[derive(Debug, Clone, Default)]
pub struct ActionApiParams {
    pub action: String,
    pub format: String,
    pub formatversion: Option<String>,
    pub page: Option<String>,
    pub prop: Option<String>,
    pub redirects: Option<String>,
    pub disableeditsection: Option<String>,
    pub disablelimitreport: Option<String>,
    pub list: Option<String>,
    pub srsearch: Option<String>,
    pub srnamespace: Option<String>,
    pub srlimit: Option<i32>,
    pub srprop: Option<String>,
    pub meta: Option<String>,
    pub siprop: Option<String>,
}

/// `GET /api.php`
///
/// MediaWiki's action API. 'parse' returns an article, 'query' a search.
///
/// parse.text is the article as HTML (with formatversion=2). parse.categories lists its categories, hidden ones (maintenance) flagged. query.search lists matches; each snippet is HTML with the match wrapped in \<span class="searchmatch"\>, though Fandom sends it empty, and searchinfo is absent there. Namespaces (meta=siteinfo, 2026-10-19): 0 main, 112 Catalog (Catalog:Dominus Infernus), 114 Player (Player:Builderman and the experiences players made, Player:Builderman/Some Game). Errors come back as error.code with HTTP 200.
///
/// Undocumented; captured 2026-10-19 https://roblox.fandom.com/api.php?action=parse&page=Catalog%3ADominus_Infernus&prop=text&format=json.
pub fn action_api_request(params: &ActionApiParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/api.php".to_string());
    r.query("action", &params.action);
    r.query("format", &params.format);
    r.query("formatversion", &params.formatversion);
    r.query("page", &params.page);
    r.query("prop", &params.prop);
    r.query("redirects", &params.redirects);
    r.query("disableeditsection", &params.disableeditsection);
    r.query("disablelimitreport", &params.disablelimitreport);
    r.query("list", &params.list);
    r.query("srsearch", &params.srsearch);
    r.query("srnamespace", &params.srnamespace);
    r.query("srlimit", &params.srlimit);
    r.query("srprop", &params.srprop);
    r.query("meta", &params.meta);
    r.query("siprop", &params.siprop);
    Ok(r)
}

/// `GET /api.php`
///
/// MediaWiki's action API. 'parse' returns an article, 'query' a search.
///
/// parse.text is the article as HTML (with formatversion=2). parse.categories lists its categories, hidden ones (maintenance) flagged. query.search lists matches; each snippet is HTML with the match wrapped in \<span class="searchmatch"\>, though Fandom sends it empty, and searchinfo is absent there. Namespaces (meta=siteinfo, 2026-10-19): 0 main, 112 Catalog (Catalog:Dominus Infernus), 114 Player (Player:Builderman and the experiences players made, Player:Builderman/Some Game). Errors come back as error.code with HTTP 200.
///
/// Undocumented; captured 2026-10-19 https://roblox.fandom.com/api.php?action=parse&page=Catalog%3ADominus_Infernus&prop=text&format=json.
pub async fn action_api(client: &Client, params: &ActionApiParams) -> Result<ActionResult> {
    client.send(action_api_request(params)?).await?.json()
}
