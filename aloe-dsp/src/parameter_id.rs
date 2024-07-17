crate::ix!();

pub mod id {

    macro_rules! parameter_id {
        ($name:ident) => {
            pub const $name: &'static str = stringify!($name);
        };
    }

    parameter_id!{ INPUT_GAIN                 } 
    parameter_id!{ OUTPUT_GAIN                } 
    parameter_id!{ PAN                        } 
    parameter_id!{ DISTORTION_ENABLED         } 
    parameter_id!{ DISTORTION_TYPE            } 
    parameter_id!{ DISTORTION_OVERSAMPLER     } 
    parameter_id!{ DISTORTION_LOWPASS         } 
    parameter_id!{ DISTORTION_HIGHPASS        } 
    parameter_id!{ DISTORTION_IN_GAIN         } 
    parameter_id!{ DISTORTION_COMP_GAIN       } 
    parameter_id!{ DISTORTION_MIX             } 
    parameter_id!{ CONVOLUTION_CAB_ENABLED    } 
    parameter_id!{ CONVOLUTION_REVERB_ENABLED } 
    parameter_id!{ CONVOLUTION_REVERB_MIX     } 
    parameter_id!{ MULTI_BAND_ENABLED         } 
    parameter_id!{ MULTI_BAND_FREQ            } 
    parameter_id!{ MULTI_BAND_LOW_VOLUME      } 
    parameter_id!{ MULTI_BAND_HIGH_VOLUME     } 
    parameter_id!{ COMPRESSOR_ENABLED         } 
    parameter_id!{ COMPRESSOR_THRESHOLD       } 
    parameter_id!{ COMPRESSOR_RATIO           } 
    parameter_id!{ COMPRESSOR_ATTACK          } 
    parameter_id!{ COMPRESSOR_RELEASE         } 
    parameter_id!{ NOISE_GATE_ENABLED         } 
    parameter_id!{ NOISE_GATE_THRESHOLD       } 
    parameter_id!{ NOISE_GATE_RATIO           } 
    parameter_id!{ NOISE_GATE_ATTACK          } 
    parameter_id!{ NOISE_GATE_RELEASE         } 
    parameter_id!{ LIMITER_ENABLED            } 
    parameter_id!{ LIMITER_THRESHOLD          } 
    parameter_id!{ LIMITER_RELEASE            } 
    parameter_id!{ DIRECT_DELAY_ENABLED       } 
    parameter_id!{ DIRECT_DELAY_TYPE          } 
    parameter_id!{ DIRECT_DELAY_VALUE         } 
    parameter_id!{ DIRECT_DELAY_SMOOTHING     } 
    parameter_id!{ DIRECT_DELAY_MIX           } 
    parameter_id!{ DELAY_EFFECT_ENABLED       } 
    parameter_id!{ DELAY_EFFECT_TYPE          } 
    parameter_id!{ DELAY_EFFECT_VALUE         } 
    parameter_id!{ DELAY_EFFECT_SMOOTHING     } 
    parameter_id!{ DELAY_EFFECT_LOWPASS       } 
    parameter_id!{ DELAY_EFFECT_FEEDBACK      } 
    parameter_id!{ DELAY_EFFECT_MIX           } 
    parameter_id!{ PHASER_ENABLED             } 
    parameter_id!{ PHASER_RATE                } 
    parameter_id!{ PHASER_DEPTH               } 
    parameter_id!{ PHASER_CENTRE_FREQUENCY    } 
    parameter_id!{ PHASER_FEEDBACK            } 
    parameter_id!{ PHASER_MIX                 } 
    parameter_id!{ CHORUS_ENABLED             } 
    parameter_id!{ CHORUS_RATE                } 
    parameter_id!{ CHORUS_DEPTH               } 
    parameter_id!{ CHORUS_CENTRE_DELAY        } 
    parameter_id!{ CHORUS_FEEDBACK            } 
    parameter_id!{ CHORUS_MIX                 } 
    parameter_id!{ LADDER_ENABLED             } 
    parameter_id!{ LADDER_CUTOFF              } 
    parameter_id!{ LADDER_RESONANCE           } 
    parameter_id!{ LADDER_DRIVE               } 
    parameter_id!{ LADDER_MODE                } 
}
