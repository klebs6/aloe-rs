crate::ix!();

#[cfg(target_os="android")]
pub struct PCMDataFormatEx {
    base:           SLDataFormat_PCM,
    representation: u32,
}

