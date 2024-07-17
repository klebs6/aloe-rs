crate::ix!();

/// byte (or other) sizes
pub type TSize   = i64;

/// result code
pub type tresult = i32;


#[cfg(SMTG_PLATFORM_64)]
pub type TPtrInt = u64;

#[cfg(not(SMTG_PLATFORM_64))] 
pub type TPtrInt = u32;

pub type TBool = u8;

#[cfg(UNICODE)]      pub type tchar = u16;
#[cfg(not(UNICODE))] pub type tchar = u8;

pub type CStringA = *const u8;
pub type CStringW = *const u16;
pub type CString  = *const u16;

pub type FIDString = &'static str;// identifier as string (used for attributes, messages)
