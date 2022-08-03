// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: crate::auth::Credential,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: crate::auth::Credential,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = "https://app.vssps.visualstudio.com";
impl ClientBuilder {
    pub fn new(credential: crate::auth::Credential) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn credential(&self) -> &crate::auth::Credential {
        &self.credential
    }
    pub(crate) async fn send(
        &self,
        request: impl Into<azure_core::Request>,
    ) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: crate::auth::Credential,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn profiles(&self) -> profiles::Client {
        profiles::Client(self.clone())
    }
}
pub mod profiles {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a user profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The ID of the target user profile within the same organization, or 'me' to get the profile of the current authenticated user."]
        pub fn get(&self, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                id: id.into(),
                details: None,
                with_attributes: None,
                partition: None,
                core_attributes: None,
                force_refresh: None,
            }
        }
    }
    pub mod get {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::Profile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) details: Option<bool>,
            pub(crate) with_attributes: Option<bool>,
            pub(crate) partition: Option<String>,
            pub(crate) core_attributes: Option<String>,
            pub(crate) force_refresh: Option<bool>,
        }
        impl Builder {
            pub fn details(mut self, details: bool) -> Self {
                self.details = Some(details);
                self
            }
            pub fn with_attributes(mut self, with_attributes: bool) -> Self {
                self.with_attributes = Some(with_attributes);
                self
            }
            pub fn partition(mut self, partition: impl Into<String>) -> Self {
                self.partition = Some(partition.into());
                self
            }
            pub fn core_attributes(mut self, core_attributes: impl Into<String>) -> Self {
                self.core_attributes = Some(core_attributes.into());
                self
            }
            pub fn force_refresh(mut self, force_refresh: bool) -> Self {
                self.force_refresh = Some(force_refresh);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/_apis/profile/profiles/{}",
                            this.client.endpoint(),
                            &this.id
                        );
                        let mut url = url::Url::parse(url_str)
                            .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        req_builder = req_builder.header(
                            http::header::AUTHORIZATION,
                            &this
                                .client
                                .credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        url.query_pairs_mut()
                            .append_pair("api-version", "7.1-preview");
                        if let Some(details) = &this.details {
                            url.query_pairs_mut()
                                .append_pair("details", &details.to_string());
                        }
                        if let Some(with_attributes) = &this.with_attributes {
                            url.query_pairs_mut()
                                .append_pair("withAttributes", &with_attributes.to_string());
                        }
                        if let Some(partition) = &this.partition {
                            url.query_pairs_mut().append_pair("partition", partition);
                        }
                        if let Some(core_attributes) = &this.core_attributes {
                            url.query_pairs_mut()
                                .append_pair("coreAttributes", core_attributes);
                        }
                        if let Some(force_refresh) = &this.force_refresh {
                            url.query_pairs_mut()
                                .append_pair("forceRefresh", &force_refresh.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder
                            .body(req_body)
                            .context(azure_core::error::ErrorKind::Other, "build request")?;
                        let rsp = this
                            .client
                            .send(req)
                            .await
                            .context(azure_core::error::ErrorKind::Io, "execute request")?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body =
                                    azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code.as_u16(),
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
