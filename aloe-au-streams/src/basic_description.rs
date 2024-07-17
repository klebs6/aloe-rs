crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAStreamBasicDescription.h]

/**
  |  CAStreamBasicDescription
  |
  |  This is a wrapper class for the
  |  AudioStreamBasicDescription struct.
  |
  |  It adds a number of convenience routines, but
  |  otherwise adds nothing to the footprint of the
  |  original struct.
  */
pub struct CAStreamBasicDescription {
    base: AudioStreamBasicDescription,
}

impl Ord for CAStreamBasicDescription {
    
    #[inline] fn cmp(&self, other: &CAStreamBasicDescription) -> std::cmp::Ordering {
        todo!();
        /*
            bool theAnswer = false;
        bool isDone = false;

        //  note that if either side is 0, that field is skipped

        //  format ID is the first order sort
        if((!isDone) && ((x.mFormatID != 0) && (y.mFormatID != 0)))
        {
            if(x.mFormatID != y.mFormatID)
            {
                //  formats are sorted numerically except that linear
                //  PCM is always first
                if(x.mFormatID == kAudioFormatLinearPCM)
                {
                    theAnswer = true;
                }
                else if(y.mFormatID == kAudioFormatLinearPCM)
                {
                    theAnswer = false;
                }
                else
                {
                    theAnswer = x.mFormatID < y.mFormatID;
                }
                isDone = true;
            }
        }

        //  mixable is always better than non-mixable for linear PCM and should be the second order sort item
        if((!isDone) && ((x.mFormatID == kAudioFormatLinearPCM) && (y.mFormatID == kAudioFormatLinearPCM)))
        {
            if(((x.mFormatFlags & kIsNonMixableFlag) == 0) && ((y.mFormatFlags & kIsNonMixableFlag) != 0))
            {
                theAnswer = true;
                isDone = true;
            }
            else if(((x.mFormatFlags & kIsNonMixableFlag) != 0) && ((y.mFormatFlags & kIsNonMixableFlag) == 0))
            {
                theAnswer = false;
                isDone = true;
            }
        }

        //  floating point vs integer for linear PCM only
        if((!isDone) && ((x.mFormatID == kAudioFormatLinearPCM) && (y.mFormatID == kAudioFormatLinearPCM)))
        {
            if((x.mFormatFlags & kAudioFormatFlagIsFloat) != (y.mFormatFlags & kAudioFormatFlagIsFloat))
            {
                //  floating point is better than integer
                theAnswer = y.mFormatFlags & kAudioFormatFlagIsFloat;
                isDone = true;
            }
        }

        //  bit depth
        if((!isDone) && ((x.mBitsPerChannel != 0) && (y.mBitsPerChannel != 0)))
        {
            if(x.mBitsPerChannel != y.mBitsPerChannel)
            {
                //  deeper bit depths are higher quality
                theAnswer = x.mBitsPerChannel < y.mBitsPerChannel;
                isDone = true;
            }
        }

        //  sample rate
        if((!isDone) && fnonzero(x.mSampleRate) && fnonzero(y.mSampleRate))
        {
            if(fnotequal(x.mSampleRate, y.mSampleRate))
            {
                //  higher sample rates are higher quality
                theAnswer = x.mSampleRate < y.mSampleRate;
                isDone = true;
            }
        }

        //  number of channels
        if((!isDone) && ((x.mChannelsPerFrame != 0) && (y.mChannelsPerFrame != 0)))
        {
            if(x.mChannelsPerFrame != y.mChannelsPerFrame)
            {
                //  more channels is higher quality
                theAnswer = x.mChannelsPerFrame < y.mChannelsPerFrame;
                //isDone = true;
            }
        }

        return theAnswer;
        */
    }
}

impl PartialOrd<CAStreamBasicDescription> for CAStreamBasicDescription {

    #[inline] fn partial_cmp(&self, other: &CAStreamBasicDescription) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<CAStreamBasicDescription> for CAStreamBasicDescription {
    
    #[inline] fn eq(&self, other: &CAStreamBasicDescription) -> bool {
        todo!();
        /*
            //  the semantics for equality are:
        //      1) Values must match exactly -- except for PCM format flags, see above.
        //      2) wildcard's are ignored in the comparison

    #define MATCH(name) ((x.name) == 0 || (y.name) == 0 || (x.name) == (y.name))

        return
        //  check all but the format flags
        CAStreamBasicDescription::FlagIndependentEquivalence(x, y)
        //  check the format flags
        && MatchFormatFlags(x, y);
        */
    }
}

impl Eq for CAStreamBasicDescription {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAStreamBasicDescription.cpp]
impl Default for CAStreamBasicDescription {
    
    fn default() -> Self {
    
        todo!();
        /*
            memset (this, 0, sizeof(AudioStreamBasicDescription));
        */
    }
}

impl CAStreamBasicDescription {
    
    pub fn new(
        in_sample_rate:    f64,
        in_num_channels:   u32,
        pcmf:              CAStreamBasicDescriptionCommonPCMFormat,
        in_is_interleaved: bool) -> Self {
    
        todo!();
        /*


            unsigned wordsize;

            mSampleRate = inSampleRate;
            mFormatID = kAudioFormatLinearPCM;
            mFormatFlags = kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;
            mFramesPerPacket = 1;
            mChannelsPerFrame = inNumChannels;
            mBytesPerFrame = mBytesPerPacket = 0;
            mReserved = 0;

            switch (pcmf) {
            default:
                return;
            case kPCMFormatFloat32:
                wordsize = 4;
                mFormatFlags |= kAudioFormatFlagIsFloat;
                break;
            case kPCMFormatFloat64:
                wordsize = 8;
                mFormatFlags |= kAudioFormatFlagIsFloat;
                break;
            case kPCMFormatInt16:
                wordsize = 2;
                mFormatFlags |= kAudioFormatFlagIsSignedInteger;
                break;
            case kPCMFormatFixed824:
                wordsize = 4;
                mFormatFlags |= kAudioFormatFlagIsSignedInteger | (24 << kLinearPCMFormatFlagsSampleFractionShift);
                break;
            }
            mBitsPerChannel = wordsize * 8;
            if (inIsInterleaved)
                mBytesPerFrame = mBytesPerPacket = wordsize * inNumChannels;
            else {
                mFormatFlags |= kAudioFormatFlagIsNonInterleaved;
                mBytesPerFrame = mBytesPerPacket = wordsize;
            }
        */
    }
    
