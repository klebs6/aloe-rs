/*!
 function: toplevel residue templates 8/11kHz
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/modes/residue_8.h]

/***** residue backends *********************************************/

lazy_static!{
    /*
    static const static_bookblock _resbook_8s_0={
      {
        {0},
        {0,0,&_8c0_s_p1_0},
        {0},
        {0,0,&_8c0_s_p3_0},
        {0,0,&_8c0_s_p4_0},
        {0,0,&_8c0_s_p5_0},
        {0,0,&_8c0_s_p6_0},
        {&_8c0_s_p7_0,&_8c0_s_p7_1},
        {&_8c0_s_p8_0,&_8c0_s_p8_1},
        {&_8c0_s_p9_0,&_8c0_s_p9_1,&_8c0_s_p9_2}
       }
    };
    */
}

lazy_static!{
    /*
    static const static_bookblock _resbook_8s_1={
      {
        {0},
        {0,0,&_8c1_s_p1_0},
        {0},
        {0,0,&_8c1_s_p3_0},
        {0,0,&_8c1_s_p4_0},
        {0,0,&_8c1_s_p5_0},
        {0,0,&_8c1_s_p6_0},
        {&_8c1_s_p7_0,&_8c1_s_p7_1},
        {&_8c1_s_p8_0,&_8c1_s_p8_1},
        {&_8c1_s_p9_0,&_8c1_s_p9_1,&_8c1_s_p9_2}
       }
    };
    */
}

lazy_static!{
    /*
    static const vorbis_residue_template _res_8s_0[]={
      {2,0,32,  &_residue_44_mid,
       &_huff_book__8c0_s_single,&_huff_book__8c0_s_single,
       &_resbook_8s_0,&_resbook_8s_0},
    };
    */
}

lazy_static!{
    /*
    static const vorbis_residue_template _res_8s_1[]={
      {2,0,32,  &_residue_44_mid,
       &_huff_book__8c1_s_single,&_huff_book__8c1_s_single,
       &_resbook_8s_1,&_resbook_8s_1},
    };
    */
}

lazy_static!{
    /*
    static const vorbis_mapping_template _mapres_template_8_stereo[2]={
      { _map_nominal, _res_8s_0 }, /* 0 */
      { _map_nominal, _res_8s_1 }, /* 1 */
    };
    */
}

lazy_static!{
    /*
    static const static_bookblock _resbook_8u_0={
      {
        {0},
        {0,0,&_8u0__p1_0},
        {0,0,&_8u0__p2_0},
        {0,0,&_8u0__p3_0},
        {0,0,&_8u0__p4_0},
        {0,0,&_8u0__p5_0},
        {&_8u0__p6_0,&_8u0__p6_1},
        {&_8u0__p7_0,&_8u0__p7_1,&_8u0__p7_2}
       }
    };
    */
}

lazy_static!{
    /*
    static const static_bookblock _resbook_8u_1={
      {
        {0},
        {0,0,&_8u1__p1_0},
        {0,0,&_8u1__p2_0},
        {0,0,&_8u1__p3_0},
        {0,0,&_8u1__p4_0},
        {0,0,&_8u1__p5_0},
        {0,0,&_8u1__p6_0},
        {&_8u1__p7_0,&_8u1__p7_1},
        {&_8u1__p8_0,&_8u1__p8_1},
        {&_8u1__p9_0,&_8u1__p9_1,&_8u1__p9_2}
       }
    };
    */
}

lazy_static!{
    /*
    static const vorbis_residue_template _res_8u_0[]={
      {1,0,32,  &_residue_44_low_un,
       &_huff_book__8u0__single,&_huff_book__8u0__single,
       &_resbook_8u_0,&_resbook_8u_0},
    };
    */
}

lazy_static!{
    /*
    static const vorbis_residue_template _res_8u_1[]={
      {1,0,32,  &_residue_44_mid_un,
       &_huff_book__8u1__single,&_huff_book__8u1__single,
       &_resbook_8u_1,&_resbook_8u_1},
    };
    */
}

lazy_static!{
    /*
    static const vorbis_mapping_template _mapres_template_8_uncoupled[2]={
      { _map_nominal_u, _res_8u_0 }, /* 0 */
      { _map_nominal_u, _res_8u_1 }, /* 1 */
    };
    */
}
