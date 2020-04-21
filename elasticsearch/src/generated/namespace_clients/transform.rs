// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Delete Transform API"]
pub enum TransformDeleteTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformDeleteTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Delete Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformDeleteTransformParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(12usize + transform_id.len());
                p.push_str("/_transform/");
                p.push_str(transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Delete Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/delete-transform.html)\n\nDeletes an existing transform."]
pub struct TransformDeleteTransform<'a, 'b> {
    client: &'a Elasticsearch,
    parts: TransformDeleteTransformParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformDeleteTransform<'a, 'b> {
    #[doc = "Creates a new instance of [TransformDeleteTransform] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TransformDeleteTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformDeleteTransform {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "When `true`, the transform is deleted regardless of its current state. The default value is `false`, meaning that the transform must be `stopped` before it can be deleted."]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Delete Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Get Transform API"]
pub enum TransformGetTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> TransformGetTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Get Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformGetTransformParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(12usize + transform_id.len());
                p.push_str("/_transform/");
                p.push_str(transform_id.as_ref());
                p.into()
            }
            TransformGetTransformParts::None => "/_transform".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Get Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-transform.html)\n\nRetrieves configuration information for transforms."]
pub struct TransformGetTransform<'a, 'b> {
    client: &'a Elasticsearch,
    parts: TransformGetTransformParts<'b>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformGetTransform<'a, 'b> {
    #[doc = "Creates a new instance of [TransformGetTransform] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TransformGetTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformGetTransform {
            client,
            parts,
            headers,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no transforms. (This includes `_all` string or when no transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of transform configs, defaults to 0"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of transforms to get, defaults to 100"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Get Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Get Transform Stats API"]
pub enum TransformGetTransformStatsParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformGetTransformStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Get Transform Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformGetTransformStatsParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(19usize + transform_id.len());
                p.push_str("/_transform/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Get Transform Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-transform-stats.html)\n\nRetrieves usage information for transforms."]
pub struct TransformGetTransformStats<'a, 'b> {
    client: &'a Elasticsearch,
    parts: TransformGetTransformStatsParts<'b>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformGetTransformStats<'a, 'b> {
    #[doc = "Creates a new instance of [TransformGetTransformStats] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TransformGetTransformStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformGetTransformStats {
            client,
            parts,
            headers,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no transforms. (This includes `_all` string or when no transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of transform stats, defaults to 0"]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of transform stats to get, defaults to 100"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Get Transform Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i64>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Preview Transform API"]
pub enum TransformPreviewTransformParts {
    #[doc = "No parts"]
    None,
}
impl TransformPreviewTransformParts {
    #[doc = "Builds a relative URL path to the Transform Preview Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformPreviewTransformParts::None => "/_transform/_preview".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Preview Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/preview-transform.html)\n\nPreviews a transform."]
pub struct TransformPreviewTransform<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: TransformPreviewTransformParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformPreviewTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformPreviewTransform]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        TransformPreviewTransform {
            client,
            parts: TransformPreviewTransformParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformPreviewTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformPreviewTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Preview Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Put Transform API"]
pub enum TransformPutTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformPutTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Put Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformPutTransformParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(12usize + transform_id.len());
                p.push_str("/_transform/");
                p.push_str(transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Put Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/put-transform.html)\n\nInstantiates a transform."]
pub struct TransformPutTransform<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: TransformPutTransformParts<'b>,
    body: Option<B>,
    defer_validation: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformPutTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformPutTransform] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TransformPutTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformPutTransform {
            client,
            parts,
            headers,
            body: None,
            defer_validation: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformPutTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformPutTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            defer_validation: self.defer_validation,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "If validations should be deferred until transform starts, defaults to false."]
    pub fn defer_validation(mut self, defer_validation: bool) -> Self {
        self.defer_validation = Some(defer_validation);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Put Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "defer_validation")]
                defer_validation: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                defer_validation: self.defer_validation,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Start Transform API"]
pub enum TransformStartTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformStartTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Start Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformStartTransformParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(19usize + transform_id.len());
                p.push_str("/_transform/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Start Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/start-transform.html)\n\nStarts one or more transforms."]
pub struct TransformStartTransform<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: TransformStartTransformParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> TransformStartTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformStartTransform] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TransformStartTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformStartTransform {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformStartTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformStartTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait for the transform to start"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Start Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Stop Transform API"]
pub enum TransformStopTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformStopTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Stop Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformStopTransformParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(18usize + transform_id.len());
                p.push_str("/_transform/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Stop Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/stop-transform.html)\n\nStops one or more transforms."]
pub struct TransformStopTransform<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: TransformStopTransformParts<'b>,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_checkpoint: Option<bool>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> TransformStopTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformStopTransform] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TransformStopTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformStopTransform {
            client,
            parts,
            headers,
            allow_no_match: None,
            body: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_checkpoint: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no transforms. (This includes `_all` string or when no transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformStopTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformStopTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_match: self.allow_no_match,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            force: self.force,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_checkpoint: self.wait_for_checkpoint,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Whether to force stop a failed transform or not. Default to false"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until the transform has stopped. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to wait for the transform to reach a checkpoint before stopping. Default to false"]
    pub fn wait_for_checkpoint(mut self, wait_for_checkpoint: bool) -> Self {
        self.wait_for_checkpoint = Some(wait_for_checkpoint);
        self
    }
    #[doc = "Whether to wait for the transform to fully stop before returning or not. Default to false"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Stop Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_checkpoint")]
                wait_for_checkpoint: Option<bool>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_checkpoint: self.wait_for_checkpoint,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Update Transform API"]
pub enum TransformUpdateTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformUpdateTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Update Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformUpdateTransformParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(20usize + transform_id.len());
                p.push_str("/_transform/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Transform Update Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/update-transform.html)\n\nUpdates certain properties of a transform."]
pub struct TransformUpdateTransform<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: TransformUpdateTransformParts<'b>,
    body: Option<B>,
    defer_validation: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformUpdateTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformUpdateTransform] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TransformUpdateTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformUpdateTransform {
            client,
            parts,
            headers,
            body: None,
            defer_validation: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformUpdateTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformUpdateTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            defer_validation: self.defer_validation,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "If validations should be deferred until transform starts, defaults to false."]
    pub fn defer_validation(mut self, defer_validation: bool) -> Self {
        self.defer_validation = Some(defer_validation);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Update Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "defer_validation")]
                defer_validation: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                defer_validation: self.defer_validation,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Transform APIs"]
