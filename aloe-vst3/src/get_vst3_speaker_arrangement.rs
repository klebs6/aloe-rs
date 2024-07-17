crate::ix!();

pub fn get_vst3_speaker_arrangement(channels: &AudioChannelSet) 
    -> SpeakerArrangement 
{
    todo!();
        /*
            using namespace typename VstSpeakerArr;

        if (channels == AudioChannelSet::disabled())            return kEmpty;
        if (channels == AudioChannelSet::mono())                return kMono;
        if (channels == AudioChannelSet::stereo())              return kStereo;
        if (channels == AudioChannelSet::createLCR())           return k30Cine;
        if (channels == AudioChannelSet::createLRS())           return k30Music;
        if (channels == AudioChannelSet::createLCRS())          return k40Cine;
        if (channels == AudioChannelSet::create5point0())       return k50;
        if (channels == AudioChannelSet::create5point1())       return k51;
        if (channels == AudioChannelSet::create6point0())       return k60Cine;
        if (channels == AudioChannelSet::create6point1())       return k61Cine;
        if (channels == AudioChannelSet::create6point0Music())  return k60Music;
        if (channels == AudioChannelSet::create6point1Music())  return k61Music;
        if (channels == AudioChannelSet::create7point0())       return k70Music;
        if (channels == AudioChannelSet::create7point0SDDS())   return k70Cine;
        if (channels == AudioChannelSet::create7point1())       return k71CineSideFill;
        if (channels == AudioChannelSet::create7point1SDDS())   return k71Cine;
        if (channels == AudioChannelSet::ambisonic())           return kAmbi1stOrderACN;
        if (channels == AudioChannelSet::quadraphonic())        return k40Music;
        if (channels == AudioChannelSet::create7point0point2()) return k71_2 & ~(typename VstkSpeakerLfe);
        if (channels == AudioChannelSet::create7point1point2()) return k71_2;
        if (channels == AudioChannelSet::create7point0point4()) return k71_4 & ~(typename VstkSpeakerLfe);
        if (channels == AudioChannelSet::create7point1point4()) return k71_4;
        if (channels == AudioChannelSet::ambisonic (0))         return (1ull << 20);
        if (channels == AudioChannelSet::ambisonic (1))         return (1ull << 20) | (1ull << 21) | (1ull << 22) | (1ull << 23);
       #if Vst_VERSION >= 0x030608
        if (channels == AudioChannelSet::ambisonic (2))         return kAmbi2cdOrderACN;
        if (channels == AudioChannelSet::ambisonic (3))         return kAmbi3rdOrderACN;
       #endif

        typename VstSpeakerArrangement result = 0;

        Vec<AudioChannelSet::ChannelType> types (channels.getChannelTypes());

        for (int i = 0; i < types.size(); ++i)
            result |= getSpeakerType (channels, types.getReference(i));

        return result;
        */
}
