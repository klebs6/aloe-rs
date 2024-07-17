crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_VSTCommon.h]

/**
  | Structure for VST speaker mappings
  | 
  | @tags{Audio}
  |
  */
// (inheritance only to give easier access to items in the namespace)
pub struct SpeakerMappings {
    base: AudioChannelSet,
}

impl SpeakerMappings {

    pub fn vst_arrangement_type_to_channel_set_with_fallback(
        arr:                   i32,
        fallback_num_channels: i32) -> AudioChannelSet {
        
        todo!();
        /*
            if      (arr == typename Vst2::kSpeakerArrEmpty)        return AudioChannelSet::disabled();
            else if (arr == typename Vst2::kSpeakerArrMono)         return AudioChannelSet::mono();
            else if (arr == typename Vst2::kSpeakerArrStereo)       return AudioChannelSet::stereo();
            else if (arr == typename Vst2::kSpeakerArr30Cine)       return AudioChannelSet::createLCR();
            else if (arr == typename Vst2::kSpeakerArr30Music)      return AudioChannelSet::createLRS();
            else if (arr == typename Vst2::kSpeakerArr40Cine)       return AudioChannelSet::createLCRS();
            else if (arr == typename Vst2::kSpeakerArr50)           return AudioChannelSet::create5point0();
            else if (arr == typename Vst2::kSpeakerArr51)           return AudioChannelSet::create5point1();
            else if (arr == typename Vst2::kSpeakerArr60Cine)       return AudioChannelSet::create6point0();
            else if (arr == typename Vst2::kSpeakerArr61Cine)       return AudioChannelSet::create6point1();
            else if (arr == typename Vst2::kSpeakerArr60Music)      return AudioChannelSet::create6point0Music();
            else if (arr == typename Vst2::kSpeakerArr61Music)      return AudioChannelSet::create6point1Music();
            else if (arr == typename Vst2::kSpeakerArr70Music)      return AudioChannelSet::create7point0();
            else if (arr == typename Vst2::kSpeakerArr70Cine)       return AudioChannelSet::create7point0SDDS();
            else if (arr == typename Vst2::kSpeakerArr71Music)      return AudioChannelSet::create7point1();
            else if (arr == typename Vst2::kSpeakerArr71Cine)       return AudioChannelSet::create7point1SDDS();
            else if (arr == typename Vst2::kSpeakerArr40Music)      return AudioChannelSet::quadraphonic();

            for (const SpekaerMapping* m = getMappings(); m->vst2 != typename Vst2::kSpeakerArrEmpty; ++m)
            {
                if (m->vst2 == arr)
                {
                    AudioChannelSet s;

                    for (int i = 0; m->channels[i] != 0; ++i)
                        s.addChannel (m->channels[i]);

                    return s;
                }
            }

            return AudioChannelSet::discreteChannels (fallbackNumChannels);
        */
    }
    
    pub fn vst_arrangement_type_to_channel_set(arr: &SpeakerArrangement) -> AudioChannelSet {
        
        todo!();
        /*
            return vstArrangementTypeToChannelSet (arr.type, arr.numChannels);
        */
    }
    
    pub fn channel_set_to_vst_arrangement_type(channels: AudioChannelSet) -> i32 {
        
        todo!();
        /*
            if      (channels == AudioChannelSet::disabled())           return typename Vst2::kSpeakerArrEmpty;
            else if (channels == AudioChannelSet::mono())               return typename Vst2::kSpeakerArrMono;
            else if (channels == AudioChannelSet::stereo())             return typename Vst2::kSpeakerArrStereo;
            else if (channels == AudioChannelSet::createLCR())          return typename Vst2::kSpeakerArr30Cine;
            else if (channels == AudioChannelSet::createLRS())          return typename Vst2::kSpeakerArr30Music;
            else if (channels == AudioChannelSet::createLCRS())         return typename Vst2::kSpeakerArr40Cine;
            else if (channels == AudioChannelSet::create5point0())      return typename Vst2::kSpeakerArr50;
            else if (channels == AudioChannelSet::create5point1())      return typename Vst2::kSpeakerArr51;
            else if (channels == AudioChannelSet::create6point0())      return typename Vst2::kSpeakerArr60Cine;
            else if (channels == AudioChannelSet::create6point1())      return typename Vst2::kSpeakerArr61Cine;
            else if (channels == AudioChannelSet::create6point0Music()) return typename Vst2::kSpeakerArr60Music;
            else if (channels == AudioChannelSet::create6point1Music()) return typename Vst2::kSpeakerArr61Music;
            else if (channels == AudioChannelSet::create7point0())      return typename Vst2::kSpeakerArr70Music;
            else if (channels == AudioChannelSet::create7point0SDDS())  return typename Vst2::kSpeakerArr70Cine;
            else if (channels == AudioChannelSet::create7point1())      return typename Vst2::kSpeakerArr71Music;
            else if (channels == AudioChannelSet::create7point1SDDS())  return typename Vst2::kSpeakerArr71Cine;
            else if (channels == AudioChannelSet::quadraphonic())       return typename Vst2::kSpeakerArr40Music;

            if (channels == AudioChannelSet::disabled())
                return typename Vst2::kSpeakerArrEmpty;

            auto chans = channels.getChannelTypes();

            for (auto* m = getMappings(); m->vst2 != typename Vst2::kSpeakerArrEmpty; ++m)
                if (m->matches (chans))
                    return m->vst2;

            return typename Vst2::kSpeakerArrUserDefined;
        */
    }
    
