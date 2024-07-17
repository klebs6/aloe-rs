crate::ix!();

/**
  | Structure describing a mapping
  |
  */
pub struct SpekaerMapping {
    vst2:     i32,
    channels: [AudioChannelSetChannelType; 13],
}

impl SpekaerMapping {

    pub fn matches(&self, chans: &[AudioChannelSetChannelType]) -> bool {
        
        todo!();
        /*
            auto n = static_cast<int> (sizeof (channels) / sizeof (ChannelType));

                for (int i = 0; i < n; ++i)
                {
                    if (channels[i] == unknown)  return (i == chans.size());
                    if (i == chans.size())       return (channels[i] == unknown);

                    if (channels[i] != chans.getUnchecked (i))
                        return false;
                }

                return true;
        */
    }
}
