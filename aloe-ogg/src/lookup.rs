/*!
  | function: lookup based functions
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/lookup.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/lookup.c]

/**
  | interpolated lookup based cos function,
  | domain 0 to PI only
  */
#[cfg(feature = "float-lookup")]
pub fn float_lookup_vorbis_coslook(a: f32) -> f32 {

    todo!();
    /*
       double d=a*(.31830989*(float)COS_LOOKUP_SZ);
       int i=vorbis_ftoi(d-.5);

       return COS_LOOKUP[i]+ (d-i)*(COS_LOOKUP[i+1]-COS_LOOKUP[i]);
       */
}

/**
  | interpolated 1./sqrt(p) where .5 <=
  | p < 1.
  */
#[cfg(feature = "float-lookup")]
pub fn float_lookup_vorbis_invsqlook(a: f32) -> f32 {
    
    todo!();
    /*
        double d=a*(2.f*(float)INVSQ_LOOKUP_SZ)-(float)INVSQ_LOOKUP_SZ;
            int i=vorbis_ftoi(d-.5f);
            return INVSQ_LOOKUP[i]+ (d-i)*(INVSQ_LOOKUP[i+1]-INVSQ_LOOKUP[i]);
    */
}


/**
  | interpolated 1./sqrt(p) where .5 <=
  | p < 1.
  |
  */
#[cfg(feature = "float-lookup")]
pub fn float_lookup_vorbis_invsq2explook(a: i32) -> f32 {
    
    todo!();
    /*
        return INVSQ2EXP_LOOKUP[a-INVSQ2EXP_LOOKUP_MIN];
    */
}

/**
  | interpolated lookup based fromdB function,
  | domain -140dB to 0dB only
  |
  */
#[cfg(feature = "float-lookup")]
pub fn float_lookup_vorbis_fromd_blook(a: f32) -> f32 {
    
    todo!();
    /*
        int i=vorbis_ftoi(a*((float)(-(1<<FROMdB2_SHIFT)))-.5f);
            return (i<0)?1.f:
                ((i>=(FROMdB_LOOKUP_SZ<<FROMdB_SHIFT))?0.f:
                 FROMdB_LOOKUP[i>>FROMdB_SHIFT]*FROMdB2_LOOKUP[i&FROMdB2_MASK]);
    */
}

#[cfg(INT_LOOKUP)]
pub mod int_lookup {
    use super::*;

    /**
      | interpolated 1./sqrt(p) where .5 <=
      | a < 1. (.100000... to .111111...) in
      | 16.16 format
      | 
      | returns in m.8 format
      |
      */
    pub fn vorbis_invsqlook_i(a: i64, e: i64) -> i64 {
        
        todo!();
        /*
            long i=(a&0x7fff)>>(INVSQ_LOOKUP_I_SHIFT-1);
              long d=(a&INVSQ_LOOKUP_I_MASK)<<(16-INVSQ_LOOKUP_I_SHIFT); /*  0.16 */
              long val=INVSQ_LOOKUP_I[i]-                                /*  1.16 */
                (((INVSQ_LOOKUP_I[i]-INVSQ_LOOKUP_I[i+1])*               /*  0.16 */
                  d)>>16);                                               /* result 1.16 */

              e+=32;
              if(e&1)val=(val*5792)>>13; /* multiply val by 1/sqrt(2) */
              e=(e>>1)-8;

              return(val>>e);
        */
    }


    /**
      | interpolated lookup based fromdB function,
      | domain -140dB to 0dB only a is in n.12
      | format
      |
      */
    pub fn vorbis_fromd_blook_i(a: i64) -> f32 {
        
        todo!();
        /*
            int i=(-a)>>(12-FROMdB2_SHIFT);
              return (i<0)?1.f:
                ((i>=(FROMdB_LOOKUP_SZ<<FROMdB_SHIFT))?0.f:
                 FROMdB_LOOKUP[i>>FROMdB_SHIFT]*FROMdB2_LOOKUP[i&FROMdB2_MASK]);
        */
    }

    /**
      | interpolated lookup based cos function,
      | domain 0 to PI only a is in 0.16 format,
      | where 0==0, 2^^16-1==PI, return 0.14
      |
      */
    pub fn vorbis_coslook_i(a: i64) -> i64 {
        
        todo!();
        /*
            int i=a>>COS_LOOKUP_I_SHIFT;
              int d=a&COS_LOOKUP_I_MASK;
              return COS_LOOKUP_I[i]- ((d*(COS_LOOKUP_I[i]-COS_LOOKUP_I[i+1]))>>
                                       COS_LOOKUP_I_SHIFT);
        */
    }
}
