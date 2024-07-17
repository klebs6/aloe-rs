crate::ix!();

/**
  | In Apple's official definition the
  | following tags exist with the same speaker
  | layout and order even when *not*
  | represented in Aloe channels
  |
  | kAudioChannelLayoutTag_Binaural = kAudioChannelLayoutTag_Stereo
  | kAudioChannelLayoutTag_MPEG_5_0_B = kAudioChannelLayoutTag_Pentagonal
  | kAudioChannelLayoutTag_ITU_2_2 = kAudioChannelLayoutTag_Quadraphonic
  | kAudioChannelLayoutTag_AudioUnit_6_0 = kAudioChannelLayoutTag_Hexagonal
  */
pub struct CoreAudioLayoutsSpeakerLayoutTable {
    base: AudioChannelSet,
}

impl CoreAudioLayoutsSpeakerLayoutTable {

    /**
       | This list has been derived from
       | https://pastebin.com/24dQ4BPJ Apple channel
       | labels have been replaced by Aloe channel
       | names This means that some layouts will be
       | identical in Aloe but not in CoreAudio
       */
    pub fn get() -> *mut CoreAudioLayoutsLayoutTagSpeakerList {
        
        todo!();
        /*
            static CoreAudioLayoutsLayoutTagSpeakerList tbl[] = {
                    // list layouts for which there is a corresponding named AudioChannelSet first
                    { kAudioChannelLayoutTag_Mono, { centre } },
                    { kAudioChannelLayoutTag_Stereo, { left, right } },
                    { kAudioChannelLayoutTag_MPEG_3_0_A, { left, right, centre } },
                    { kAudioChannelLayoutTag_ITU_2_1, { left, right, centreSurround } },
                    { kAudioChannelLayoutTag_MPEG_4_0_A, { left, right, centre, centreSurround } },
                    { kAudioChannelLayoutTag_MPEG_5_0_A, { left, right, centre, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_MPEG_5_1_A, { left, right, centre, LFE, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_AudioUnit_6_0, { left, right, leftSurround, rightSurround, centre, centreSurround } },
                    { kAudioChannelLayoutTag_MPEG_6_1_A, { left, right, centre, LFE, leftSurround, rightSurround, centreSurround } },
                    { kAudioChannelLayoutTag_DTS_6_0_A, { leftSurroundSide, rightSurroundSide, left, right, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_DTS_6_1_A, { leftSurroundSide, rightSurroundSide, left, right, leftSurround, rightSurround, LFE } },
                    { kAudioChannelLayoutTag_AudioUnit_7_0, { left, right, leftSurroundSide, rightSurroundSide, centre, leftSurroundRear, rightSurroundRear } },
                    { kAudioChannelLayoutTag_AudioUnit_7_0_Front, { left, right, leftSurround, rightSurround, centre, leftCentre, rightCentre } },
                    { kAudioChannelLayoutTag_MPEG_7_1_C, { left, right, centre, LFE, leftSurroundSide, rightSurroundSide, leftSurroundRear, rightSurroundRear } },
                    { kAudioChannelLayoutTag_MPEG_7_1_A, { left, right, centre, LFE, leftSurround, rightSurround, leftCentre, rightCentre } },
                    { kAudioChannelLayoutTag_Ambisonic_B_Format, { ambisonicW, ambisonicX, ambisonicY, ambisonicZ } },
                    { kAudioChannelLayoutTag_Quadraphonic, { left, right, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_Pentagonal, { left, right, leftSurroundRear, rightSurroundRear, centre } },
                    { kAudioChannelLayoutTag_Hexagonal, { left, right, leftSurroundRear, rightSurroundRear, centre, centreSurround } },
                    { kAudioChannelLayoutTag_Octagonal, { left, right, leftSurround, rightSurround, centre, centreSurround, wideLeft, wideRight } },

                    // more uncommon layouts
                    { kAudioChannelLayoutTag_StereoHeadphones, { left, right } },
                    { kAudioChannelLayoutTag_MatrixStereo, { left, right } },
                    { kAudioChannelLayoutTag_MidSide, { centre, discreteChannel0 } },
                    { kAudioChannelLayoutTag_XY, { ambisonicX, ambisonicY } },
                    { kAudioChannelLayoutTag_Binaural, { left, right } },
                    { kAudioChannelLayoutTag_Cube, { left, right, leftSurround, rightSurround, topFrontLeft, topFrontRight, topRearLeft, topRearRight } },
                    { kAudioChannelLayoutTag_MPEG_3_0_B, { centre, left, right } },
                    { kAudioChannelLayoutTag_MPEG_4_0_B, { centre, left, right, centreSurround } },
                    { kAudioChannelLayoutTag_MPEG_5_0_B, { left, right, leftSurround, rightSurround, centre } },
                    { kAudioChannelLayoutTag_MPEG_5_0_C, { left, centre, right, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_MPEG_5_0_D, { centre, left, right, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_MPEG_5_1_B, { left, right, leftSurround, rightSurround, centre, LFE } },
                    { kAudioChannelLayoutTag_MPEG_5_1_C, { left, centre, right, leftSurround, rightSurround, LFE } },
                    { kAudioChannelLayoutTag_MPEG_5_1_D, { centre, left, right, leftSurround, rightSurround, LFE } },
                    { kAudioChannelLayoutTag_MPEG_7_1_B, { centre, leftCentre, rightCentre, left, right, leftSurround, rightSurround, LFE } },
                    { kAudioChannelLayoutTag_Emagic_Default_7_1, { left, right, leftSurround, rightSurround, centre, LFE, leftCentre, rightCentre } },
                    { kAudioChannelLayoutTag_SMPTE_DTV, { left, right, centre, LFE, leftSurround, rightSurround, discreteChannel0 /* leftMatrixTotal */, (ChannelType) (discreteChannel0 + 1) /* rightMatrixTotal */} },
                    { kAudioChannelLayoutTag_ITU_2_2, { left, right, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_DVD_4, { left, right, LFE } },
                    { kAudioChannelLayoutTag_DVD_5, { left, right, LFE, centreSurround } },
                    { kAudioChannelLayoutTag_DVD_6, { left, right, LFE, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_DVD_10, { left, right, centre, LFE } },
                    { kAudioChannelLayoutTag_DVD_11, { left, right, centre, LFE, centreSurround } },
                    { kAudioChannelLayoutTag_DVD_18, { left, right, leftSurround, rightSurround, LFE } },
                    { kAudioChannelLayoutTag_AAC_6_0, { centre, left, right, leftSurround, rightSurround, centreSurround } },
                    { kAudioChannelLayoutTag_AAC_6_1, { centre, left, right, leftSurround, rightSurround, centreSurround, LFE } },
                    { kAudioChannelLayoutTag_AAC_7_0, { centre, left, right, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear } },
                    { kAudioChannelLayoutTag_AAC_7_1_B, { centre, left, right, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear, LFE } },
                    { kAudioChannelLayoutTag_AAC_7_1_C, { centre, left, right, leftSurround, rightSurround, LFE, topFrontLeft, topFrontRight } },
                    { kAudioChannelLayoutTag_AAC_Octagonal, { centre, left, right, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear, centreSurround } },
                    { kAudioChannelLayoutTag_TMH_10_2_std, { left, right, centre, topFrontCentre, leftSurroundSide, rightSurroundSide, leftSurround, rightSurround, topFrontLeft, topFrontRight, wideLeft, wideRight, topRearCentre, centreSurround, LFE, LFE2 } },
                    { kAudioChannelLayoutTag_AC3_1_0_1, { centre, LFE } },
                    { kAudioChannelLayoutTag_AC3_3_0, { left, centre, right } },
                    { kAudioChannelLayoutTag_AC3_3_1, { left, centre, right, centreSurround } },
                    { kAudioChannelLayoutTag_AC3_3_0_1, { left, centre, right, LFE } },
                    { kAudioChannelLayoutTag_AC3_2_1_1, { left, right, centreSurround, LFE } },
                    { kAudioChannelLayoutTag_AC3_3_1_1, { left, centre, right, centreSurround, LFE } },
                    { kAudioChannelLayoutTag_EAC_6_0_A, { left, centre, right, leftSurround, rightSurround, centreSurround } },
                    { kAudioChannelLayoutTag_EAC_7_0_A, { left, centre, right, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear } },
                    { kAudioChannelLayoutTag_EAC3_6_1_A, { left, centre, right, leftSurround, rightSurround, LFE, centreSurround } },
                    { kAudioChannelLayoutTag_EAC3_6_1_B, { left, centre, right, leftSurround, rightSurround, LFE, centreSurround } },
                    { kAudioChannelLayoutTag_EAC3_6_1_C, { left, centre, right, leftSurround, rightSurround, LFE, topFrontCentre } },
                    { kAudioChannelLayoutTag_EAC3_7_1_A, { left, centre, right, leftSurround, rightSurround, LFE, leftSurroundRear, rightSurroundRear } },
                    { kAudioChannelLayoutTag_EAC3_7_1_B, { left, centre, right, leftSurround, rightSurround, LFE, leftCentre, rightCentre } },
                    { kAudioChannelLayoutTag_EAC3_7_1_C, { left, centre, right, leftSurround, rightSurround, LFE, leftSurroundSide, rightSurroundSide } },
                    { kAudioChannelLayoutTag_EAC3_7_1_D, { left, centre, right, leftSurround, rightSurround, LFE, wideLeft, wideRight } },
                    { kAudioChannelLayoutTag_EAC3_7_1_E, { left, centre, right, leftSurround, rightSurround, LFE, topFrontLeft, topFrontRight } },
                    { kAudioChannelLayoutTag_EAC3_7_1_F, { left, centre, right, leftSurround, rightSurround, LFE, centreSurround, topMiddle } },
                    { kAudioChannelLayoutTag_EAC3_7_1_G, { left, centre, right, leftSurround, rightSurround, LFE, centreSurround, topFrontCentre } },
                    { kAudioChannelLayoutTag_EAC3_7_1_H, { left, centre, right, leftSurround, rightSurround, LFE, centreSurround, topFrontCentre } },
                    { kAudioChannelLayoutTag_DTS_3_1, { centre, left, right, LFE } },
                    { kAudioChannelLayoutTag_DTS_4_1, { centre, left, right, centreSurround, LFE } },
                    { kAudioChannelLayoutTag_DTS_6_0_B, { centre, left, right, leftSurroundRear, rightSurroundRear, centreSurround } },
                    { kAudioChannelLayoutTag_DTS_6_0_C, { centre, centreSurround, left, right, leftSurroundRear, rightSurroundRear } },
                    { kAudioChannelLayoutTag_DTS_6_1_B, { centre, left, right, leftSurroundRear, rightSurroundRear, centreSurround, LFE } },
                    { kAudioChannelLayoutTag_DTS_6_1_C, { centre, centreSurround, left, right, leftSurroundRear, rightSurroundRear, LFE } },
                    { kAudioChannelLayoutTag_DTS_6_1_D, { centre, left, right, leftSurround, rightSurround, LFE, centreSurround } },
                    { kAudioChannelLayoutTag_DTS_7_0, { leftCentre, centre, rightCentre, left, right, leftSurround, rightSurround } },
                    { kAudioChannelLayoutTag_DTS_7_1, { leftCentre, centre, rightCentre, left, right, leftSurround, rightSurround, LFE } },
                    { kAudioChannelLayoutTag_DTS_8_0_A, { leftCentre, rightCentre, left, right, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear } },
                    { kAudioChannelLayoutTag_DTS_8_0_B, { leftCentre, centre, rightCentre, left, right, leftSurround, centreSurround, rightSurround } },
                    { kAudioChannelLayoutTag_DTS_8_1_A, { leftCentre, rightCentre, left, right, leftSurround, rightSurround, leftSurroundRear, rightSurroundRear, LFE } },
                    { kAudioChannelLayoutTag_DTS_8_1_B, { leftCentre, centre, rightCentre, left, right, leftSurround, centreSurround, rightSurround, LFE } },
                    { 0, {} }
                };

                return tbl;
        */
    }
}

