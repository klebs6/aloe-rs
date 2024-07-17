#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

pub const MP3_DECODER_DCT_SUB_BAND_LIMIT: usize = 32;

pub const MP3_DECODER_DCT_COS6_1: f32   = 0.866025388;
pub const MP3_DECODER_DCT_COS6_2: f32   = 0.5;
pub const MP3_DECODER_DCT_COS9:  &[f32] = &[ 1.0, 0.98480773, 0.939692616, 0.866025388, 0.766044438, 0.642787635, 0.5, 0.342020154, 0.173648179 ];
pub const MP3_DECODER_DCT_COS36: &[f32] = &[ 0.501909912, 0.517638087, 0.551688969, 0.610387266, 0.707106769, 0.871723413, 1.18310082, 1.93185163, 5.73685646 ];
pub const MP3_DECODER_DCT_COS12: &[f32] = &[ 0.517638087, 0.707106769, 1.93185163 ];

#[inline] pub fn mp3_decoder_dct36_0(
        v:      i32,
        ts:     *mut f32,
        out1:   *mut f32,
        out2:   *mut f32,
        wintab: *const f32,
        sum0:   f32,
        sum1:   f32)  {
    
    todo!();
        /*
            auto tmp = sum0 + sum1;
            out2[9 + v] = tmp * wintab[27 + v];
            out2[8 - v] = tmp * wintab[26 - v];
            sum0 -= sum1;
            ts[subBandLimit * (8 - v)] = out1[8 - v] + sum0 * wintab[8 - v];
            ts[subBandLimit * (9 + v)] = out1[9 + v] + sum0 * wintab[9 + v];
        */
}

#[inline] pub fn mp3_decoder_dct36_12(
        v1:     i32,
        v2:     i32,
        ts:     *mut f32,
        out1:   *mut f32,
        out2:   *mut f32,
        wintab: *const f32,
        tmp1a:  f32,
        tmp1b:  f32,
        tmp2a:  f32,
        tmp2b:  f32)  {
    
    todo!();
        /*
            dct36_0 (v1, ts, out1, out2, wintab, tmp1a + tmp2a, (tmp1b + tmp2b) * cos36[v1]);
            dct36_0 (v2, ts, out1, out2, wintab, tmp2a - tmp1a, (tmp2b - tmp1b) * cos36[v2]);
        */
}

pub fn mp3_decoder_dct36(
        in_:    *mut f32,
        out1:   *mut f32,
        out2:   *mut f32,
        wintab: *const f32,
        ts:     *mut f32)  {
    
    todo!();
        /*
            in[17] += in[16]; in[16] += in[15]; in[15] += in[14]; in[14] += in[13]; in[13] += in[12];
            in[12] += in[11]; in[11] += in[10]; in[10] += in[9];  in[9]  += in[8];  in[8]  += in[7];
            in[7]  += in[6];  in[6]  += in[5];  in[5]  += in[4];  in[4]  += in[3];  in[3]  += in[2];
            in[2]  += in[1];  in[1]  += in[0];  in[17] += in[15]; in[15] += in[13]; in[13] += in[11];
            in[11] += in[9];  in[9]  += in[7];  in[7]  += in[5];  in[5]  += in[3];  in[3]  += in[1];

            auto ta33 = in[6]  * cos9[3];
            auto ta66 = in[12] * cos9[6];
            auto tb33 = in[7]  * cos9[3];
            auto tb66 = in[13] * cos9[6];

            dct36_12 (0, 8, ts, out1, out2, wintab,
                      in[2] * cos9[1] + ta33 + in[10] * cos9[5] + in[14] * cos9[7],
                      in[3] * cos9[1] + tb33 + in[11] * cos9[5] + in[15] * cos9[7],
                      in[0] + in[4] * cos9[2] + in[8] * cos9[4] + ta66 + in[16] * cos9[8],
                      in[1] + in[5] * cos9[2] + in[9] * cos9[4] + tb66 + in[17] * cos9[8]);

            dct36_12 (1, 7, ts, out1, out2, wintab,
                      (in[2] - in[10] - in[14]) * cos9[3],
                      (in[3] - in[11] - in[15]) * cos9[3],
                      (in[4] - in[8] - in[16]) * cos9[6] - in[12] + in[0],
                      (in[5] - in[9] - in[17]) * cos9[6] - in[13] + in[1]);

            dct36_12 (2, 6, ts, out1, out2, wintab,
                      in[2] * cos9[5] - ta33 - in[10] * cos9[7] + in[14] * cos9[1],
                      in[3] * cos9[5] - tb33 - in[11] * cos9[7] + in[15] * cos9[1],
                      in[0] - in[4] * cos9[8] - in[8] * cos9[2] + ta66 + in[16] * cos9[4],
                      in[1] - in[5] * cos9[8] - in[9] * cos9[2] + tb66 + in[17] * cos9[4]);

            dct36_12 (3, 5, ts, out1, out2, wintab,
                      in[2] * cos9[7] - ta33 + in[10] * cos9[1] - in[14] * cos9[5],
                      in[3] * cos9[7] - tb33 + in[11] * cos9[1] - in[15] * cos9[5],
                      in[0] - in[4] * cos9[4] + in[8] * cos9[8] + ta66 - in[16] * cos9[2],
                      in[1] - in[5] * cos9[4] + in[9] * cos9[8] + tb66 - in[17] * cos9[2]);

            dct36_0 (4, ts, out1, out2, wintab,
                     in[0] - in[4] + in[8] - in[12] + in[16],
                     (in[1] - in[5] + in[9] - in[13] + in[17]) * cos36[4]);
        */
}

