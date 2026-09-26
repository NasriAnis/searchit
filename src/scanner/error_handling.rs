use crate::workers::PoolError;
use pdf_extract::OutputError;

#[derive(Debug)]
pub enum ScanError {
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
