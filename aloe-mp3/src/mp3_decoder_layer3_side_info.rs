#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub struct Mp3DecoderLayer3SideInfo {
    ch:              [Layer3SideInfoPair; 2],
    main_data_start: u32,
    private_bits:    u32,
}

pub struct Layer3SideInfoPair {
    gr: [Layer3SideInfo; 2],
}

//-------------------------
pub struct Layer3SideInfo {
    scfsi:                    i32,
    part2_3length:            u32,
    big_values:               u32,
    scale_factor_compression: u32,
    block_type:               u32,
    mixed_block_flag:         u32,
    table_select:             [u32; 3],
    max_band:                 [u32; 3],
    max_bandl:                u32,
    maxb:                     u32,
    region_1start:            u32,
    region_2start:            u32,
    preflag:                  u32,
    scale_factor_scale:       u32,
    count_1table_select:      u32,
    full_gain:                [*const f32; 3],
    pow2gain:                 *const f32,
}

impl Layer3SideInfo {

    pub fn do_antialias(&self, xr: [[f32; 32]; 18])  {
        
        todo!();
        /*
            float* xr1 = xr[1];
                int sb;

                if (blockType == 2)
                {
                    if (mixedBlockFlag == 0)
                        return;

                    sb = 1;
                }
                else
                    sb = (int) maxb - 1;

                for (; sb != 0; --sb, xr1 += 10)
                {
                    auto* cs = constants.antiAliasingCs;
                    auto* ca = constants.antiAliasingCa;
                    auto* xr2 = xr1;

                    for (int ss = 7; ss >= 0; --ss)
                    {
                        const float bu = *--xr2, bd = *xr1;
                        *xr2   = (bu * *cs)   - (bd * *ca);
                        *xr1++ = (bd * *cs++) + (bu * *ca++);
                    }
                }
        */
    }
    
    pub fn do_istereo(&self, 
        xr_buffer:     [[[f32; 2]; 32]; 18],
        scale_factors: *const i32,
        sample_rate:   i32,
        ms_stereo:     bool,
        lsf:           i32)  {
        
        todo!();
        /*
            float (*xr) [32 * 18] = (float (*) [32 * 18]) xrBuffer;
                auto& bi = bandInfo[sampleRate];
                const float* tabl1, *tabl2;

                if (lsf != 0)
                {
                    auto p = scaleFactorCompression & 1;

                    if (msStereo)
                    {
                        tabl1 = constants.pow1_2[p];
                        tabl2 = constants.pow2_2[p];
                    }
                    else
                    {
                        tabl1 = constants.pow1_1[p];
                        tabl2 = constants.pow2_1[p];
                    }
                }
                else
                {
                    if (msStereo)
                    {
                        tabl1 = constants.tan1_2;
                        tabl2 = constants.tan2_2;
                    }
                    else
                    {
                        tabl1 = constants.tan1_1;
                        tabl2 = constants.tan2_1;
                    }
                }

                if (blockType == 2)
                {
                    bool doL = mixedBlockFlag != 0;

                    for (uint32 lwin = 0; lwin < 3; ++lwin)
                    {
                        uint32 sfb = maxBand[lwin];
                        doL = doL && (sfb <= 3);

                        for (; sfb < 12; ++sfb)
                        {
                            auto p = scaleFactors[sfb * 3 + lwin - mixedBlockFlag];

                            if (p != 7)
                            {
                                auto t1 = tabl1[p];
                                auto t2 = tabl2[p];
                                int sb = bi.shortDiff[sfb];
                                auto index = (uint32) sb + lwin;

                                for (; sb > 0; --sb, index += 3)
                                {
                                    float v = xr[0][index];
                                    xr[0][index] = v * t1;
                                    xr[1][index] = v * t2;
                                }
                            }
                        }

                        auto p = scaleFactors[11 * 3 + lwin - mixedBlockFlag];

                        if (p != 7)
                        {
                            auto t1 = tabl1[p];
                            auto t2 = tabl2[p];
                            int sb = bi.shortDiff[12];
                            auto index = (uint32) sb + lwin;

                            for (; sb > 0; --sb, index += 3)
                            {
                                float v = xr[0][index];
                                xr[0][index] = v * t1;
                                xr[1][index] = v * t2;
                            }
                        }
                    }

                    if (doL)
                    {
                        int index = bi.longIndex[maxBandl];

                        for (uint32 sfb = maxBandl; sfb < 8; ++sfb)
                        {
                            int sb = bi.longDiff[sfb];
                            auto p = scaleFactors[sfb];

                            if (p != 7)
                            {
                                auto t1 = tabl1[p];
                                auto t2 = tabl2[p];

                                for (; sb > 0; --sb, ++index)
                                {
                                    float v = xr[0][index];
                                    xr[0][index] = v * t1;
                                    xr[1][index] = v * t2;
                                }
                            }
                            else
                                index += sb;
                        }
                    }
                }
                else
                {
                    int index = bi.longIndex[maxBandl];

                    for (uint32 sfb = maxBandl; sfb < 21; ++sfb)
                    {
                        int sb = bi.longDiff[sfb];
                        auto p = scaleFactors[sfb];

                        if (p != 7)
                        {
                            auto t1 = tabl1[p];
                            auto t2 = tabl2[p];

                            for (; sb > 0; --sb, ++index)
                            {
                                const float v = xr[0][index];
                                xr[0][index] = v * t1;
                                xr[1][index] = v * t2;
                            }
                        }
                        else
                            index += sb;
                    }

                    auto p = scaleFactors[20];

                    if (p != 7)
                    {
                        auto t1 = tabl1[p], t2 = tabl2[p];

                        for (int sb = bi.longDiff[21]; sb > 0; --sb, ++index)
                        {
                            const float v = xr[0][index];
                            xr[0][index] = v * t1;
                            xr[1][index] = v * t2;
                        }
                    }
                }
        */
    }
}
