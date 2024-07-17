crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/buffers/aloe_AudioChannelSet.h]

pub const AUDIO_CHANNEL_SET_MAX_CHANNELS_OF_NAMED_LAYOUT: usize = 36;

/**
  | Represents a set of audio channel types.
  |
  | For example, you might have a set of left
  | + right channels, which is a stereo channel
  | set. 
  |
  | It is a collection of values from the
  | AudioChannelSet::AudioChannelType enum, where each
  | type may only occur once within the set.
  |
  | The documentation below lists which
  | AudioChannelSet corresponds to which native
  | layouts used by AAX, Vst2/Vst3 and
  | CoreAudio/AU. 
  |
  | The layout tags in CoreAudio are particularly
  | confusing. For example, the layout which is
  | labeled as "7.1 SDDS" in Logic Pro,
  | corresponds to CoreAudio/AU's
  | kAudioChannelLayoutTag_DTS_7_0 tag, whereas
  | AAX's DTS 7.1 Layout corresponds to
  | CoreAudio/AU's
  | kAudioChannelLayoutTag_MPEG_7_1_A format, etc. 
  |
  | Please do not use the CoreAudio tag as an
  | indication to the actual layout of the
  | speakers.
  |
  | @see Bus
  |
  | @tags{Audio}
  |
  | Default creates an empty channel set.
  |
  | You can call addChannel to add channels to the
  | set.
  */
#[derive(Default)]
pub struct AudioChannelSet {
    channels: BigInteger,
}

impl PartialEq<AudioChannelSet> for AudioChannelSet {
    
    #[inline] fn eq(&self, other: &AudioChannelSet) -> bool {
        todo!();
        /*
            return channels == other.channels;
        */
    }
}

impl Eq for AudioChannelSet {}

impl Ord for AudioChannelSet {
    
    #[inline] fn cmp(&self, other: &AudioChannelSet) -> std::cmp::Ordering {
        todo!();
        /*
            return channels <  other.channels;
        */
    }
}

impl PartialOrd<AudioChannelSet> for AudioChannelSet {

    #[inline] fn partial_cmp(&self, other: &AudioChannelSet) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/buffers/aloe_AudioChannelSet.cpp]
impl AudioChannelSet {

    /**
      | Returns true if there are no channels
      | in the set.
      |
      */
    pub fn is_disabled(&self) -> bool {
        
        todo!();
        /*
            return size() == 0;
        */
    }

    /**
      | Intersect two channel layouts.
      |
      */
    pub fn intersect(&mut self, other: &AudioChannelSet)  {
        
        todo!();
        /*
            channels &= other.channels;
        */
    }

    /**
      | Conversion between wave and aloe channel
      | layout identifiers
      |
      */
    pub fn new_from_u32(c: u32) -> Self {
    
        todo!();
        /*
            : channels (static_cast<int64> (c))
        */
    }
    
    pub fn new_from_channel_type_slice(c: &[AudioChannelType]) -> Self {
    
        todo!();
        /*
            for (auto channel : c)
            addChannel (channel);
        */
    }
    
