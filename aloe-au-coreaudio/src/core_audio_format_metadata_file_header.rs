crate::ix!();

pub struct CoreAudioFormatMetadataFileHeader
{
    file_type:    u32,
    file_version: u16,
    file_flags:   u16,
}

impl CoreAudioFormatMetadataFileHeader {
    
    pub fn new<R: Read>(input: &mut R) -> Self {
    
        todo!();
        /*


            fileType    = (uint32) input.readIntBigEndian();
                fileVersion = (uint16) input.readShortBigEndian();
                fileFlags   = (uint16) input.readShortBigEndian();
        */
    }
}
