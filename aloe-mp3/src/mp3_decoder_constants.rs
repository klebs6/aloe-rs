#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub struct Mp3DecoderConstants {
    muls:             [[f32; 27]; 64],
    n_to_the_4over3:  [f32; 8207],
    anti_aliasing_ca: [f32; 8],
    anti_aliasing_cs: [f32; 8],
    win:              [[f32; 4]; 36],
    win1:             [[f32; 4]; 36],
    pow_to_gains:     [f32; 256 + 118 + 4],
    long_limit:       [[i32; 9]; 23],
    short_limit:      [[i32; 9]; 14],
    tan1_1:           [f32; 16],
    tan2_1:           [f32; 16],
    tan1_2:           [f32; 16],
    tan2_2:           [f32; 16],
    pow1_1:           [[f32; 2]; 16],
    pow2_1:           [[f32; 2]; 16],
    pow1_2:           [[f32; 2]; 16],
    pow2_2:           [[f32; 2]; 16],
    map:              [[*mut i32; 9]; 3],
    map_end:          [[*mut i32; 9]; 3],
    n_length2:        [u32; 512],
    length2:          [u32; 256],
    decode_win:       [f32; 512 + 32],
    cos_tables:       [*mut f32; 5],
    mapbuf0:          [[i32; 9]; 152],
    mapbuf1:          [[i32; 9]; 156],
    mapbuf2:          [[i32; 9]; 44],
    cos64:            [f32; 16],
    cos32:            [f32; 8],
    cos16:            [f32; 4],
    cos8:             [f32; 2],
    cos4:             [f32; 1],
    group3tab:        [u8; 32 * 3],
    group5tab:        [u8; 128 * 3],
    group9tab:        [u8; 1024 * 3],
}

impl Default for Mp3DecoderConstants {
    
    fn default() -> Self {
        todo!();
        /*


            cosTables[0] = cos64; cosTables[1] = cos32; cosTables[2] = cos16; cosTables[3] = cos8; cosTables[4] = cos4;
            initDecodeTables();
            initLayer2Tables();
            initLayer3Tables();
        */
    }
}

impl Mp3DecoderConstants {

    pub fn get_group_table(&self, 
        d1:    i16,
        index: u32) -> *const u8 {
        
        todo!();
        /*
            switch (d1)
            {
                case 3:   return &group3tab[3 * jmin (index, 3u * 3u * 3u)];
                case 5:   return &group5tab[3 * jmin (index, 5u * 5u * 5u)];
                case 9:   return &group9tab[3 * jmin (index, 9u * 9u * 9u)];
                default:  break;
            }

            static constexpr uint8 dummy[] = { 0, 0, 0 };
            return dummy;
        */
    }
    
    pub fn init_decode_tables(&mut self)  {
        
        todo!();
        /*
            int i, j, scaleval = -1;
            float* table = decodeWin;

            for (i = 0; i < 5; ++i)
            {
                int kr = 0x10 >> i;
                int divv = 0x40 >> i;
                float* costab = cosTables[i];

                for (int k = 0; k < kr; ++k)
                    costab[k] = (float) (1.0 / (2.0 * std::cos (MathConstants<double>::pi * (k * 2 + 1) / divv)));
            }

            for (i = 0, j = 0; i < 256; ++i, ++j, table += 32)
            {
                if (table < decodeWin + 512 + 16)
                    table[16] = table[0] = (float) (decodeWindow[j] * scaleval);
                if (i % 32 == 31)
                    table -= 1023;
                if (i % 64 == 63)
                    scaleval = -scaleval;
            }

            for (; i < 512; ++i, --j, table += 32)
            {
                if (table < decodeWin + 512 + 16)
                    table[16] = table[0] = (float) (decodeWindow[j] * scaleval);

                if (i % 32 == 31) table -= 1023;
                if (i % 64 == 63) scaleval = -scaleval;
            }
        */
    }
    