    /**
      | Returns the name of a given channel type.
      | For example, this method may return
      | "Surround Left".
      |
      */
    pub fn get_channel_type_name(&mut self, ty: AudioChannelType) -> String {
        
        todo!();
        /*
            if (type >= discreteChannel0)
            return "Discrete " + String (type - discreteChannel0 + 1);

        switch (type)
        {
            case left:                return NEEDS_TRANS("Left");
            case right:               return NEEDS_TRANS("Right");
            case centre:              return NEEDS_TRANS("Centre");
            case LFE:                 return NEEDS_TRANS("LFE");
            case leftSurround:        return NEEDS_TRANS("Left Surround");
            case rightSurround:       return NEEDS_TRANS("Right Surround");
            case leftCentre:          return NEEDS_TRANS("Left Centre");
            case rightCentre:         return NEEDS_TRANS("Right Centre");
            case centreSurround:      return NEEDS_TRANS("Centre Surround");
            case leftSurroundRear:    return NEEDS_TRANS("Left Surround Rear");
            case rightSurroundRear:   return NEEDS_TRANS("Right Surround Rear");
            case topMiddle:           return NEEDS_TRANS("Top Middle");
            case topFrontLeft:        return NEEDS_TRANS("Top Front Left");
            case topFrontCentre:      return NEEDS_TRANS("Top Front Centre");
            case topFrontRight:       return NEEDS_TRANS("Top Front Right");
            case topRearLeft:         return NEEDS_TRANS("Top Rear Left");
            case topRearCentre:       return NEEDS_TRANS("Top Rear Centre");
            case topRearRight:        return NEEDS_TRANS("Top Rear Right");
            case wideLeft:            return NEEDS_TRANS("Wide Left");
            case wideRight:           return NEEDS_TRANS("Wide Right");
            case LFE2:                return NEEDS_TRANS("LFE 2");
            case leftSurroundSide:    return NEEDS_TRANS("Left Surround Side");
            case rightSurroundSide:   return NEEDS_TRANS("Right Surround Side");
            case ambisonicW:          return NEEDS_TRANS("Ambisonic W");
            case ambisonicX:          return NEEDS_TRANS("Ambisonic X");
            case ambisonicY:          return NEEDS_TRANS("Ambisonic Y");
            case ambisonicZ:          return NEEDS_TRANS("Ambisonic Z");
            case topSideLeft:         return NEEDS_TRANS("Top Side Left");
            case topSideRight:        return NEEDS_TRANS("Top Side Right");
            case ambisonicACN4:       return NEEDS_TRANS("Ambisonic 4");
            case ambisonicACN5:       return NEEDS_TRANS("Ambisonic 5");
            case ambisonicACN6:       return NEEDS_TRANS("Ambisonic 6");
            case ambisonicACN7:       return NEEDS_TRANS("Ambisonic 7");
            case ambisonicACN8:       return NEEDS_TRANS("Ambisonic 8");
            case ambisonicACN9:       return NEEDS_TRANS("Ambisonic 9");
            case ambisonicACN10:      return NEEDS_TRANS("Ambisonic 10");
            case ambisonicACN11:      return NEEDS_TRANS("Ambisonic 11");
            case ambisonicACN12:      return NEEDS_TRANS("Ambisonic 12");
            case ambisonicACN13:      return NEEDS_TRANS("Ambisonic 13");
            case ambisonicACN14:      return NEEDS_TRANS("Ambisonic 14");
            case ambisonicACN15:      return NEEDS_TRANS("Ambisonic 15");
            case ambisonicACN16:      return NEEDS_TRANS("Ambisonic 16");
            case ambisonicACN17:      return NEEDS_TRANS("Ambisonic 17");
            case ambisonicACN18:      return NEEDS_TRANS("Ambisonic 18");
            case ambisonicACN19:      return NEEDS_TRANS("Ambisonic 19");
            case ambisonicACN20:      return NEEDS_TRANS("Ambisonic 20");
            case ambisonicACN21:      return NEEDS_TRANS("Ambisonic 21");
            case ambisonicACN22:      return NEEDS_TRANS("Ambisonic 22");
            case ambisonicACN23:      return NEEDS_TRANS("Ambisonic 23");
            case ambisonicACN24:      return NEEDS_TRANS("Ambisonic 24");
            case ambisonicACN25:      return NEEDS_TRANS("Ambisonic 25");
            case ambisonicACN26:      return NEEDS_TRANS("Ambisonic 26");
            case ambisonicACN27:      return NEEDS_TRANS("Ambisonic 27");
            case ambisonicACN28:      return NEEDS_TRANS("Ambisonic 28");
            case ambisonicACN29:      return NEEDS_TRANS("Ambisonic 29");
            case ambisonicACN30:      return NEEDS_TRANS("Ambisonic 30");
            case ambisonicACN31:      return NEEDS_TRANS("Ambisonic 31");
            case ambisonicACN32:      return NEEDS_TRANS("Ambisonic 32");
            case ambisonicACN33:      return NEEDS_TRANS("Ambisonic 33");
            case ambisonicACN34:      return NEEDS_TRANS("Ambisonic 34");
            case ambisonicACN35:      return NEEDS_TRANS("Ambisonic 35");
            case bottomFrontLeft:     return NEEDS_TRANS("Bottom Front Left");
            case bottomFrontCentre:   return NEEDS_TRANS("Bottom Front Centre");
            case bottomFrontRight:    return NEEDS_TRANS("Bottom Front Right");
            case proximityLeft:       return NEEDS_TRANS("Proximity Left");
            case proximityRight:      return NEEDS_TRANS("Proximity Right");
            case bottomSideLeft:      return NEEDS_TRANS("Bottom Side Left");
            case bottomSideRight:     return NEEDS_TRANS("Bottom Side Right");
            case bottomRearLeft:      return NEEDS_TRANS("Bottom Rear Left");
            case bottomRearCentre:    return NEEDS_TRANS("Bottom Rear Centre");
            case bottomRearRight:     return NEEDS_TRANS("Bottom Rear Right");
            case discreteChannel0:
            case unknown:
            default:                  break;
        }

        return "Unknown";
        */
    }
    
    /**
      | Returns the abbreviated name of a channel
      | type. For example, this method may return
      | "Ls".
      |
      */
    pub fn get_abbreviated_channel_type_name(&mut self, ty: AudioChannelType) -> String {
        
        todo!();
        /*
            if (type >= discreteChannel0)
            return String (type - discreteChannel0 + 1);

        switch (type)
        {
            case left:                return "L";
            case right:               return "R";
            case centre:              return "C";
            case LFE:                 return "Lfe";
            case leftSurround:        return "Ls";
            case rightSurround:       return "Rs";
            case leftCentre:          return "Lc";
            case rightCentre:         return "Rc";
            case centreSurround:      return "Cs";
            case leftSurroundRear:    return "Lrs";
            case rightSurroundRear:   return "Rrs";
            case topMiddle:           return "Tm";
            case topFrontLeft:        return "Tfl";
            case topFrontCentre:      return "Tfc";
            case topFrontRight:       return "Tfr";
            case topRearLeft:         return "Trl";
            case topRearCentre:       return "Trc";
            case topRearRight:        return "Trr";
            case wideLeft:            return "Wl";
            case wideRight:           return "Wr";
            case LFE2:                return "Lfe2";
            case leftSurroundSide:    return "Lss";
            case rightSurroundSide:   return "Rss";
            case ambisonicACN0:       return "ACN0";
            case ambisonicACN1:       return "ACN1";
            case ambisonicACN2:       return "ACN2";
            case ambisonicACN3:       return "ACN3";
            case ambisonicACN4:       return "ACN4";
            case ambisonicACN5:       return "ACN5";
            case ambisonicACN6:       return "ACN6";
            case ambisonicACN7:       return "ACN7";
            case ambisonicACN8:       return "ACN8";
            case ambisonicACN9:       return "ACN9";
            case ambisonicACN10:      return "ACN10";
            case ambisonicACN11:      return "ACN11";
            case ambisonicACN12:      return "ACN12";
            case ambisonicACN13:      return "ACN13";
            case ambisonicACN14:      return "ACN14";
            case ambisonicACN15:      return "ACN15";
            case ambisonicACN16:      return "ACN16";
            case ambisonicACN17:      return "ACN17";
            case ambisonicACN18:      return "ACN18";
            case ambisonicACN19:      return "ACN19";
            case ambisonicACN20:      return "ACN20";
            case ambisonicACN21:      return "ACN21";
            case ambisonicACN22:      return "ACN22";
            case ambisonicACN23:      return "ACN23";
            case ambisonicACN24:      return "ACN24";
            case ambisonicACN25:      return "ACN25";
            case ambisonicACN26:      return "ACN26";
            case ambisonicACN27:      return "ACN27";
            case ambisonicACN28:      return "ACN28";
            case ambisonicACN29:      return "ACN29";
            case ambisonicACN30:      return "ACN30";
            case ambisonicACN31:      return "ACN31";
            case ambisonicACN32:      return "ACN32";
            case ambisonicACN33:      return "ACN33";
            case ambisonicACN34:      return "ACN34";
            case ambisonicACN35:      return "ACN35";
            case topSideLeft:         return "Tsl";
            case topSideRight:        return "Tsr";
            case bottomFrontLeft:     return "Bfl";
            case bottomFrontCentre:   return "Bfc";
            case bottomFrontRight:    return "Bfr";
            case proximityLeft:       return "Pl";
            case proximityRight:      return "Pr";
            case bottomSideLeft:      return "Bsl";
            case bottomSideRight:     return "Bsr";
            case bottomRearLeft:      return "Brl";
            case bottomRearCentre:    return "Brc";
            case bottomRearRight:     return "Brr";
            case discreteChannel0:
            case unknown:
            default:                  break;
        }

        if (type >= ambisonicACN4 && type <= ambisonicACN35)
            return "ACN" + String (type - ambisonicACN4 + 4);

        return {};
        */
    }
    
