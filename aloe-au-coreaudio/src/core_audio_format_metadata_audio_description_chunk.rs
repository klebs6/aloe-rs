crate::ix!();

pub struct CoreAudioFormatMetadataAudioDescriptionChunk
{
    sample_rate:        f64,
    formatid:           u32,
    format_flags:       u32,
    bytes_per_packet:   u32,
    frames_per_packet:  u32,
    channels_per_frame: u32,
    bits_per_channel:   u32,
}

impl CoreAudioFormatMetadataAudioDescriptionChunk {

    pub fn new<R: Read>(input: &mut R) -> Self {

        todo!();
        /*


            sampleRate          = input.readDoubleBigEndian();
                formatID            = (uint32) input.readIntBigEndian();
                formatFlags         = (uint32) input.readIntBigEndian();
                bytesPerPacket      = (uint32) input.readIntBigEndian();
                framesPerPacket     = (uint32) input.readIntBigEndian();
                channelsPerFrame    = (uint32) input.readIntBigEndian();
                bitsPerChannel      = (uint32) input.readIntBigEndian();
        */
    }
}