    pub fn init_layer_2tables(&mut self)  {
        
        todo!();
        /*
            static const uint8 base[3][9] =
            {
                { 1, 0, 2 },
                { 17, 18, 0, 19, 20 },
                { 21, 1, 22, 23, 0, 24, 25, 2, 26 }
            };

            static constexpr int tableLengths[] = { 3, 5, 9 };
            static uint8* tables[] = { group3tab, group5tab, group9tab };

            for (int i = 0; i < 3; ++i)
            {
                uint8* table = tables[i];
                const int len = tableLengths[i];

                for (int j = 0; j < len; ++j)
                    for (int k = 0; k < len; ++k)
                        for (int l = 0; l < len; ++l)
                        {
                            *table++ = base[i][l];
                            *table++ = base[i][k];
                            *table++ = base[i][j];
                        }
            }

            for (int k = 0; k < 27; ++k)
            {
                static constexpr double multipliers[] =
                {
                    0, -2.0 / 3.0, 2.0 / 3.0, 2.0 / 7.0, 2.0 / 15.0, 2.0 / 31.0, 2.0 / 63.0, 2.0 / 127.0, 2.0 / 255.0,
                    2.0 / 511.0, 2.0 / 1023.0, 2.0 / 2047.0, 2.0 / 4095.0, 2.0 / 8191.0, 2.0 / 16383.0, 2.0 / 32767.0, 2.0 / 65535.0,
                    -4.0 / 5.0, -2.0 / 5.0, 2.0 / 5.0, 4.0 / 5.0, -8.0 / 9.0, -4.0 / 9.0, -2.0 / 9.0, 2.0 / 9.0, 4.0 / 9.0, 8.0 / 9.0
                };

                float* table = muls[k];
                for (int j = 3, i = 0; i < 63; ++i, --j)
                    *table++ = (float) (multipliers[k] * std::pow (2.0, j / 3.0));
                *table++ = 0;
            }
        */
    }
    
