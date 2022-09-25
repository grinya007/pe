use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum Error {
    DisputeWithdrawal,
    ResolveNonDisputed,
    ChargeBackNonDisputed,
    AlreadyInDispute,
    AlreadyChargedBack,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Deposit,
    Withdrawal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    id: u32,
    client_id: u16,
    amount: f32,
    action: Action,
    in_dispute: bool,
    charged_back: bool,
}

impl Transaction {
    #[must_use]
    pub fn new(id: u32, client_id: u16, amount: f32, action: Action) -> Self {
        Self {
            id,
            client_id,
            amount,
            action,
            in_dispute: false,
            charged_back: false,
        }
    }

    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    #[must_use]
    pub fn amount(&self) -> f32 {
        self.amount
    }

    #[must_use]
    pub fn action(&self) -> Action {
        self.action.clone()
    }

    /// # Errors
    pub fn dispute(&mut self) -> Result<(), Error> {
        if let Action::Withdrawal = self.action {
            Err(Error::DisputeWithdrawal)
        } else if self.in_dispute {
            Err(Error::AlreadyInDispute)
        } else if self.charged_back {
            Err(Error::AlreadyChargedBack)
        } else {
            self.in_dispute = true;
            Ok(())
        }
    }

    /// # Errors
    pub fn resolve(&mut self) -> Result<(), Error> {
        if self.in_dispute {
            self.in_dispute = false;
            Ok(())
        } else {
            Err(Error::ResolveNonDisputed)
        }
    }

    /// # Errors
    pub fn chargeback(&mut self) -> Result<(), Error> {
        if self.in_dispute {
            self.in_dispute = false;
            self.charged_back = true;
            Ok(())
        } else {
            Err(Error::ChargeBackNonDisputed)
        }
    }
}
