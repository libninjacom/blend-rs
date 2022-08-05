//! [`BlendClient`](struct.BlendClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
pub mod model;
pub mod request_model;
use crate::model::*;

pub struct BlendClient {
    pub(crate) client: httpclient::Client,
    authentication: Option<BlendAuthentication>,
}
impl BlendClient {}
impl BlendClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        let authentication = None;
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: BlendAuthentication) -> Self {
        self.authentication = Some(authentication);
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        if let Some(ref authentication) = self.authentication {
            match authentication {
                BlendAuthentication::BasicAuth { basic_auth, target_instance } => {
                    r = r.basic_auth(basic_auth);
                    r = r.header("blend-target-instance", target_instance);
                }
                BlendAuthentication::BearerAuth { bearer_auth, target_instance } => {
                    r = r.bearer_auth(bearer_auth);
                    r = r.header("blend-target-instance", target_instance);
                }
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///Get Current User
    pub fn get_current_user(&self) -> request_model::GetCurrentUserRequest {
        request_model::GetCurrentUserRequest {
            client: &self,
        }
    }
}
pub enum BlendAuthentication {
    BasicAuth { basic_auth: String, target_instance: String },
    BearerAuth { bearer_auth: String, target_instance: String },
}
impl BlendAuthentication {
    pub fn from_env() -> Self {
        Self::BasicAuth {
            basic_auth: std::env::var("BLEND_BASIC_AUTH")
                .expect("Environment variable BLEND_BASIC_AUTH is not set."),
            target_instance: std::env::var("BLEND_TARGET_INSTANCE")
                .expect("Environment variable BLEND_TARGET_INSTANCE is not set."),
        }
    }
}
