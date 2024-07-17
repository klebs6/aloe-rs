/*!
 function: registry for time, floor, res backends
 and channel mappings
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/registry.h]

pub const VI_TRANSFORMB: usize = 1;
pub const VI_WINDOWB:    usize = 1;
pub const VI_TIMEB:      usize = 1;
pub const VI_FLOORB:     usize = 2;
pub const VI_RESB:       usize = 3;
pub const VI_MAPB:       usize = 1;

lazy_static!{
    /*
    extern const vorbis_func_floor     *const _floor_P[];
    extern const vorbis_func_residue   *const _residue_P[];
    extern const vorbis_func_mapping   *const _mapping_P[];
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/registry.c]

/**
  | seems like major overkill now; the backend
  | numbers will grow into the infrastructure
  | soon enough
  |
  */
lazy_static!{
    /*
    extern const vorbis_func_floor     floor0_exportbundle;
    extern const vorbis_func_floor     floor1_exportbundle;
    extern const vorbis_func_residue   residue0_exportbundle;
    extern const vorbis_func_residue   residue1_exportbundle;
    extern const vorbis_func_residue   residue2_exportbundle;
    extern const vorbis_func_mapping   mapping0_exportbundle;
    */
}

lazy_static!{
    /*
    static const vorbis_func_floor     *const _floor_P[]={
      &floor0_exportbundle,
      &floor1_exportbundle,
    };

    static const vorbis_func_residue   *const _residue_P[]={
      &residue0_exportbundle,
      &residue1_exportbundle,
      &residue2_exportbundle,
    };

    static const vorbis_func_mapping   *const _mapping_P[] = {
      &mapping0_exportbundle,
    };
    */
}