    pub fn channel_set_to_vst_arrangement(
        channels: &AudioChannelSet,
        result:   &mut SpeakerArrangement

    ) {
        
        todo!();
        /*
            result.type = channelSetToVstArrangementType (channels);
            result.numChannels = channels.size();

            for (int i = 0; i < result.numChannels; ++i)
            {
                auto& speaker = result.speakers[i];

                zeromem (&speaker, sizeof (typename Vst2::VstSpeakerProperties));
                speaker.type = getSpeakerType (channels.getTypeOfChannel (i));
            }
        */
    }
    
    pub fn get_mappings() -> *const SpekaerMapping {
        
        todo!();
        /*
            static const SpekaerMapping mappings[] =
            {
                { typename Vst2::kSpeakerArrMono,           { centre, unknown } },
                { typename Vst2::kSpeakerArrStereo,         { left, right, unknown } },
                { typename Vst2::kSpeakerArrStereoSurround, { leftSurround, rightSurround, unknown } },
                { typename Vst2::kSpeakerArrStereoCenter,   { leftCentre, rightCentre, unknown } },
                { typename Vst2::kSpeakerArrStereoSide,     { leftSurroundRear, rightSurroundRear, unknown } },
                { typename Vst2::kSpeakerArrStereoCLfe,     { centre, LFE, unknown } },
                { typename Vst2::kSpeakerArr30Cine,         { left, right, centre, unknown } },
                { typename Vst2::kSpeakerArr30Music,        { left, right, surround, unknown } },
                { typename Vst2::kSpeakerArr31Cine,         { left, right, centre, LFE, unknown } },
                { typename Vst2::kSpeakerArr31Music,        { left, right, LFE, surround, unknown } },
                { typename Vst2::kSpeakerArr40Cine,         { left, right, centre, surround, unknown } },
                { typename Vst2::kSpeakerArr40Music,        { left, right, leftSurround, rightSurround, unknown } },
                { typename Vst2::kSpeakerArr41Cine,         { left, right, centre, LFE, surround, unknown } },
                { typename Vst2::kSpeakerArr41Music,        { left, right, LFE, leftSurround, rightSurround, unknown } },
                { typename Vst2::kSpeakerArr50,             { left, right, centre, leftSurround, rightSurround, unknown } },
                { typename Vst2::kSpeakerArr51,             { left, right, centre, LFE, leftSurround, rightSurround, unknown } },
                { typename Vst2::kSpeakerArr60Cine,         { left, right, centre, leftSurround, rightSurround, surround, unknown } },
                { typename Vst2::kSpeakerArr60Music,        { left, right, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear, unknown } },
                { typename Vst2::kSpeakerArr61Cine,         { left, right, centre, LFE, leftSurround, rightSurround, surround, unknown } },
                { typename Vst2::kSpeakerArr61Music,        { left, right, LFE, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear, unknown } },
                { typename Vst2::kSpeakerArr70Cine,         { left, right, centre, leftSurround, rightSurround, topFrontLeft, topFrontRight, unknown } },
                { typename Vst2::kSpeakerArr70Music,        { left, right, centre, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear, unknown } },
                { typename Vst2::kSpeakerArr71Cine,         { left, right, centre, LFE, leftSurround, rightSurround, topFrontLeft, topFrontRight, unknown } },
                { typename Vst2::kSpeakerArr71Music,        { left, right, centre, LFE, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear, unknown } },
                { typename Vst2::kSpeakerArr80Cine,         { left, right, centre, leftSurround, rightSurround, topFrontLeft, topFrontRight, surround, unknown } },
                { typename Vst2::kSpeakerArr80Music,        { left, right, centre, leftSurround, rightSurround, surround, leftSurroundRear, rightSurroundRear, unknown } },
                { typename Vst2::kSpeakerArr81Cine,         { left, right, centre, LFE, leftSurround, rightSurround, topFrontLeft, topFrontRight, surround, unknown } },
                { typename Vst2::kSpeakerArr81Music,        { left, right, centre, LFE, leftSurround, rightSurround, surround, leftSurroundRear, rightSurroundRear, unknown } },
                { typename Vst2::kSpeakerArr102,            { left, right, centre, LFE, leftSurround, rightSurround, topFrontLeft, topFrontCentre, topFrontRight, topRearLeft, topRearRight, LFE2, unknown } },
                { typename Vst2::kSpeakerArrEmpty,          { unknown } }
            };

            return mappings;
        */
    }
    
