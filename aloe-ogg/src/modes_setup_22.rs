/*!
 function: 22kHz settings
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_22.h]

pub const RATE_MAPPING_22:           [f64; 4] = [ 15000.0,20000.0,44000.0,86000.0 ];
pub const RATE_MAPPING_22_UNCOUPLED: [f64; 4] = [ 16000.0,28000.0,50000.0,90000.0 ];

pub const PSY_LOWPASS_22: [f64; 4] = [9.5,11.0,30.0,99.0];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_22_stereo={
      3,
      rate_mapping_22,
      quality_mapping_16,
      2,
      19000,
      26000,

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

      _psy_lowpass_22,

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
    static const ve_setup_data_template ve_setup_22_uncoupled={
      3,
      rate_mapping_22_uncoupled,
      quality_mapping_16,
      -1,
      19000,
      26000,

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

      _psy_lowpass_22,

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
