crate::ix!();

/**
  | kAudioChannelLayoutTag_HOA_ACN_SN3D
  |
  */
pub const coreAudioHOASN3DLayoutTag: usize = (190 << 16) | 0;

pub struct CoreAudioLayoutsLayoutTagSpeakerList
{
    tag:           AudioChannelLayoutTag,
    channel_types: [AudioChannelSetChannelType; 16],
}