    pub fn init_layer_3tables(&mut self)  {
        
        todo!();
        /*
            int i, j;
            for (i = -256; i < 118 + 4; ++i)
                powToGains[i + 256] = (float) std::pow (2.0, -0.25 * (i + 210));

            for (i = 0; i < 8207; ++i)
                nToThe4Over3[i] = (float) std::pow ((double) i, 4.0 / 3.0);

            for (i = 0; i < 8; ++i)
            {
                static constexpr double Ci[] = { -0.6, -0.535, -0.33, -0.185, -0.095, -0.041, -0.0142, -0.0037 };
                const double sq = sqrt (1.0 + Ci[i] * Ci[i]);
                antiAliasingCs[i] = (float) (1.0 / sq);
                antiAliasingCa[i] = (float) (Ci[i] / sq);
            }

            for (i = 0; i < 18; ++i)
            {
                win[0][i]      = win[1][i]      = (float) (0.5 * std::sin (MathConstants<double>::pi / 72.0 * (2 * i + 1))        / std::cos (MathConstants<double>::pi * (2 * i + 19)        / 72.0));
                win[0][i + 18] = win[3][i + 18] = (float) (0.5 * std::sin (MathConstants<double>::pi / 72.0 * (2 * (i + 18) + 1)) / std::cos (MathConstants<double>::pi * (2 * (i + 18) + 19) / 72.0));
            }

            const double piOver72 = MathConstants<double>::pi / 72.0;

            for (i = 0; i < 6; ++i)
            {
                win[1][i + 18] = (float) (0.5 / std::cos (piOver72 * (2 * (i + 18) + 19)));
                win[3][i + 12] = (float) (0.5 / std::cos (piOver72 * (2 * (i + 12) + 19)));
                win[1][i + 24] = (float) (0.5 * std::sin (MathConstants<double>::pi / 24.0 * (2 * i + 13)) / std::cos (piOver72 * (2 * (i + 24) + 19)));
                win[1][i + 30] = win[3][i] = 0;
                win[3][i + 6]  = (float) (0.5 * std::sin (MathConstants<double>::pi / 24.0 * (2 * i + 1)) / std::cos (piOver72 * (2 * (i + 6) + 19)));
            }

            for (i = 0; i < 12; ++i)
                win[2][i] = (float) (0.5 * std::sin (MathConstants<double>::pi / 24.0 * (2 * i + 1)) / std::cos (MathConstants<double>::pi * (2 * i + 7) / 24.0));

            for (j = 0; j < 4; ++j)
            {
                static constexpr int len[4] = { 36, 36, 12, 36 };
                for (i = 0; i < len[j]; i += 2)   win1[j][i] =  win[j][i];
                for (i = 1; i < len[j]; i += 2)   win1[j][i] = -win[j][i];
            }

            const double sqrt2 = 1.41421356237309504880168872420969808;

            for (i = 0; i < 16; ++i)
            {
                const double t = std::tan (i * MathConstants<double>::pi / 12.0);
                tan1_1[i] = (float) (t / (1.0 + t));
                tan2_1[i] = (float) (1.0 / (1.0 + t));
                tan1_2[i] = (float) (sqrt2 * t / (1.0 + t));
                tan2_2[i] = (float) (sqrt2 / (1.0 + t));

                for (j = 0; j < 2; ++j)
                {
                    double p1 = 1.0, p2 = 1.0;

                    if (i > 0)
                    {
                        const double base = std::pow (2.0, -0.25 * (j + 1));

                        if (i & 1)
                            p1 = std::pow (base, (i + 1) * 0.5);
                        else
                            p2 = std::pow (base, i * 0.5);
                    }

                    pow1_1[j][i] = (float) p1;
                    pow2_1[j][i] = (float) p2;
                    pow1_2[j][i] = (float) (sqrt2 * p1);
                    pow2_2[j][i] = (float) (sqrt2 * p2);
                }
            }

            for (j = 0; j < 9; ++j)
            {
                const Mp3DecoderBandInfoStruct& bi = bandInfo[j];
                int cb;
                int* mp = map[j][0] = mapbuf0[j];
                const int16* bdf = bi.longDiff;

                for (i = 0, cb = 0; cb < 8; ++cb, i += *bdf++)
                {
                    *mp++ = (*bdf) >> 1;
                    *mp++ = i;
                    *mp++ = 3;
                    *mp++ = cb;
                }
                bdf = bi.shortDiff + 3;

                for (cb = 3; cb < 13; ++cb)
                {
                    const int l = (*bdf++) >> 1;

                    for (int lwin = 0; lwin < 3; ++lwin)
                    {
                        *mp++ = l;
                        *mp++ = i + lwin;
                        *mp++ = lwin;
                        *mp++ = cb;
                    }
                    i += 6 * l;
                }

                mapEnd[j][0] = mp;
                mp = map[j][1] = mapbuf1[j];
                bdf = bi.shortDiff;

                for (i = 0, cb = 0; cb < 13; ++cb)
                {
                    const int l = (*bdf++) >> 1;
                    for (int lwin = 0; lwin < 3; ++lwin)
                    {
                        *mp++ = l;
                        *mp++ = i + lwin;
                        *mp++ = lwin;
                        *mp++ = cb;
                    }
                    i += 6 * l;
                }
                mapEnd[j][1] = mp;

                mp = map[j][2] = mapbuf2[j];
                bdf = bi.longDiff;
                for (cb = 0; cb < 22; ++cb)
                {
                    *mp++ = (*bdf++) >> 1;
                    *mp++ = cb;
                }
                mapEnd[j][2] = mp;

            }

            for (j = 0; j < 9; ++j)
            {
                for (i = 0; i < 23; ++i)    longLimit[j][i]  = jmin (32, (bandInfo[j].longIndex[i] - 1 + 8) / 18 + 1);
                for (i = 0; i < 14; ++i)    shortLimit[j][i] = jmin (32, (bandInfo[j].shortIndex[i] - 1) / 18 + 1);
            }

            for (i = 0; i < 5; ++i)
                for (j = 0; j < 6; ++j)
                    for (int k = 0; k < 6; ++k)
                    {
                        const int n = k + j * 6 + i * 36;
                        iLength2[n] = (unsigned int) (i | (j << 3) | (k << 6) | (3 << 12));
                    }

            for (i = 0; i < 4; ++i)
                for (j = 0; j < 4; ++j)
                    for (int k = 0; k < 4; ++k)
                    {
                        const int n = k + j * 4 + i * 16;
                        iLength2[n + 180] = (unsigned int) (i | (j << 3) | (k << 6) | (4 << 12));
                    }

            for (i = 0; i < 4; ++i)
                for (j = 0; j < 3; ++j)
                {
                    const int n = j + i * 3;
                    iLength2[n + 244] = (unsigned int) (i | (j << 3) | (5 << 12));
                    nLength2[n + 500] = (unsigned int) (i | (j << 3) | (2 << 12) | (1 << 15));
                }

            for (i = 0; i < 5; ++i)
                for (j = 0; j < 5; ++j)
                    for (int k = 0; k < 4; ++k)
                        for (int l = 0; l < 4; ++l)
                        {
                            const int n = l + k * 4 + j * 16 + i * 80;
                            nLength2[n] = (unsigned int) (i | (j << 3) | (k << 6) | (l << 9) | (0 << 12));
                        }

            for (i = 0; i < 5; ++i)
                for (j = 0; j < 5; ++j)
                    for (int k = 0; k < 4; ++k)
                    {
                        const int n = k + j * 4 + i * 20;
                        nLength2[n + 400] = (unsigned int) (i | (j << 3) | (k << 6) | (1 << 12));
                    }
        */
    }
}

lazy_static!{
    /*
    static const Mp3DecoderConstants constants;
    */
}
