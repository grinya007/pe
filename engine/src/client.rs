use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize, Serialize as SerializeMacro,
};

#[derive(Debug, Error)]
pub enum Error {
    /// An attempt to modify a frozen account
    ClientLocked,
    /// An attempt to withdraw more than there is `available`
    WithdrawInsufficientFunds,
    /// An attempt to dispute more than there is `available`
    DisputeInsufficientFunds,
}

#[allow(clippy::module_name_repetitions)]
pub struct ClientCSV {
    pub id: u16,
    available: f32,
    held: f32,
    locked: bool,
}

impl Serialize for ClientCSV {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ClientCSV", 5)?;
        state.serialize_field("client", &self.id)?;
        state.serialize_field("available", &format!("{:.4}", self.available))?;
        state.serialize_field("held", &format!("{:.4}", self.held))?;
        state.serialize_field("total", &format!("{:.4}", (self.available + self.held)))?;
        state.serialize_field("locked", &self.locked)?;
        state.end()
    }
}

impl From<&Client> for ClientCSV {
    fn from(client: &Client) -> Self {
        Self {
            id: client.id,
            available: client.available,
            held: client.held,
            locked: client.locked,
        }
    }
}

#[derive(Clone, Debug, SerializeMacro, Deserialize)]
pub struct Client {
    id: u16,
    available: f32,
    held: f32,
    locked: bool,
}

impl Client {
    #[must_use]
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            locked: false,
        }
    }

    #[must_use]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// # Errors
    pub fn deposit(&mut self, amount: f32) -> Result<(), Error> {
        if self.locked {
            Err(Error::ClientLocked)
        } else {
            self.available += amount;
            Ok(())
        }
    }

    /// # Errors
    pub fn withdraw(&mut self, amount: f32) -> Result<(), Error> {
        if self.locked {
            Err(Error::ClientLocked)
        } else if amount > self.available {
            Err(Error::WithdrawInsufficientFunds)
        } else {
            self.available -= amount;
            Ok(())
        }
    }

    /// # Errors
    pub fn dispute(&mut self, amount: f32) -> Result<(), Error> {
        if self.locked {
            Err(Error::ClientLocked)
        } else if amount > self.available {
            Err(Error::DisputeInsufficientFunds)
        } else {
            self.available -= amount;
            self.held += amount;
            Ok(())
        }
    }

    /// # Errors
    pub fn resolve(&mut self, amount: f32) -> Result<(), Error> {
        if self.locked {
            Err(Error::ClientLocked)
        } else {
            self.available += amount;
            self.held -= amount;
            Ok(())
        }
    }

    /// # Errors
    pub fn chargeback(&mut self, amount: f32) -> Result<(), Error> {
        if self.locked {
            Err(Error::ClientLocked)
        } else {
            self.locked = true;
            self.held -= amount;
            Ok(())
        }
    }
}
