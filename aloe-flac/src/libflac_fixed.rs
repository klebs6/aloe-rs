crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/fixed.c]

macro_rules! local_abs {
    ($x:ident) => {
        /*
                ((unsigned)((x)<0? -(x) : (x)))
        */
    }
}

/** 
 | rbps stands for residual bits per sample
 |
 |             (ln(2) * err)
 | rbps = log  (-----------)
 |           2 (     n     )
 */
#[cfg(INTEGER_ONLY_LIBRARY)]
pub fn local_compute_rbps_integerized(err: u32, n: u32) -> fixedpoint {
    
    todo!();
        /*
            u32 rbps;
        unsigned bits; /* the number of bits required to represent a number */
        int fracbits; /* the number of bits of rbps that comprise the fractional part */

        ASSERT(sizeof(rbps) == sizeof(fixedpoint));
        ASSERT(err > 0);
        ASSERT(n > 0);

        ASSERT(n <= MAX_BLOCK_SIZE);
        if(err <= n)
            return 0;
        /*
         * The above two things tell us 1) n fits in 16 bits; 2) err/n > 1.
         * These allow us later to know we won't lose too much precision in the
         * fixed-point division (err<<fracbits)/n.
         */

        fracbits = (8*sizeof(err)) - (bitmath_ilog2(err)+1);

        err <<= fracbits;
        err /= n;
        /* err now holds err/n with fracbits fractional bits */

        /*
         * Whittle err down to 16 bits max.  16 significant bits is enough for
         * our purposes.
         */
        ASSERT(err > 0);
        bits = bitmath_ilog2(err)+1;
        if(bits > 16) {
            err >>= (bits-16);
            fracbits -= (bits-16);
        }
        rbps = (u32)err;

        /* Multiply by fixed-point version of ln(2), with 16 fractional bits */
        rbps *= FP_LN2;
        fracbits += 16;
        ASSERT(fracbits >= 0);

        /* fixedpoint_log2 requires fracbits%4 to be 0 */
        {
            const int f = fracbits & 3;
            if(f) {
                rbps >>= f;
                fracbits -= f;
            }
        }

        rbps = fixedpoint_log2(rbps, fracbits, (unsigned)(-1));

        if(rbps == 0)
            return 0;

        /*
         * The return value must have 16 fractional bits.  Since the whole part
         * of the base-2 log of a 32 bit number must fit in 5 bits, and fracbits
         * must be >= -3, these assertion allows us to be able to shift rbps
         * left if necessary to get 16 fracbits without losing any bits of the
         * whole part of rbps.
         *
         * There is a slight chance due to accumulated error that the whole part
         * will require 6 bits, so we use 6 in the assertion.  Really though as
         * long as it fits in 13 bits (32 - (16 - (-3))) we are fine.
         */
        ASSERT((int)bitmath_ilog2(rbps)+1 <= fracbits + 6);
        ASSERT(fracbits >= -3);

        /* now shift the decimal point into place */
        if(fracbits < 16)
            return rbps << (16-fracbits);
        else if(fracbits > 16)
            return rbps >> (fracbits-16);
        else
            return rbps;
        */
}

