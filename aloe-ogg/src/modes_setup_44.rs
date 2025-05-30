/*!
 function: toplevel settings for 44.1/48kHz
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/setup_44.h]

pub const RATE_MAPPING_44_STEREO: [f64; 12] = [
    22500.,32000.,40000.,48000.,56000.,64000.,
    80000.,96000.,112000.,128000.,160000.,250001.
];

pub const QUALITY_MAPPING_44: [f64; 12] = [
    -0.1,0.0,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1.0
];

pub const BLOCKSIZE_SHORT_44: [i32; 11] = [
    512,256,256,256,256,256,256,256,256,256,256
];

pub const BLOCKSIZE_LONG_44: [i32; 11] = [
    4096,2048,2048,2048,2048,2048,2048,2048,2048,2048,2048
];

pub const PSY_COMPAND_SHORT_MAPPING: [f64; 12] = [
    0.5, 1., 1., 1.3, 1.6, 2., 2., 2., 2., 2., 2., 2.
];

pub const PSY_COMPAND_LONG_MAPPING: [f64; 12] = [
    3.5, 4., 4., 4.3, 4.6, 5., 5., 5., 5., 5., 5., 5.
];

pub const GLOBAL_MAPPING_44: [f64; 12] = [
    /*  1., 1., 1.5, 2., 2., 2.5, 2.7, 3.0, 3.5, 4., 4. */
    0., 1., 1., 1.5, 2., 2., 2.5, 2.7, 3.0, 3.7, 4., 4.
];

pub const FLOOR_MAPPING_44A: &[i32] = &[
    1,0,0,2,2,4,5,5,5,5,5
];

pub const FLOOR_MAPPING_44B: &[i32] = &[
    8,7,7,7,7,7,7,7,7,7,7
];

pub const FLOOR_MAPPING_44C: &[i32] = &[
    10,10,10,10,10,10,10,10,10,10,10
];

pub const FLOOR_MAPPING_44: &[&[i32]] = &[
    FLOOR_MAPPING_44A,
    FLOOR_MAPPING_44B,
    FLOOR_MAPPING_44C,
];

lazy_static!{
    /*
    static const ve_setup_data_template ve_setup_44_stereo={
      11,
      rate_mapping_44_stereo,
      quality_mapping_44,
      2,
      40000,
      50000,

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