    pub fn assign_from(&mut self, v: &AudioStreamBasicDescription) -> &mut CAStreamBasicDescription {
        
        todo!();
        /*
            SetFrom(v); return *this;
        */
    }
    
    pub fn set_from(&mut self, desc: &AudioStreamBasicDescription)  {
        
        todo!();
        /*
            memcpy(this, &desc, sizeof(AudioStreamBasicDescription));
        */
    }
    
    pub fn from_text(&mut self, in_text_desc: *const u8) -> bool {
        
        todo!();
        /*
            return FromText(inTextDesc, *this);
        */
    }
    
    /* ----------------- interrogation  ----------------- */
    
    pub fn ispcm(&self) -> bool {
        
        todo!();
        /*
            return mFormatID == kAudioFormatLinearPCM;
        */
    }
    
    pub fn packedness_is_significant(&self) -> bool {
        
        todo!();
        /*
            Assert(IsPCM(), "PackednessIsSignificant only applies for PCM");
            return (SampleWordSize() << 3) != mBitsPerChannel;
        */
    }
    
    pub fn alignment_is_significant(&self) -> bool {
        
        todo!();
        /*
            return PackednessIsSignificant() || (mBitsPerChannel & 7) != 0;
        */
    }
    
    pub fn is_interleaved(&self) -> bool {
        
        todo!();
        /*
            return !(mFormatFlags & kAudioFormatFlagIsNonInterleaved);
        */
    }
    
    pub fn is_signed_integer(&self) -> bool {
        
        todo!();
        /*
            return IsPCM() && (mFormatFlags & kAudioFormatFlagIsSignedInteger);
        */
    }
    
    pub fn is_float(&self) -> bool {
        
        todo!();
        /*
            return IsPCM() && (mFormatFlags & kAudioFormatFlagIsFloat);
        */
    }
    
    pub fn is_native_endian(&self) -> bool {
        
        todo!();
        /*
            return (mFormatFlags & kAudioFormatFlagIsBigEndian) == kAudioFormatFlagsNativeEndian;
        */
    }

    /**
      | for sanity with interleaved/deinterleaved
      | possibilities, never access mChannelsPerFrame,
      | use these:
      |
      */
    pub fn number_interleaved_channels(&self) -> u32 {
        
        todo!();
        /*
            return IsInterleaved() ? mChannelsPerFrame : 1;
        */
    }
    
    pub fn number_channel_streams(&self) -> u32 {
        
        todo!();
        /*
            return IsInterleaved() ? 1 : mChannelsPerFrame;
        */
    }
    
    pub fn number_channels(&self) -> u32 {
        
        todo!();
        /*
            return mChannelsPerFrame;
        */
    }
    
    pub fn sample_word_size(&self) -> u32 {
        
        todo!();
        /*
            return (mBytesPerFrame > 0 && NumberInterleavedChannels()) ? mBytesPerFrame / NumberInterleavedChannels() :  0;
        */
    }
    
    pub fn frames_to_bytes(&self, nframes: u32) -> u32 {
        
        todo!();
        /*
            return nframes * mBytesPerFrame;
        */
    }
    
    pub fn bytes_to_frames(&self, nbytes: u32) -> u32 {
        
        todo!();
        /*
            Assert(mBytesPerFrame > 0, "bytesPerFrame must be > 0 in BytesToFrames");
            return nbytes / mBytesPerFrame;
        */
    }
    
    pub fn same_channels_and_interleaving(&self, a: &CAStreamBasicDescription) -> bool {
        
        todo!();
        /*
            return this->NumberChannels() == a.NumberChannels() && this->IsInterleaved() == a.IsInterleaved();
        */
    }
    
    pub fn identify_common_pcm_format(
        &self, 
        out_format:         &mut CAStreamBasicDescriptionCommonPCMFormat,
        out_is_interleaved: *mut bool
    ) -> bool {

        todo!();
        /*
            // return true if it's a valid PCM format.

            outFormat = kPCMFormatOther;
            // trap out patently invalid formats.
            if (mFormatID != kAudioFormatLinearPCM || mFramesPerPacket != 1 || mBytesPerFrame != mBytesPerPacket || mBitsPerChannel/8 > mBytesPerFrame || mChannelsPerFrame == 0)
                return false;
            bool interleaved = (mFormatFlags & kAudioFormatFlagIsNonInterleaved) == 0;
            if (outIsInterleaved != NULL) *outIsInterleaved = interleaved;
            unsigned wordsize = mBytesPerFrame;
            if (interleaved) {
                if (wordsize % mChannelsPerFrame != 0) return false;
                wordsize /= mChannelsPerFrame;
            }

            if ((mFormatFlags & kAudioFormatFlagIsBigEndian) == kAudioFormatFlagsNativeEndian
            && wordsize * 8 == mBitsPerChannel) {
                // packed and native endian, good
                if (mFormatFlags & kLinearPCMFormatFlagIsFloat) {
                    // float: reject nonsense bits
                    if (mFormatFlags & (kLinearPCMFormatFlagIsSignedInteger | kLinearPCMFormatFlagsSampleFractionMask))
                        return false;
                    if (wordsize == 4)
                        outFormat = kPCMFormatFloat32;
                    if (wordsize == 8)
                        outFormat = kPCMFormatFloat64;
                } else if (mFormatFlags & kLinearPCMFormatFlagIsSignedInteger) {
                    // signed int
                    unsigned fracbits = (mFormatFlags & kLinearPCMFormatFlagsSampleFractionMask) >> kLinearPCMFormatFlagsSampleFractionShift;
                    if (wordsize == 4 && fracbits == 24)
                        outFormat = kPCMFormatFixed824;
                    else if (wordsize == 2 && fracbits == 0)
                        outFormat = kPCMFormatInt16;
                }
            }
            return true;
        */
    }
    
