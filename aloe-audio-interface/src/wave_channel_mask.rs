crate::ix!();

pub trait FromWaveChannelMask {

    /**
      | Create an dyn AudioChannelSetInterface from a WAVEFORMATEXTENSIBLE
      | channelMask (typically used in .wav
      | files).
      */
    fn from_wave_channel_mask(&mut self, dw_channel_mask: i32) -> dyn AudioChannelSetInterface;
}

pub trait GetWaveChannelMask {

    /** 
      | Returns a WAVEFORMATEXTENSIBLE channelMask
      | representation (typically used in .wav files)
      | of the receiver.
      |
      | Returns -1 if the receiver cannot be
      | represented in a WAVEFORMATEXTENSIBLE
      | channelMask representation.
      */
    fn get_wave_channel_mask(&self) -> i32;
}
