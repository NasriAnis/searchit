use crate::workers::PoolError;
use pdf_extract::OutputError;

#[derive(Debug)]
pub enum ScanError {
    CommandError(String),
    Io(std::io::Error),
    PdfExtract(pdf_extract::OutputError),
    LockPoisoned,
}

impl From<std::io::Error> for ScanError {
    fn from(e: std::io::Error) -> Self {
        ScanError::Io(e)
    }
}
impl From<OutputError> for ScanError {
    fn from(e: OutputError) -> Self {
        ScanError::PdfExtract(e)
    }
}
impl From<PoolError> for ScanError {
    fn from(e: PoolError) -> Self {
        match e {
            PoolError::LockPoisoned => ScanError::LockPoisoned,
        }
    }
}
impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::CommandError(e) => write!(f, "Command Error: {}", e),
            ScanError::Io(e) => write!(f, "IO error: {}", e),
            ScanError::PdfExtract(e) => write!(f, "PDF extraction error: {}", e),
            ScanError::LockPoisoned => write!(f, "lock poisoned"),
        }
    }
}