/*!
  | function: toplevel settings for 32kHz
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_32.h]

pub const RATE_MAPPING_32: [f64; 12] = [
  18000.,28000.,35000.,45000.,56000.,60000.,
  75000.,90000.,100000.,115000.,150000.,190000.,
];

pub const RATE_MAPPING_32_UN: [f64; 12] = [
  30000.,42000.,52000.,64000.,72000.,78000.,
  86000.,92000.,110000.,120000.,140000.,190000.,
];

pub const PSY_LOWPASS_32: [f64; 12] = [
  12.3,13.,13.,14.,15.,99.,99.,99.,99.,99.,99.,99.
];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_32_stereo={
      11,
      rate_mapping_32,
      quality_mapping_44,
      2,
      26000,
      40000,

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

      _psy_lowpass_32,

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
    static const ve_setup_data_template ve_setup_32_uncoupled={
      11,
      rate_mapping_32_un,
      quality_mapping_44,
      -1,
      26000,
      40000,

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

      _psy_lowpass_32,

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
