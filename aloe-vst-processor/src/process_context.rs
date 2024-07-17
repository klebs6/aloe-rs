crate::ix!();

/**
  | Audio processing context.
  | 
  | For each processing block the host provides
  | timing information and musical parameters
  | that can change over time. For a host
  | that supports jumps (like cycle) it
  | is possible to split up a processing
  | block into multiple parts in order to
  | provide a correct project time inside
  | of every block, but this behavior is
  | not mandatory. Since the timing will
  | be correct at the beginning of the next
  | block again, a host that is dependent
  | on a fixed processing block size can
  | choose to neglect this problem.
  | 
  | \see IAudioProcessor, ProcessData
  |
  */
pub struct ProcessContext {

    /**
      | a combination of the values from \ref
      | ProcessContextStatesAndFlags
      |
      */
    state:                  u32,

    /**
      | current sample rate (always valid)
      |
      */
    sample_rate:            f64,

    /**
      | project time in samples (always valid)
      |
      */
    project_time_samples:   TSamples,

    /**
      | system time in nanoseconds (optional)
      |
      */
    system_time:            i64,

    /**
      | project time, without loop (optional)
      |
      */
    continous_time_samples: TSamples,

    /**
      | musical position in quarter notes (1.0
      | equals 1 quarter note) (optional)
      |
      */
    project_time_music:     TQuarterNotes,

    /**
      | last bar start position, in quarter
      | notes (optional)
      |
      */
    bar_position_music:     TQuarterNotes,

    /**
      | cycle start in quarter notes (optional)
      |
      */
    cycle_start_music:      TQuarterNotes,

    /**
      | cycle end in quarter notes (optional)
      |
      */
    cycle_end_music:        TQuarterNotes,

    /**
      | tempo in BPM (Beats Per Minute) (optional)
      |
      */
    tempo:                  f64,

    /**
      | time signature numerator (e.g. 3 for
      | 3/4) (optional)
      |
      */
    time_sig_numerator:     i32,

    /**
      | time signature denominator (e.g. 4
      | for 3/4) (optional)
      |
      */
    time_sig_denominator:   i32,

    /**
      | musical info (optional)
      |
      */
    chord:                  Chord,

    /**
      | SMPTE (sync) offset in subframes (1/80
      | of frame) (optional)
      |
      */
    smpte_offset_subframes: i32,

    /**
      | frame rate (optional)
      |
      */
    frame_rate:             FrameRate,

    /**
      | MIDI Clock Resolution (24 Per Quarter
      | Note), can be negative (nearest) (optional)
      |
      */
    samples_to_next_clock:  i32,
}
