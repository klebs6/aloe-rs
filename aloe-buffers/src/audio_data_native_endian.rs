crate::ix!();

/** 
 | Used as a template parameter for
 | AudioData::AudioDataPointer. Indicates that the samples
 | are stored in the CPU's native endianness. 
 */
#[cfg(ALOE_BIG_ENDIAN)]
pub type AudioDataNativeEndian = AudioDataBigEndian;

#[cfg(not(ALOE_BIG_ENDIAN))]
pub type AudioDataNativeEndian = AudioDataLittleEndian;
