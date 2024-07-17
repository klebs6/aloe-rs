crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/native/aloe_mac_CoreAudioLayouts.h]

/**
  | Convert CoreAudio's native AudioChannelLayout
  | to Aloe's AudioChannelSet.
  | 
  | Note that this method cannot preserve
  | the order of channels.
  |
  */
pub fn from_core_audio(x: &AudioChannelLayout) -> AudioChannelSet {

    todo!();
    /*
       return AudioChannelSet::channelSetWithChannels (getCoreAudioLayoutChannels (layout));
       */
}

/**
  | Convert CoreAudio's native AudioChannelLayoutTag
  | to Aloe's AudioChannelSet.
  | 
  | Note that this method cannot preserve
  | the order of channels.
  |
  */
pub fn from_core_audio_tag(layout_tag: AudioChannelLayoutTag) -> AudioChannelSet {
    
    todo!();
    /*
        return AudioChannelSet::channelSetWithChannels (getSpeakerLayoutForCoreAudioTag (layoutTag));
    */
}

/**
  | Convert Aloe's AudioChannelSet to
  | CoreAudio's AudioChannelLayoutTag.
  | 
  | Note that this method cannot preserve
  | the order of channels.
  |
  */
pub fn to_core_audio(set: &AudioChannelSet) -> AudioChannelLayoutTag {
    
    todo!();
    /*
        if (set.getAmbisonicOrder() >= 0)
            return coreAudioHOASN3DLayoutTag | static_cast<unsigned> (set.size());

        for (auto* tbl = CoreAudioLayoutsSpeakerLayoutTable::get(); tbl->tag != 0; ++tbl)
        {
            AudioChannelSet caSet;

            for (int i = 0; i < numElementsInArray (tbl->channelTypes)
                 && tbl->channelTypes[i] != AudioChannelSet::unknown; ++i)
                caSet.addChannel (tbl->channelTypes[i]);

            if (caSet == set)
                return tbl->tag;
        }

        return kAudioChannelLayoutTag_DiscreteInOrder | static_cast<AudioChannelLayoutTag> (set.size());
    */
}

pub fn core_audio_layouts_get_known_core_audio_tags<'a>() -> &'a [AudioChannelLayoutTag] {
    
    todo!();
    /*
        static Vec<AudioChannelLayoutTag> tags (createKnownCoreAudioTags());
        return tags;
    */
}

/**
  | Convert CoreAudio's native AudioChannelLayout
  | to an array of Aloe ChannelTypes.
  |
  */
pub fn core_audio_layouts_get_core_audio_layout_channels(layout: &AudioChannelLayout) -> Vec<AudioChannelSetChannelType> {
    
    todo!();
    /*
        switch (layout.mChannelLayoutTag & 0xffff0000)
        {
            case kAudioChannelLayoutTag_UseChannelBitmap:
                return AudioChannelSet::fromWaveChannelMask (static_cast<int> (layout.mChannelBitmap)).getChannelTypes();
            case kAudioChannelLayoutTag_UseChannelDescriptions:
            {
                Vec<AudioChannelSet::ChannelType> channels;

                for (UInt32 i = 0; i < layout.mNumberChannelDescriptions; ++i)
                    channels.addIfNotAlreadyThere (getChannelTypeFromAudioChannelLabel (layout.mChannelDescriptions[i].mChannelLabel));

                // different speaker mappings may point to the same Aloe speaker so fill up
                // this array with discrete channels
                for (int j = 0; channels.size() < static_cast<int> (layout.mNumberChannelDescriptions); ++j)
                    channels.addIfNotAlreadyThere (static_cast<AudioChannelSet::ChannelType> (AudioChannelSet::discreteChannel0 + j));

                return channels;
            }
            case kAudioChannelLayoutTag_DiscreteInOrder:
                return AudioChannelSet::discreteChannels (static_cast<int> (layout.mChannelLayoutTag) & 0xffff).getChannelTypes();
            default:
                break;
        }

        return getSpeakerLayoutForCoreAudioTag (layout.mChannelLayoutTag);
    */
}

