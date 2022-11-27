use uuid::Uuid;

use crate::error::{FailedResponseError, Result};
use crate::money::Money;
use crate::types::PaymentParamsReferences;
use crate::AekosiaAPI;

impl AekosiaAPI {
    pub async fn get_balance(&self, id: &Uuid) -> Result<Money> {
        let resp = self
            .client
            .get(self.eco_balance.clone() + &id.as_simple().to_string())
            .send()
            .await?
            .verify_success()
            .await?;

        Ok(Money(resp.text().await?.parse()?))
    }

    pub async fn print_money(&self, id: &Uuid, amount: &Money) -> Result<()> {
        self
            .client
            .post(format!("{}{}/{}", self.eco_print, id.as_simple(), *amount))
            .send()
            .await?
            .verify_success()
            .await?;

        Ok(())
    }

    pub async fn payment(&self, from: &Uuid, to: &Uuid, amount: &Money) -> Result<()> {
        let params = PaymentParamsReferences {
            to,
            from,
            amount: &amount.0,
            force: None,
        };

        self.client
            .post(&self.eco_payment)
            .query(&params)
            .send()
            .await?
            .verify_success()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre::Result;
    use tokio;
    use uuid::uuid;

    #[tokio::test]
    #[ignore = "Need a running server with the right conditions to run these!"]
    async fn get_balance() -> Result<()> {
        let a = AekosiaAPI::new_test()
            .get_balance(&uuid!("01844ffb50ee7275af11e47e51bc92e7"))
            .await?;

        assert_eq!(Money(0), a);

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Need a running server with the right conditions to run these!"]
    async fn print_money() -> Result<()> {
        AekosiaAPI::new_test()
            .print_money(&uuid!("01844ffb50ee7275af11e47e51bc92e7"), &Money(10))
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Need a running server with the right conditions to run these!"]
    async fn payment() -> Result<()> {
        let client = AekosiaAPI::new_test();

        let before_from = client
            .get_balance(&uuid!("01844a37-ec48-7579-86bd-0cdb39f6cd24"))
            .await?;
        let before_to = client
            .get_balance(&uuid!("01844b8a-0108-76e7-8bd6-049df41106cd"))
            .await?;

        client
            .payment(
                &uuid!("01844a37-ec48-7579-86bd-0cdb39f6cd24"),
                &uuid!("01844b8a-0108-76e7-8bd6-049df41106cd"),
                &Money(10),
            )
            .await?;

        let after_from = client
            .get_balance(&uuid!("01844a37-ec48-7579-86bd-0cdb39f6cd24"))
            .await?;
        let after_to = client
            .get_balance(&uuid!("01844b8a-0108-76e7-8bd6-049df41106cd"))
            .await?;

        assert_eq!(before_from - 10, after_from);
        assert_eq!(before_to + 10, after_to);

        Ok(())
    }
}
