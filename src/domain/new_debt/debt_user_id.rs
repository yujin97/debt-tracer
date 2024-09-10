use uuid::Uuid;

#[derive(Debug)]
pub struct DebtUserId(Uuid);

impl AsRef<Uuid> for DebtUserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl DebtUserId {
    pub fn parse(s: &str) -> Result<Self, String> {
        let uuid = Uuid::parse_str(s).map_err(|_| format!("{} is not valid UUID", s))?;

        Ok(Self(uuid))
    }
}