    /**
      | Returns the channel type from an abbreviated
      | name.
      |
      */
    pub fn get_channel_type_from_abbreviation(&mut self, abbr: &String) -> AudioChannelType {
        
        todo!();
        /*
            if (abbr.length() > 0 && (abbr[0] >= '0' && abbr[0] <= '9'))
            return static_cast<audio_channel_set::AudioChannelType> (static_cast<int> (discreteChannel0)
                                                                   + abbr.getIntValue() - 1);

        if (abbr == "L")     return left;
        if (abbr == "R")     return right;
        if (abbr == "C")     return centre;
        if (abbr == "Lfe")   return LFE;
        if (abbr == "Ls")    return leftSurround;
        if (abbr == "Rs")    return rightSurround;
        if (abbr == "Lc")    return leftCentre;
        if (abbr == "Rc")    return rightCentre;
        if (abbr == "Cs")    return centreSurround;
        if (abbr == "Lrs")   return leftSurroundRear;
        if (abbr == "Rrs")   return rightSurroundRear;
        if (abbr == "Tm")    return topMiddle;
        if (abbr == "Tfl")   return topFrontLeft;
        if (abbr == "Tfc")   return topFrontCentre;
        if (abbr == "Tfr")   return topFrontRight;
        if (abbr == "Trl")   return topRearLeft;
        if (abbr == "Trc")   return topRearCentre;
        if (abbr == "Trr")   return topRearRight;
        if (abbr == "Wl")    return wideLeft;
        if (abbr == "Wr")    return wideRight;
        if (abbr == "Lfe2")  return LFE2;
        if (abbr == "Lss")   return leftSurroundSide;
        if (abbr == "Rss")   return rightSurroundSide;
        if (abbr == "W")     return ambisonicW;
        if (abbr == "X")     return ambisonicX;
        if (abbr == "Y")     return ambisonicY;
        if (abbr == "Z")     return ambisonicZ;
        if (abbr == "ACN0")  return ambisonicACN0;
        if (abbr == "ACN1")  return ambisonicACN1;
        if (abbr == "ACN2")  return ambisonicACN2;
        if (abbr == "ACN3")  return ambisonicACN3;
        if (abbr == "ACN4")  return ambisonicACN4;
        if (abbr == "ACN5")  return ambisonicACN5;
        if (abbr == "ACN6")  return ambisonicACN6;
        if (abbr == "ACN7")  return ambisonicACN7;
        if (abbr == "ACN8")  return ambisonicACN8;
        if (abbr == "ACN9")  return ambisonicACN9;
        if (abbr == "ACN10") return ambisonicACN10;
        if (abbr == "ACN11") return ambisonicACN11;
        if (abbr == "ACN12") return ambisonicACN12;
        if (abbr == "ACN13") return ambisonicACN13;
        if (abbr == "ACN14") return ambisonicACN14;
        if (abbr == "ACN15") return ambisonicACN15;
        if (abbr == "ACN16") return ambisonicACN16;
        if (abbr == "ACN17") return ambisonicACN17;
        if (abbr == "ACN18") return ambisonicACN18;
        if (abbr == "ACN19") return ambisonicACN19;
        if (abbr == "ACN20") return ambisonicACN20;
        if (abbr == "ACN21") return ambisonicACN21;
        if (abbr == "ACN22") return ambisonicACN22;
        if (abbr == "ACN23") return ambisonicACN23;
        if (abbr == "ACN24") return ambisonicACN24;
        if (abbr == "ACN25") return ambisonicACN25;
        if (abbr == "ACN26") return ambisonicACN26;
        if (abbr == "ACN27") return ambisonicACN27;
        if (abbr == "ACN28") return ambisonicACN28;
        if (abbr == "ACN29") return ambisonicACN29;
        if (abbr == "ACN30") return ambisonicACN30;
        if (abbr == "ACN31") return ambisonicACN31;
        if (abbr == "ACN32") return ambisonicACN32;
        if (abbr == "ACN33") return ambisonicACN33;
        if (abbr == "ACN34") return ambisonicACN34;
        if (abbr == "ACN35") return ambisonicACN35;
        if (abbr == "Tsl")   return topSideLeft;
        if (abbr == "Tsr")   return topSideRight;
        if (abbr == "Bfl")   return bottomFrontLeft;
        if (abbr == "Bfc")   return bottomFrontCentre;
        if (abbr == "Bfr")   return bottomFrontRight;
        if (abbr == "Bsl")   return bottomSideLeft;
        if (abbr == "Bsr")   return bottomSideRight;
        if (abbr == "Brl")   return bottomRearLeft;
        if (abbr == "Brc")   return bottomRearCentre;
        if (abbr == "Brr")   return bottomRearRight;
        return unknown;
        */
    }
    
