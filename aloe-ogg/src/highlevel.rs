/*!
  | function: highlevel encoder setup
  | struct separated out for vorbisenc
  | clarity
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/highlevel.h]

pub struct HighLevelByBlockType {
    tone_mask_setting:      f64,
    tone_peaklimit_setting: f64,
    noise_bias_setting:     f64,
    noise_compand_setting:  f64,
}

pub struct HighLevelEncodeSetup {
    set_in_stone:              i32,
    setup:                     *const c_void,
    base_setting:              f64,
    impulse_noisetune:         f64,

    /**
      | bitrate management below all settable
      |
      */
    req:                       f32,

    managed:                   i32,
    bitrate_min:               i64,
    bitrate_av:                i64,
    bitrate_av_damp:           f64,
    bitrate_max:               i64,
    bitrate_reservoir:         i64,
    bitrate_reservoir_bias:    f64,
    impulse_block_p:           i32,
    noise_normalize_p:         i32,
    coupling_p:                i32,
    stereo_point_setting:      f64,
    lowpass_k_hz:              f64,
    lowpass_altered:           i32,
    ath_floating_db:           f64,
    ath_absolute_db:           f64,
    amplitude_track_d_bpersec: f64,
    trigger_setting:           f64,

    /**
      | padding, impulse, transition, long
      |
      */
    block:                     [HighLevelByBlockType; 4],
}
