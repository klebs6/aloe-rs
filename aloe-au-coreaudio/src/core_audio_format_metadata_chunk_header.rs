crate::ix!();

pub struct CoreAudioFormatMetadataChunkHeader
{
    chunk_type: u32,
    chunk_size: i64,
}

impl CoreAudioFormatMetadataChunkHeader {
    
    pub fn new<R: Read>(input: &mut R) -> Self {
    
        todo!();
        /*


            chunkType = (uint32) input.readIntBigEndian();
                chunkSize = (int64)  input.readInt64BigEndian();
        */
    }
}
