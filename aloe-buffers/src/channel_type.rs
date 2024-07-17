crate::ix!();

pub type AudioChannelSetChannelType = AudioChannelType; //TODO is this correct?

/**
  | Represents different audio channel
  | types.
  |
  */
pub enum AudioChannelType
{
    /**
      | Unknown channel type.
      |
      */
    unknown, 

    /**
      | L channel.
      |
      */
    left, 

    /**
      | R channel.
      |
      */
    right, 

    /**
      | C channel. (Sometimes M for mono)
      |
      */
    centre, 

    /**
      | LFE channel.
      |
      */
    LFE,              

    /**
      | Ls channel.
      |
      */
    leftSurround,              

    /**
      | Rs channel.
      |
      */
    rightSurround,              

    /**
      | Lc (AAX/VST), Lc used as Lss in AU for
      | most layouts.
      |
      */
    leftCentre,              

    /**
      | Rc (AAX/VST), Rc used as Rss in AU for
      | most layouts.
      |
      */
    rightCentre,              

    /**
      | Cs/S channel.
      |
      */
    centreSurround,              

    /**
      | Same as Centre Surround channel.
      |
      */
    surround, 

    /**
      | Lss (AXX), Side Left "Sl" (VST), Left
      | Centre "LC" (AU) channel.
      |
      */
    leftSurroundSide,             

    /**
      | Rss (AXX), Side right "Sr" (VST),
      | Right Centre "Rc" (AU) channel.
      |
      */
    rightSurroundSide,             

    /**
      | Top Middle channel.
      |
      */
    topMiddle,             

    /**
      | Top Front Left channel.
      |
      */
    topFrontLeft,             

    /**
      | Top Front Centre channel.
      |
      */
    topFrontCentre,             

    /**
      | Top Front Right channel.
      |
      */
    topFrontRight,             

    /**
      | Top Rear Left channel.
      |
      */
    topRearLeft,             

    /**
      | Top Rear Centre channel.
      |
      */
    topRearCentre,             

    /**
      | Top Rear Right channel.
      |
      */
    topRearRight,             

    /**
      | Second LFE channel.
      |
      */
    LFE2,             

    /**
      | Lsr (AAX), Lcs (VST), Rls (AU) channel.
      |
      */
    leftSurroundRear,             

    /**
      | Rsr (AAX), Rcs (VST), Rrs (AU) channel.
      |
      */
    rightSurroundRear,             

    /**
      | Wide Left channel.
      |
      */
    wideLeft,             

    /**
      | Wide Right channel.
      |
      */
    wideRight,             

    /* ------ * Used by Dolby Atmos 7.0.2 and 7.1.2  ------ */

    /**
      | Lts (AAX), Tsl (VST) channel for Dolby
      | Atmos.
      |
      */
    topSideLeft, 

    /**
      | Rts (AAX), Tsr (VST) channel for Dolby
      | Atmos.
      |
      */
    topSideRight, 

    /* Ambisonic ACN formats - all channels are SN3D normalised */

    /* ----- * zero-th and first-order ambisonic ACN  ----- */

    /**
      | Zero-th ambisonic channel number 0.
      |
      */
    ambisonicACN0, 

    /**
      | First-order ambisonic channel number
      | 1.
      |
      */
    ambisonicACN1, 

    /**
      | First-order ambisonic channel number
      | 2.
      |
      */
    ambisonicACN2, 

    /**
      | First-order ambisonic channel number
      | 3.
      |
      */
    ambisonicACN3, 

    /* ------------ * second-order ambisonic  ------------ */

    /**
      | Second-order ambisonic channel number
      | 4.
      |
      */
    ambisonicACN4, 

    /**
      | Second-order ambisonic channel number
      | 5.
      |
      */
    ambisonicACN5, 

    /**
      | Second-order ambisonic channel number
      | 6.
      |
      */
    ambisonicACN6, 

    /**
      | Second-order ambisonic channel number
      | 7.
      |
      */
    ambisonicACN7, 

    /**
      | Second-order ambisonic channel number
      | 8.
      |
      */
    ambisonicACN8, 

    /* ------------- * third-order ambisonic  ------------- */

    /**
      | Third-order ambisonic channel number
      | 9.
      |
      */
    ambisonicACN9, 

    /**
      | Third-order ambisonic channel number
      | 10.
      |
      */
    ambisonicACN10, 

    /**
      | Third-order ambisonic channel number
      | 11.
      |
      */
    ambisonicACN11, 