///---------------------
pub struct Mp3DecoderDCT12Inputs {
    in0: f32,
    in1: f32,
    in2: f32,
    in3: f32,
    in4: f32,
    in5: f32,
}

impl Mp3DecoderDCT12Inputs {

    pub fn new(in_: *const f32) -> Self {
    
        todo!();
        /*


            in5 = in[5 * 3] + (in4 = in[4 * 3]);
                in4 += (in3 = in[3 * 3]);
                in3 += (in2 = in[2 * 3]);
                in2 += (in1 = in[1 * 3]);
                in1 += (in0 = in[0 * 3]);
                in5 += in3; in3 += in1;
                in2 *= cos6_1;
                in3 *= cos6_1;
        */
    }
    
    #[inline] pub fn process(&mut self)  {
        
        todo!();
        /*
            in0 += in4 * cos6_2;
                in4 = in0 + in2; in0 -= in2;
                in1 += in5 * cos6_2;
                in5 = (in1 + in3) * cos12[0];
                in1 = (in1 - in3) * cos12[2];
                in3 = in4 + in5; in4 -= in5;
                in2 = in0 + in1; in0 -= in1;
        */
    }
}

///---------------------
pub fn mp3_decoder_dct12(
        in_:  *const f32,
        out1: *mut f32,
        out2: *mut f32,
        wi:   *const f32,
        ts:   *mut f32)  {
    
    todo!();
        /*
            {
                ts[0] = out1[0];
                ts[subBandLimit * 1] = out1[1];
                ts[subBandLimit * 2] = out1[2];
                ts[subBandLimit * 3] = out1[3];
                ts[subBandLimit * 4] = out1[4];
                ts[subBandLimit * 5] = out1[5];

                Mp3DecoderDCT12Inputs inputs (in);

                {
                    auto tmp1 = (inputs.in0 - inputs.in4);
                    auto tmp2 = (inputs.in1 - inputs.in5) * cos12[1];
                    auto tmp0 = tmp1 + tmp2;
                    tmp1 -= tmp2;

                    ts[16 * subBandLimit] = out1[16] + tmp0 * wi[10];
                    ts[13 * subBandLimit] = out1[13] + tmp0 * wi[7];
                    ts[7  * subBandLimit] = out1[7]  + tmp1 * wi[1];
                    ts[10 * subBandLimit] = out1[10] + tmp1 * wi[4];
                }

                inputs.process();

                ts[17 * subBandLimit] = out1[17] + inputs.in2 * wi[11];
                ts[12 * subBandLimit] = out1[12] + inputs.in2 * wi[6];
                ts[14 * subBandLimit] = out1[14] + inputs.in3 * wi[8];
                ts[15 * subBandLimit] = out1[15] + inputs.in3 * wi[9];

                ts[6  * subBandLimit] = out1[6]  + inputs.in0 * wi[0];
                ts[11 * subBandLimit] = out1[11] + inputs.in0 * wi[5];
                ts[8  * subBandLimit] = out1[8]  + inputs.in4 * wi[2];
                ts[9  * subBandLimit] = out1[9]  + inputs.in4 * wi[3];
            }

            {
                Mp3DecoderDCT12Inputs inputs (++in);
                auto tmp1 = (inputs.in0 - inputs.in4);
                auto tmp2 = (inputs.in1 - inputs.in5) * cos12[1];
                auto tmp0 = tmp1 + tmp2;
                tmp1 -= tmp2;
                out2[4] = tmp0 * wi[10];
                out2[1] = tmp0 * wi[7];
                ts[13 * subBandLimit] += tmp1 * wi[1];
                ts[16 * subBandLimit] += tmp1 * wi[4];

                inputs.process();

                out2[5] = inputs.in2 * wi[11];
                out2[0] = inputs.in2 * wi[6];
                out2[2] = inputs.in3 * wi[8];
                out2[3] = inputs.in3 * wi[9];
                ts[12 * subBandLimit] += inputs.in0 * wi[0];
                ts[17 * subBandLimit] += inputs.in0 * wi[5];
                ts[14 * subBandLimit] += inputs.in4 * wi[2];
                ts[15 * subBandLimit] += inputs.in4 * wi[5 - 2];
            }

            {
                Mp3DecoderDCT12Inputs inputs (++in);
                out2[12] = out2[13] = out2[14] = out2[15] = out2[16] = out2[17] = 0;

                auto tmp1 = (inputs.in0 - inputs.in4);
                auto tmp2 = (inputs.in1 - inputs.in5) * cos12[1];
                auto tmp0 = tmp1 + tmp2;
                tmp1 -= tmp2;

                out2[10] = tmp0 * wi[10];
                out2[7]  = tmp0 * wi[7];
                out2[1] += tmp1 * wi[1];
                out2[4] += tmp1 * wi[4];

                inputs.process();

                out2[11] = inputs.in2 * wi[11];
                out2[6]  = inputs.in2 * wi[6];
                out2[8]  = inputs.in3 * wi[8];
                out2[9]  = inputs.in3 * wi[9];
                out2[0] += inputs.in0 * wi[0];
                out2[5] += inputs.in0 * wi[5];
                out2[2] += inputs.in4 * wi[2];
                out2[3] += inputs.in4 * wi[3];
            }
        */
}

