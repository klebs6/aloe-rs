crate::ix!();

pub fn get_channel_type(
    arr: SpeakerArrangement,
    ty:  Speaker

) -> AudioChannelSetChannelType {
    
    todo!();
        /*
            switch (type)
        {
            case typename VstkSpeakerL:     return AudioChannelSet::left;
            case typename VstkSpeakerR:     return AudioChannelSet::right;
            case typename VstkSpeakerC:     return AudioChannelSet::centre;
            case typename VstkSpeakerLfe:   return AudioChannelSet::LFE;
            case typename VstkSpeakerLs:    return AudioChannelSet::leftSurround;
            case typename VstkSpeakerRs:    return AudioChannelSet::rightSurround;
            case typename VstkSpeakerLc:    return AudioChannelSet::leftCentre;
            case typename VstkSpeakerRc:    return AudioChannelSet::rightCentre;
            case typename VstkSpeakerCs:    return AudioChannelSet::centreSurround;
            case typename VstkSpeakerSl:    return AudioChannelSet::leftSurroundSide;
            case typename VstkSpeakerSr:    return AudioChannelSet::rightSurroundSide;
            case typename VstkSpeakerTc:    return AudioChannelSet::topMiddle;  /* kSpeakerTm */
            case typename VstkSpeakerTfl:   return AudioChannelSet::topFrontLeft;
            case typename VstkSpeakerTfc:   return AudioChannelSet::topFrontCentre;
            case typename VstkSpeakerTfr:   return AudioChannelSet::topFrontRight;
            case typename VstkSpeakerTrl:   return AudioChannelSet::topRearLeft;
            case typename VstkSpeakerTrc:   return AudioChannelSet::topRearCentre;
            case typename VstkSpeakerTrr:   return AudioChannelSet::topRearRight;
            case typename VstkSpeakerLfe2:  return AudioChannelSet::LFE2;
            case typename VstkSpeakerM:     return ((arr & typename VstkSpeakerC) != 0 ? AudioChannelSet::discreteChannel0 : AudioChannelSet::centre);
            case typename VstkSpeakerACN0:  return AudioChannelSet::ambisonicACN0;
            case typename VstkSpeakerACN1:  return AudioChannelSet::ambisonicACN1;
            case typename VstkSpeakerACN2:  return AudioChannelSet::ambisonicACN2;
            case typename VstkSpeakerACN3:  return AudioChannelSet::ambisonicACN3;
            case typename VstkSpeakerACN4:  return AudioChannelSet::ambisonicACN4;
            case typename VstkSpeakerACN5:  return AudioChannelSet::ambisonicACN5;
            case typename VstkSpeakerACN6:  return AudioChannelSet::ambisonicACN6;
            case typename VstkSpeakerACN7:  return AudioChannelSet::ambisonicACN7;
            case typename VstkSpeakerACN8:  return AudioChannelSet::ambisonicACN8;
            case typename VstkSpeakerACN9:  return AudioChannelSet::ambisonicACN9;
            case typename VstkSpeakerACN10: return AudioChannelSet::ambisonicACN10;
            case typename VstkSpeakerACN11: return AudioChannelSet::ambisonicACN11;
            case typename VstkSpeakerACN12: return AudioChannelSet::ambisonicACN12;
            case typename VstkSpeakerACN13: return AudioChannelSet::ambisonicACN13;
            case typename VstkSpeakerACN14: return AudioChannelSet::ambisonicACN14;
            case typename VstkSpeakerACN15: return AudioChannelSet::ambisonicACN15;
            case typename VstkSpeakerTsl:   return AudioChannelSet::topSideLeft;
            case typename VstkSpeakerTsr:   return AudioChannelSet::topSideRight;
            case typename VstkSpeakerLcs:   return AudioChannelSet::leftSurroundRear;
            case typename VstkSpeakerRcs:   return AudioChannelSet::rightSurroundRear;
            case typename VstkSpeakerBfl:   return AudioChannelSet::bottomFrontLeft;
            case typename VstkSpeakerBfc:   return AudioChannelSet::bottomFrontCentre;
            case typename VstkSpeakerBfr:   return AudioChannelSet::bottomFrontRight;
            case typename VstkSpeakerPl:    return AudioChannelSet::wideLeft;
            case typename VstkSpeakerPr:    return AudioChannelSet::wideRight;
            case typename VstkSpeakerBsl:   return AudioChannelSet::bottomSideLeft;
            case typename VstkSpeakerBsr:   return AudioChannelSet::bottomSideRight;
            case typename VstkSpeakerBrl:   return AudioChannelSet::bottomRearLeft;
            case typename VstkSpeakerBrc:   return AudioChannelSet::bottomRearCentre;
            case typename VstkSpeakerBrr:   return AudioChannelSet::bottomRearRight;
            default: break;
        }

        auto channelType = BigInteger (static_cast<int64> (type)).findNextSetBit (0);

        // Vst3 <-> Aloe layout conversion error: report this bug to the Aloe forum
        jassert (channelType >= 33);

        return static_cast<AudioChannelSet::ChannelType> (static_cast<int> (AudioChannelSet::discreteChannel0) + 6 + (channelType - 33));
        */
}