    /**
      | Returns a string containing a whitespace-separated
      | list of speaker types corresponding to each channel.
      | For example in a 5.1 arrangement, the string may
      | be "L R C Lfe Ls Rs". If the speaker arrangement
      | is unknown, the returned string will be empty.
      */
    pub fn get_speaker_arrangement_as_string(&self) -> String {
        
        todo!();
        /*
            StringArray speakerTypes;

        for (auto& speaker : getChannelTypes())
        {
            auto name = getAbbreviatedChannelTypeName (speaker);

            if (name.isNotEmpty())
                speakerTypes.add (name);
        }

        return speakerTypes.joinIntoString (" ");
        */
    }
    
    /**
      | Returns an AudioChannelSet from a string
      | returned by getSpeakerArrangementAsString
      | @see getSpeakerArrangementAsString
      */
    pub fn from_abbreviated_string(&mut self, str_: &String) -> AudioChannelSet {
        
        todo!();
        /*
            AudioChannelSet set;

        for (auto& abbr : StringArray::fromTokens (str, true))
        {
            auto type = getChannelTypeFromAbbreviation (abbr);

            if (type != unknown)
                set.addChannel (type);
        }

        return set;
        */
    }
    
    /**
      | Returns the description of the current
      | layout. For example, this method may
      | return "Quadraphonic". Note that the
      | returned string may not be unique.
      */
    pub fn get_description(&self) -> String {
        
        todo!();
        /*
            if (isDiscreteLayout())            return "Discrete #" + String (size());
        if (*this == disabled())           return "Disabled";
        if (*this == mono())               return "Mono";
        if (*this == stereo())             return "Stereo";

        if (*this == createLCR())          return "LCR";
        if (*this == createLRS())          return "LRS";
        if (*this == createLCRS())         return "LCRS";

        if (*this == create5point0())       return "5.0 Surround";
        if (*this == create5point1())       return "5.1 Surround";
        if (*this == create6point0())       return "6.0 Surround";
        if (*this == create6point1())       return "6.1 Surround";
        if (*this == create6point0Music())  return "6.0 (Music) Surround";
        if (*this == create6point1Music())  return "6.1 (Music) Surround";
        if (*this == create7point0())       return "7.0 Surround";
        if (*this == create7point1())       return "7.1 Surround";
        if (*this == create7point0SDDS())   return "7.0 Surround SDDS";
        if (*this == create7point1SDDS())   return "7.1 Surround SDDS";
        if (*this == create7point0point2()) return "7.0.2 Surround";
        if (*this == create7point1point2()) return "7.1.2 Surround";

        if (*this == quadraphonic())       return "Quadraphonic";
        if (*this == pentagonal())         return "Pentagonal";
        if (*this == hexagonal())          return "Hexagonal";
        if (*this == octagonal())          return "Octagonal";

        // ambisonics
        {
            auto order = getAmbisonicOrder();

            if (order >= 0)
            {
                String suffix;

                switch (order)
                {
                    case 1:  suffix = "st"; break;
                    case 2:  suffix = "nd"; break;
                    case 3:  suffix = "rd"; break;
                    default: suffix = "th"; break;
                }

                return String (order) + suffix + " Order Ambisonics";
            }
        }

        return "Unknown";
        */
    }
    
    /**
      Returns if this is a channel layout made-up
      of discrete channels.
      */
    pub fn is_discrete_layout(&self) -> bool {
        
        todo!();
        /*
            for (auto& speaker : getChannelTypes())
            if (speaker <= ambisonicACN35)
                return false;

        return true;
        */
    }
    
    /**
      | Returns the number of channels in the
      | set.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return channels.countNumberOfSetBits();
        */
    }
    
    /**
      | Returns the type of one of the channels
      | in the set, by index.
      |
      */
    pub fn get_type_of_channel(&self, index: i32) -> AudioChannelType {
        
        todo!();
        /*
            int bit = channels.findNextSetBit(0);

        for (int i = 0; i < index && bit >= 0; ++i)
            bit = channels.findNextSetBit (bit + 1);

        return static_cast<AudioChannelType> (bit);
        */
    }
    
    /**
      | Returns the index for a particular channel-type.
      | Will return -1 if the this set does not contain
      | a channel of this type.
      */
    pub fn get_channel_index_for_type(&self, ty: AudioChannelType) -> i32 {
        
        todo!();
        /*
            int idx = 0;

        for (int bit = channels.findNextSetBit (0); bit >= 0; bit = channels.findNextSetBit (bit + 1))
        {
            if (static_cast<AudioChannelType> (bit) == type)
                return idx;

            idx++;
        }

        return -1;
        */
    }
    