pub fn mp3_decoder_dct64(
        out0:    *mut f32,
        out1:    *mut f32,
        samples: *const f32)  {
    
    todo!();
        /*
            float b1[32], b2[32];

            {
                auto* costab = constants.cosTables[0];
                b1[0x00] = samples[0x00] + samples[0x1F];   b1[0x1F] = (samples[0x00] - samples[0x1F]) * costab[0x0];
                b1[0x01] = samples[0x01] + samples[0x1E];   b1[0x1E] = (samples[0x01] - samples[0x1E]) * costab[0x1];
                b1[0x02] = samples[0x02] + samples[0x1D];   b1[0x1D] = (samples[0x02] - samples[0x1D]) * costab[0x2];
                b1[0x03] = samples[0x03] + samples[0x1C];   b1[0x1C] = (samples[0x03] - samples[0x1C]) * costab[0x3];
                b1[0x04] = samples[0x04] + samples[0x1B];   b1[0x1B] = (samples[0x04] - samples[0x1B]) * costab[0x4];
                b1[0x05] = samples[0x05] + samples[0x1A];   b1[0x1A] = (samples[0x05] - samples[0x1A]) * costab[0x5];
                b1[0x06] = samples[0x06] + samples[0x19];   b1[0x19] = (samples[0x06] - samples[0x19]) * costab[0x6];
                b1[0x07] = samples[0x07] + samples[0x18];   b1[0x18] = (samples[0x07] - samples[0x18]) * costab[0x7];
                b1[0x08] = samples[0x08] + samples[0x17];   b1[0x17] = (samples[0x08] - samples[0x17]) * costab[0x8];
                b1[0x09] = samples[0x09] + samples[0x16];   b1[0x16] = (samples[0x09] - samples[0x16]) * costab[0x9];
                b1[0x0A] = samples[0x0A] + samples[0x15];   b1[0x15] = (samples[0x0A] - samples[0x15]) * costab[0xA];
                b1[0x0B] = samples[0x0B] + samples[0x14];   b1[0x14] = (samples[0x0B] - samples[0x14]) * costab[0xB];
                b1[0x0C] = samples[0x0C] + samples[0x13];   b1[0x13] = (samples[0x0C] - samples[0x13]) * costab[0xC];
                b1[0x0D] = samples[0x0D] + samples[0x12];   b1[0x12] = (samples[0x0D] - samples[0x12]) * costab[0xD];
                b1[0x0E] = samples[0x0E] + samples[0x11];   b1[0x11] = (samples[0x0E] - samples[0x11]) * costab[0xE];
                b1[0x0F] = samples[0x0F] + samples[0x10];   b1[0x10] = (samples[0x0F] - samples[0x10]) * costab[0xF];
            }

            {
                auto* costab = constants.cosTables[1];
                b2[0x00] = b1[0x00] + b1[0x0F];   b2[0x0F] = (b1[0x00] - b1[0x0F]) * costab[0];
                b2[0x01] = b1[0x01] + b1[0x0E];   b2[0x0E] = (b1[0x01] - b1[0x0E]) * costab[1];
                b2[0x02] = b1[0x02] + b1[0x0D];   b2[0x0D] = (b1[0x02] - b1[0x0D]) * costab[2];
                b2[0x03] = b1[0x03] + b1[0x0C];   b2[0x0C] = (b1[0x03] - b1[0x0C]) * costab[3];
                b2[0x04] = b1[0x04] + b1[0x0B];   b2[0x0B] = (b1[0x04] - b1[0x0B]) * costab[4];
                b2[0x05] = b1[0x05] + b1[0x0A];   b2[0x0A] = (b1[0x05] - b1[0x0A]) * costab[5];
                b2[0x06] = b1[0x06] + b1[0x09];   b2[0x09] = (b1[0x06] - b1[0x09]) * costab[6];
                b2[0x07] = b1[0x07] + b1[0x08];   b2[0x08] = (b1[0x07] - b1[0x08]) * costab[7];
                b2[0x10] = b1[0x10] + b1[0x1F];   b2[0x1F] = (b1[0x1F] - b1[0x10]) * costab[0];
                b2[0x11] = b1[0x11] + b1[0x1E];   b2[0x1E] = (b1[0x1E] - b1[0x11]) * costab[1];
                b2[0x12] = b1[0x12] + b1[0x1D];   b2[0x1D] = (b1[0x1D] - b1[0x12]) * costab[2];
                b2[0x13] = b1[0x13] + b1[0x1C];   b2[0x1C] = (b1[0x1C] - b1[0x13]) * costab[3];
                b2[0x14] = b1[0x14] + b1[0x1B];   b2[0x1B] = (b1[0x1B] - b1[0x14]) * costab[4];
                b2[0x15] = b1[0x15] + b1[0x1A];   b2[0x1A] = (b1[0x1A] - b1[0x15]) * costab[5];
                b2[0x16] = b1[0x16] + b1[0x19];   b2[0x19] = (b1[0x19] - b1[0x16]) * costab[6];
                b2[0x17] = b1[0x17] + b1[0x18];   b2[0x18] = (b1[0x18] - b1[0x17]) * costab[7];
            }

            {
                auto* costab = constants.cosTables[2];
                b1[0x00] = b2[0x00] + b2[0x07];   b1[0x07] = (b2[0x00] - b2[0x07]) * costab[0];
                b1[0x01] = b2[0x01] + b2[0x06];   b1[0x06] = (b2[0x01] - b2[0x06]) * costab[1];
                b1[0x02] = b2[0x02] + b2[0x05];   b1[0x05] = (b2[0x02] - b2[0x05]) * costab[2];
                b1[0x03] = b2[0x03] + b2[0x04];   b1[0x04] = (b2[0x03] - b2[0x04]) * costab[3];
                b1[0x08] = b2[0x08] + b2[0x0F];   b1[0x0F] = (b2[0x0F] - b2[0x08]) * costab[0];
                b1[0x09] = b2[0x09] + b2[0x0E];   b1[0x0E] = (b2[0x0E] - b2[0x09]) * costab[1];
                b1[0x0A] = b2[0x0A] + b2[0x0D];   b1[0x0D] = (b2[0x0D] - b2[0x0A]) * costab[2];
                b1[0x0B] = b2[0x0B] + b2[0x0C];   b1[0x0C] = (b2[0x0C] - b2[0x0B]) * costab[3];
                b1[0x10] = b2[0x10] + b2[0x17];   b1[0x17] = (b2[0x10] - b2[0x17]) * costab[0];
                b1[0x11] = b2[0x11] + b2[0x16];   b1[0x16] = (b2[0x11] - b2[0x16]) * costab[1];
                b1[0x12] = b2[0x12] + b2[0x15];   b1[0x15] = (b2[0x12] - b2[0x15]) * costab[2];
                b1[0x13] = b2[0x13] + b2[0x14];   b1[0x14] = (b2[0x13] - b2[0x14]) * costab[3];
                b1[0x18] = b2[0x18] + b2[0x1F];   b1[0x1F] = (b2[0x1F] - b2[0x18]) * costab[0];
                b1[0x19] = b2[0x19] + b2[0x1E];   b1[0x1E] = (b2[0x1E] - b2[0x19]) * costab[1];
                b1[0x1A] = b2[0x1A] + b2[0x1D];   b1[0x1D] = (b2[0x1D] - b2[0x1A]) * costab[2];
                b1[0x1B] = b2[0x1B] + b2[0x1C];   b1[0x1C] = (b2[0x1C] - b2[0x1B]) * costab[3];
            }

            {
                auto cos0 = constants.cosTables[3][0];
                auto cos1 = constants.cosTables[3][1];
                b2[0x00] = b1[0x00] + b1[0x03];   b2[0x03] = (b1[0x00] - b1[0x03]) * cos0;
                b2[0x01] = b1[0x01] + b1[0x02];   b2[0x02] = (b1[0x01] - b1[0x02]) * cos1;
                b2[0x04] = b1[0x04] + b1[0x07];   b2[0x07] = (b1[0x07] - b1[0x04]) * cos0;
                b2[0x05] = b1[0x05] + b1[0x06];   b2[0x06] = (b1[0x06] - b1[0x05]) * cos1;
                b2[0x08] = b1[0x08] + b1[0x0B];   b2[0x0B] = (b1[0x08] - b1[0x0B]) * cos0;
                b2[0x09] = b1[0x09] + b1[0x0A];   b2[0x0A] = (b1[0x09] - b1[0x0A]) * cos1;
                b2[0x0C] = b1[0x0C] + b1[0x0F];   b2[0x0F] = (b1[0x0F] - b1[0x0C]) * cos0;
                b2[0x0D] = b1[0x0D] + b1[0x0E];   b2[0x0E] = (b1[0x0E] - b1[0x0D]) * cos1;
                b2[0x10] = b1[0x10] + b1[0x13];   b2[0x13] = (b1[0x10] - b1[0x13]) * cos0;
                b2[0x11] = b1[0x11] + b1[0x12];   b2[0x12] = (b1[0x11] - b1[0x12]) * cos1;
                b2[0x14] = b1[0x14] + b1[0x17];   b2[0x17] = (b1[0x17] - b1[0x14]) * cos0;
                b2[0x15] = b1[0x15] + b1[0x16];   b2[0x16] = (b1[0x16] - b1[0x15]) * cos1;
                b2[0x18] = b1[0x18] + b1[0x1B];   b2[0x1B] = (b1[0x18] - b1[0x1B]) * cos0;
                b2[0x19] = b1[0x19] + b1[0x1A];   b2[0x1A] = (b1[0x19] - b1[0x1A]) * cos1;
                b2[0x1C] = b1[0x1C] + b1[0x1F];   b2[0x1F] = (b1[0x1F] - b1[0x1C]) * cos0;
                b2[0x1D] = b1[0x1D] + b1[0x1E];   b2[0x1E] = (b1[0x1E] - b1[0x1D]) * cos1;
            }

            {
                auto cos0 = constants.cosTables[4][0];
                b1[0x00] = b2[0x00] + b2[0x01];   b1[0x01] = (b2[0x00] - b2[0x01]) * cos0;
                b1[0x02] = b2[0x02] + b2[0x03];   b1[0x03] = (b2[0x03] - b2[0x02]) * cos0;  b1[0x02] += b1[0x03];
                b1[0x04] = b2[0x04] + b2[0x05];   b1[0x05] = (b2[0x04] - b2[0x05]) * cos0;
                b1[0x06] = b2[0x06] + b2[0x07];   b1[0x07] = (b2[0x07] - b2[0x06]) * cos0;
                b1[0x06] += b1[0x07];   b1[0x04] += b1[0x06];  b1[0x06] += b1[0x05]; b1[0x05] += b1[0x07];
                b1[0x08] = b2[0x08] + b2[0x09];   b1[0x09] = (b2[0x08] - b2[0x09]) * cos0;
                b1[0x0A] = b2[0x0A] + b2[0x0B];   b1[0x0B] = (b2[0x0B] - b2[0x0A]) * cos0;  b1[0x0A] += b1[0x0B];
                b1[0x0C] = b2[0x0C] + b2[0x0D];   b1[0x0D] = (b2[0x0C] - b2[0x0D]) * cos0;
                b1[0x0E] = b2[0x0E] + b2[0x0F];   b1[0x0F] = (b2[0x0F] - b2[0x0E]) * cos0;
                b1[0x0E] += b1[0x0F];   b1[0x0C] += b1[0x0E];  b1[0x0E] += b1[0x0D]; b1[0x0D] += b1[0x0F];
                b1[0x10] = b2[0x10] + b2[0x11];   b1[0x11] = (b2[0x10] - b2[0x11]) * cos0;
                b1[0x12] = b2[0x12] + b2[0x13];   b1[0x13] = (b2[0x13] - b2[0x12]) * cos0;  b1[0x12] += b1[0x13];
                b1[0x14] = b2[0x14] + b2[0x15];   b1[0x15] = (b2[0x14] - b2[0x15]) * cos0;
                b1[0x16] = b2[0x16] + b2[0x17];   b1[0x17] = (b2[0x17] - b2[0x16]) * cos0;
                b1[0x16] += b1[0x17];    b1[0x14] += b1[0x16]; b1[0x16] += b1[0x15];  b1[0x15] += b1[0x17];
                b1[0x18] = b2[0x18] + b2[0x19];   b1[0x19] = (b2[0x18] - b2[0x19]) * cos0;
                b1[0x1A] = b2[0x1A] + b2[0x1B];   b1[0x1B] = (b2[0x1B] - b2[0x1A]) * cos0;  b1[0x1A] += b1[0x1B];
                b1[0x1C] = b2[0x1C] + b2[0x1D];   b1[0x1D] = (b2[0x1C] - b2[0x1D]) * cos0;
                b1[0x1E] = b2[0x1E] + b2[0x1F];   b1[0x1F] = (b2[0x1F] - b2[0x1E]) * cos0;
                b1[0x1E] += b1[0x1F];    b1[0x1C] += b1[0x1E]; b1[0x1E] += b1[0x1D];  b1[0x1D] += b1[0x1F];
            }

            out0[0x10 * 16] = b1[0x00];  out0[0x10 * 12] = b1[0x04]; out0[0x10 * 8]  = b1[0x02];  out0[0x10 * 4]  = b1[0x06];
            out0[0] = b1[0x01];  out1[0]  = b1[0x01]; out1[0x10 * 4]  = b1[0x05];  out1[0x10 * 8]  = b1[0x03];
            out1[0x10 * 12] = b1[0x07];

            b1[0x08] += b1[0x0C];  out0[0x10 * 14] = b1[0x08];  b1[0x0C] += b1[0x0a];  out0[0x10 * 10] = b1[0x0C];
            b1[0x0A] += b1[0x0E];  out0[0x10 * 6]  = b1[0x0A];  b1[0x0E] += b1[0x09];  out0[0x10 * 2]  = b1[0x0E];
            b1[0x09] += b1[0x0D];  out1[0x10 * 2]  = b1[0x09];  b1[0x0D] += b1[0x0B];  out1[0x10 * 6]  = b1[0x0D];
            b1[0x0B] += b1[0x0F];  out1[0x10 * 10] = b1[0x0B];  out1[0x10 * 14] = b1[0x0F];

            b1[0x18] += b1[0x1C];  out0[0x10 * 15] = b1[0x10] + b1[0x18];   out0[0x10 * 13] = b1[0x18] + b1[0x14];
            b1[0x1C] += b1[0x1a];  out0[0x10 * 11] = b1[0x14] + b1[0x1C];   out0[0x10 * 9]  = b1[0x1C] + b1[0x12];
            b1[0x1A] += b1[0x1E];  out0[0x10 * 7]  = b1[0x12] + b1[0x1A];   out0[0x10 * 5]  = b1[0x1A] + b1[0x16];
            b1[0x1E] += b1[0x19];  out0[0x10 * 3]  = b1[0x16] + b1[0x1E];   out0[0x10 * 1]  = b1[0x1E] + b1[0x11];
            b1[0x19] += b1[0x1D];  out1[0x10 * 1]  = b1[0x11] + b1[0x19];   out1[0x10 * 3]  = b1[0x19] + b1[0x15];
            b1[0x1D] += b1[0x1B];  out1[0x10 * 5]  = b1[0x15] + b1[0x1D];   out1[0x10 * 7]  = b1[0x1D] + b1[0x13];
            b1[0x1B] += b1[0x1F];  out1[0x10 * 9]  = b1[0x13] + b1[0x1B];   out1[0x10 * 11] = b1[0x1B] + b1[0x17];
            out1[0x10 * 13] = b1[0x17] + b1[0x1F];  out1[0x10 * 15] = b1[0x1F];
        */
}
