crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphUtilities.h]

/* -- This was copied from audio_utils/primitives.h  -- */

/**
  | Convert a single-precision floating
  | point value to a Q0.31 integer value.
  | 
  | Rounds to nearest, ties away from 0.
  | 
  | Values outside the range [-1.0, 1.0)
  | are properly clamped to -2147483648
  | and 2147483647, including -Inf and
  | +Inf. NaN values are considered undefined,
  | and behavior may change depending on
  | hardware and future implementation
  | of this function.
  |
  */
pub fn clamp_32from_float(f: f32) -> i32 {
    
    todo!();
    /*
        static const float scale = (float)(1UL << 31);
        static const float limpos = 1.;
        static const float limneg = -1.;

        if (f <= limneg) {
            return -0x80000000; /* or 0x80000000 */
        } else if (f >= limpos) {
            return 0x7fffffff;
        }
        f *= scale;
        /* integer conversion is through truncation (though int to float is not).
         * ensure that we round to nearest, ties away from 0.
         */
        return f > 0 ? f + 0.5 : f - 0.5;
    */
}

pub trait DataConversionFlowGraphInterface {}
