crate::ix!();

/* ----------------- Result Codes  ----------------- */

#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const NoInterface:     usize = 0x80004002;    // E_NOINTERFACE
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const ResultOk:        usize = 0x00000000;    // S_OK
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const ResultTrue:      usize = ResultOk;
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const ResultFalse:     usize = 0x00000001;    // S_FALSE
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const InvalidArgument: usize = 0x80070057;    // E_INVALIDARG
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const NotImplemented:  usize = 0x80004001;    // E_NOTIMPL
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const InternalError:   usize = 0x80004005;    // E_FAIL
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const NotInitialized:  usize = 0x8000FFFF;    // E_UNEXPECTED
#[cfg(COM_COMPATIBLE)] #[cfg(SMTG_OS_WINDOWS)]      pub const OutOfMemory:     usize = 0x8007000E;     // E_OUTOFMEMORY

#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const NoInterface:     usize = 0x80000004;    // E_NOINTERFACE
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const ResultOk:        usize = 0x00000000;    // S_OK
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const ResultTrue:      usize = ResultOk;
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const ResultFalse:     usize = 0x00000001;    // S_FALSE
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const InvalidArgument: usize = 0x80000003;    // E_INVALIDARG
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const NotImplemented:  usize = 0x80000001;    // E_NOTIMPL
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const InternalError:   usize = 0x80000008;    // E_FAIL
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const NotInitialized:  usize = 0x8000FFFF;    // E_UNEXPECTED
#[cfg(COM_COMPATIBLE)] #[cfg(not(SMTG_OS_WINDOWS))] pub const OutOfMemory:     usize = 0x80000002;     // E_OUTOFMEMORY

#[cfg(not(COM_COMPATIBLE))] pub const NoInterface:     isize = -1;
#[cfg(not(COM_COMPATIBLE))] pub const ResultOk:        usize = 0;
#[cfg(not(COM_COMPATIBLE))] pub const ResultTrue:      usize = ResultOk;
#[cfg(not(COM_COMPATIBLE))] pub const ResultFalse:     usize = 1;
#[cfg(not(COM_COMPATIBLE))] pub const InvalidArgument: usize = 2;
#[cfg(not(COM_COMPATIBLE))] pub const NotImplemented:  usize = 3;
#[cfg(not(COM_COMPATIBLE))] pub const InternalError:   usize = 4;
#[cfg(not(COM_COMPATIBLE))] pub const NotInitialized:  usize = 5;
#[cfg(not(COM_COMPATIBLE))] pub const OutOfMemory:     usize = 6;
