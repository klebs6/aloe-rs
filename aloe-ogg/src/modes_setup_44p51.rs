/*!
 function: toplevel settings for 44.1/48kHz 5.1
 surround modes
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_44p51.h]

pub const RATE_MAPPING_44P51: [f64; 12] = [
    14000.0, 20000.0, 28000.0,  38000.0,  46000.0,  54000.0, 
    75000.0, 96000.0, 120000.0, 140000.0, 180000.0, 240001.0
];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_44_51={
      11,
      rate_mapping_44p51,
      quality_mapping_44,
      6,
      40000,
      70000,

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
      3,
      _floor_mapping_44,

      _mapres_template_44_51
    };
    */
}