    pub fn get_speaker_type(ty: AudioChannelSetChannelType) -> i32 {
        
        todo!();
        /*
            static const std::map<AudioChannelSet::ChannelType, int32> speakerTypeMap =
            {
                { AudioChannelSet::left,              typename Vst2::kSpeakerL },
                { AudioChannelSet::right,             typename Vst2::kSpeakerR },
                { AudioChannelSet::centre,            typename Vst2::kSpeakerC },
                { AudioChannelSet::LFE,               typename Vst2::kSpeakerLfe },
                { AudioChannelSet::leftSurround,      typename Vst2::kSpeakerLs },
                { AudioChannelSet::rightSurround,     typename Vst2::kSpeakerRs },
                { AudioChannelSet::leftCentre,        typename Vst2::kSpeakerLc },
                { AudioChannelSet::rightCentre,       typename Vst2::kSpeakerRc },
                { AudioChannelSet::surround,          typename Vst2::kSpeakerS },
                { AudioChannelSet::leftSurroundRear,  typename Vst2::kSpeakerSl },
                { AudioChannelSet::rightSurroundRear, typename Vst2::kSpeakerSr },
                { AudioChannelSet::topMiddle,         typename Vst2::kSpeakerTm },
                { AudioChannelSet::topFrontLeft,      typename Vst2::kSpeakerTfl },
                { AudioChannelSet::topFrontCentre,    typename Vst2::kSpeakerTfc },
                { AudioChannelSet::topFrontRight,     typename Vst2::kSpeakerTfr },
                { AudioChannelSet::topRearLeft,       typename Vst2::kSpeakerTrl },
                { AudioChannelSet::topRearCentre,     typename Vst2::kSpeakerTrc },
                { AudioChannelSet::topRearRight,      typename Vst2::kSpeakerTrr },
                { AudioChannelSet::LFE2,              typename Vst2::kSpeakerLfe2 }
            };

            if (speakerTypeMap.find (type) == speakerTypeMap.end())
                return 0;

            return speakerTypeMap.at (type);
        */
    }
    
    pub fn get_channel_type(ty: i32) -> AudioChannelSetChannelType {
        
        todo!();
        /*
            switch (type)
            {
                case typename Vst2::kSpeakerL:      return AudioChannelSet::left;
                case typename Vst2::kSpeakerR:      return AudioChannelSet::right;
                case typename Vst2::kSpeakerC:      return AudioChannelSet::centre;
                case typename Vst2::kSpeakerLfe:    return AudioChannelSet::LFE;
                case typename Vst2::kSpeakerLs:     return AudioChannelSet::leftSurround;
                case typename Vst2::kSpeakerRs:     return AudioChannelSet::rightSurround;
                case typename Vst2::kSpeakerLc:     return AudioChannelSet::leftCentre;
                case typename Vst2::kSpeakerRc:     return AudioChannelSet::rightCentre;
                case typename Vst2::kSpeakerS:      return AudioChannelSet::surround;
                case typename Vst2::kSpeakerSl:     return AudioChannelSet::leftSurroundRear;
                case typename Vst2::kSpeakerSr:     return AudioChannelSet::rightSurroundRear;
                case typename Vst2::kSpeakerTm:     return AudioChannelSet::topMiddle;
                case typename Vst2::kSpeakerTfl:    return AudioChannelSet::topFrontLeft;
                case typename Vst2::kSpeakerTfc:    return AudioChannelSet::topFrontCentre;
                case typename Vst2::kSpeakerTfr:    return AudioChannelSet::topFrontRight;
                case typename Vst2::kSpeakerTrl:    return AudioChannelSet::topRearLeft;
                case typename Vst2::kSpeakerTrc:    return AudioChannelSet::topRearCentre;
                case typename Vst2::kSpeakerTrr:    return AudioChannelSet::topRearRight;
                case typename Vst2::kSpeakerLfe2:   return AudioChannelSet::LFE2;
                default: break;
            }

            return AudioChannelSet::unknown;
        */
    }
}