pub struct Transform<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Transform<'a> {
    #[doc = "Creates a new instance of [Transform]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Transform Delete Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/delete-transform.html)\n\nDeletes an existing transform."]
    pub fn delete_transform<'b>(
        &'a self,
        parts: TransformDeleteTransformParts<'b>,
    ) -> TransformDeleteTransform<'a, 'b> {
        TransformDeleteTransform::new(&self.client, parts)
    }
    #[doc = "[Transform Get Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-transform.html)\n\nRetrieves configuration information for transforms."]
    pub fn get_transform<'b>(
        &'a self,
        parts: TransformGetTransformParts<'b>,
    ) -> TransformGetTransform<'a, 'b> {
        TransformGetTransform::new(&self.client, parts)
    }
    #[doc = "[Transform Get Transform Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-transform-stats.html)\n\nRetrieves usage information for transforms."]
    pub fn get_transform_stats<'b>(
        &'a self,
        parts: TransformGetTransformStatsParts<'b>,
    ) -> TransformGetTransformStats<'a, 'b> {
        TransformGetTransformStats::new(&self.client, parts)
    }
    #[doc = "[Transform Preview Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/preview-transform.html)\n\nPreviews a transform."]
    pub fn preview_transform<'b>(&'a self) -> TransformPreviewTransform<'a, 'b, ()> {
        TransformPreviewTransform::new(&self.client)
    }
    #[doc = "[Transform Put Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/put-transform.html)\n\nInstantiates a transform."]
    pub fn put_transform<'b>(
        &'a self,
        parts: TransformPutTransformParts<'b>,
    ) -> TransformPutTransform<'a, 'b, ()> {
        TransformPutTransform::new(&self.client, parts)
    }
    #[doc = "[Transform Start Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/start-transform.html)\n\nStarts one or more transforms."]
    pub fn start_transform<'b>(
        &'a self,
        parts: TransformStartTransformParts<'b>,
    ) -> TransformStartTransform<'a, 'b, ()> {
        TransformStartTransform::new(&self.client, parts)
    }
    #[doc = "[Transform Stop Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/stop-transform.html)\n\nStops one or more transforms."]
    pub fn stop_transform<'b>(
        &'a self,
        parts: TransformStopTransformParts<'b>,
    ) -> TransformStopTransform<'a, 'b, ()> {
        TransformStopTransform::new(&self.client, parts)
    }
    #[doc = "[Transform Update Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/update-transform.html)\n\nUpdates certain properties of a transform."]
    pub fn update_transform<'b>(
        &'a self,
        parts: TransformUpdateTransformParts<'b>,
    ) -> TransformUpdateTransform<'a, 'b, ()> {
        TransformUpdateTransform::new(&self.client, parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Transform APIs"]
    pub fn transform(&self) -> Transform {
        Transform::new(&self)
    }
}