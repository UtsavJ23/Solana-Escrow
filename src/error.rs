use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError{
    #[error("InvalidInstructions")]
    InvalidInstructions,
}
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}