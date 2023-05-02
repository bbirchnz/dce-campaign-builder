pub trait ValidateSelf {
    fn validate_self(&self) -> Result<(), anyhow::Error>;
}


