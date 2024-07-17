/*!
 function: 11kHz settings
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_11.h]

pub const BLOCKSIZE_11: [i32; 2] = [
  512,512
];

pub const FLOOR_MAPPING_11A: &[i32] = &[
  6,6
];

pub const FLOOR_MAPPING_11: &[&[i32]] = &[ FLOOR_MAPPING_11A ];

pub const RATE_MAPPING_11: [f64; 3] = [
  8000.0,13000.0,44000.0,
];

pub const RATE_MAPPING_11_UNCOUPLED: [f64; 3] = [
  12000.0,20000.0,50000.0,
];

pub const QUALITY_MAPPING_11: [f64; 3] = [
  -0.1,0.0,1.0
];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_11_stereo={
      2,
      rate_mapping_11,
      quality_mapping_11,
      2,
      9000,
      15000,

      blocksize_11,
      blocksize_11,

      _psy_tone_masteratt_11,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_11,
      NULL,
      _vp_tonemask_adj_11,

      _psy_noiseguards_8,
      _psy_noisebias_11,
      _psy_noisebias_11,
      NULL,
      NULL,
      _psy_noise_suppress,

      _psy_compand_8,
      _psy_compand_8_mapping,
      NULL,

      {_noise_start_8,_noise_start_8},
      {_noise_part_8,_noise_part_8},
      _noise_thresh_11,

      _psy_ath_floater_8,
      _psy_ath_abs_8,

      _psy_lowpass_11,

      _psy_global_44,
      _global_mapping_8,
      _psy_stereo_modes_8,

      _floor_books,
      _floor,
      1,
      _floor_mapping_11,

      _mapres_template_8_stereo
    };
    */
}

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_11_uncoupled={
      2,
      rate_mapping_11_uncoupled,
      quality_mapping_11,
      -1,
      9000,
      15000,

      blocksize_11,
      blocksize_11,

      _psy_tone_masteratt_11,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_11,
      NULL,
      _vp_tonemask_adj_11,

      _psy_noiseguards_8,
      _psy_noisebias_11,
      _psy_noisebias_11,
      NULL,
      NULL,
      _psy_noise_suppress,

      _psy_compand_8,
      _psy_compand_8_mapping,
      NULL,

      {_noise_start_8,_noise_start_8},
      {_noise_part_8,_noise_part_8},
      _noise_thresh_11,

      _psy_ath_floater_8,
      _psy_ath_abs_8,

      _psy_lowpass_11,

      _psy_global_44,
      _global_mapping_8,
      _psy_stereo_modes_8,

      _floor_books,
      _floor,
      1,
      _floor_mapping_11,

      _mapres_template_8_uncoupled
    };
    */
}