    pub fn is_common_float32(&self, out_is_interleaved: *mut bool) -> bool {

        todo!();
        /*
            CAStreamBasicDescriptionCommonPCMFormat fmt;
            return IdentifyCommonPCMFormat(fmt, outIsInterleaved) && fmt == kPCMFormatFloat32;
        */
    }
    
    pub fn is_common_float64(&self, out_is_interleaved: *mut bool) -> bool {

        todo!();
        /*
            CAStreamBasicDescriptionCommonPCMFormat fmt;
            return IdentifyCommonPCMFormat(fmt, outIsInterleaved) && fmt == kPCMFormatFloat64;
        */
    }
    
    pub fn is_common_fixed824(&self, out_is_interleaved: *mut bool) -> bool {

        todo!();
        /*
            CAStreamBasicDescriptionCommonPCMFormat fmt;
            return IdentifyCommonPCMFormat(fmt, outIsInterleaved) && fmt == kPCMFormatFixed824;
        */
    }
    
    pub fn is_common_int16(&self, out_is_interleaved: *mut bool) -> bool {

        todo!();
        /*
            CAStreamBasicDescriptionCommonPCMFormat fmt;
            return IdentifyCommonPCMFormat(fmt, outIsInterleaved) && fmt == kPCMFormatInt16;
        */
    }

    /* ----------------- manipulation  ----------------- */

    /**
      | note: leaves sample rate untouched
      |
      */
    pub fn set_canonical(&mut self, 
        n_channels:  u32,
        interleaved: bool)  {
        
        todo!();
        /*
            mFormatID = kAudioFormatLinearPCM;
            UInt32 sampleSize = SizeOf32(AudioSampleType);
            mFormatFlags = kAudioFormatFlagsCanonical;
            mBitsPerChannel = 8 * sampleSize;
            mChannelsPerFrame = nChannels;
            mFramesPerPacket = 1;
            if (interleaved)
                mBytesPerPacket = mBytesPerFrame = nChannels * sampleSize;
            else {
                mBytesPerPacket = mBytesPerFrame = sampleSize;
                mFormatFlags |= kAudioFormatFlagIsNonInterleaved;
            }
        */
    }
    
    pub fn is_canonical(&self) -> bool {
        
        todo!();
        /*
            if (mFormatID != kAudioFormatLinearPCM) return false;
            UInt32 reqFormatFlags;
            UInt32 flagsMask = (kLinearPCMFormatFlagIsFloat | kLinearPCMFormatFlagIsBigEndian | kLinearPCMFormatFlagIsSignedInteger | kLinearPCMFormatFlagIsPacked | kLinearPCMFormatFlagsSampleFractionMask);
            bool interleaved = (mFormatFlags & kAudioFormatFlagIsNonInterleaved) == 0;
            unsigned sampleSize = SizeOf32(AudioSampleType);
            reqFormatFlags = kAudioFormatFlagsCanonical;
            UInt32 reqFrameSize = interleaved ? (mChannelsPerFrame * sampleSize) : sampleSize;

            return ((mFormatFlags & flagsMask) == reqFormatFlags
                && mBitsPerChannel == 8 * sampleSize
                && mFramesPerPacket == 1
                && mBytesPerFrame == reqFrameSize
                && mBytesPerPacket == reqFrameSize);
        */
    }
    
    pub fn set_au_canonical(&mut self, 
        n_channels:  u32,
        interleaved: bool)  {
        
        todo!();
        /*
            mFormatID = kAudioFormatLinearPCM;
    #if CA_PREFER_FIXED_POINT
            mFormatFlags = kAudioFormatFlagsCanonical | (kAudioUnitSampleFractionBits << kLinearPCMFormatFlagsSampleFractionShift);
    #else
            mFormatFlags = kAudioFormatFlagsCanonical;
    #endif
            mChannelsPerFrame = nChannels;
            mFramesPerPacket = 1;
            mBitsPerChannel = 8 * SizeOf32(AudioUnitSampleType);
            if (interleaved)
                mBytesPerPacket = mBytesPerFrame = nChannels * SizeOf32(AudioUnitSampleType);
            else {
                mBytesPerPacket = mBytesPerFrame = SizeOf32(AudioUnitSampleType);
                mFormatFlags |= kAudioFormatFlagIsNonInterleaved;
            }
        */
    }

    /**
      | alter an existing format
      |
      */
    pub fn change_number_channels(&mut self, 
        n_channels:  u32,
        interleaved: bool)  {
        
        todo!();
        /*
            Assert(IsPCM(), "ChangeNumberChannels only works for PCM formats");
            UInt32 wordSize = SampleWordSize(); // get this before changing ANYTHING
            if (wordSize == 0)
                wordSize = (mBitsPerChannel + 7) / 8;
            mChannelsPerFrame = nChannels;
            mFramesPerPacket = 1;
            if (interleaved) {
                mBytesPerPacket = mBytesPerFrame = nChannels * wordSize;
                mFormatFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsNonInterleaved);
            } else {
                mBytesPerPacket = mBytesPerFrame = wordSize;
                mFormatFlags |= kAudioFormatFlagIsNonInterleaved;
            }
        */
    }

    /* --------------------- other  --------------------- */
    
    pub fn print(&self)  {
        
        todo!();
        /*
            Print (stdout);
        */
    }
    
    pub fn print_file(&self, file: *mut libc::FILE)  {
        
        todo!();
        /*
            PrintFormat (file, "", "AudioStreamBasicDescription:");
        */
    }
    
    pub fn print_format(&self, 
        f:      *mut libc::FILE,
        indent: *const u8,
        name:   *const u8)  {
        
        todo!();
        /*
            char buf[256];
            fprintf(f, "%s%s %s\n", indent, name, AsString(buf, sizeof(buf)));
        */
    }
    