pub fn core_audio_layouts_get_speaker_layout_for_core_audio_tag(tag: AudioChannelLayoutTag) -> Vec<AudioChannelSetChannelType> {
    
    todo!();
    /*
        // You need to specify the full AudioChannelLayout when using
        // the UseChannelBitmap and UseChannelDescriptions layout tag
        jassert (tag != kAudioChannelLayoutTag_UseChannelBitmap && tag != kAudioChannelLayoutTag_UseChannelDescriptions);

        Vec<AudioChannelSet::ChannelType> speakers;

        for (auto* tbl = CoreAudioLayoutsSpeakerLayoutTable::get(); tbl->tag != 0; ++tbl)
        {
            if (tag == tbl->tag)
            {
                for (int i = 0; i < numElementsInArray (tbl->channelTypes)
                                  && tbl->channelTypes[i] != AudioChannelSet::unknown; ++i)
                    speakers.add (tbl->channelTypes[i]);

                return speakers;
            }
        }

        auto numChannels = tag & 0xffff;
        if (tag >= coreAudioHOASN3DLayoutTag && tag <= (coreAudioHOASN3DLayoutTag | 0xffff))
        {
            auto sqrtMinusOne   = std::sqrt (static_cast<float> (numChannels)) - 1.0f;
            auto ambisonicOrder = jmax (0, static_cast<int> (std::floor (sqrtMinusOne)));

            if (static_cast<float> (ambisonicOrder) == sqrtMinusOne)
                return AudioChannelSet::ambisonic (ambisonicOrder).getChannelTypes();
        }

        for (UInt32 i = 0; i < numChannels; ++i)
            speakers.add (static_cast<AudioChannelSet::ChannelType> (AudioChannelSet::discreteChannel0 + i));

        return speakers;
    */
}

pub fn core_audio_layouts_create_known_core_audio_tags() -> Vec<AudioChannelLayoutTag> {
    
    todo!();
    /*
        Vec<AudioChannelLayoutTag> tags;

        for (auto* tbl = CoreAudioLayoutsSpeakerLayoutTable::get(); tbl->tag != 0; ++tbl)
            tags.addIfNotAlreadyThere (tbl->tag);

        for (unsigned order = 0; order <= 5; ++order)
            tags.addIfNotAlreadyThere (coreAudioHOASN3DLayoutTag | ((order + 1) * (order + 1)));

        return tags;
    */
}

pub fn core_audio_layouts_get_channel_type_from_audio_channel_label(label: AudioChannelLabel) -> AudioChannelSetChannelType {
    
    todo!();
    /*
        if (label >= kAudioChannelLabel_Discrete_0 && label <= kAudioChannelLabel_Discrete_65535)
        {
            const unsigned int discreteChannelNum = label - kAudioChannelLabel_Discrete_0;
            return static_cast<AudioChannelSet::ChannelType> (AudioChannelSet::discreteChannel0 + discreteChannelNum);
        }

        switch (label)
        {
            case kAudioChannelLabel_Center:
            case kAudioChannelLabel_Mono:                   return AudioChannelSet::centre;
            case kAudioChannelLabel_Left:
            case kAudioChannelLabel_HeadphonesLeft:         return AudioChannelSet::left;
            case kAudioChannelLabel_Right:
            case kAudioChannelLabel_HeadphonesRight:        return AudioChannelSet::right;
            case kAudioChannelLabel_LFEScreen:              return AudioChannelSet::LFE;
            case kAudioChannelLabel_LeftSurround:           return AudioChannelSet::leftSurround;
            case kAudioChannelLabel_RightSurround:          return AudioChannelSet::rightSurround;
            case kAudioChannelLabel_LeftCenter:             return AudioChannelSet::leftCentre;
            case kAudioChannelLabel_RightCenter:            return AudioChannelSet::rightCentre;
            case kAudioChannelLabel_CenterSurround:         return AudioChannelSet::surround;
            case kAudioChannelLabel_LeftSurroundDirect:     return AudioChannelSet::leftSurroundSide;
            case kAudioChannelLabel_RightSurroundDirect:    return AudioChannelSet::rightSurroundSide;
            case kAudioChannelLabel_TopCenterSurround:      return AudioChannelSet::topMiddle;
            case kAudioChannelLabel_VerticalHeightLeft:     return AudioChannelSet::topFrontLeft;
            case kAudioChannelLabel_VerticalHeightRight:    return AudioChannelSet::topFrontRight;
            case kAudioChannelLabel_VerticalHeightCenter:   return AudioChannelSet::topFrontCentre;
            case kAudioChannelLabel_TopBackLeft:            return AudioChannelSet::topRearLeft;
            case kAudioChannelLabel_RearSurroundLeft:       return AudioChannelSet::leftSurroundRear;
            case kAudioChannelLabel_TopBackRight:           return AudioChannelSet::topRearRight;
            case kAudioChannelLabel_RearSurroundRight:      return AudioChannelSet::rightSurroundRear;
            case kAudioChannelLabel_TopBackCenter:          return AudioChannelSet::topRearCentre;
            case kAudioChannelLabel_LFE2:                   return AudioChannelSet::LFE2;
            case kAudioChannelLabel_LeftWide:               return AudioChannelSet::wideLeft;
            case kAudioChannelLabel_RightWide:              return AudioChannelSet::wideRight;
            case kAudioChannelLabel_Ambisonic_W:            return AudioChannelSet::ambisonicW;
            case kAudioChannelLabel_Ambisonic_X:            return AudioChannelSet::ambisonicX;
            case kAudioChannelLabel_Ambisonic_Y:            return AudioChannelSet::ambisonicY;
            case kAudioChannelLabel_Ambisonic_Z:            return AudioChannelSet::ambisonicZ;
            default:                                        return AudioChannelSet::unknown;
        }
    */
}