    /**
      | Third-order ambisonic channel number
      | 12.
      |
      */
    ambisonicACN12, 

    /**
      | Third-order ambisonic channel number
      | 13.
      |
      */
    ambisonicACN13, 

    /**
      | Third-order ambisonic channel number
      | 14.
      |
      */
    ambisonicACN14, 

    /**
      | Third-order ambisonic channel number
      | 15.
      |
      */
    ambisonicACN15, 

    /* ------------ * fourth-order ambisonic  ------------ */

    /**
      | Fourth-order ambisonic channel number
      | 16.
      |
      */
    ambisonicACN16, 

    /**
      | Fourth-order ambisonic channel number
      | 17.
      |
      */
    ambisonicACN17, 

    /**
      | Fourth-order ambisonic channel number
      | 18.
      |
      */
    ambisonicACN18, 

    /**
      | Fourth-order ambisonic channel number
      | 19.
      |
      */
    ambisonicACN19, 

    /**
      | Fourth-order ambisonic channel number
      | 20.
      |
      */
    ambisonicACN20, 

    /**
      | Fourth-order ambisonic channel number
      | 21.
      |
      */
    ambisonicACN21, 

    /**
      | Fourth-order ambisonic channel number
      | 22.
      |
      */
    ambisonicACN22, 

    /**
      | Fourth-order ambisonic channel number
      | 23.
      |
      */
    ambisonicACN23, 

    /**
      | Fourth-order ambisonic channel number
      | 24.
      |
      */
    ambisonicACN24, 

    /* ------------- * fifth-order ambisonic  ------------- */

    /**
      | Fifth-order ambisonic channel number
      | 25.
      |
      */
    ambisonicACN25, 

    /**
      | Fifth-order ambisonic channel number
      | 26.
      |
      */
    ambisonicACN26, 

    /**
      | Fifth-order ambisonic channel number
      | 27.
      |
      */
    ambisonicACN27, 

    /**
      | Fifth-order ambisonic channel number
      | 28.
      |
      */
    ambisonicACN28, 

    /**
      | Fifth-order ambisonic channel number
      | 29.
      |
      */
    ambisonicACN29, 

    /**
      | Fifth-order ambisonic channel number
      | 30.
      |
      */
    ambisonicACN30, 

    /**
      | Fifth-order ambisonic channel number
      | 31.
      |
      */
    ambisonicACN31, 

    /**
      | Fifth-order ambisonic channel number
      | 32.
      |
      */
    ambisonicACN32, 

    /**
      | Fifth-order ambisonic channel number
      | 33.
      |
      */
    ambisonicACN33, 

    /**
      | Fifth-order ambisonic channel number
      | 34.
      |
      */
    ambisonicACN34, 

    /**
      | Fifth-order ambisonic channel number
      | 35.
      |
      */
    ambisonicACN35, 

    /**
      | Same as zero-th ambisonic channel number
      | 0.
      |
      */
    ambisonicW, 

    /**
      | Same as first-order ambisonic channel
      | number 3.
      |
      */
    ambisonicX, 

    /**
      | Same as first-order ambisonic channel
      | number 1.
      |
      */
    ambisonicY, 

    /**
      | Same as first-order ambisonic channel
      | number 2.
      |
      */
    ambisonicZ, 

    /**
      | Bottom Front Left (Bfl)
      |
      */
    bottomFrontLeft, 

    /**
      | Bottom Front Centre (Bfc)
      |
      */
    bottomFrontCentre, 

    /**
      | Bottom Front Right (Bfr)
      |
      */
    bottomFrontRight, 

    /**
      | Proximity Left (Pl)
      |
      */
    proximityLeft, 

    /**
      | Proximity Right (Pr)
      |
      */
    proximityRight, 

    /**
      | Bottom Side Left (Bsl)
      |
      */
    bottomSideLeft, 

    /**
      | Bottom Side Right (Bsr)
      |
      */
    bottomSideRight, 

    /**
      | Bottom Rear Left (Brl)
      |
      */
    bottomRearLeft, 

    /**
      | Bottom Rear Center (Brc)
      |
      */
    bottomRearCentre, 

    /**
      | Bottom Rear Right (Brr)
      |
      */
    bottomRearRight, 

    /**
      | Non-typed individual channels are
      | indexed upwards from this value.
      |
      */
    discreteChannel0
}

impl AudioChannelType {

