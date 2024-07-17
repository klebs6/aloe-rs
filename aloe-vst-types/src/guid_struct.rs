crate::ix!();

#[cfg(COM_COMPATIBLE)]
#[cfg(SMTG_OS_WINDOWS)]
pub type GuidStruct = GUID;

#[cfg(COM_COMPATIBLE)]
#[cfg(not(SMTG_OS_WINDOWS))]
pub struct GuidStruct
{
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}
