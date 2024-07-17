crate::ix!();

pub fn get_channel_set_for_speaker_arrangement(arr: SpeakerArrangement) 
    -> AudioChannelSet 
{
    todo!();
        /*
            using namespace typename VstSpeakerArr;

        switch (arr)
        {
            case kEmpty:                                          return AudioChannelSet::disabled();
            case kMono:                                           return AudioChannelSet::mono();
            case kStereo:                                         return AudioChannelSet::stereo();
            case k30Cine:                                         return AudioChannelSet::createLCR();
            case k30Music:                                        return AudioChannelSet::createLRS();
            case k40Cine:                                         return AudioChannelSet::createLCRS();
            case k50:                                             return AudioChannelSet::create5point0();
            case k51:                                             return AudioChannelSet::create5point1();
            case k60Cine:                                         return AudioChannelSet::create6point0();
            case k61Cine:                                         return AudioChannelSet::create6point1();
            case k60Music:                                        return AudioChannelSet::create6point0Music();
            case k61Music:                                        return AudioChannelSet::create6point1Music();
            case k70Music:                                        return AudioChannelSet::create7point0();
            case k70Cine:                                         return AudioChannelSet::create7point0SDDS();
            case k71CineSideFill:                                 return AudioChannelSet::create7point1();
            case k71Cine:                                         return AudioChannelSet::create7point1SDDS();
            case k40Music:                                        return AudioChannelSet::quadraphonic();
            case k71_2:                                           return AudioChannelSet::create7point1point2();
            case k71_2 & ~(typename VstkSpeakerLfe):          return AudioChannelSet::create7point0point2();
            case k71_4:                                           return AudioChannelSet::create7point1point4();
            case k71_4 & ~(typename VstkSpeakerLfe):          return AudioChannelSet::create7point0point4();
            case (1 << 20):                                       return AudioChannelSet::ambisonic (0);
            case kAmbi1stOrderACN:                                return AudioChannelSet::ambisonic (1);
           #if Vst_VERSION >= 0x030608
            case kAmbi2cdOrderACN:                                return AudioChannelSet::ambisonic (2);
            case kAmbi3rdOrderACN:                                return AudioChannelSet::ambisonic (3);
           #endif
        }

        AudioChannelSet result;

        BigInteger vstChannels (static_cast<int64> (arr));
        for (auto bit = vstChannels.findNextSetBit (0); bit != -1; bit = vstChannels.findNextSetBit (bit + 1))
        {
            AudioChannelSet::ChannelType channelType = getChannelType (arr, 1ull << static_cast<uint64> (bit));
            if (channelType != AudioChannelSet::unknown)
                result.addChannel (channelType);
        }

        // Vst3 <-> Aloe layout conversion error: report this bug to the Aloe forum
        jassert (result.size() == vstChannels.countNumberOfSetBits());

        return result;
        */
}
