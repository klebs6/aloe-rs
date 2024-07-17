crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCTimeTag.h]

pub const MILLISECONDS_BETWEEN_OSC_AND_ALOE_EPOCHS: u64 = 2208988800000;
pub const RAW_TIME_TAG_REPRESENTING_IMMEDIATELY:    u64 = 0x0000000000000001;

/**
  | An OSC time tag.
  | 
  | OSC time tags are part of OSCBundle objects.
  | 
  | In accordance with the OSC 1.0 specification,
  | the internal timestamp stored in
  | 
  | OSCTimeTag uses the same binary format
  | as NTP timestamps. The representation
  | is by a 64 bit fixed point number. The
  | first 32 bits specify the number of seconds
  | since midnight on January 1, 1900, and
  | the last 32 bits specify fractional
  | parts of a second to a precision of about
  | 200 picoseconds.
  | 
  | The time tag value consisting of 63 zero
  | bits followed by a one in the least significant
  | bit is a special case meaning "immediately".
  | 
  | For a more user-friendly time format,
  | convert OSCTimeTag to a Time object
  | using toTime().
  | 
  | @tags{OSC}
  |
  */
pub struct OSCTimeTag {
    raw_time_tag: u64,
}

pub mod osc_time_tag {
    use super::*;

    /**
      | The special value representing "immediately".
      |
      */
    lazy_static!{
        /*
           static const OSCTimeTag immediately;
           const OSCTimeTag OSCTimeTag::immediately;
        */
    }
}

impl Default for OSCTimeTag {
    
    /**
      | Default constructor.
      | 
      | Constructs an OSCTimeTag object with
      | the special value representing "immediately".
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : raw_time_tag(rawTimeTagRepresentingImmediately),

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCTimeTag.cpp]
impl From<u64> for OSCTimeTag {

    /**
      | Constructs an OSCTimeTag object from
      | a raw binary OSC time tag.
      |
      */
    fn from(t: u64) -> Self {
    
        todo!();
        /*
        : raw_time_tag(t),

        
        */
    }
}
    
impl From<Time> for OSCTimeTag {

    /**
      | Constructs an OSCTimeTag object from
      | a Time object.
      |
      */
    fn from(time: Time) -> Self {
    
        todo!();
        /*

            const uint64 milliseconds = (uint64) time.toMilliseconds() + millisecondsBetweenOscAndAloeEpochs;

        uint64 seconds = milliseconds / 1000;
        uint32 fractionalPart = uint32 (4294967.296 * (milliseconds % 1000));

        rawTimeTag = (seconds << 32) + fractionalPart;
        */
    }
}

impl OSCTimeTag {

    /**
      | Returns the raw binary OSC time tag representation.
      |
      */
    pub fn get_raw_time_tag(&self) -> u64 {
        
        todo!();
        /*
            return rawTimeTag;
        */
    }
    
    /**
      | Returns a Time object representing
      | the same time as the OSCTimeTag.
      | 
      | If the OSCTimeTag has the special value
      | representing "immediately", the resulting
      | Time object will represent an arbitrary
      | point of time (but guaranteed to be in
      | the past), since Time does not have such
      | a special value.
      |
      */
    pub fn to_time(&self) -> Time {
        
        todo!();
        /*
            const uint64 seconds = rawTimeTag >> 32;
        const uint32 fractionalPart = (rawTimeTag & 0x00000000FFFFFFFFULL);

        const auto fractionalPartInMillis = (double) fractionalPart / 4294967.296;

        // now using signed integer, because this is allowed to become negative:
        const auto aloeTimeInMillis = (int64) (seconds * 1000)
                                    + (int64) roundToInt (fractionalPartInMillis)
                                    - (int64) millisecondsBetweenOscAndAloeEpochs;

        return Time (aloeTimeInMillis);
        */
    }
    
    /**
      | Returns true if the OSCTimeTag object
      | has the special value representing
      | "immediately".
      |
      */
    pub fn is_immediately(&self) -> bool {
        
        todo!();
        /*
            return rawTimeTag == rawTimeTagRepresentingImmediately;
        */
    }
}