    /**
      | Returns an array of all the types in this
      | channel set.
      |
      */
    pub fn get_channel_types(&self) -> Vec<AudioChannelType> {
        
        todo!();
        /*
            Vec<AudioChannelType> result;

        for (int bit = channels.findNextSetBit(0); bit >= 0; bit = channels.findNextSetBit (bit + 1))
            result.add (static_cast<AudioChannelType> (bit));

        return result;
        */
    }
    
    /**
      | Adds a channel to the set.
      |
      */
    pub fn add_channel(&mut self, new_channel: AudioChannelType)  {
        
        todo!();
        /*
            const int bit = static_cast<int> (newChannel);
        jassert (bit >= 0 && bit < 1024);
        channels.setBit (bit);
        */
    }
    
    /**
      | Removes a channel from the set.
      |
      */
    pub fn remove_channel(&mut self, new_channel: AudioChannelType)  {
        
        todo!();
        /*
            const int bit = static_cast<int> (newChannel);
        jassert (bit >= 0 && bit < 1024);
        channels.clearBit (bit);
        */
    }
    
    /**
      | Creates a zero-channel set which can
      | be used to indicate that a bus is disabled.
      |
      */
    pub fn disabled(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return {};
        */
    }
    
    /**
      | Creates a one-channel mono set (centre).
      | Is equivalent to: kMonoAAX (Vst), AAX_eStemFormat_Mono
      | (AAX), kAudioChannelLayoutTag_Mono
      | (CoreAudio)
      |
      */
    pub fn mono(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet (1u << centre);
        */
    }
    
