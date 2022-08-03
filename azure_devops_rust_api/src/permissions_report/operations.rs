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
pub const DEFAULT_ENDPOINT: &str = "https://dev.azure.com";
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
    pub fn permissions_report(&self) -> permissions_report::Client {
        permissions_report::Client(self.clone())
    }
    pub fn permissions_report_download(&self) -> permissions_report_download::Client {
        permissions_report_download::Client(self.clone())
    }
}
pub mod permissions_report {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of permissions reports"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Request a permissions report to be created asyncronously"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Request configuration to be included in the permissions report"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PermissionsReportRequest>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
        #[doc = "Get a specific permissions report"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: The ID (GUID) of the permissions report"]
        pub fn get(&self, organization: impl Into<String>, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id: id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::PermissionsReportList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/permissionsreport",
                            this.client.endpoint(),
                            &this.organization
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
                                let rsp_value: models::PermissionsReportList =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod create {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::ReferenceLinks;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PermissionsReportRequest,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/permissionsreport",
                            this.client.endpoint(),
                            &this.organization
                        );
                        let mut url = url::Url::parse(url_str)
                            .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
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
                        req_builder = req_builder.header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
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
                                let rsp_value: models::ReferenceLinks =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod get {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::PermissionsReport;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/permissionsreport/{}",
                            this.client.endpoint(),
                            &this.organization,
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
                                let rsp_value: models::PermissionsReport =
                                    serde_json::from_slice(&rsp_body)?;
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
pub mod permissions_report_download {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Download the json results of a permissions report"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: The ID (GUID) of the permissions report"]
        pub fn download(
            &self,
            organization: impl Into<String>,
            id: impl Into<String>,
        ) -> download::Builder {
            download::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id: id.into(),
            }
        }
    }
    pub mod download {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/permissionsreport/{}/download",
                            this.client.endpoint(),
                            &this.organization,
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
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
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
