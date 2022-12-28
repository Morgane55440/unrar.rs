use native;
use std::result::Result;
use num::FromPrimitive;
use std::fmt;
use std::error;
use std::ffi;

enum_from_primitive! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    #[repr(i32)]
    pub enum Code {
        Success = native::ERAR_SUCCESS,
        EndArchive = native::ERAR_END_ARCHIVE,
        NoMemory = native::ERAR_NO_MEMORY,
        BadData = native::ERAR_BAD_DATA,
        BadArchive = native::ERAR_BAD_ARCHIVE,
        UnknownFormat = native::ERAR_UNKNOWN_FORMAT,
        EOpen = native::ERAR_EOPEN,
        ECreate = native::ERAR_ECREATE,
        EClose = native::ERAR_ECLOSE,
        ERead = native::ERAR_EREAD,
        EWrite = native::ERAR_EWRITE,
        SmallBuf = native::ERAR_SMALL_BUF,
        Unknown = native::ERAR_UNKNOWN,
        MissingPassword = native::ERAR_MISSING_PASSWORD,
        // From the UnRARDLL docs:
        // When attempting to unpack a reference record (see RAR -oi switch),
        // source file for this reference was not found.
        // Entire archive needs to be unpacked to properly create file references.
        // This error is returned when attempting to unpack the reference
        // record without its source file.
        EReference = native::ERAR_EREFERENCE,
        BadPassword = native::ERAR_BAD_PASSWORD,

        // our own codes here
        ENul = 0x10000,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum When {
    Open,
    Read,
    Process,
}

impl Code {
    pub fn from(code: i32) -> Option<Self> {
        Code::from_i32(code)
    }
}

#[derive(PartialEq)]
pub struct UnrarError {
    pub code: Code,
    pub when: When,
}

impl std::error::Error for UnrarError {}

impl fmt::Debug for UnrarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}@{:?}", self.code, self.when)?;
        write!(f, " ({})", self)
    }
}

impl fmt::Display for UnrarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Code::*;
        use self::When::*;
        match (self.code, self.when) {
            (BadData, Open) => write!(f, "Archive header damaged"),
            (BadData, Read) => write!(f, "File header damaged"),
            (BadData, Process) => write!(f, "File CRC error"),
            (UnknownFormat, Open) => write!(f, "Unknown encryption"),
            (EOpen, Process) => write!(f, "Could not open next volume"),
            (UnknownFormat, _) => write!(f, "Unknown archive format"),
            (EOpen, _) => write!(f, "Could not open archive"),
            (NoMemory, _) => write!(f, "Not enough memory"),
            (BadArchive, _) => write!(f, "Not a RAR archive"),
            (ECreate, _) => write!(f, "Could not create file"),
            (EClose, _) => write!(f, "Could not close file"),
            (ERead, _) => write!(f, "Read error"),
            (EWrite, _) => write!(f, "Write error"),
            (SmallBuf, _) => write!(f, "Archive comment was truncated to fit to buffer"),
            (MissingPassword, _) => write!(f, "Password for encrypted archive not specified"),
            (EReference, _) => write!(f, "Cannot open file source for reference record"),
            (BadPassword, _) => write!(f, "Wrong password was specified"),
            (Unknown, _) => write!(f, "Unknown error"),
            (EndArchive, _) => write!(f, "Archive end"),
            (Success, _) => write!(f, "Success"),
            (ENul, _) => write!(f, "Nul error (nul found in String)"),
        }
    }
}

impl UnrarError {
    pub fn from(code: Code, when: When) -> Self {
        UnrarError {
            code: code,
            when: when,
        }
    }
}

pub type UnrarResult<T> = Result<T, UnrarError>;

#[derive(Debug)]
pub struct NulError(usize);

impl fmt::Display for NulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nul value found at position: {}", self.0)
    }
}

impl error::Error for NulError {
    fn description(&self) -> &str {
        "nul value found"
    }
}

impl<C: widestring::UChar> From<widestring::NulError<C>> for NulError {
    fn from(e: widestring::NulError<C>) -> NulError {
        NulError(e.nul_position())
    }
}

impl From<ffi::NulError> for NulError {
    fn from(e: ffi::NulError) -> NulError {
        NulError(e.nul_position())
    }
}