    /**
      | Creates a set containing a stereo set
      | (left, right). Is equivalent to: kStereo
      | (Vst), AAX_eStemFormat_Stereo (AAX),
      | kAudioChannelLayoutTag_Stereo (CoreAudio)
      |
      */
    pub fn stereo(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right));
        */
    }
    
    /** 
      | Creates a set containing an LCR set (left,
      | right, centre).
      |
      | Is equivalent to: k30Cine (Vst),
      | AAX_eStemFormat_LCR (AAX),
      | kAudioChannelLayoutTag_MPEG_3_0_A
      | (CoreAudio)
      |
      | This format is referred to as "LRC" in
      | Cubase.
      |
      | This format is referred to as "LCR" in Pro
      | Tools.
    */
    pub fn createlcr(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre));
        */
    }
    
    /** 
      | Creates a set containing an LRS set (left,
      | right, surround).
      |
      | Is equivalent to: k30Music (Vst), n/a
      | (AAX), kAudioChannelLayoutTag_ITU_2_1
      | (CoreAudio)
      |
      | This format is referred to as "LRS" in
      | Cubase.
      */
    pub fn createlrs(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << surround));
        */
    }
    
    /** 
      | Creates a set containing an LCRS set (left,
      | right, centre, surround).
      |
      | Is equivalent to: k40Cine (Vst),
      | AAX_eStemFormat_LCRS (AAX),
      | kAudioChannelLayoutTag_MPEG_4_0_A
      | (CoreAudio)
      |
      | This format is referred to as "LCRS (Pro
      | Logic)" in Logic Pro.
      |
      | This format is referred to as "LRCS" in
      | Cubase.
      |
      | This format is referred to as "LCRS" in
      | Pro Tools.
      */
    pub fn createlcrs(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << surround));
        */
    }
    
    /** 
      | Creates a set for a 5.0 surround setup (left,
      | right, centre, leftSurround, rightSurround).
      |
      | Is equivalent to: k50 (Vst),
      | AAX_eStemFormat_5_0 (AAX),
      | kAudioChannelLayoutTag_MPEG_5_0_A
      | (CoreAudio)
      |
      | This format is referred to as "5.0" in
      | Cubase.
      |
      | This format is referred to as "5.0" in Pro
      | Tools.
    */
    pub fn create5point0(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurround) | (1u << rightSurround));
        */
    }
    
    /** 
      | Creates a set for a 5.1 surround setup (left,
      | right, centre, leftSurround, rightSurround,
      | LFE).
      |
      | Is equivalent to: k51 (Vst),
      | AAX_eStemFormat_5_1 (AAX),
      | kAudioChannelLayoutTag_MPEG_5_1_A
      | (CoreAudio)
      |
      | This format is referred to as "5.1 (ITU
      | 775)" in Logic Pro.
      |
      | This format is referred to as "5.1" in
      | Cubase.
      |
      | This format is referred to as "5.1" in Pro
      | Tools.
      */
    pub fn create5point1(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << LFE) | (1u << leftSurround) | (1u << rightSurround));
        */
    }
    
    /** 
     | Creates a set for a 6.0 Cine surround setup
     | (left, right, centre, leftSurround,
     | rightSurround, centreSurround).
     |
     | Is equivalent to: k60Cine (Vst),
     | AAX_eStemFormat_6_0 (AAX),
     | kAudioChannelLayoutTag_AudioUnit_6_0
     | (CoreAudio)
     |
     | Logic Pro incorrectly uses this for the
     | surround format labeled "6.1 (ES/EX)".
     |
     | This format is referred to as "6.0 Cine"
     | in Cubase.
     |
     | This format is referred to as "6.0" in Pro
     | Tools.
    */
    pub fn create6point0(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurround) | (1u << rightSurround) | (1u << centreSurround));
        */
    }
    
    /** 
      | Creates a set for a 6.1 Cine surround setup
      | (left, right, centre, leftSurround,
      | rightSurround, centreSurround, LFE).
      |
      | Is equivalent to: k61Cine (Vst),
      | AAX_eStemFormat_6_1 (AAX),
      | kAudioChannelLayoutTag_MPEG_6_1_A
      | (CoreAudio)
      |
      | This format is referred to as "6.1" in
      | Pro Tools.
     */
    pub fn create6point1(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << LFE) | (1u << leftSurround) | (1u << rightSurround) | (1u << centreSurround));
        */
    }
    
    /** 
      | Creates a set for a 6.0 Music surround setup
      | (left, right, leftSurround, rightSurround,
      | leftSurroundSide, rightSurroundSide).
      |
      | Is equivalent to: k60Music (Vst), n/a
      | (AAX), kAudioChannelLayoutTag_DTS_6_0_A
      | (CoreAudio)
      |
      | This format is referred to as "6.0 Music"
      | in Cubase.
      */
    pub fn create6point_0music(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << leftSurround) | (1u << rightSurround) | (1u << leftSurroundSide) | (1u << rightSurroundSide));
        */
    }
    
    /** 
      | Creates a set for a 6.0 Music surround setup
      | (left, right, leftSurround, rightSurround,
      | leftSurroundSide, rightSurroundSide, LFE).
      |
      | Is equivalent to: k61Music (Vst), n/a
      | (AAX), kAudioChannelLayoutTag_DTS_6_1_A
      | (CoreAudio)
      */
    pub fn create6point_1music(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << LFE) | (1u << leftSurround) | (1u << rightSurround) | (1u << leftSurroundSide) | (1u << rightSurroundSide));
        */
    }
    
    /** 
     | Creates a set for a DTS 7.0 surround setup
     | (left, right, centre, leftSurroundSide,
     | rightSurroundSide, leftSurroundRear,
     | rightSurroundRear).
     |
     | Is equivalent to: k70Music (Vst),
     | AAX_eStemFormat_7_0_DTS (AAX),
     | kAudioChannelLayoutTag_AudioUnit_7_0
     | (CoreAudio)
     |
     | This format is referred to as "7.0" in Pro
     | Tools.
    */
    pub fn create7point0(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurroundSide) | (1u << rightSurroundSide) | (1u << leftSurroundRear) | (1u << rightSurroundRear));
        */
    }
    
    /** 
      | Creates a set for a SDDS 7.0 surround setup
      | (left, right, centre, leftSurround,
      | rightSurround, leftCentre, rightCentre).
      |
      | Is equivalent to: k70Cine (Vst),
      | AAX_eStemFormat_7_0_SDDS (AAX),
      | kAudioChannelLayoutTag_AudioUnit_7_0_Front
      | (CoreAudio)
      |
      | This format is referred to as "7.0 SDDS"
      | in Pro Tools.
      */
    pub fn create7point0sdds(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurround) | (1u << rightSurround) | (1u << leftCentre) | (1u << rightCentre));
        */
    }
    
    /** 
      | Creates a set for a DTS 7.1 surround setup
      | (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, LFE).
      |
      | Is equivalent to: k71CineSideFill (Vst),
      | AAX_eStemFormat_7_1_DTS (AAX),
      | kAudioChannelLayoutTag_MPEG_7_1_C/kAudioChannelLayoutTag_ITU_3_4_1
      | (CoreAudio)
      |
      | This format is referred to as "7.1 (3/4.1)"
      | in Logic Pro.
      |
      | This format is referred to as "7.1" in Pro
      | Tools.
      */
    pub fn create7point1(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << LFE) | (1u << leftSurroundSide) | (1u << rightSurroundSide) | (1u << leftSurroundRear) | (1u << rightSurroundRear));
        */
    }
    
    /** 
      | Creates a set for a 7.1 surround setup
      | (left, right, centre, leftSurround,
      | rightSurround, leftCentre, rightCentre,
      | LFE).
      |
      | Is equivalent to: k71Cine (Vst),
      | AAX_eStemFormat_7_1_SDDS (AAX),
      | kAudioChannelLayoutTag_MPEG_7_1_A
      | (CoreAudio)
      |
      | This format is referred to as "7.1 (SDDS)"
      | in Logic Pro.
      |
      | This format is referred to as "7.1 SDDS" in
      | Pro Tools.
      */
    pub fn create7point1sdds(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << LFE) | (1u << leftSurround) | (1u << rightSurround) | (1u << leftCentre) | (1u << rightCentre));
        */
    }
    
    /** 
      | Creates a set for quadraphonic surround setup
      | (left, right, leftSurround, rightSurround)
      |
      | Is equivalent to: k40Music (Vst),
      | AAX_eStemFormat_Quad (AAX),
      | kAudioChannelLayoutTag_Quadraphonic
      | (CoreAudio)
      |
      | This format is referred to as
      | "Quadraphonic" in Logic Pro.
      |
      | This format is referred to as "Quadro" in
      | Cubase.
      |
      | This format is referred to as "Quad" in
      | Pro Tools.
      |
      */
    pub fn quadraphonic(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << leftSurround) | (1u << rightSurround));
        */
    }
    
    /** 
      | Creates a set for pentagonal surround setup
      | (left, right, centre, leftSurroundRear,
      | rightSurroundRear).
      |
      | Is equivalent to: n/a (Vst), n/a (AAX),
      | kAudioChannelLayoutTag_Pentagonal
      | (CoreAudio)
      */
    pub fn pentagonal(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurroundRear) | (1u << rightSurroundRear));
        */
    }
    
    /** 
      | Creates a set for hexagonal surround setup
      | (left, right, leftSurroundRear,
      | rightSurroundRear, centre, surroundCentre).
      |
      | Is equivalent to: n/a (Vst), n/a (AAX),
      | kAudioChannelLayoutTag_Hexagonal (CoreAudio)
      */
    pub fn hexagonal(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << centreSurround) | (1u << leftSurroundRear) | (1u << rightSurroundRear));
        */
    }
    
    /** 
      | Creates a set for octagonal surround setup
      | (left, right, leftSurround, rightSurround,
      | centre, centreSurround, wideLeft, wideRight).
      |
      | Is equivalent to: n/a (Vst), n/a (AAX),
      | kAudioChannelLayoutTag_Octagonal (CoreAudio)
      */
    pub fn octagonal(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurround) | (1u << rightSurround) | (1u << centreSurround) | (1u << wideLeft) | (1u << wideRight));
        */
    }
    
    /** 
      | Creates a set for Dolby Atmos 7.0.2 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, topSideLeft,
      | topSideRight).
      |
      | Is equivalent to: n/a (Vst),
      | AAX_eStemFormat_7_0_2 (AAX), n/a (CoreAudio)
      */
    pub fn create7point0point2(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurroundSide) | (1u << rightSurroundSide) | (1u << leftSurroundRear) | (1u << rightSurroundRear) | (1u << topSideLeft) | (1u << topSideRight));
        */
    }
    
    /** 
      | Creates a set for Dolby Atmos 7.1.2 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, LFE, topSideLeft,
      | topSideRight).
      |
      | Is equivalent to: k71_2 (Vst),
      | AAX_eStemFormat_7_1_2 (AAX), n/a
      | (CoreAudio)
      */
    pub fn create7point1point2(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << LFE) | (1u << leftSurroundSide) | (1u << rightSurroundSide) | (1u << leftSurroundRear) | (1u << rightSurroundRear) | (1u << topSideLeft) | (1u << topSideRight));
        */
    }
    
    /** 
      | Creates a set for Dolby Atmos 7.0.4 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, topFrontLeft,
      | topFrontRight, topRearLeft, topRearRight).
      |
      | Is equivalent to: n/a (Vst), n/a (AAX),
      | n/a (CoreAudio)
      */
    pub fn create7point0point4(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << leftSurroundSide) | (1u << rightSurroundSide) | (1u << leftSurroundRear) | (1u << rightSurroundRear) | (1u << topFrontLeft) | (1u << topFrontRight) | (1u << topRearLeft) | (1u << topRearRight));
        */
    }
    
    /** 
      | Creates a set for Dolby Atmos 7.1.4 surround
      | setup (left, right, centre, leftSurroundSide,
      | rightSurroundSide, leftSurroundRear,
      | rightSurroundRear, LFE, topFrontLeft,
      | topFrontRight, topRearLeft, topRearRight).
      |
      | Is equivalent to: k71_4 (Vst), n/a (AAX),
      | n/a (CoreAudio)
      */
    pub fn create7point1point4(&mut self) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet ((1u << left) | (1u << right) | (1u << centre) | (1u << LFE) | (1u << leftSurroundSide) | (1u << rightSurroundSide) | (1u << leftSurroundRear) | (1u << rightSurroundRear) | (1u << topFrontLeft) | (1u << topFrontRight) | (1u << topRearLeft) | (1u << topRearRight));
        */
    }
    
    /** 
      | Creates a set for ACN, SN3D normalised
      | ambisonic surround setups with a given order.
      |
      | Is equivalent to: kAmbiXXXOrderACN (Vst),
      | AAX_eStemFormat_Ambi_XXX_ACN (AAX),
      | kAudioChannelLayoutTag_HOA_ACN_SN3D
      | (CoreAudio)
      */
    pub fn ambisonic(&mut self, order: Option<i32>) -> AudioChannelSet {

        let order: i32 = order.unwrap_or(1);
        
        todo!();
        /*
            jassert (isPositiveAndBelow (order, 6));

        if (order == 0)
            return AudioChannelSet ((uint32) (1 << ambisonicACN0));

        AudioChannelSet set ((1u << ambisonicACN0) | (1u << ambisonicACN1) | (1u << ambisonicACN2) | (1u << ambisonicACN3));

        auto numAmbisonicChannels = (order + 1) * (order + 1);
        set.channels.setRange (ambisonicACN4, numAmbisonicChannels - 4, true);

        return set;
        */
    }
    
    /** 
      | Returns the order of the ambisonic layout
      | represented by this AudioChannelSet. If the
      | AudioChannelSet is not an ambisonic layout,
      | then this method will return -1.
      */
    pub fn get_ambisonic_order(&self) -> i32 {
        
        todo!();
        /*
            auto ambisonicOrder = getAmbisonicOrderForNumChannels (size());

        if (ambisonicOrder >= 0)
            return (*this == ambisonic (ambisonicOrder) ? ambisonicOrder : -1);

        return -1;
        */
    }
    
    /**
      | Creates a set of untyped discrete channels.
      |
      */
    pub fn discrete_channels(&mut self, num_channels: i32) -> AudioChannelSet {
        
        todo!();
        /*
            AudioChannelSet s;
        s.channels.setRange (discreteChannel0, numChannels, true);
        return s;
        */
    }
    
    /**
      | Create a canonical channel set for a given
      | number of channels. For example,
      | numChannels = 1 will return mono,
      | numChannels = 2 will return stereo, etc.
      */
    pub fn canonical_channel_set(&mut self, num_channels: i32) -> AudioChannelSet {
        
        todo!();
        /*
            if (numChannels == 1)  return AudioChannelSet::mono();
        if (numChannels == 2)  return AudioChannelSet::stereo();
        if (numChannels == 3)  return AudioChannelSet::createLCR();
        if (numChannels == 4)  return AudioChannelSet::quadraphonic();
        if (numChannels == 5)  return AudioChannelSet::create5point0();
        if (numChannels == 6)  return AudioChannelSet::create5point1();
        if (numChannels == 7)  return AudioChannelSet::create7point0();
        if (numChannels == 8)  return AudioChannelSet::create7point1();

        return discreteChannels (numChannels);
        */
    }
    
    /** 
      | Create a channel set for a given number of
      | channels which is non-discrete.
      | If numChannels is larger than the number
      | of channels of the surround format with
      | the maximum amount of channels (currently
      | 7.1 Surround), then this function returns
      | an empty set.
      */
    pub fn named_channel_set(&mut self, num_channels: i32) -> AudioChannelSet {
        
        todo!();
        /*
            if (numChannels == 1)  return AudioChannelSet::mono();
        if (numChannels == 2)  return AudioChannelSet::stereo();
        if (numChannels == 3)  return AudioChannelSet::createLCR();
        if (numChannels == 4)  return AudioChannelSet::quadraphonic();
        if (numChannels == 5)  return AudioChannelSet::create5point0();
        if (numChannels == 6)  return AudioChannelSet::create5point1();
        if (numChannels == 7)  return AudioChannelSet::create7point0();
        if (numChannels == 8)  return AudioChannelSet::create7point1();

        return {};
        */
    }
    
    /**
      | Return an array of channel sets which
      | have a given number of channels
      |
      */
    pub fn channel_sets_with_number_of_channels(&mut self, num_channels: i32) -> Vec<AudioChannelSet> {
        
        todo!();
        /*
            Vec<AudioChannelSet> retval;

        if (numChannels != 0)
        {
            retval.add (AudioChannelSet::discreteChannels (numChannels));

            if      (numChannels == 1)
            {
                retval.add (AudioChannelSet::mono());
            }
            else if (numChannels == 2)
            {
                retval.add (AudioChannelSet::stereo());
            }
            else if (numChannels == 3)
            {
                retval.add (AudioChannelSet::createLCR());
                retval.add (AudioChannelSet::createLRS());
            }
            else if (numChannels == 4)
            {
                retval.add (AudioChannelSet::quadraphonic());
                retval.add (AudioChannelSet::createLCRS());
            }
            else if (numChannels == 5)
            {
                retval.add (AudioChannelSet::create5point0());
                retval.add (AudioChannelSet::pentagonal());
            }
            else if (numChannels == 6)
            {
                retval.add (AudioChannelSet::create5point1());
                retval.add (AudioChannelSet::create6point0());
                retval.add (AudioChannelSet::create6point0Music());
                retval.add (AudioChannelSet::hexagonal());
            }
            else if (numChannels == 7)
            {
                retval.add (AudioChannelSet::create7point0());
                retval.add (AudioChannelSet::create7point0SDDS());
                retval.add (AudioChannelSet::create6point1());
                retval.add (AudioChannelSet::create6point1Music());
            }
            else if (numChannels == 8)
            {
                retval.add (AudioChannelSet::create7point1());
                retval.add (AudioChannelSet::create7point1SDDS());
                retval.add (AudioChannelSet::octagonal());
            }

            auto order = getAmbisonicOrderForNumChannels (numChannels);
            if (order >= 0)
                retval.add (AudioChannelSet::ambisonic (order));
        }

        return retval;
        */
    }
    
    /**
      | Creates a channel set for a list of channel
      | types. This function will assert if
      | you supply a duplicate channel. Note
      | that this method ignores the order in
      | which the channels are given, i.e. two
      | arrays with the same elements but in
      | a different order will still result
      | in the same channel set.
      |
      */
    pub fn channel_set_with_channels(&mut self, channel_array: &[AudioChannelType]) -> AudioChannelSet {
        
        todo!();
        /*
            AudioChannelSet set;

        for (auto ch : channelArray)
        {
            jassert (! set.channels[static_cast<int> (ch)]);

            set.addChannel (ch);
        }

        return set;
        */
    }
    
    /**
      | Create an AudioChannelSet from a WAVEFORMATEXTENSIBLE
      | channelMask (typically used in .wav
      | files).
      */
    pub fn from_wave_channel_mask(&mut self, dw_channel_mask: i32) -> AudioChannelSet {
        
        todo!();
        /*
            return AudioChannelSet (static_cast<uint32> ((dwChannelMask & ((1 << 18) - 1)) << 1));
        */
    }
    
    /** 
      | Returns a WAVEFORMATEXTENSIBLE channelMask
      | representation (typically used in .wav files)
      | of the receiver.
      |
      | Returns -1 if the receiver cannot be
      | represented in a WAVEFORMATEXTENSIBLE
      | channelMask representation.
      */
    pub fn get_wave_channel_mask(&self) -> i32 {
        
        todo!();
        /*
            if (channels.getHighestBit() > topRearRight)
            return -1;

        return (channels.toInteger() >> 1);
        */
    }
    
    pub fn get_ambisonic_order_for_num_channels(&mut self, num_channels: i32) -> i32 {
        
        todo!();
        /*
            auto sqrtMinusOne   = std::sqrt (static_cast<float> (numChannels)) - 1.0f;
        auto ambisonicOrder = jmax (0, static_cast<int> (std::floor (sqrtMinusOne)));

        if (ambisonicOrder > 5)
            return -1;

        return (static_cast<float> (ambisonicOrder) == sqrtMinusOne ? ambisonicOrder : -1);
        */
    }
}