    pub fn value(&self) -> usize {
        match self {
            AudioChannelType::unknown           => 0,
            AudioChannelType::left              => 1,
            AudioChannelType::right             => 2,
            AudioChannelType::centre            => 3,
            AudioChannelType::LFE               => 4,
            AudioChannelType::leftSurround      => 5,
            AudioChannelType::rightSurround     => 6,
            AudioChannelType::leftCentre        => 7,
            AudioChannelType::rightCentre       => 8,
            AudioChannelType::centreSurround    => 9,
            AudioChannelType::surround          => AudioChannelType::centreSurround.value(),
            AudioChannelType::leftSurroundSide  => 10,
            AudioChannelType::rightSurroundSide => 11,
            AudioChannelType::topMiddle         => 12,
            AudioChannelType::topFrontLeft      => 13,
            AudioChannelType::topFrontCentre    => 14,
            AudioChannelType::topFrontRight     => 15,
            AudioChannelType::topRearLeft       => 16,
            AudioChannelType::topRearCentre     => 17,
            AudioChannelType::topRearRight      => 18,
            AudioChannelType::LFE2              => 19,
            AudioChannelType::leftSurroundRear  => 20,
            AudioChannelType::rightSurroundRear => 21,
            AudioChannelType::wideLeft          => 22,
            AudioChannelType::wideRight         => 23,
            AudioChannelType::topSideLeft       => 28,
            AudioChannelType::topSideRight      => 29,
            AudioChannelType::ambisonicACN0     => 24,
            AudioChannelType::ambisonicACN1     => 25,
            AudioChannelType::ambisonicACN2     => 26,
            AudioChannelType::ambisonicACN3     => 27,
            AudioChannelType::ambisonicACN4     => 30,
            AudioChannelType::ambisonicACN5     => 31,
            AudioChannelType::ambisonicACN6     => 32,
            AudioChannelType::ambisonicACN7     => 33,
            AudioChannelType::ambisonicACN8     => 34,
            AudioChannelType::ambisonicACN9     => 35,
            AudioChannelType::ambisonicACN10    => 36,
            AudioChannelType::ambisonicACN11    => 37,
            AudioChannelType::ambisonicACN12    => 38,
            AudioChannelType::ambisonicACN13    => 39,
            AudioChannelType::ambisonicACN14    => 40,
            AudioChannelType::ambisonicACN15    => 41,
            AudioChannelType::ambisonicACN16    => 42,
            AudioChannelType::ambisonicACN17    => 43,
            AudioChannelType::ambisonicACN18    => 44,
            AudioChannelType::ambisonicACN19    => 45,
            AudioChannelType::ambisonicACN20    => 46,
            AudioChannelType::ambisonicACN21    => 47,
            AudioChannelType::ambisonicACN22    => 48,
            AudioChannelType::ambisonicACN23    => 49,
            AudioChannelType::ambisonicACN24    => 50,
            AudioChannelType::ambisonicACN25    => 51,
            AudioChannelType::ambisonicACN26    => 52,
            AudioChannelType::ambisonicACN27    => 53,
            AudioChannelType::ambisonicACN28    => 54,
            AudioChannelType::ambisonicACN29    => 55,
            AudioChannelType::ambisonicACN30    => 56,
            AudioChannelType::ambisonicACN31    => 57,
            AudioChannelType::ambisonicACN32    => 58,
            AudioChannelType::ambisonicACN33    => 59,
            AudioChannelType::ambisonicACN34    => 60,
            AudioChannelType::ambisonicACN35    => 61,
            AudioChannelType::ambisonicW        => AudioChannelType::ambisonicACN0.value(),
            AudioChannelType::ambisonicX        => AudioChannelType::ambisonicACN3.value(),
            AudioChannelType::ambisonicY        => AudioChannelType::ambisonicACN1.value(),
            AudioChannelType::ambisonicZ        => AudioChannelType::ambisonicACN2.value(),
            AudioChannelType::bottomFrontLeft   => 62,
            AudioChannelType::bottomFrontCentre => 63,
            AudioChannelType::bottomFrontRight  => 64,
            AudioChannelType::proximityLeft     => 65,
            AudioChannelType::proximityRight    => 66,
            AudioChannelType::bottomSideLeft    => 67,
            AudioChannelType::bottomSideRight   => 68,
            AudioChannelType::bottomRearLeft    => 69,
            AudioChannelType::bottomRearCentre  => 70,
            AudioChannelType::bottomRearRight   => 71,
            AudioChannelType::discreteChannel0  => 128
        }
    }
}
