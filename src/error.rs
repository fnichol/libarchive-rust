use archive;

pub type ArchiveResult<T> = Result<T, ArchiveError>;

#[derive(Debug)]
pub struct ErrCode(i32);

#[derive(Debug)]
pub enum ArchiveError {
    Consumed,
    Sys(ErrCode, String),
    ReadFailure,
    WriteFailure,
    HeaderPosition,
}

impl<'a> From<&'a archive::Handle> for ArchiveError {
    fn from(handle: &'a archive::Handle) -> ArchiveError {
        ArchiveError::Sys(ErrCode::from(handle), handle.err_msg())
    }
}

impl<'a> From<&'a archive::Handle> for ErrCode {
    fn from(handle: &'a archive::Handle) -> ErrCode {
        ErrCode(handle.err_code())
    }
}

impl<'a> From<&'a archive::Handle> for ArchiveResult<()> {
    fn from(handle: &'a archive::Handle) -> ArchiveResult<()> {
        match handle.err_code() {
            0 => Ok(()),
            _ => Err(ArchiveError::from(handle)),
        }
    }
}
