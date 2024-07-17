/*!
 function: 8kHz settings
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_8.h]

pub const BLOCKSIZE_8:              [i32; 2]  = [ 512    , 512 ];
pub const RATE_MAPPING_8:           [f64; 3]  = [ 6000.0 , 9000.0   , 32000.0 ];
pub const RATE_MAPPING_8_UNCOUPLED: [f64; 3]  = [ 8000.0 , 14000.0  , 42000.0 ];
pub const QUALITY_MAPPING_8:        [f64; 3]  = [ -0.1   , 0.0      , 1.0 ];
pub const PSY_COMPAND_8_MAPPING:    [f64; 3]  = [ 0.0    , 1.0      , 1.0];
pub const GLOBAL_MAPPING_8:         [f64; 3]  = [ 1.0    , 2.0      , 3.0 ];

pub const FLOOR_MAPPING_8A: &[i32]     = &[ 6, 6 ];
pub const FLOOR_MAPPING_8:  &[&[i32]]  = &[ FLOOR_MAPPING_8A ];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_8_stereo={
      2,
      rate_mapping_8,
      quality_mapping_8,
      2,
      8000,
      9000,

      blocksize_8,
      blocksize_8,

      _psy_tone_masteratt_8,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_8,
      NULL,
      _vp_tonemask_adj_8,

      _psy_noiseguards_8,
      _psy_noisebias_8,
      _psy_noisebias_8,
      NULL,
      NULL,
      _psy_noise_suppress,

      _psy_compand_8,
      _psy_compand_8_mapping,
      NULL,

      {_noise_start_8,_noise_start_8},
      {_noise_part_8,_noise_part_8},
      _noise_thresh_5only,

      _psy_ath_floater_8,
      _psy_ath_abs_8,

      _psy_lowpass_8,

      _psy_global_44,
      _global_mapping_8,
      _psy_stereo_modes_8,

      _floor_books,
      _floor,
      1,
      _floor_mapping_8,

      _mapres_template_8_stereo
    };
    */
}

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_8_uncoupled={
      2,
      rate_mapping_8_uncoupled,
      quality_mapping_8,
      -1,
      8000,
      9000,

      blocksize_8,
      blocksize_8,

      _psy_tone_masteratt_8,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_8,
      NULL,
      _vp_tonemask_adj_8,

      _psy_noiseguards_8,
      _psy_noisebias_8,
      _psy_noisebias_8,
      NULL,
      NULL,
      _psy_noise_suppress,

      _psy_compand_8,
      _psy_compand_8_mapping,
      NULL,

      {_noise_start_8,_noise_start_8},
      {_noise_part_8,_noise_part_8},
      _noise_thresh_5only,

      _psy_ath_floater_8,
      _psy_ath_abs_8,

      _psy_lowpass_8,

      _psy_global_44,
      _global_mapping_8,
      _psy_stereo_modes_8,

      _floor_books,
      _floor,
      1,
      _floor_mapping_8,

      _mapres_template_8_uncoupled
    };
    */
}
