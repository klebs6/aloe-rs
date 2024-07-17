crate::ix!();

/**
  | This structure is filled-in by the
  | AudioPlayHead::getCurrentPosition()
  | method.
  |
  */
pub struct AudioPlayHeadCurrentPositionInfo
{
    /**
      | The tempo in BPM
      |
      */
    bpm:                            f64, // default = 120.0

    /**
      | Time signature numerator, e.g. the
      | 3 of a 3/4 time sig
      |
      */
    time_sig_numerator:             i32, // default = 4

    /**
      | Time signature denominator, e.g. the
      | 4 of a 3/4 time sig
      |
      */
    time_sig_denominator:           i32, // default = 4

    /**
      | The current play position, in samples
      | from the start of the timeline.
      |
      */
    time_in_samples:                i64, // default = 0

    /**
      | The current play position, in seconds
      | from the start of the timeline.
      |
      */
    time_in_seconds:                f64, // default = 0

    /**
      | For timecode, the position of the start
      | of the timeline, in seconds from 00:00:00:00.
      |
      */
    edit_origin_time:               f64, // default = 0

    /**
      | The current play position, in units
      | of quarter-notes.
      |
      */
    ppq_position:                   f64, // default = 0

    /**
      | The position of the start of the last bar, in units
      | of quarter-notes. This is the time from the start
      | of the timeline to the start of the current bar,
      | in ppq units. Note - this value may be unavailable
      | on some hosts, e.g. Pro-Tools. If it's not available,
      | the value will be 0.
      */
    ppq_position_of_last_bar_start: f64, // default = 0

    /**
      | The video frame rate, if applicable.
      |
      */
    frame_rate:                     FrameRateType, // default = FrameRateType::fps23976

    /**
      | True if the transport is currently playing.
      |
      */
    is_playing:                     bool, // default = false

    /**
      | True if the transport is currently recording. (When
      | isRecording is true, then isPlaying will also be
      | true).
      */
    is_recording:                   bool, // default = false

    /**
      | The current cycle start position in units of quarter-notes.
      | Note that not all hosts or plugin formats may provide
      | this value. @see isLooping
      */
    ppq_loop_start:                 f64, // default = 0

    /**
      | The current cycle end position in units of quarter-notes.
      | Note that not all hosts or plugin formats may provide
      | this value. @see isLooping
      */
    ppq_loop_end:                   f64, // default = 0

    /**
      | True if the transport is currently looping.
      |
      */
    is_looping:                     bool, // default = false
}

impl PartialEq<AudioPlayHeadCurrentPositionInfo> for AudioPlayHeadCurrentPositionInfo {
    
    #[inline] fn eq(&self, other: &AudioPlayHeadCurrentPositionInfo) -> bool {
        todo!();
        /*
            auto tie = [] (const AudioPlayHeadCurrentPositionInfo& i)
                {
                    return std::tie (i.timeInSamples,
                                     i.ppqPosition,
                                     i.editOriginTime,
                                     i.ppqPositionOfLastBarStart,
                                     i.frameRate,
                                     i.isPlaying,
                                     i.isRecording,
                                     i.bpm,
                                     i.timeSigNumerator,
                                     i.timeSigDenominator,
                                     i.ppqLoopStart,
                                     i.ppqLoopEnd,
                                     i.isLooping);
                };

                return tie (*this) == tie (other);
        */
    }
}

impl Eq for AudioPlayHeadCurrentPositionInfo {}

impl AudioPlayHeadCurrentPositionInfo {

    pub fn reset_to_default(&mut self)  {
        
        todo!();
        /*
            *this = AudioPlayHeadCurrentPositionInfo{};
        */
    }
}
