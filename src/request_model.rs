use serde_json::json;
use crate::model;
use crate::model::*;
use crate::BlendClient;
pub struct GetCurrentUserRequest<'a> {
    pub(crate) client: &'a BlendClient,
}
impl<'a> GetCurrentUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CurrentUser> {
        let mut r = self.client.client.get("/authentication-status");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
