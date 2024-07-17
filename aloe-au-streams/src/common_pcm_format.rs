crate::ix!();

lazy_static!{
    /*
    static const AudioStreamBasicDescription    sEmpty;
    static const char *sTextParsingUsageString;
    const AudioStreamBasicDescription   CAStreamBasicDescription::sEmpty = { 0.0, 0, 0, 0, 0, 0, 0, 0, 0 };

    const char *CAStreamBasicDescription::sTextParsingUsageString =
    "format[@sample_rate_hz][/format_flags][#frames_per_packet][:LHbytesPerFrame][,channelsDI].\n"
    "Format for PCM is [-][BE|LE]{F|I|UI}{bitdepth}; else a 4-char format code (e.g. aac, alac).\n";
    */
}

pub enum CAStreamBasicDescriptionCommonPCMFormat {
    kPCMFormatOther     = 0,
    kPCMFormatFloat32   = 1,
    kPCMFormatInt16     = 2,
    kPCMFormatFixed824  = 3,
    kPCMFormatFloat64   = 4
}
