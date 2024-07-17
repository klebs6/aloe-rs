/*!
  |=============================================================================
  |  CAHostTimeBase
  |
  |  This class provides platform independent access to the host's time base.
  |=============================================================================
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAHostTimeBase.h]

lazy_static!{
    /*
    static pthread_once_t   sIsInited;

        static Float64          sFrequency;
        static Float64          sInverseFrequency;
        static UInt32           sMinDelta;
        static UInt32           sToNanosNumerator;
        static UInt32           sToNanosDenominator;
    #if Track_Host_TimeBase
        static UInt64           sLastTime;
    #endif
    */
}

#[cfg(TARGET_OS_MAC)]
pub fn ca_host_time_base_get_current_time() -> u64 {
    
    todo!();
    /*
        return GetTheCurrentTime();
    */
}

pub fn ca_host_time_base_get_frequency() -> f64 {
    
    todo!();
    /*
        pthread_once(&sIsInited, Initialize); return sFrequency;
    */
}

pub fn ca_host_time_base_get_inverse_frequency() -> f64 {
    
    todo!();
    /*
        pthread_once(&sIsInited, Initialize); return sInverseFrequency;
    */
}

pub fn ca_host_time_base_get_minimum_delta() -> u32 {
    
    todo!();
    /*
        pthread_once(&sIsInited, Initialize); return sMinDelta;
    */
}

pub fn ca_host_time_base_initialize()  {
    
    todo!();
    /*
    
    */
}

#[inline] pub fn ca_host_time_base_get_the_current_time() -> u64 {
    
    todo!();
    /*
        UInt64 theTime = 0;

    #if TARGET_OS_MAC
        theTime = mach_absolute_time();
    #elif TARGET_OS_WIN32
        LARGE_INTEGER theValue;
        QueryPerformanceCounter(&theValue);
        theTime = *((UInt64*)&theValue);
    #endif

    #if Track_Host_TimeBase
        if(sLastTime != 0)
        {
            if(theTime <= sLastTime)
            {
                DebugPrintf("CAHostTimeBase::GetTheCurrentTime: the current time is earlier than the last time, now: %qd, then: %qd", theTime, sLastTime);
            }
            sLastTime = theTime;
        }
        else
        {
            sLastTime = theTime;
        }
    #endif

    return theTime;
    */
}

#[inline] pub fn ca_host_time_base_convert_to_nanos(in_host_time: u64) -> u64 {
    
    todo!();
    /*
        pthread_once(&sIsInited, Initialize);

    UInt64 theAnswer = MultiplyByRatio(inHostTime, sToNanosNumerator, sToNanosDenominator);
    #if CoreAudio_Debug
        if(((sToNanosNumerator > sToNanosDenominator) && (theAnswer < inHostTime)) || ((sToNanosDenominator > sToNanosNumerator) && (theAnswer > inHostTime)))
        {
            DebugPrintf("CAHostTimeBase::ConvertToNanos: The conversion wrapped");
        }
    #endif

    return theAnswer;
    */
}

#[inline] pub fn ca_host_time_base_convert_from_nanos(in_nanos: u64) -> u64 {
    
    todo!();
    /*
        pthread_once(&sIsInited, Initialize);

    UInt64 theAnswer = MultiplyByRatio(inNanos, sToNanosDenominator, sToNanosNumerator);
    #if CoreAudio_Debug
        if(((sToNanosDenominator > sToNanosNumerator) && (theAnswer < inNanos)) || ((sToNanosNumerator > sToNanosDenominator) && (theAnswer > inNanos)))
        {
            DebugPrintf("CAHostTimeBase::ConvertFromNanos: The conversion wrapped");
        }
    #endif

    return theAnswer;
    */
}

#[inline] pub fn ca_host_time_base_get_current_time_in_nanos() -> u64 {
    
    todo!();
    /*
        return ConvertToNanos(GetTheCurrentTime());
    */
}

#[inline] pub fn ca_host_time_base_absolute_host_delta_to_nanos(
    in_start_time: u64,
    in_end_time:   u64) -> u64 {
    
    todo!();
    /*
        UInt64 theAnswer;

    if(inStartTime <= inEndTime)
    {
        theAnswer = inEndTime - inStartTime;
    }
    else
    {
        theAnswer = inStartTime - inEndTime;
    }

    return ConvertToNanos(theAnswer);
    */
}

#[inline] pub fn ca_host_time_base_host_delta_to_nanos(
    in_start_time: u64,
    in_end_time:   u64) -> SInt64 {
    
    todo!();
    /*
        SInt64 theAnswer;
    SInt64 theSign = 1;

    if(inStartTime <= inEndTime)
    {
        theAnswer = static_cast<SInt64>(inEndTime - inStartTime);
    }
    else
    {
        theAnswer = static_cast<SInt64>(inStartTime - inEndTime);
        theSign = -1;
    }

    return theSign * static_cast<SInt64>(ConvertToNanos(static_cast<UInt64>(theAnswer)));
    */
}

#[inline] pub fn ca_host_time_base_multiply_by_ratio(
    in_muliplicand: u64,
    in_numerator:   u32,
    in_denominator: u32) -> u64 {
    
    todo!();
    /*
        #if TARGET_OS_MAC && TARGET_RT_64_BIT
    __uint128_t theAnswer = inMuliplicand;
#else
    long double theAnswer = inMuliplicand;
#endif
    if(inNumerator != inDenominator)
    {
        theAnswer *= inNumerator;
        theAnswer /= inDenominator;
    }
    return static_cast<UInt64>(theAnswer);
    */
}
