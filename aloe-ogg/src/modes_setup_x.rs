/*!
 function: catch-all toplevel settings for q modes
 only
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_X.h]

pub const RATE_MAPPING_X: [f64; 12] = [
  -1.0,-1.0,-1.0,-1.0,-1.0,-1.0,
  -1.0,-1.0,-1.0,-1.0,-1.0,-1.0
];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_X_stereo={
      11,
      rate_mapping_X,
      quality_mapping_44,
      2,
      50000,
      200000,

      blocksize_short_44,
      blocksize_long_44,

      _psy_tone_masteratt_44,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_otherblock,
      _vp_tonemask_adj_longblock,
      _vp_tonemask_adj_otherblock,

      _psy_noiseguards_44,
      _psy_noisebias_impulse,
      _psy_noisebias_padding,
      _psy_noisebias_trans,
      _psy_noisebias_long,
      _psy_noise_suppress,

      _psy_compand_44,
      _psy_compand_short_mapping,
      _psy_compand_long_mapping,

      {_noise_start_short_44,_noise_start_long_44},
      {_noise_part_short_44,_noise_part_long_44},
      _noise_thresh_44,

      _psy_ath_floater,
      _psy_ath_abs,

      _psy_lowpass_44,

      _psy_global_44,
      _global_mapping_44,
      _psy_stereo_modes_44,

      _floor_books,
      _floor,
      2,
      _floor_mapping_44,

      _mapres_template_44_stereo
    };
    */
}

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_X_uncoupled={
      11,
      rate_mapping_X,
      quality_mapping_44,
      -1,
      50000,
      200000,

      blocksize_short_44,
      blocksize_long_44,

      _psy_tone_masteratt_44,
      _psy_tone_0dB,
      _psy_tone_suppress,

      _vp_tonemask_adj_otherblock,
      _vp_tonemask_adj_longblock,
      _vp_tonemask_adj_otherblock,

      _psy_noiseguards_44,
      _psy_noisebias_impulse,
      _psy_noisebias_padding,
      _psy_noisebias_trans,
      _psy_noisebias_long,
      _psy_noise_suppress,

      _psy_compand_44,
      _psy_compand_short_mapping,
      _psy_compand_long_mapping,

      {_noise_start_short_44,_noise_start_long_44},
      {_noise_part_short_44,_noise_part_long_44},
      _noise_thresh_44,

      _psy_ath_floater,
      _psy_ath_abs,

      _psy_lowpass_44,

      _psy_global_44,
      _global_mapping_44,
      NULL,

      _floor_books,
      _floor,
      2,
      _floor_mapping_44,

      _mapres_template_44_uncoupled
    };
    */
}

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_XX_stereo={
      2,
      rate_mapping_X,
      quality_mapping_8,
      2,
      0,
      8000,

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
    static const ve_setup_data_template ve_setup_XX_uncoupled={
      2,
      rate_mapping_X,
      quality_mapping_8,
      -1,
      0,
      8000,

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