#[cfg(INTEGER_ONLY_LIBRARY)]
pub fn local_compute_rbps_wide_integerized(err: u64, n: u32) -> fixedpoint {
    
    todo!();
        /*
            u32 rbps;
        unsigned bits; /* the number of bits required to represent a number */
        int fracbits; /* the number of bits of rbps that comprise the fractional part */

        ASSERT(sizeof(rbps) == sizeof(fixedpoint));
        ASSERT(err > 0);
        ASSERT(n > 0);

        ASSERT(n <= MAX_BLOCK_SIZE);
        if(err <= n)
            return 0;
        /*
         * The above two things tell us 1) n fits in 16 bits; 2) err/n > 1.
         * These allow us later to know we won't lose too much precision in the
         * fixed-point division (err<<fracbits)/n.
         */

        fracbits = (8*sizeof(err)) - (bitmath_ilog2_wide(err)+1);

        err <<= fracbits;
        err /= n;
        /* err now holds err/n with fracbits fractional bits */

        /*
         * Whittle err down to 16 bits max.  16 significant bits is enough for
         * our purposes.
         */
        ASSERT(err > 0);
        bits = bitmath_ilog2_wide(err)+1;
        if(bits > 16) {
            err >>= (bits-16);
            fracbits -= (bits-16);
        }
        rbps = (u32)err;

        /* Multiply by fixed-point version of ln(2), with 16 fractional bits */
        rbps *= FP_LN2;
        fracbits += 16;
        ASSERT(fracbits >= 0);

        /* fixedpoint_log2 requires fracbits%4 to be 0 */
        {
            const int f = fracbits & 3;
            if(f) {
                rbps >>= f;
                fracbits -= f;
            }
        }

        rbps = fixedpoint_log2(rbps, fracbits, (unsigned)(-1));

        if(rbps == 0)
            return 0;

        /*
         * The return value must have 16 fractional bits.  Since the whole part
         * of the base-2 log of a 32 bit number must fit in 5 bits, and fracbits
         * must be >= -3, these assertion allows us to be able to shift rbps
         * left if necessary to get 16 fracbits without losing any bits of the
         * whole part of rbps.
         *
         * There is a slight chance due to accumulated error that the whole part
         * will require 6 bits, so we use 6 in the assertion.  Really though as
         * long as it fits in 13 bits (32 - (16 - (-3))) we are fine.
         */
        ASSERT((int)bitmath_ilog2(rbps)+1 <= fracbits + 6);
        ASSERT(fracbits >= -3);

        /* now shift the decimal point into place */
        if(fracbits < 16)
            return rbps << (16-fracbits);
        else if(fracbits > 16)
            return rbps >> (fracbits-16);
        else
            return rbps;
        */
}

