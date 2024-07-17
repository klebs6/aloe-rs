/*!
 function: 16kHz settings
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_16.h]

pub const FLOOR_MAPPING_16A:         &[i32]   = &[ 9,3,3 ];
pub const FLOOR_MAPPING_16B:         &[i32]   = &[ 9,9,9 ];

pub const FLOOR_MAPPING_16: &[&[i32]] = &[
  FLOOR_MAPPING_16A,
  FLOOR_MAPPING_16B
];

pub const BLOCKSIZE_16_SHORT:        [i32; 3] = [ 1024,512,512 ];
pub const BLOCKSIZE_16_LONG:         [i32; 3] = [ 1024,1024,1024 ];
pub const RATE_MAPPING_16:           [f64; 4] = [ 12000.0,20000.0,44000.0,86000.0 ];
pub const RATE_MAPPING_16_UNCOUPLED: [f64; 4] = [ 16000.0,28000.0,64000.0,100000.0 ];
pub const GLOBAL_MAPPING_16:         [f64; 4] = [ 1.0, 2.0, 3.0, 4.0 ];
pub const QUALITY_MAPPING_16:        [f64; 4] = [ -0.1,0.05,0.5,1.0 ];
pub const PSY_COMPAND_16_MAPPING:    [f64; 4] = [ 0.0, 0.8, 1.0, 1.0];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_16_stereo={
      3,
      rate_mapping_16,
      quality_mapping_16,
      2,
      15000,
      19000,

      blocksize_16_short,
      blocksize_16_long,

      _psy_tone_masteratt_16,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_16,
      _vp_tonemask_adj_16,
      _vp_tonemask_adj_16,

      _psy_noiseguards_16,
      _psy_noisebias_16_impulse,
      _psy_noisebias_16_short,
      _psy_noisebias_16_short,
      _psy_noisebias_16,
      _psy_noise_suppress,

      _psy_compand_8,
      _psy_compand_16_mapping,
      _psy_compand_16_mapping,

      {_noise_start_16,_noise_start_16},
      { _noise_part_16, _noise_part_16},
      _noise_thresh_16,

      _psy_ath_floater_16,
      _psy_ath_abs_16,

      _psy_lowpass_16,

      _psy_global_44,
      _global_mapping_16,
      _psy_stereo_modes_16,

      _floor_books,
      _floor,
      2,
      _floor_mapping_16,

      _mapres_template_16_stereo
    };
    */
}

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_16_uncoupled={
      3,
      rate_mapping_16_uncoupled,
      quality_mapping_16,
      -1,
      15000,
      19000,

      blocksize_16_short,
      blocksize_16_long,

      _psy_tone_masteratt_16,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_16,
      _vp_tonemask_adj_16,
      _vp_tonemask_adj_16,

      _psy_noiseguards_16,
      _psy_noisebias_16_impulse,
      _psy_noisebias_16_short,
      _psy_noisebias_16_short,
      _psy_noisebias_16,
      _psy_noise_suppress,

      _psy_compand_8,
      _psy_compand_16_mapping,
      _psy_compand_16_mapping,

      {_noise_start_16,_noise_start_16},
      { _noise_part_16, _noise_part_16},
      _noise_thresh_16,

      _psy_ath_floater_16,
      _psy_ath_abs_16,

      _psy_lowpass_16,

      _psy_global_44,
      _global_mapping_16,
      _psy_stereo_modes_16,

      _floor_books,
      _floor,
      2,
      _floor_mapping_16,

      _mapres_template_16_uncoupled
    };
    */
}
