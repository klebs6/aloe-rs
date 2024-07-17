crate::ix!();

pub struct AUTimestampGeneratorState {
    current_input_time:                 AudioTimeStamp,
    next_input_sample_time:             f64,
    next_output_sample_time:            f64,
    input_sample_time_for_output_pull:  f64,
    last_output_time:                   AudioTimeStamp,
    current_output_time:                AudioTimeStamp,

    /**
      | if true, input timeline starts at 0,
      | else it starts synced with the output
      | timeline
      |
      */
    start_input_at_zero:                bool,

    discontinuous:                      bool,
    bypassed:                           bool,
    discontinuity_delta_samples:        f64,
    rate_scalar_adj:                    f64,

    /**
      | If true, propagate timestamp discontinuities
      | using host time.
      |
      */
    host_time_discontinuity_correction: bool,
}