    pub fn print_format2(&self, 
        f:      *mut libc::FILE,
        indent: *const u8,
        name:   *const u8)  {
        
        todo!();
        /*
            // no trailing newline
            char buf[256];
            fprintf(f, "%s%s %s", indent, name, AsString(buf, sizeof(buf)));
        */
    }
    
    pub fn print_desc(in_desc: &AudioStreamBasicDescription)  {
        
        todo!();
        /*
            CAStreamBasicDescription desc(inDesc);
            desc.Print ();
        */
    }
    
    pub fn save(&self, out_data: *mut CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
    
    pub fn restore(&mut self, in_data: &mut CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    /* ------------------ Operations  ------------------ */
    
    pub fn is_mixable(in_description: &AudioStreamBasicDescription) -> bool {
        
        todo!();
        /*
            return (inDescription.mFormatID == kAudioFormatLinearPCM) && ((inDescription.mFormatFlags & kIsNonMixableFlag) == 0);
        */
    }
    
    pub fn new_from_desc(desc: &AudioStreamBasicDescription) -> Self {
    
        todo!();
        /*
            SetFrom(desc);
        */
    }
    
    pub fn new_with(
        in_sample_rate:        f64,
        in_formatid:           u32,
        in_bytes_per_packet:   u32,
        in_frames_per_packet:  u32,
        in_bytes_per_frame:    u32,
        in_channels_per_frame: u32,
        in_bits_per_channel:   u32,
        in_format_flags:       u32) -> Self {
    
        todo!();
        /*


            mSampleRate = inSampleRate;
        mFormatID = inFormatID;
        mBytesPerPacket = inBytesPerPacket;
        mFramesPerPacket = inFramesPerPacket;
        mBytesPerFrame = inBytesPerFrame;
        mChannelsPerFrame = inChannelsPerFrame;
        mBitsPerChannel = inBitsPerChannel;
        mFormatFlags = inFormatFlags;
        mReserved = 0;
        */
    }
    
    pub fn as_string(
        &self, 
        buf:     *mut u8,
        bufsize: usize,
        brief:   Option<bool>

    ) -> *mut u8 {

        let brief: bool = brief.unwrap_or(false);

        todo!();
        /*
            int bufsize = (int)_bufsize;    // must be signed to protect against overflow
        char *theBuffer = buf;
        int nc;
        char formatID[24];
        CAStringForOSType(mFormatID, formatID, sizeof(formatID));
        if (brief) {
            CAStreamBasicDescriptionCommonPCMFormat com;
            bool interleaved;
            if (IdentifyCommonPCMFormat(com, &interleaved) && com != kPCMFormatOther) {
                const char *desc;
                switch (com) {
                case kPCMFormatInt16:
                    desc = "Int16";
                    break;
                case kPCMFormatFixed824:
                    desc = "Int8.24";
                    break;
                case kPCMFormatFloat32:
                    desc = "Float32";
                    break;
                case kPCMFormatFloat64:
                    desc = "Float64";
                    break;
                default:
                    desc = NULL;
                    break;
                }
                if (desc) {
                    const char *inter ="";
                    if (mChannelsPerFrame > 1)
                        inter = !interleaved ? ", non-inter" : ", inter";
                    snprintf(buf, static_cast<size_t>(bufsize), "%2d ch, %6.0f Hz, %s%s", (int)mChannelsPerFrame, mSampleRate, desc, inter);
                    return theBuffer;
                }
            }
            if (mChannelsPerFrame == 0 && mSampleRate == 0.0 && mFormatID == 0) {
                snprintf(buf, static_cast<size_t>(bufsize), "%2d ch, %6.0f Hz", (int)mChannelsPerFrame, mSampleRate);
                return theBuffer;
            }
        }

        nc = snprintf(buf, static_cast<size_t>(bufsize), "%2d ch, %6.0f Hz, %s (0x%08X) ", (int)NumberChannels(), mSampleRate, formatID, (int)mFormatFlags);
        buf += nc; if ((bufsize -= nc) <= 0) goto exit;
        if (mFormatID == kAudioFormatLinearPCM) {
            bool isInt = !(mFormatFlags & kLinearPCMFormatFlagIsFloat);
            int wordSize = static_cast<int>(SampleWordSize());
            const char *endian = (wordSize > 1) ?
                ((mFormatFlags & kLinearPCMFormatFlagIsBigEndian) ? " big-endian" : " little-endian" ) : "";
            const char *sign = isInt ?
                ((mFormatFlags & kLinearPCMFormatFlagIsSignedInteger) ? " signed" : " unsigned") : "";
            const char *floatInt = isInt ? "integer" : "float";
            char packed[32];
            if (wordSize > 0 && PackednessIsSignificant()) {
                if (mFormatFlags & kLinearPCMFormatFlagIsPacked)
                    snprintf(packed, sizeof(packed), "packed in %d bytes", wordSize);
                else
                    snprintf(packed, sizeof(packed), "unpacked in %d bytes", wordSize);
            } else
                packed[0] = '\0';
            const char *align = (wordSize > 0 && AlignmentIsSignificant()) ?
                ((mFormatFlags & kLinearPCMFormatFlagIsAlignedHigh) ? " high-aligned" : " low-aligned") : "";
            const char *deinter = (mFormatFlags & kAudioFormatFlagIsNonInterleaved) ? ", deinterleaved" : "";
            const char *commaSpace = (packed[0]!='\0') || (align[0]!='\0') ? ", " : "";
            char bitdepth[20];

            int fracbits = (mFormatFlags & kLinearPCMFormatFlagsSampleFractionMask) >> kLinearPCMFormatFlagsSampleFractionShift;
            if (fracbits > 0)
                snprintf(bitdepth, sizeof(bitdepth), "%d.%d", (int)mBitsPerChannel - fracbits, fracbits);
            else
                snprintf(bitdepth, sizeof(bitdepth), "%d", (int)mBitsPerChannel);

            /*nc =*/ snprintf(buf, static_cast<size_t>(bufsize), "%s-bit%s%s %s%s%s%s%s",
                bitdepth, endian, sign, floatInt,
                commaSpace, packed, align, deinter);
            // buf += nc; if ((bufsize -= nc) <= 0) goto exit;
        } else if (mFormatID == kAudioFormatAppleLossless) {
            int sourceBits = 0;
            switch (mFormatFlags)
            {
                case 1: //  kAppleLosslessFormatFlag_16BitSourceData
                    sourceBits = 16;
                    break;
                case 2: //  kAppleLosslessFormatFlag_20BitSourceData
                    sourceBits = 20;
                    break;
                case 3: //  kAppleLosslessFormatFlag_24BitSourceData
                    sourceBits = 24;
                    break;
                case 4: //  kAppleLosslessFormatFlag_32BitSourceData
                    sourceBits = 32;
                    break;
            }
            if (sourceBits)
                nc = snprintf(buf, static_cast<size_t>(bufsize), "from %d-bit source, ", sourceBits);
            else
                nc = snprintf(buf, static_cast<size_t>(bufsize), "from UNKNOWN source bit depth, ");
            buf += nc; if ((bufsize -= nc) <= 0) goto exit;
            /*nc =*/ snprintf(buf, static_cast<size_t>(bufsize), "%d frames/packet", (int)mFramesPerPacket);
            //  buf += nc; if ((bufsize -= nc) <= 0) goto exit;
        }
        else
            /*nc =*/ snprintf(buf, static_cast<size_t>(bufsize), "%d bits/channel, %d bytes/packet, %d frames/packet, %d bytes/frame",
                (int)mBitsPerChannel, (int)mBytesPerPacket, (int)mFramesPerPacket, (int)mBytesPerFrame);
    exit:
        return theBuffer;
        */
    }
    
    pub fn normalize_linear_pcm_format(
        &mut self, 
        io_description: &mut AudioStreamBasicDescription
    ) {
        
        todo!();
        /*
            //  the only thing that changes is to make mixable linear PCM into the canonical linear PCM format
        if((ioDescription.mFormatID == kAudioFormatLinearPCM) && ((ioDescription.mFormatFlags & kIsNonMixableFlag) == 0))
        {
            //  the canonical linear PCM format
            ioDescription.mFormatFlags = kAudioFormatFlagsCanonical;
            ioDescription.mBytesPerPacket = SizeOf32(AudioSampleType) * ioDescription.mChannelsPerFrame;
            ioDescription.mFramesPerPacket = 1;
            ioDescription.mBytesPerFrame = SizeOf32(AudioSampleType) * ioDescription.mChannelsPerFrame;
            ioDescription.mBitsPerChannel = 8 * SizeOf32(AudioSampleType);
        }
        */
    }
    
    pub fn normalize_linear_pcm_format_with_endianness(
        &mut self, 
        in_native_endian: bool,
        io_description:   &mut AudioStreamBasicDescription
    ) {
        
        todo!();
        /*
            //  the only thing that changes is to make mixable linear PCM into the canonical linear PCM format
        if((ioDescription.mFormatID == kAudioFormatLinearPCM) && ((ioDescription.mFormatFlags & kIsNonMixableFlag) == 0))
        {
            //  the canonical linear PCM format
            ioDescription.mFormatFlags = kAudioFormatFlagIsFloat | kAudioFormatFlagIsPacked;
            if(inNativeEndian)
            {
    #if TARGET_RT_BIG_ENDIAN
                ioDescription.mFormatFlags |= kAudioFormatFlagIsBigEndian;
    #endif
            }
            else
            {
    #if TARGET_RT_LITTLE_ENDIAN
                ioDescription.mFormatFlags |= kAudioFormatFlagIsBigEndian;
    #endif
            }
            ioDescription.mBytesPerPacket = SizeOf32(AudioSampleType) * ioDescription.mChannelsPerFrame;
            ioDescription.mFramesPerPacket = 1;
            ioDescription.mBytesPerFrame = SizeOf32(AudioSampleType) * ioDescription.mChannelsPerFrame;
            ioDescription.mBitsPerChannel = 8 * SizeOf32(AudioSampleType);
        }
        */
    }
    
    pub fn reset_format(&mut self, io_description: &mut AudioStreamBasicDescription)  {
        
        todo!();
        /*
            ioDescription.mSampleRate = 0;
        ioDescription.mFormatID = 0;
        ioDescription.mBytesPerPacket = 0;
        ioDescription.mFramesPerPacket = 0;
        ioDescription.mBytesPerFrame = 0;
        ioDescription.mChannelsPerFrame = 0;
        ioDescription.mBitsPerChannel = 0;
        ioDescription.mFormatFlags = 0;
        */
    }
    
    pub fn fill_out_format(&mut self, 
        io_description:          &mut AudioStreamBasicDescription,
        in_template_description: &AudioStreamBasicDescription)  {
        
        todo!();
        /*
            if(fiszero(ioDescription.mSampleRate))
        {
            ioDescription.mSampleRate = inTemplateDescription.mSampleRate;
        }
        if(ioDescription.mFormatID == 0)
        {
            ioDescription.mFormatID = inTemplateDescription.mFormatID;
        }
        if(ioDescription.mFormatFlags == 0)
        {
            ioDescription.mFormatFlags = inTemplateDescription.mFormatFlags;
        }
        if(ioDescription.mBytesPerPacket == 0)
        {
            ioDescription.mBytesPerPacket = inTemplateDescription.mBytesPerPacket;
        }
        if(ioDescription.mFramesPerPacket == 0)
        {
            ioDescription.mFramesPerPacket = inTemplateDescription.mFramesPerPacket;
        }
        if(ioDescription.mBytesPerFrame == 0)
        {
            ioDescription.mBytesPerFrame = inTemplateDescription.mBytesPerFrame;
        }
        if(ioDescription.mChannelsPerFrame == 0)
        {
            ioDescription.mChannelsPerFrame = inTemplateDescription.mChannelsPerFrame;
        }
        if(ioDescription.mBitsPerChannel == 0)
        {
            ioDescription.mBitsPerChannel = inTemplateDescription.mBitsPerChannel;
        }
        */
    }
    
    pub fn get_simple_name(
        &mut self, 
        in_description:         &AudioStreamBasicDescription,
        out_name:               *mut u8,
        in_max_name_length:     u32,
        in_abbreviate:          bool,
        in_include_sample_rate: Option<bool>
    ) {

        let in_include_sample_rate: bool = in_include_sample_rate.unwrap_or(false);
        
        todo!();
        /*
            if(inIncludeSampleRate)
        {
            int theCharactersWritten = snprintf(outName, inMaxNameLength, "%.0f ", inDescription.mSampleRate);
            outName += theCharactersWritten;
            inMaxNameLength -= static_cast<UInt32>(theCharactersWritten);
        }

        switch(inDescription.mFormatID)
        {
            case kAudioFormatLinearPCM:
                {
                    const char* theEndianString = NULL;
                    if((inDescription.mFormatFlags & kAudioFormatFlagIsBigEndian) != 0)
                    {
                        #if TARGET_RT_LITTLE_ENDIAN
                            theEndianString = "Big Endian";
                        #endif
                    }
                    else
                    {
                        #if TARGET_RT_BIG_ENDIAN
                            theEndianString = "Little Endian";
                        #endif
                    }

                    const char* theKindString = NULL;
                    if((inDescription.mFormatFlags & kAudioFormatFlagIsFloat) != 0)
                    {
                        theKindString = (inAbbreviate ? "Float" : "Floating Point");
                    }
                    else if((inDescription.mFormatFlags & kAudioFormatFlagIsSignedInteger) != 0)
                    {
                        theKindString = (inAbbreviate ? "SInt" : "Signed Integer");
                    }
                    else
                    {
                        theKindString = (inAbbreviate ? "UInt" : "Unsigned Integer");
                    }

                    const char* thePackingString = NULL;
                    if((inDescription.mFormatFlags & kAudioFormatFlagIsPacked) == 0)
                    {
                        if((inDescription.mFormatFlags & kAudioFormatFlagIsAlignedHigh) != 0)
                        {
                            thePackingString = "High";
                        }
                        else
                        {
                            thePackingString = "Low";
                        }
                    }

                    const char* theMixabilityString = NULL;
                    if((inDescription.mFormatFlags & kIsNonMixableFlag) == 0)
                    {
                        theMixabilityString = "Mixable";
                    }
                    else
                    {
                        theMixabilityString = "Unmixable";
                    }

                    if(inAbbreviate)
                    {
                        if(theEndianString != NULL)
                        {
                            if(thePackingString != NULL)
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Ch %s %s %s%d/%s%d", theMixabilityString, (int)inDescription.mChannelsPerFrame, theEndianString, thePackingString, theKindString, (int)inDescription.mBitsPerChannel, theKindString, (int)(inDescription.mBytesPerFrame / inDescription.mChannelsPerFrame) * 8);
                            }
                            else
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Ch %s %s%d", theMixabilityString, (int)inDescription.mChannelsPerFrame, theEndianString, theKindString, (int)inDescription.mBitsPerChannel);
                            }
                        }
                        else
                        {
                            if(thePackingString != NULL)
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Ch %s %s%d/%s%d", theMixabilityString, (int)inDescription.mChannelsPerFrame, thePackingString, theKindString, (int)inDescription.mBitsPerChannel, theKindString, (int)((inDescription.mBytesPerFrame / inDescription.mChannelsPerFrame) * 8));
                            }
                            else
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Ch %s%d", theMixabilityString, (int)inDescription.mChannelsPerFrame, theKindString, (int)inDescription.mBitsPerChannel);
                            }
                        }
                    }
                    else
                    {
                        if(theEndianString != NULL)
                        {
                            if(thePackingString != NULL)
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Channel %d Bit %s %s Aligned %s in %d Bits", theMixabilityString, (int)inDescription.mChannelsPerFrame, (int)inDescription.mBitsPerChannel, theEndianString, theKindString, thePackingString, (int)(inDescription.mBytesPerFrame / inDescription.mChannelsPerFrame) * 8);
                            }
                            else
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Channel %d Bit %s %s", theMixabilityString, (int)inDescription.mChannelsPerFrame, (int)inDescription.mBitsPerChannel, theEndianString, theKindString);
                            }
                        }
                        else
                        {
                            if(thePackingString != NULL)
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Channel %d Bit %s Aligned %s in %d Bits", theMixabilityString, (int)inDescription.mChannelsPerFrame, (int)inDescription.mBitsPerChannel, theKindString, thePackingString, (int)(inDescription.mBytesPerFrame / inDescription.mChannelsPerFrame) * 8);
                            }
                            else
                            {
                                snprintf(outName, inMaxNameLength, "%s %d Channel %d Bit %s", theMixabilityString, (int)inDescription.mChannelsPerFrame, (int)inDescription.mBitsPerChannel, theKindString);
                            }
                        }
                    }
                }
                break;

            case kAudioFormatAC3:
                strlcpy(outName, "AC-3", sizeof(outName));
                break;

            case kAudioFormat60958AC3:
                strlcpy(outName, "AC-3 for SPDIF", sizeof(outName));
                break;

            default:
                CACopy4CCToCString(outName, inDescription.mFormatID);
                break;
        };
        */
    }

    #[cfg(CoreAudio_Debug)]
    pub fn print_to_log(&mut self, in_desc: &AudioStreamBasicDescription)  {
        
        todo!();
        /*
            PrintFloat      ("  Sample Rate:        ", inDesc.mSampleRate);
        Print4CharCode  ("  Format ID:          ", inDesc.mFormatID);
        PrintHex        ("  Format Flags:       ", inDesc.mFormatFlags);
        PrintInt        ("  Bytes per Packet:   ", inDesc.mBytesPerPacket);
        PrintInt        ("  Frames per Packet:  ", inDesc.mFramesPerPacket);
        PrintInt        ("  Bytes per Frame:    ", inDesc.mBytesPerFrame);
        PrintInt        ("  Channels per Frame: ", inDesc.mChannelsPerFrame);
        PrintInt        ("  Bits per Channel:   ", inDesc.mBitsPerChannel);
        */
    }
    
    pub fn modify_format_flags_for_matching(&mut self, 
        x:              &AudioStreamBasicDescription,
        y:              &AudioStreamBasicDescription,
        x_flags:        &mut u32,
        y_flags:        &mut u32,
        converter_only: bool)  {
        
        todo!();
        /*
            // match wildcards
        if (x.mFormatID == 0 || y.mFormatID == 0 || xFlags == 0 || yFlags == 0)
        {
            // Obliterate all flags.
            xFlags = yFlags = 0;
            return;
        }

        if (x.mFormatID == kAudioFormatLinearPCM) {
            // knock off the all clear flag
            xFlags = xFlags & ~kAudioFormatFlagsAreAllClear;
            yFlags = yFlags & ~kAudioFormatFlagsAreAllClear;

            // if both kAudioFormatFlagIsPacked bits are set, then we don't care about the kAudioFormatFlagIsAlignedHigh bit.
            if (xFlags & yFlags & kAudioFormatFlagIsPacked) {
                xFlags = xFlags & ~static_cast<UInt32>(kAudioFormatFlagIsAlignedHigh);
                yFlags = yFlags & ~static_cast<UInt32>(kAudioFormatFlagIsAlignedHigh);
            }

            // if both kAudioFormatFlagIsFloat bits are set, then we don't care about the kAudioFormatFlagIsSignedInteger bit.
            if (xFlags & yFlags & kAudioFormatFlagIsFloat) {
                xFlags = xFlags & ~static_cast<UInt32>(kAudioFormatFlagIsSignedInteger);
                yFlags = yFlags & ~static_cast<UInt32>(kAudioFormatFlagIsSignedInteger);
            }

            //  if the bit depth is 8 bits or less and the format is packed, we don't care about endianness
            if((x.mBitsPerChannel <= 8) && ((xFlags & kAudioFormatFlagIsPacked) == kAudioFormatFlagIsPacked))
            {
                xFlags = xFlags & ~static_cast<UInt32>(kAudioFormatFlagIsBigEndian);
            }
            if((y.mBitsPerChannel <= 8) && ((yFlags & kAudioFormatFlagIsPacked) == kAudioFormatFlagIsPacked))
            {
                yFlags = yFlags & ~static_cast<UInt32>(kAudioFormatFlagIsBigEndian);
            }

            //  if the number of channels is 1, we don't care about non-interleavedness
            if (x.mChannelsPerFrame == 1 && y.mChannelsPerFrame == 1) {
                xFlags &= ~static_cast<UInt32>(kLinearPCMFormatFlagIsNonInterleaved);
                yFlags &= ~static_cast<UInt32>(kLinearPCMFormatFlagIsNonInterleaved);
            }

            if (converterOnly) {
                CAStreamBasicDescription cas_x = CAStreamBasicDescription(x);
                CAStreamBasicDescription cas_y = CAStreamBasicDescription(y);
                if (!cas_x.PackednessIsSignificant() && !cas_y.PackednessIsSignificant()) {
                    xFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsPacked);
                    yFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsPacked);
                }
                if (!cas_x.AlignmentIsSignificant() && !cas_y.AlignmentIsSignificant()) {
                    xFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsAlignedHigh);
                    yFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsAlignedHigh);
                }
                // We don't care about whether the streams are mixable in this case
                xFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsNonMixable);
                yFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsNonMixable);
            }
        }
        */
    }
    
    pub fn flag_independent_equivalence(&mut self, 
        x: &AudioStreamBasicDescription,
        y: &AudioStreamBasicDescription) -> bool {
        
        todo!();
        /*
            return
        //  check the sample rate
        (fiszero(x.mSampleRate) || fiszero(y.mSampleRate) || fequal(x.mSampleRate, y.mSampleRate))

        //  check the format ids
        && MATCH(mFormatID)

        //  check the bytes per packet
        && MATCH(mBytesPerPacket)

        //  check the frames per packet
        && MATCH(mFramesPerPacket)

        //  check the bytes per frame
        && MATCH(mBytesPerFrame)

        //  check the channels per frame
        && MATCH(mChannelsPerFrame)

        //  check the channels per frame
        && MATCH(mBitsPerChannel) ;
        */
    }
    
    pub fn is_equal(
        &self, 
        other:                  &AudioStreamBasicDescription,
        interpreting_wildcards: Option<bool>

    ) -> bool {

        let interpreting_wildcards: bool = interpreting_wildcards.unwrap_or(true);
        
        todo!();
        /*
            if (interpretingWildcards)
            return *this == other;
        return memcmp(this, &other, offsetof(AudioStreamBasicDescription, mReserved)) == 0;
        */
    }
    
    pub fn is_functionally_equivalent(&mut self, 
        x: &AudioStreamBasicDescription,
        y: &AudioStreamBasicDescription) -> bool {
        
        todo!();
        /*
            UInt32 xFlags = x.mFormatFlags, yFlags = y.mFormatFlags;
        CAStreamBasicDescription::ModifyFormatFlagsForMatching(x, y, xFlags, yFlags, true);

        return
        // check all but the format flags
        CAStreamBasicDescription::FlagIndependentEquivalence(x, y)
        // check the format flags with converter focus
        && (xFlags == yFlags);
        */
    }
    
    /**
      | return true if parsing was successful
      |
      */
    pub fn from_text_with_fmt(
        &mut self, 
        in_text_desc: *const u8,
        fmt:          &mut AudioStreamBasicDescription
    ) -> bool {
        
        todo!();
        /*
            const char *p = inTextDesc;

        memset(&fmt, 0, sizeof(fmt));

        bool isPCM = true;  // until proven otherwise
        UInt32 pcmFlags = kAudioFormatFlagIsPacked | kAudioFormatFlagIsSignedInteger;

        if (p[0] == '-')    // previously we required a leading dash on PCM formats
            ++p;

        if (p[0] == 'B' && p[1] == 'E') {
            pcmFlags |= kLinearPCMFormatFlagIsBigEndian;
            p += 2;
        } else if (p[0] == 'L' && p[1] == 'E') {
            p += 2;
        } else {
            // default is native-endian
    #if TARGET_RT_BIG_ENDIAN
            pcmFlags |= kLinearPCMFormatFlagIsBigEndian;
    #endif
        }
        if (p[0] == 'F') {
            pcmFlags = (pcmFlags & ~static_cast<UInt32>(kAudioFormatFlagIsSignedInteger)) | kAudioFormatFlagIsFloat;
            ++p;
        } else {
            if (p[0] == 'U') {
                pcmFlags &= ~static_cast<UInt32>(kAudioFormatFlagIsSignedInteger);
                ++p;
            }
            if (p[0] == 'I')
                ++p;
            else {
                // it's not PCM; presumably some other format (NOT VALIDATED; use AudioFormat for that)
                isPCM = false;
                p = inTextDesc; // go back to the beginning
                char buf[4] = { ' ',' ',' ',' ' };
                for (int i = 0; i < 4; ++i) {
                    if (*p != '\\') {
                        if ((buf[i] = *p++) == '\0') {
                            // special-case for 'aac'
                            if (i != 3) return false;
                            --p;    // keep pointing at the terminating null
                            buf[i] = ' ';
                            break;
                        }
                    } else {
                        // "\xNN" is a hex byte
                        if (*++p != 'x') return false;
                        int x;
                        if (sscanf(++p, "%02X", &x) != 1) return false;
                        buf[i] = static_cast<char>(x);
                        p += 2;
                    }
                }

                if (strchr("-@/#", buf[3])) {
                    // further special-casing for 'aac'
                    buf[3] = ' ';
                    --p;
                }

                memcpy(&fmt.mFormatID, buf, 4);
                fmt.mFormatID = CFSwapInt32BigToHost(fmt.mFormatID);
            }
        }

        if (isPCM) {
            fmt.mFormatID = kAudioFormatLinearPCM;
            fmt.mFormatFlags = pcmFlags;
            fmt.mFramesPerPacket = 1;
            fmt.mChannelsPerFrame = 1;
            UInt32 bitdepth = 0, fracbits = 0;
            while (isdigit(*p))
                bitdepth = 10 * bitdepth + static_cast<UInt32>(*p++ - '0');
            if (*p == '.') {
                ++p;
                if (!isdigit(*p)) {
                    fprintf(stderr, "Expected fractional bits following '.'\n");
                    goto Bail;
                }
                while (isdigit(*p))
                    fracbits = 10 * fracbits + static_cast<UInt32>(*p++ - '0');
                bitdepth += fracbits;
                fmt.mFormatFlags |= (fracbits << kLinearPCMFormatFlagsSampleFractionShift);
            }
            fmt.mBitsPerChannel = bitdepth;
            fmt.mBytesPerPacket = fmt.mBytesPerFrame = (bitdepth + 7) / 8;
            if (bitdepth & 7) {
                // assume unpacked. (packed odd bit depths are describable but not supported in AudioConverter.)
                fmt.mFormatFlags &= ~static_cast<UInt32>(kLinearPCMFormatFlagIsPacked);
                // alignment matters; default to high-aligned. use ':L_' for low.
                fmt.mFormatFlags |= kLinearPCMFormatFlagIsAlignedHigh;
            }
        }
        if (*p == '@') {
            ++p;
            while (isdigit(*p))
                fmt.mSampleRate = 10 * fmt.mSampleRate + (*p++ - '0');
        }
        if (*p == '/') {
            UInt32 flags = 0;
            while (true) {
                char c = *++p;
                if (c >= '0' && c <= '9')
                    flags = (flags << 4) | static_cast<UInt32>(c - '0');
                else if (c >= 'A' && c <= 'F')
                    flags = (flags << 4) | static_cast<UInt32>(c - 'A' + 10);
                else if (c >= 'a' && c <= 'f')
                    flags = (flags << 4) | static_cast<UInt32>(c - 'a' + 10);
                else break;
            }
            fmt.mFormatFlags = flags;
        }
        if (*p == '#') {
            ++p;
            while (isdigit(*p))
                fmt.mFramesPerPacket = 10 * fmt.mFramesPerPacket + static_cast<UInt32>(*p++ - '0');
        }
        if (*p == ':') {
            ++p;
            fmt.mFormatFlags &= ~static_cast<UInt32>(kLinearPCMFormatFlagIsPacked);
            if (*p == 'L')
                fmt.mFormatFlags &= ~static_cast<UInt32>(kLinearPCMFormatFlagIsAlignedHigh);
            else if (*p == 'H')
                fmt.mFormatFlags |= kLinearPCMFormatFlagIsAlignedHigh;
            else
                goto Bail;
            ++p;
            UInt32 bytesPerFrame = 0;
            while (isdigit(*p))
                bytesPerFrame = 10 * bytesPerFrame + static_cast<UInt32>(*p++ - '0');
            fmt.mBytesPerFrame = fmt.mBytesPerPacket = bytesPerFrame;
        }
        if (*p == ',') {
            ++p;
            int ch = 0;
            while (isdigit(*p))
                ch = 10 * ch + (*p++ - '0');
            fmt.mChannelsPerFrame = static_cast<UInt32>(ch);
            if (*p == 'D') {
                ++p;
                if (fmt.mFormatID != kAudioFormatLinearPCM) {
                    fprintf(stderr, "non-interleaved flag invalid for non-PCM formats\n");
                    goto Bail;
                }
                fmt.mFormatFlags |= kAudioFormatFlagIsNonInterleaved;
            } else {
                if (*p == 'I') ++p; // default
                if (fmt.mFormatID == kAudioFormatLinearPCM)
                    fmt.mBytesPerPacket = fmt.mBytesPerFrame *= static_cast<UInt32>(ch);
            }
        }
        if (*p != '\0') {
            fprintf(stderr, "extra characters at end of format string: %s\n", p);
            goto Bail;
        }
        return true;

    Bail:
        fprintf(stderr, "Invalid format string: %s\n", inTextDesc);
        fprintf(stderr, "Syntax of format strings is: \n");
        return false;
        */
    }
}
