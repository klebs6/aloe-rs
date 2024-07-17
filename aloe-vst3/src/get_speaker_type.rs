crate::ix!();

pub fn get_speaker_type(
    set: &AudioChannelSet,
    ty:  AudioChannelSetChannelType

) -> Speaker {
    
    todo!();
        /*
            switch (type)
        {
            case AudioChannelSet::left:              return typename VstkSpeakerL;
            case AudioChannelSet::right:             return typename VstkSpeakerR;
            case AudioChannelSet::centre:            return (set == AudioChannelSet::mono() ? typename VstkSpeakerM : typename VstkSpeakerC);

            case AudioChannelSet::LFE:               return typename VstkSpeakerLfe;
            case AudioChannelSet::leftSurround:      return typename VstkSpeakerLs;
            case AudioChannelSet::rightSurround:     return typename VstkSpeakerRs;
            case AudioChannelSet::leftCentre:        return typename VstkSpeakerLc;
            case AudioChannelSet::rightCentre:       return typename VstkSpeakerRc;
            case AudioChannelSet::centreSurround:    return typename VstkSpeakerCs;
            case AudioChannelSet::leftSurroundSide:  return typename VstkSpeakerSl;
            case AudioChannelSet::rightSurroundSide: return typename VstkSpeakerSr;
            case AudioChannelSet::topMiddle:         return (1ull << 11); /* kSpeakerTm */
            case AudioChannelSet::topFrontLeft:      return typename VstkSpeakerTfl;
            case AudioChannelSet::topFrontCentre:    return typename VstkSpeakerTfc;
            case AudioChannelSet::topFrontRight:     return typename VstkSpeakerTfr;
            case AudioChannelSet::topRearLeft:       return typename VstkSpeakerTrl;
            case AudioChannelSet::topRearCentre:     return typename VstkSpeakerTrc;
            case AudioChannelSet::topRearRight:      return typename VstkSpeakerTrr;
            case AudioChannelSet::LFE2:              return typename VstkSpeakerLfe2;
            case AudioChannelSet::leftSurroundRear:  return typename VstkSpeakerLcs;
            case AudioChannelSet::rightSurroundRear: return typename VstkSpeakerRcs;
            case AudioChannelSet::wideLeft:          return typename VstkSpeakerPl;
            case AudioChannelSet::wideRight:         return typename VstkSpeakerPr;
            case AudioChannelSet::ambisonicACN0:     return typename VstkSpeakerACN0;
            case AudioChannelSet::ambisonicACN1:     return typename VstkSpeakerACN1;
            case AudioChannelSet::ambisonicACN2:     return typename VstkSpeakerACN2;
            case AudioChannelSet::ambisonicACN3:     return typename VstkSpeakerACN3;
            case AudioChannelSet::ambisonicACN4:     return typename VstkSpeakerACN4;
            case AudioChannelSet::ambisonicACN5:     return typename VstkSpeakerACN5;
            case AudioChannelSet::ambisonicACN6:     return typename VstkSpeakerACN6;
            case AudioChannelSet::ambisonicACN7:     return typename VstkSpeakerACN7;
            case AudioChannelSet::ambisonicACN8:     return typename VstkSpeakerACN8;
            case AudioChannelSet::ambisonicACN9:     return typename VstkSpeakerACN9;
            case AudioChannelSet::ambisonicACN10:    return typename VstkSpeakerACN10;
            case AudioChannelSet::ambisonicACN11:    return typename VstkSpeakerACN11;
            case AudioChannelSet::ambisonicACN12:    return typename VstkSpeakerACN12;
            case AudioChannelSet::ambisonicACN13:    return typename VstkSpeakerACN13;
            case AudioChannelSet::ambisonicACN14:    return typename VstkSpeakerACN14;
            case AudioChannelSet::ambisonicACN15:    return typename VstkSpeakerACN15;
            case AudioChannelSet::topSideLeft:       return typename VstkSpeakerTsl;
            case AudioChannelSet::topSideRight:      return typename VstkSpeakerTsr;
            case AudioChannelSet::bottomFrontLeft:   return typename VstkSpeakerBfl;
            case AudioChannelSet::bottomFrontCentre: return typename VstkSpeakerBfc;
            case AudioChannelSet::bottomFrontRight:  return typename VstkSpeakerBfr;
            case AudioChannelSet::bottomSideLeft:    return typename VstkSpeakerBsl;
            case AudioChannelSet::bottomSideRight:   return typename VstkSpeakerBsr;
            case AudioChannelSet::bottomRearLeft:    return typename VstkSpeakerBrl;
            case AudioChannelSet::bottomRearCentre:  return typename VstkSpeakerBrc;
            case AudioChannelSet::bottomRearRight:   return typename VstkSpeakerBrr;

            case AudioChannelSet::discreteChannel0:  return typename VstkSpeakerM;

            case AudioChannelSet::ambisonicACN16:
            case AudioChannelSet::ambisonicACN17:
            case AudioChannelSet::ambisonicACN18:
            case AudioChannelSet::ambisonicACN19:
            case AudioChannelSet::ambisonicACN20:
            case AudioChannelSet::ambisonicACN21:
            case AudioChannelSet::ambisonicACN22:
            case AudioChannelSet::ambisonicACN23:
            case AudioChannelSet::ambisonicACN24:
            case AudioChannelSet::ambisonicACN25:
            case AudioChannelSet::ambisonicACN26:
            case AudioChannelSet::ambisonicACN27:
            case AudioChannelSet::ambisonicACN28:
            case AudioChannelSet::ambisonicACN29:
            case AudioChannelSet::ambisonicACN30:
            case AudioChannelSet::ambisonicACN31:
            case AudioChannelSet::ambisonicACN32:
            case AudioChannelSet::ambisonicACN33:
            case AudioChannelSet::ambisonicACN34:
            case AudioChannelSet::ambisonicACN35:
            case AudioChannelSet::proximityLeft:
            case AudioChannelSet::proximityRight:
            case AudioChannelSet::unknown:
            default:
                break;
        }

        auto channelIndex = static_cast<typename VstSpeaker> (type) - (static_cast<typename VstSpeaker> (AudioChannelSet::discreteChannel0) + 6ull);
        return (1ull << (channelIndex + 33ull /* last speaker in vst layout + 1 */));
        */
}