pub fn flac_fixed_compute_best_predictor(
        data:                     &[i32],
        data_len:                 u32,

        #[cfg(INTEGER_ONLY_LIBRARY)]
        residual_bits_per_sample: [fixedpoint; MAX_FIXED_ORDER+1],

        #[cfg(not(INTEGER_ONLY_LIBRARY))]
        residual_bits_per_sample: [float; MAX_FIXED_ORDER+1]
        ) -> u32 {
    
    todo!();
        /*
            i32 last_error_0 = data[-1];
        i32 last_error_1 = data[-1] - data[-2];
        i32 last_error_2 = last_error_1 - (data[-2] - data[-3]);
        i32 last_error_3 = last_error_2 - (data[-2] - 2*data[-3] + data[-4]);
        i32 error, save;
        u32 total_error_0 = 0, total_error_1 = 0, total_error_2 = 0, total_error_3 = 0, total_error_4 = 0;
        unsigned i, order;

        for(i = 0; i < data_len; i++) {
            error  = data[i]     ; total_error_0 += local_abs(error);                      save = error;
            error -= last_error_0; total_error_1 += local_abs(error); last_error_0 = save; save = error;
            error -= last_error_1; total_error_2 += local_abs(error); last_error_1 = save; save = error;
            error -= last_error_2; total_error_3 += local_abs(error); last_error_2 = save; save = error;
            error -= last_error_3; total_error_4 += local_abs(error); last_error_3 = save;
        }

        if(total_error_0 < flac_min(flac_min(flac_min(total_error_1, total_error_2), total_error_3), total_error_4))
            order = 0;
        else if(total_error_1 < flac_min(flac_min(total_error_2, total_error_3), total_error_4))
            order = 1;
        else if(total_error_2 < flac_min(total_error_3, total_error_4))
            order = 2;
        else if(total_error_3 < total_error_4)
            order = 3;
        else
            order = 4;

        /* Estimate the expected number of bits per residual signal sample. */
        /* 'total_error*' is linearly related to the variance of the residual */
        /* signal, so we use it directly to compute E(|x|) */
        ASSERT(data_len > 0 || total_error_0 == 0);
        ASSERT(data_len > 0 || total_error_1 == 0);
        ASSERT(data_len > 0 || total_error_2 == 0);
        ASSERT(data_len > 0 || total_error_3 == 0);
        ASSERT(data_len > 0 || total_error_4 == 0);
    #ifndef INTEGER_ONLY_LIBRARY
        residual_bits_per_sample[0] = (float)((total_error_0 > 0) ? log(M_LN2 * (double)total_error_0 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[1] = (float)((total_error_1 > 0) ? log(M_LN2 * (double)total_error_1 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[2] = (float)((total_error_2 > 0) ? log(M_LN2 * (double)total_error_2 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[3] = (float)((total_error_3 > 0) ? log(M_LN2 * (double)total_error_3 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[4] = (float)((total_error_4 > 0) ? log(M_LN2 * (double)total_error_4 / (double)data_len) / M_LN2 : 0.0);
    #else
        residual_bits_per_sample[0] = (total_error_0 > 0) ? local__compute_rbps_integerized(total_error_0, data_len) : 0;
        residual_bits_per_sample[1] = (total_error_1 > 0) ? local__compute_rbps_integerized(total_error_1, data_len) : 0;
        residual_bits_per_sample[2] = (total_error_2 > 0) ? local__compute_rbps_integerized(total_error_2, data_len) : 0;
        residual_bits_per_sample[3] = (total_error_3 > 0) ? local__compute_rbps_integerized(total_error_3, data_len) : 0;
        residual_bits_per_sample[4] = (total_error_4 > 0) ? local__compute_rbps_integerized(total_error_4, data_len) : 0;
    #endif

        return order;
        */
}

pub fn flac_fixed_compute_best_predictor_wide(
    data:                     &[i32],
    data_len:                 u32,

    #[cfg(not(INTEGER_ONLY_LIBRARY))]
    residual_bits_per_sample: [float; MAX_FIXED_ORDER+1],

    #[cfg(INTEGER_ONLY_LIBRARY)]
    residual_bits_per_sample: [fixedpoint; MAX_FIXED_ORDER+1]
) -> u32 {
    
    todo!();
        /*
            i32 last_error_0 = data[-1];
        i32 last_error_1 = data[-1] - data[-2];
        i32 last_error_2 = last_error_1 - (data[-2] - data[-3]);
        i32 last_error_3 = last_error_2 - (data[-2] - 2*data[-3] + data[-4]);
        i32 error, save;
        /* total_error_* are 64-bits to avoid overflow when encoding
         * erratic signals when the bits-per-sample and blocksize are
         * large.
         */
        u64 total_error_0 = 0, total_error_1 = 0, total_error_2 = 0, total_error_3 = 0, total_error_4 = 0;
        unsigned i, order;

        for(i = 0; i < data_len; i++) {
            error  = data[i]     ; total_error_0 += local_abs(error);                      save = error;
            error -= last_error_0; total_error_1 += local_abs(error); last_error_0 = save; save = error;
            error -= last_error_1; total_error_2 += local_abs(error); last_error_1 = save; save = error;
            error -= last_error_2; total_error_3 += local_abs(error); last_error_2 = save; save = error;
            error -= last_error_3; total_error_4 += local_abs(error); last_error_3 = save;
        }

        if(total_error_0 < flac_min(flac_min(flac_min(total_error_1, total_error_2), total_error_3), total_error_4))
            order = 0;
        else if(total_error_1 < flac_min(flac_min(total_error_2, total_error_3), total_error_4))
            order = 1;
        else if(total_error_2 < flac_min(total_error_3, total_error_4))
            order = 2;
        else if(total_error_3 < total_error_4)
            order = 3;
        else
            order = 4;

        /* Estimate the expected number of bits per residual signal sample. */
        /* 'total_error*' is linearly related to the variance of the residual */
        /* signal, so we use it directly to compute E(|x|) */
        ASSERT(data_len > 0 || total_error_0 == 0);
        ASSERT(data_len > 0 || total_error_1 == 0);
        ASSERT(data_len > 0 || total_error_2 == 0);
        ASSERT(data_len > 0 || total_error_3 == 0);
        ASSERT(data_len > 0 || total_error_4 == 0);
    #ifndef INTEGER_ONLY_LIBRARY
        residual_bits_per_sample[0] = (float)((total_error_0 > 0) ? log(M_LN2 * (double)total_error_0 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[1] = (float)((total_error_1 > 0) ? log(M_LN2 * (double)total_error_1 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[2] = (float)((total_error_2 > 0) ? log(M_LN2 * (double)total_error_2 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[3] = (float)((total_error_3 > 0) ? log(M_LN2 * (double)total_error_3 / (double)data_len) / M_LN2 : 0.0);
        residual_bits_per_sample[4] = (float)((total_error_4 > 0) ? log(M_LN2 * (double)total_error_4 / (double)data_len) / M_LN2 : 0.0);
    #else
        residual_bits_per_sample[0] = (total_error_0 > 0) ? local__compute_rbps_wide_integerized(total_error_0, data_len) : 0;
        residual_bits_per_sample[1] = (total_error_1 > 0) ? local__compute_rbps_wide_integerized(total_error_1, data_len) : 0;
        residual_bits_per_sample[2] = (total_error_2 > 0) ? local__compute_rbps_wide_integerized(total_error_2, data_len) : 0;
        residual_bits_per_sample[3] = (total_error_3 > 0) ? local__compute_rbps_wide_integerized(total_error_3, data_len) : 0;
        residual_bits_per_sample[4] = (total_error_4 > 0) ? local__compute_rbps_wide_integerized(total_error_4, data_len) : 0;
    #endif

        return order;
        */
}

pub fn flac_fixed_compute_residual(
        data:     &[i32],
        data_len: u32,
        order:    u32,
        residual: &[i32])  {
    
    todo!();
        /*
            const int idata_len = (int)data_len;
        int i;

        switch(order) {
            case 0:
                ASSERT(sizeof(residual[0]) == sizeof(data[0]));
                memcpy(residual, data, sizeof(residual[0])*data_len);
                break;
            case 1:
                for(i = 0; i < idata_len; i++)
                    residual[i] = data[i] - data[i-1];
                break;
            case 2:
                for(i = 0; i < idata_len; i++)
    #if 1 /* OPT: may be faster with some compilers on some systems */
                    residual[i] = data[i] - (data[i-1] << 1) + data[i-2];
    #else
                    residual[i] = data[i] - 2*data[i-1] + data[i-2];
    #endif
                break;
            case 3:
                for(i = 0; i < idata_len; i++)
    #if 1 /* OPT: may be faster with some compilers on some systems */
                    residual[i] = data[i] - (((data[i-1]-data[i-2])<<1) + (data[i-1]-data[i-2])) - data[i-3];
    #else
                    residual[i] = data[i] - 3*data[i-1] + 3*data[i-2] - data[i-3];
    #endif
                break;
            case 4:
                for(i = 0; i < idata_len; i++)
    #if 1 /* OPT: may be faster with some compilers on some systems */
                    residual[i] = data[i] - ((data[i-1]+data[i-3])<<2) + ((data[i-2]<<2) + (data[i-2]<<1)) + data[i-4];
    #else
                    residual[i] = data[i] - 4*data[i-1] + 6*data[i-2] - 4*data[i-3] + data[i-4];
    #endif
                break;
            default:
                ASSERT(0);
        }
        */
}

pub fn flac_fixed_restore_signal(
        residual: &[i32],
        data_len: u32,
        order:    u32,
        data:     &[i32])  {
    
    todo!();
        /*
            int i, idata_len = (int)data_len;

        switch(order) {
            case 0:
                ASSERT(sizeof(residual[0]) == sizeof(data[0]));
                memcpy(data, residual, sizeof(residual[0])*data_len);
                break;
            case 1:
                for(i = 0; i < idata_len; i++)
                    data[i] = residual[i] + data[i-1];
                break;
            case 2:
                for(i = 0; i < idata_len; i++)
    #if 1 /* OPT: may be faster with some compilers on some systems */
                    data[i] = residual[i] + (data[i-1]<<1) - data[i-2];
    #else
                    data[i] = residual[i] + 2*data[i-1] - data[i-2];
    #endif
                break;
            case 3:
                for(i = 0; i < idata_len; i++)
    #if 1 /* OPT: may be faster with some compilers on some systems */
                    data[i] = residual[i] + (((data[i-1]-data[i-2])<<1) + (data[i-1]-data[i-2])) + data[i-3];
    #else
                    data[i] = residual[i] + 3*data[i-1] - 3*data[i-2] + data[i-3];
    #endif
                break;
            case 4:
                for(i = 0; i < idata_len; i++)
    #if 1 /* OPT: may be faster with some compilers on some systems */
                    data[i] = residual[i] + ((data[i-1]+data[i-3])<<2) - ((data[i-2]<<2) + (data[i-2]<<1)) - data[i-4];
    #else
                    data[i] = residual[i] + 4*data[i-1] - 6*data[i-2] + 4*data[i-3] - data[i-4];
    #endif
                break;
            default:
                ASSERT(0);
        }
        */
}
