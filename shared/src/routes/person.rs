use std::str::FromStr;

use uuid::Uuid;

use crate::error::{FailedResponseError, Result};
use crate::types::PersonInfo;
use crate::AekosiaAPI;

impl AekosiaAPI {
    pub async fn get_person(&self, id: &Uuid) -> Result<PersonInfo> {
        let resp = self
            .client
            .get(self.person_get.clone() + &id.as_simple().to_string())
            .send()
            .await?
            .verify_success()
            .await?;

        Ok(resp.json().await?)
    }

    pub async fn get_id_from_discord(&self, id: &u64) -> Result<Uuid> {
        let resp = self
            .client
            .get(self.person_get_discord.clone() + &id.to_string())
            .send()
            .await?
            .verify_success()
            .await?;

        Ok(Uuid::from_str(&resp.text().await?)?)
    }

    pub async fn register_person(&self, discord_id: &u64) -> Result<Uuid> {
        let res = self
            .client
            .post(self.person_register.clone() + &discord_id.to_string())
            .send()
            .await?
            .verify_success()
            .await?;

        Ok(Uuid::parse_str(&res.text().await?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use color_eyre::eyre::Result;
    use reqwest::StatusCode;
    use tokio;
    use uuid::uuid;

    #[tokio::test]
    #[ignore = "Need a running server with the right conditions to run these!"]
    async fn get_person() -> Result<()> {
        let a = AekosiaAPI::new_test()
            .get_person(&uuid!("01844ffb50ee7275af11e47e51bc92e7"))
            .await?;

        assert_eq!(
            PersonInfo {
                discord_id: 153_555_060_926_840_833,
                balance: 0
            },
            a
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Need a running server with the right conditions to run these!"]
    async fn register_person() -> Result<()> {
        _ = AekosiaAPI::new_test().register_person(&222).await?;

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Need a running server with the right conditions to run these!"]
    async fn register_404() -> Result<()> {
        _ = AekosiaAPI::new_test().register_person(&1000).await?;
        let err = AekosiaAPI::new_test().register_person(&900).await;

        if let Err(Error::FailedResponse { code, message }) = err {
            assert_eq!(code, StatusCode::from_u16(400)?);
            assert_eq!(message, "This person is already registered!");
        } else {
            err?;
            panic!("Didn't catch 404!")
        }
        
        Ok(())
    }
}
