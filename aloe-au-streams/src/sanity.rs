crate::ix!();

lazy_static!{
    /*
    extern char *CAStringForOSType (OSType t, char *writeLocation, size_t bufsize);
    */
}

/**
  -----------------------------
  define Leopard specific symbols for backward
  compatibility if applicable
  */
#[cfg(COREAUDIOTYPES_VERSION_LT_1050)]
pub type AudioSampleType = f32;

#[cfg(COREAUDIOTYPES_VERSION_LT_1050)]
pub const kAudioFormatFlagsCanonical: usize = kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;

///-------------------------------------------
#[cfg(COREAUDIOTYPES_VERSION_LT_1051)]
pub type AudioUnitSampleType = f32;

#[cfg(COREAUDIOTYPES_VERSION_LT_1051)]
pub const kLinearPCMFormatFlagsSampleFractionShift: usize = 7;

#[cfg(COREAUDIOTYPES_VERSION_LT_1051)]
pub const kLinearPCMFormatFlagsSampleFractionMask:  usize = 0x3F << kLinearPCMFormatFlagsSampleFractionShift;

/**
  -------------------------------------------
    define the IsMixable format flag for all
    versions of the system
  */
#[cfg(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_3)]
pub const kIsNonMixableFlag: usize = kAudioFormatFlagIsNonMixable;

#[cfg(not(MAC_OS_X_VERSION_MAX_ALLOWED_GTE_MAC_OS_X_VERSION_10_3))]
pub const kIsNonMixableFlag: usize = 1 << 6;

pub fn match_format_flags(
        x: &AudioStreamBasicDescription,
        y: &AudioStreamBasicDescription) -> bool {
    
    todo!();
        /*
            UInt32 xFlags = x.mFormatFlags;
        UInt32 yFlags = y.mFormatFlags;

        CAStreamBasicDescription::ModifyFormatFlagsForMatching(x, y, xFlags, yFlags, false);
        return xFlags == yFlags;
        */
}

pub fn sanity_check(x: &AudioStreamBasicDescription) -> bool {
    
    todo!();
        /*
            // This function returns false if there are sufficiently insane values in any field.
        // It is very conservative so even some very unlikely values will pass.
        // This is just meant to catch the case where the data from a file is corrupted.

        return
            (x.mSampleRate >= 0.)
            && (x.mSampleRate < 3e6)    // SACD sample rate is 2.8224 MHz
            && (x.mBytesPerPacket < 1000000)
            && (x.mFramesPerPacket < 1000000)
            && (x.mBytesPerFrame < 1000000)
            && (x.mChannelsPerFrame <= 1024)
            && (x.mBitsPerChannel <= 1024)
            && (x.mFormatID != 0)
            && !(x.mFormatID == kAudioFormatLinearPCM && (x.mFramesPerPacket != 1 || x.mBytesPerPacket != x.mBytesPerFrame));
        */
}

pub fn ca_string_for_os_type(
        t:              OSType,
        write_location: *mut u8,
        bufsize:        usize) -> *mut u8 {
    
    todo!();
        /*
            if (bufsize > 0) {
            char *p = writeLocation, *pend = writeLocation + bufsize;
            union { UInt32 i; unsigned char str[4]; } u;
            unsigned char *q = u.str;
            u.i = CFSwapInt32HostToBig(t);

            bool hasNonPrint = false;
            for (int i = 0; i < 4; ++i) {
                if (!(isprint(*q) && *q != '\\')) {
                    hasNonPrint = true;
                    break;
                }
                q++;
            }
            q = u.str;

            if (hasNonPrint)
                p += snprintf (p, pend - p, "0x");
            else if (p < pend)
                *p++ = '\'';

            for (int i = 0; i < 4 && p < pend; ++i) {
                if (hasNonPrint) {
                    p += snprintf(p, pend - p, "%02X", *q++);
                } else {
                    *p++ = *q++;
                }
            }
            if (!hasNonPrint && p < pend)
                *p++ = '\'';
            if (p >= pend) p -= 1;
            *p = '\0';
        }
        return writeLocation;
        */
}
