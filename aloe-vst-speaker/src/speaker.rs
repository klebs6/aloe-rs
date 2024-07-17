/*!
  | \defgroup speakerArrangements Speaker
  | Arrangements \image html "vst3_speaker_types.jpg"
  | \n
  | 
  | A SpeakerArrangement is a bitset combination
  | of speakers. For example:
  | 
  | -----------
  | @code
  | 
  | const SpeakerArrangement kStereo = kSpeakerL | kSpeakerR; // => hex: 0x03 / binary: 0011.
  | \see IAudioProcessor::getBusArrangement
  | () and IAudioProcessor::setBusArrangements
  | ()
  |
  | Speaker Definitions. \ingroup speakerArrangements
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/vstspeaker.h]

//pub const SPEAKER_ARR_SPEAKERL:   Speaker = 1 << 0;    // Left (L)
pub const SPEAKER_L:                Speaker = 1 << 0;    // Left (L)
pub const SPEAKER_R:                Speaker = 1 << 1;    // Right (R)
pub const SPEAKER_C:                Speaker = 1 << 2;    // Center (C)
pub const SPEAKER_LFE:              Speaker = 1 << 3;    // Subbass (Lfe)
pub const SPEAKER_LS:               Speaker = 1 << 4;    // Left Surround (Ls)
pub const SPEAKER_RS:               Speaker = 1 << 5;    // Right Surround (Rs)
pub const SPEAKER_LC:               Speaker = 1 << 6;    // Left of Center (Lc) - Front Left Center
pub const SPEAKER_RC:               Speaker = 1 << 7;    // Right of Center (Rc) - Front Right Center
pub const SPEAKER_S:                Speaker = 1 << 8;    // Surround (S)
pub const SPEAKER_CS:               Speaker = SPEAKER_S; // Center of Surround (Cs) - Back Center - Surround (S)
pub const SPEAKER_SL:               Speaker = 1 << 9;    // Side Left (Sl)
pub const SPEAKER_SR:               Speaker = 1 << 10;   // Side Right (Sr)
pub const SPEAKER_TC:               Speaker = 1 << 11;   // Top Center Over-head, Top Middle (Tc)
pub const SPEAKER_TFL:              Speaker = 1 << 12;   // Top Front Left (Tfl)
pub const SPEAKER_TFC:              Speaker = 1 << 13;   // Top Front Center (Tfc)
pub const SPEAKER_TFR:              Speaker = 1 << 14;   // Top Front Right (Tfr)
pub const SPEAKER_TRL:              Speaker = 1 << 15;   // Top Rear/Back Left (Trl)
pub const SPEAKER_TRC:              Speaker = 1 << 16;   // Top Rear/Back Center (Trc)
pub const SPEAKER_TRR:              Speaker = 1 << 17;   // Top Rear/Back Right (Trr)
pub const SPEAKER_LFE2:             Speaker = 1 << 18;   // Subbass 2 (Lfe2)
pub const SPEAKER_M:                Speaker = 1 << 19;   // Mono (M)
pub const SPEAKER_ACN0:             Speaker = 1 << 20;   // Ambisonic ACN 0
pub const SPEAKER_ACN1:             Speaker = 1 << 21;   // Ambisonic ACN 1
pub const SPEAKER_ACN2:             Speaker = 1 << 22;   // Ambisonic ACN 2
pub const SPEAKER_ACN3:             Speaker = 1 << 23;   // Ambisonic ACN 3
pub const SPEAKER_ACN4:             Speaker = 1 << 38;   // Ambisonic ACN 4
pub const SPEAKER_ACN5:             Speaker = 1 << 39;   // Ambisonic ACN 5
pub const SPEAKER_ACN6:             Speaker = 1 << 40;   // Ambisonic ACN 6
pub const SPEAKER_ACN7:             Speaker = 1 << 41;   // Ambisonic ACN 7
pub const SPEAKER_ACN8:             Speaker = 1 << 42;   // Ambisonic ACN 8
pub const SPEAKER_ACN9:             Speaker = 1 << 43;   // Ambisonic ACN 9
pub const SPEAKER_ACN10:            Speaker = 1 << 44;   // Ambisonic ACN 10
pub const SPEAKER_ACN11:            Speaker = 1 << 45;   // Ambisonic ACN 11
pub const SPEAKER_ACN12:            Speaker = 1 << 46;   // Ambisonic ACN 12
pub const SPEAKER_ACN13:            Speaker = 1 << 47;   // Ambisonic ACN 13
pub const SPEAKER_ACN14:            Speaker = 1 << 48;   // Ambisonic ACN 14
pub const SPEAKER_ACN15:            Speaker = 1 << 49;   // Ambisonic ACN 15
pub const SPEAKER_TSL:              Speaker = 1 << 24;   // Top Side Left (Tsl)
pub const SPEAKER_TSR:              Speaker = 1 << 25;   // Top Side Right (Tsr)
pub const SPEAKER_LCS:              Speaker = 1 << 26;   // Left of Center Surround (Lcs) - Back Left Center
pub const SPEAKER_RCS:              Speaker = 1 << 27;   // Right of Center Surround (Rcs) - Back Right Center
pub const SPEAKER_BFL:              Speaker = 1 << 28;   // Bottom Front Left (Bfl)
pub const SPEAKER_BFC:              Speaker = 1 << 29;   // Bottom Front Center (Bfc)
pub const SPEAKER_BFR:              Speaker = 1 << 30;   // Bottom Front Right (Bfr)
pub const SPEAKER_PL:               Speaker = 1 << 31;   // Proximity Left (Pl)
pub const SPEAKER_PR:               Speaker = 1 << 32;   // Proximity Right (Pr)
pub const SPEAKER_BSL:              Speaker = 1 << 33;   // Bottom Side Left (Bsl)
pub const SPEAKER_BSR:              Speaker = 1 << 34;   // Bottom Side Right (Bsr)
pub const SPEAKER_BRL:              Speaker = 1 << 35;   // Bottom Rear Left (Brl)
pub const SPEAKER_BRC:              Speaker = 1 << 36;   // Bottom Rear Center (Brc)
pub const SPEAKER_BRR:              Speaker = 1 << 37;   // Bottom Rear Right (Brr)
