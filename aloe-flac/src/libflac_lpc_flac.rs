crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/lpc_flac.c]

/**
  | OPT: #undef'ing this may improve the
  | speed on some architectures
  |
  */
pub const LPC_UNROLLED_FILTER_LOOPS: bool = true;

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub use flac_integer_only_library::*;

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub mod flac_integer_only_library {
    use super::*;

    /**
      | If this fails, we are in the presence
      | of a mid 90's compiler, move along...
      |
      */
    #[cfg(not(HAVE_LROUND))]
    #[inline] pub fn lround(x: f64) -> i64 {
        
        todo!();
            /*
                return (long)(x + copysign (0.5, x));
            */
    }

    pub fn flac_lpc_window_data(
            in_:      &[i32],
            window:   &[real],
            out:      &[real],
            data_len: u32)  {
        
        todo!();
            /*
                unsigned i;
            for(i = 0; i < data_len; i++)
                out[i] = in[i] * window[i];
            */
    }

    pub fn flac_lpc_compute_autocorrelation(
            data:     &[real],
            data_len: u32,
            lag:      u32,
            autoc:    &[real])  {
        
        todo!();
            /*
                /* a readable, but slower, version */
        #if 0
            real d;
            unsigned i;

            ASSERT(lag > 0);
            ASSERT(lag <= data_len);

            /*
             * Technically we should subtract the mean first like so:
             *   for(i = 0; i < data_len; i++)
             *     data[i] -= mean;
             * but it appears not to make enough of a difference to matter, and
             * most signals are already closely centered around zero
             */
            while(lag--) {
                for(i = lag, d = 0.0; i < data_len; i++)
                    d += data[i] * data[i - lag];
                autoc[lag] = d;
            }
        #endif

            /*
             * this version tends to run faster because of better data locality
             * ('data_len' is usually much larger than 'lag')
             */
            real d;
            unsigned sample, coeff;
            const unsigned limit = data_len - lag;

            ASSERT(lag > 0);
            ASSERT(lag <= data_len);

            for(coeff = 0; coeff < lag; coeff++)
                autoc[coeff] = 0.0;
            for(sample = 0; sample <= limit; sample++) {
                d = data[sample];
                for(coeff = 0; coeff < lag; coeff++)
                    autoc[coeff] += d * data[sample+coeff];
            }
            for(; sample < data_len; sample++) {
                d = data[sample];
                for(coeff = 0; coeff < data_len - sample; coeff++)
                    autoc[coeff] += d * data[sample+coeff];
            }
            */
    }

    pub fn flac_lpc_compute_lp_coefficients(
            autoc:     &[real],
            max_order: *mut u32,
            lp_coeff:  [&[real]; MAX_LPC_ORDER],
            error:     &[f64])  {
        
        todo!();
            /*
                unsigned i, j;
            double r, err, lpc[MAX_LPC_ORDER];

            ASSERT(0 != max_order);
            ASSERT(0 < *max_order);
            ASSERT(*max_order <= MAX_LPC_ORDER);
            ASSERT(autoc[0] != 0.0);

            err = autoc[0];

            for(i = 0; i < *max_order; i++) {
                /* Sum up this iteration's reflection coefficient. */
                r = -autoc[i+1];
                for(j = 0; j < i; j++)
                    r -= lpc[j] * autoc[i-j];
                r /= err;

                /* Update LPC coefficients and total error. */
                lpc[i]=r;
                for(j = 0; j < (i>>1); j++) {
                    double tmp = lpc[j];
                    lpc[j] += r * lpc[i-1-j];
                    lpc[i-1-j] += r * tmp;
                }
                if(i & 1)
                    lpc[j] += lpc[j] * r;

                err *= (1.0 - r * r);

                /* save this order */
                for(j = 0; j <= i; j++)
                    lp_coeff[i][j] = (real)(-lpc[j]); /* negate FIR filter coeff to get predictor coeff */
                error[i] = err;

                /* see SF bug https://sourceforge.net/p/flac/bugs/234/ */
                if(err == 0.0) {
                    *max_order = i+1;
                    return;
                }
            }
            */
    }

    pub fn flac_lpc_quantize_coefficients(
            lp_coeff:  &[real],
            order:     u32,
            precision: u32,
            qlp_coeff: &[i32],
            shift:     *mut i32) -> i32 {
        
        todo!();
            /*
                unsigned i;
            double cmax;
            i32 qmax, qmin;

            ASSERT(precision > 0);
            ASSERT(precision >= MIN_QLP_COEFF_PRECISION);

            /* drop one bit for the sign; from here on out we consider only |lp_coeff[i]| */
            precision--;
            qmax = 1 << precision;
            qmin = -qmax;
            qmax--;

            /* calc cmax = max( |lp_coeff[i]| ) */
            cmax = 0.0;
            for(i = 0; i < order; i++) {
                const double d = fabs(lp_coeff[i]);
                if(d > cmax)
                    cmax = d;
            }

            if(cmax <= 0.0) {
                /* => coefficients are all 0, which means our constant-detect didn't work */
                return 2;
            }
            else {
                const int max_shiftlimit = (1 << (SUBFRAME_LPC_QLP_SHIFT_LEN-1)) - 1;
                const int min_shiftlimit = -max_shiftlimit - 1;
                int log2cmax;

                (void)frexp(cmax, &log2cmax);
                log2cmax--;
                *shift = (int)precision - log2cmax - 1;

                if(*shift > max_shiftlimit)
                    *shift = max_shiftlimit;
                else if(*shift < min_shiftlimit)
                    return 1;
            }

            if(*shift >= 0) {
                double error = 0.0;
                i32 q;
                for(i = 0; i < order; i++) {
                    error += lp_coeff[i] * (1 << *shift);
                    q = lround(error);

        #ifdef OVERFLOW_DETECT
                    if(q > qmax+1) /* we expect q==qmax+1 occasionally due to rounding */
                        fprintf(stderr,"lpc_quantize_coefficients: quantizer overflow: q>qmax %d>%d shift=%d cmax=%f precision=%u lpc[%u]=%f\n",q,qmax,*shift,cmax,precision+1,i,lp_coeff[i]);
                    else if(q < qmin)
                        fprintf(stderr,"lpc_quantize_coefficients: quantizer overflow: q<qmin %d<%d shift=%d cmax=%f precision=%u lpc[%u]=%f\n",q,qmin,*shift,cmax,precision+1,i,lp_coeff[i]);
        #endif
                    if(q > qmax)
                        q = qmax;
                    else if(q < qmin)
                        q = qmin;
                    error -= q;
                    qlp_coeff[i] = q;
                }
            }
            /* negative shift is very rare but due to design flaw, negative shift is
             * a NOP in the decoder, so it must be handled specially by scaling down
             * coeffs
             */
            else {
                const int nshift = -(*shift);
                double error = 0.0;
                i32 q;
        #ifdef DEBUG
                fprintf(stderr,"lpc_quantize_coefficients: negative shift=%d order=%u cmax=%f\n", *shift, order, cmax);
        #endif
                for(i = 0; i < order; i++) {
                    error += lp_coeff[i] / (1 << nshift);
                    q = lround(error);
        #ifdef OVERFLOW_DETECT
                    if(q > qmax+1) /* we expect q==qmax+1 occasionally due to rounding */
                        fprintf(stderr,"lpc_quantize_coefficients: quantizer overflow: q>qmax %d>%d shift=%d cmax=%f precision=%u lpc[%u]=%f\n",q,qmax,*shift,cmax,precision+1,i,lp_coeff[i]);
                    else if(q < qmin)
                        fprintf(stderr,"lpc_quantize_coefficients: quantizer overflow: q<qmin %d<%d shift=%d cmax=%f precision=%u lpc[%u]=%f\n",q,qmin,*shift,cmax,precision+1,i,lp_coeff[i]);
        #endif
                    if(q > qmax)
                        q = qmax;
                    else if(q < qmin)
                        q = qmin;
                    error -= q;
                    qlp_coeff[i] = q;
                }
                *shift = 0;
            }

            return 0;
            */
    }

    #[cfg(any(OVERFLOW_DETECT,not(LPC_UNROLLED_FILTER_LOOPS)))]
    pub fn flac_lpc_compute_residual_from_qlp_coefficients(
            data:            *const i32,
            data_len:        u32,
            qlp_coeff:       *const i32,
            order:           u32,
            lp_quantization: i32,
            residual:        *mut i32)  {
        
        todo!();
            /*
                i64 sumo;
            unsigned i, j;
            i32 sum;
            const i32 *history;

        #ifdef OVERFLOW_DETECT_VERBOSE
            fprintf(stderr,"lpc_compute_residual_from_qlp_coefficients: data_len=%d, order=%u, lpq=%d",data_len,order,lp_quantization);
            for(i=0;i<order;i++)
                fprintf(stderr,", q[%u]=%d",i,qlp_coeff[i]);
            fprintf(stderr,"\n");
        #endif
            ASSERT(order > 0);

            for(i = 0; i < data_len; i++) {
                sumo = 0;
                sum = 0;
                history = data;
                for(j = 0; j < order; j++) {
                    sum += qlp_coeff[j] * (*(--history));
                    sumo += (i64)qlp_coeff[j] * (i64)(*history);
                        fprintf(stderr,"lpc_compute_residual_from_qlp_coefficients: OVERFLOW, i=%u, j=%u, c=%d, d=%d, sumo=%" PRId64 "\n",i,j,qlp_coeff[j],*history,sumo);
                }
                *(residual++) = *(data++) - (sum >> lp_quantization);
            }

            /* Here's a slower but clearer version:
            for(i = 0; i < data_len; i++) {
                sum = 0;
                for(j = 0; j < order; j++)
                    sum += qlp_coeff[j] * data[i-j-1];
                residual[i] = data[i] - (sum >> lp_quantization);
            }
            */
            */
    }

    /**
      | fully unrolled version for normal use
      |
      */
    #[cfg(not(any(OVERFLOW_DETECT,not(LPC_UNROLLED_FILTER_LOOPS))))]
    pub fn flac_lpc_compute_residual_from_qlp_coefficients(
            data:            *const i32,
            data_len:        u32,
            qlp_coeff:       *const i32,
            order:           u32,
            lp_quantization: i32,
            residual:        *mut i32)  {
        
        todo!();
            /*
                int i;
            i32 sum;

            ASSERT(order > 0);
            ASSERT(order <= 32);

            /*
             * We do unique versions up to 12th order since that's the subset limit.
             * Also they are roughly ordered to match frequency of occurrence to
             * minimize branching.
             */
            if(order <= 12) {
                if(order > 8) {
                    if(order > 10) {
                        if(order == 12) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[11] * data[i-12];
                                sum += qlp_coeff[10] * data[i-11];
                                sum += qlp_coeff[9] * data[i-10];
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 11 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[10] * data[i-11];
                                sum += qlp_coeff[9] * data[i-10];
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 10) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[9] * data[i-10];
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 9 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                    }
                }
                else if(order > 4) {
                    if(order > 6) {
                        if(order == 8) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 7 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 6) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 5 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                    }
                }
                else {
                    if(order > 2) {
                        if(order == 4) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 3 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 2) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                residual[i] = data[i] - (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 1 */
                            for(i = 0; i < (int)data_len; i++)
                                residual[i] = data[i] - ((qlp_coeff[0] * data[i-1]) >> lp_quantization);
                        }
                    }
                }
            }
            else { /* order > 12 */
                for(i = 0; i < (int)data_len; i++) {
                    sum = 0;
                    switch(order) {
                        case 32: sum += qlp_coeff[31] * data[i-32];
                        case 31: sum += qlp_coeff[30] * data[i-31];
                        case 30: sum += qlp_coeff[29] * data[i-30];
                        case 29: sum += qlp_coeff[28] * data[i-29];
                        case 28: sum += qlp_coeff[27] * data[i-28];
                        case 27: sum += qlp_coeff[26] * data[i-27];
                        case 26: sum += qlp_coeff[25] * data[i-26];
                        case 25: sum += qlp_coeff[24] * data[i-25];
                        case 24: sum += qlp_coeff[23] * data[i-24];
                        case 23: sum += qlp_coeff[22] * data[i-23];
                        case 22: sum += qlp_coeff[21] * data[i-22];
                        case 21: sum += qlp_coeff[20] * data[i-21];
                        case 20: sum += qlp_coeff[19] * data[i-20];
                        case 19: sum += qlp_coeff[18] * data[i-19];
                        case 18: sum += qlp_coeff[17] * data[i-18];
                        case 17: sum += qlp_coeff[16] * data[i-17];
                        case 16: sum += qlp_coeff[15] * data[i-16];
                        case 15: sum += qlp_coeff[14] * data[i-15];
                        case 14: sum += qlp_coeff[13] * data[i-14];
                        case 13: sum += qlp_coeff[12] * data[i-13];
                                 sum += qlp_coeff[11] * data[i-12];
                                 sum += qlp_coeff[10] * data[i-11];
                                 sum += qlp_coeff[ 9] * data[i-10];
                                 sum += qlp_coeff[ 8] * data[i- 9];
                                 sum += qlp_coeff[ 7] * data[i- 8];
                                 sum += qlp_coeff[ 6] * data[i- 7];
                                 sum += qlp_coeff[ 5] * data[i- 6];
                                 sum += qlp_coeff[ 4] * data[i- 5];
                                 sum += qlp_coeff[ 3] * data[i- 4];
                                 sum += qlp_coeff[ 2] * data[i- 3];
                                 sum += qlp_coeff[ 1] * data[i- 2];
                                 sum += qlp_coeff[ 0] * data[i- 1];
                    }
                    residual[i] = data[i] - (sum >> lp_quantization);
                }
            }
            */
    }

    pub fn flac_lpc_compute_residual_from_qlp_coefficients_wide(
            data:            *const i32,
            data_len:        u32,
            qlp_coeff:       *const i32,
            order:           u32,
            lp_quantization: i32,
            residual:        *mut i32)  {
        
        todo!();
            /*
                #if defined(OVERFLOW_DETECT) || !defined(LPC_UNROLLED_FILTER_LOOPS)
        {
            unsigned i, j;
            i64 sum;
            const i32 *history;

        #ifdef OVERFLOW_DETECT_VERBOSE
            fprintf(stderr,"lpc_compute_residual_from_qlp_coefficients_wide: data_len=%d, order=%u, lpq=%d",data_len,order,lp_quantization);
            for(i=0;i<order;i++)
                fprintf(stderr,", q[%u]=%d",i,qlp_coeff[i]);
            fprintf(stderr,"\n");
        #endif
            ASSERT(order > 0);

            for(i = 0; i < data_len; i++) {
                sum = 0;
                history = data;
                for(j = 0; j < order; j++)
                    sum += (i64)qlp_coeff[j] * (i64)(*(--history));
                if(bitmath_silog2_wide(sum >> lp_quantization) > 32) {
                    fprintf(stderr,"lpc_compute_residual_from_qlp_coefficients_wide: OVERFLOW, i=%u, sum=%" PRId64 "\n", i, (sum >> lp_quantization));
                    break;
                }
                if(bitmath_silog2_wide((i64)(*data) - (sum >> lp_quantization)) > 32) {
                    fprintf(stderr,"lpc_compute_residual_from_qlp_coefficients_wide: OVERFLOW, i=%u, data=%d, sum=%" PRId64 ", residual=%" PRId64 "\n", i, *data, (long long)(sum >> lp_quantization), ((i64)(*data) - (sum >> lp_quantization)));
                    break;
                }
                *(residual++) = *(data++) - (i32)(sum >> lp_quantization);
            }
        }
        #else /* fully unrolled version for normal use */
        {
            int i;
            i64 sum;

            ASSERT(order > 0);
            ASSERT(order <= 32);

            /*
             * We do unique versions up to 12th order since that's the subset limit.
             * Also they are roughly ordered to match frequency of occurrence to
             * minimize branching.
             */
            if(order <= 12) {
                if(order > 8) {
                    if(order > 10) {
                        if(order == 12) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[11] * (i64)data[i-12];
                                sum += qlp_coeff[10] * (i64)data[i-11];
                                sum += qlp_coeff[9] * (i64)data[i-10];
                                sum += qlp_coeff[8] * (i64)data[i-9];
                                sum += qlp_coeff[7] * (i64)data[i-8];
                                sum += qlp_coeff[6] * (i64)data[i-7];
                                sum += qlp_coeff[5] * (i64)data[i-6];
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                        else { /* order == 11 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[10] * (i64)data[i-11];
                                sum += qlp_coeff[9] * (i64)data[i-10];
                                sum += qlp_coeff[8] * (i64)data[i-9];
                                sum += qlp_coeff[7] * (i64)data[i-8];
                                sum += qlp_coeff[6] * (i64)data[i-7];
                                sum += qlp_coeff[5] * (i64)data[i-6];
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 10) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[9] * (i64)data[i-10];
                                sum += qlp_coeff[8] * (i64)data[i-9];
                                sum += qlp_coeff[7] * (i64)data[i-8];
                                sum += qlp_coeff[6] * (i64)data[i-7];
                                sum += qlp_coeff[5] * (i64)data[i-6];
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                        else { /* order == 9 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[8] * (i64)data[i-9];
                                sum += qlp_coeff[7] * (i64)data[i-8];
                                sum += qlp_coeff[6] * (i64)data[i-7];
                                sum += qlp_coeff[5] * (i64)data[i-6];
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                    }
                }
                else if(order > 4) {
                    if(order > 6) {
                        if(order == 8) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[7] * (i64)data[i-8];
                                sum += qlp_coeff[6] * (i64)data[i-7];
                                sum += qlp_coeff[5] * (i64)data[i-6];
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                        else { /* order == 7 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[6] * (i64)data[i-7];
                                sum += qlp_coeff[5] * (i64)data[i-6];
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 6) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[5] * (i64)data[i-6];
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                        else { /* order == 5 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[4] * (i64)data[i-5];
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                    }
                }
                else {
                    if(order > 2) {
                        if(order == 4) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[3] * (i64)data[i-4];
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                        else { /* order == 3 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[2] * (i64)data[i-3];
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 2) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[1] * (i64)data[i-2];
                                sum += qlp_coeff[0] * (i64)data[i-1];
                                residual[i] = data[i] - (i32)(sum >> lp_quantization);
                            }
                        }
                        else { /* order == 1 */
                            for(i = 0; i < (int)data_len; i++)
                                residual[i] = data[i] - (i32)((qlp_coeff[0] * (i64)data[i-1]) >> lp_quantization);
                        }
                    }
                }
            }
            else { /* order > 12 */
                for(i = 0; i < (int)data_len; i++) {
                    sum = 0;
                    switch(order) {
                        case 32: sum += qlp_coeff[31] * (i64)data[i-32];
                        case 31: sum += qlp_coeff[30] * (i64)data[i-31];
                        case 30: sum += qlp_coeff[29] * (i64)data[i-30];
                        case 29: sum += qlp_coeff[28] * (i64)data[i-29];
                        case 28: sum += qlp_coeff[27] * (i64)data[i-28];
                        case 27: sum += qlp_coeff[26] * (i64)data[i-27];
                        case 26: sum += qlp_coeff[25] * (i64)data[i-26];
                        case 25: sum += qlp_coeff[24] * (i64)data[i-25];
                        case 24: sum += qlp_coeff[23] * (i64)data[i-24];
                        case 23: sum += qlp_coeff[22] * (i64)data[i-23];
                        case 22: sum += qlp_coeff[21] * (i64)data[i-22];
                        case 21: sum += qlp_coeff[20] * (i64)data[i-21];
                        case 20: sum += qlp_coeff[19] * (i64)data[i-20];
                        case 19: sum += qlp_coeff[18] * (i64)data[i-19];
                        case 18: sum += qlp_coeff[17] * (i64)data[i-18];
                        case 17: sum += qlp_coeff[16] * (i64)data[i-17];
                        case 16: sum += qlp_coeff[15] * (i64)data[i-16];
                        case 15: sum += qlp_coeff[14] * (i64)data[i-15];
                        case 14: sum += qlp_coeff[13] * (i64)data[i-14];
                        case 13: sum += qlp_coeff[12] * (i64)data[i-13];
                                 sum += qlp_coeff[11] * (i64)data[i-12];
                                 sum += qlp_coeff[10] * (i64)data[i-11];
                                 sum += qlp_coeff[ 9] * (i64)data[i-10];
                                 sum += qlp_coeff[ 8] * (i64)data[i- 9];
                                 sum += qlp_coeff[ 7] * (i64)data[i- 8];
                                 sum += qlp_coeff[ 6] * (i64)data[i- 7];
                                 sum += qlp_coeff[ 5] * (i64)data[i- 6];
                                 sum += qlp_coeff[ 4] * (i64)data[i- 5];
                                 sum += qlp_coeff[ 3] * (i64)data[i- 4];
                                 sum += qlp_coeff[ 2] * (i64)data[i- 3];
                                 sum += qlp_coeff[ 1] * (i64)data[i- 2];
                                 sum += qlp_coeff[ 0] * (i64)data[i- 1];
                    }
                    residual[i] = data[i] - (i32)(sum >> lp_quantization);
                }
            }
        }
        #endif
            */
    }
}

pub fn flac_lpc_restore_signal(
        residual:        *const i32,
        data_len:        u32,
        qlp_coeff:       *const i32,
        order:           u32,
        lp_quantization: i32,
        data:            *mut i32)  {
    
    todo!();
        /*
            #if defined(OVERFLOW_DETECT) || !defined(LPC_UNROLLED_FILTER_LOOPS)
        {
            i64 sumo;
            unsigned i, j;
            i32 sum;
            const i32 *r = residual, *history;

        #ifdef OVERFLOW_DETECT_VERBOSE
            fprintf(stderr,"lpc_restore_signal: data_len=%d, order=%u, lpq=%d",data_len,order,lp_quantization);
            for(i=0;i<order;i++)
                fprintf(stderr,", q[%u]=%d",i,qlp_coeff[i]);
            fprintf(stderr,"\n");
        #endif
            ASSERT(order > 0);

            for(i = 0; i < data_len; i++) {
                sumo = 0;
                sum = 0;
                history = data;
                for(j = 0; j < order; j++) {
                    sum += qlp_coeff[j] * (*(--history));
                    sumo += (i64)qlp_coeff[j] * (i64)(*history);
                    if(sumo > 2147483647ll || sumo < -2147483648ll)
                        fprintf(stderr,"lpc_restore_signal: OVERFLOW, i=%u, j=%u, c=%d, d=%d, sumo=%" PRId64 "\n",i,j,qlp_coeff[j],*history,sumo);
                }
                *(data++) = *(r++) + (sum >> lp_quantization);
            }

            /* Here's a slower but clearer version:
            for(i = 0; i < data_len; i++) {
                sum = 0;
                for(j = 0; j < order; j++)
                    sum += qlp_coeff[j] * data[i-j-1];
                data[i] = residual[i] + (sum >> lp_quantization);
            }
            */
        }
        #else /* fully unrolled version for normal use */
        {
            int i;
            i32 sum;

            ASSERT(order > 0);
            ASSERT(order <= 32);

            /*
             * We do unique versions up to 12th order since that's the subset limit.
             * Also they are roughly ordered to match frequency of occurrence to
             * minimize branching.
             */
            if(order <= 12) {
                if(order > 8) {
                    if(order > 10) {
                        if(order == 12) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[11] * data[i-12];
                                sum += qlp_coeff[10] * data[i-11];
                                sum += qlp_coeff[9] * data[i-10];
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 11 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[10] * data[i-11];
                                sum += qlp_coeff[9] * data[i-10];
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 10) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[9] * data[i-10];
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 9 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[8] * data[i-9];
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                    }
                }
                else if(order > 4) {
                    if(order > 6) {
                        if(order == 8) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[7] * data[i-8];
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 7 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[6] * data[i-7];
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 6) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[5] * data[i-6];
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 5 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[4] * data[i-5];
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                    }
                }
                else {
                    if(order > 2) {
                        if(order == 4) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[3] * data[i-4];
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 3 */
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[2] * data[i-3];
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                    }
                    else {
                        if(order == 2) {
                            for(i = 0; i < (int)data_len; i++) {
                                sum = 0;
                                sum += qlp_coeff[1] * data[i-2];
                                sum += qlp_coeff[0] * data[i-1];
                                data[i] = residual[i] + (sum >> lp_quantization);
                            }
                        }
                        else { /* order == 1 */
                            for(i = 0; i < (int)data_len; i++)
                                data[i] = residual[i] + ((qlp_coeff[0] * data[i-1]) >> lp_quantization);
                        }
                    }
                }
            }
            else { /* order > 12 */
                for(i = 0; i < (int)data_len; i++) {
                    sum = 0;
                    switch(order) {
                        case 32: sum += qlp_coeff[31] * data[i-32];
                        case 31: sum += qlp_coeff[30] * data[i-31];
                        case 30: sum += qlp_coeff[29] * data[i-30];
                        case 29: sum += qlp_coeff[28] * data[i-29];
                        case 28: sum += qlp_coeff[27] * data[i-28];
                        case 27: sum += qlp_coeff[26] * data[i-27];
                        case 26: sum += qlp_coeff[25] * data[i-26];
                        case 25: sum += qlp_coeff[24] * data[i-25];
                        case 24: sum += qlp_coeff[23] * data[i-24];
                        case 23: sum += qlp_coeff[22] * data[i-23];
                        case 22: sum += qlp_coeff[21] * data[i-22];
                        case 21: sum += qlp_coeff[20] * data[i-21];
                        case 20: sum += qlp_coeff[19] * data[i-20];
                        case 19: sum += qlp_coeff[18] * data[i-19];
                        case 18: sum += qlp_coeff[17] * data[i-18];
                        case 17: sum += qlp_coeff[16] * data[i-17];
                        case 16: sum += qlp_coeff[15] * data[i-16];
                        case 15: sum += qlp_coeff[14] * data[i-15];
                        case 14: sum += qlp_coeff[13] * data[i-14];
                        case 13: sum += qlp_coeff[12] * data[i-13];
                                 sum += qlp_coeff[11] * data[i-12];
                                 sum += qlp_coeff[10] * data[i-11];
                                 sum += qlp_coeff[ 9] * data[i-10];
                                 sum += qlp_coeff[ 8] * data[i- 9];
                                 sum += qlp_coeff[ 7] * data[i- 8];
                                 sum += qlp_coeff[ 6] * data[i- 7];
                                 sum += qlp_coeff[ 5] * data[i- 6];
                                 sum += qlp_coeff[ 4] * data[i- 5];
                                 sum += qlp_coeff[ 3] * data[i- 4];
                                 sum += qlp_coeff[ 2] * data[i- 3];
                                 sum += qlp_coeff[ 1] * data[i- 2];
                                 sum += qlp_coeff[ 0] * data[i- 1];
                    }
                    data[i] = residual[i] + (sum >> lp_quantization);
                }
            }
        }
        */
}

pub fn flac_lpc_restore_signal_wide(
        residual:        *const i32,
        data_len:        u32,
        qlp_coeff:       *const i32,
        order:           u32,
        lp_quantization: i32,
        data:            *mut i32)  {
    
    todo!();
        /*
            #if defined(OVERFLOW_DETECT) || !defined(LPC_UNROLLED_FILTER_LOOPS)
    {
        unsigned i, j;
        i64 sum;
        const i32 *r = residual, *history;

    #ifdef OVERFLOW_DETECT_VERBOSE
        fprintf(stderr,"lpc_restore_signal_wide: data_len=%d, order=%u, lpq=%d",data_len,order,lp_quantization);
        for(i=0;i<order;i++)
            fprintf(stderr,", q[%u]=%d",i,qlp_coeff[i]);
        fprintf(stderr,"\n");
    #endif
        ASSERT(order > 0);

        for(i = 0; i < data_len; i++) {
            sum = 0;
            history = data;
            for(j = 0; j < order; j++)
                sum += (i64)qlp_coeff[j] * (i64)(*(--history));
            if(bitmath_silog2_wide(sum >> lp_quantization) > 32) {
                fprintf(stderr,"lpc_restore_signal_wide: OVERFLOW, i=%u, sum=%" PRId64 "\n", i, (sum >> lp_quantization));
                break;
            }
            if(bitmath_silog2_wide((i64)(*r) + (sum >> lp_quantization)) > 32) {
                fprintf(stderr,"lpc_restore_signal_wide: OVERFLOW, i=%u, residual=%d, sum=%" PRId64 ", data=%" PRId64 "\n", i, *r, (sum >> lp_quantization), ((i64)(*r) + (sum >> lp_quantization)));
                break;
            }
            *(data++) = *(r++) + (i32)(sum >> lp_quantization);
        }
    }
    #else /* fully unrolled version for normal use */
    {
        int i;
        i64 sum;

        ASSERT(order > 0);
        ASSERT(order <= 32);

        /*
         * We do unique versions up to 12th order since that's the subset limit.
         * Also they are roughly ordered to match frequency of occurrence to
         * minimize branching.
         */
        if(order <= 12) {
            if(order > 8) {
                if(order > 10) {
                    if(order == 12) {
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[11] * (i64)data[i-12];
                            sum += qlp_coeff[10] * (i64)data[i-11];
                            sum += qlp_coeff[9] * (i64)data[i-10];
                            sum += qlp_coeff[8] * (i64)data[i-9];
                            sum += qlp_coeff[7] * (i64)data[i-8];
                            sum += qlp_coeff[6] * (i64)data[i-7];
                            sum += qlp_coeff[5] * (i64)data[i-6];
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                    else { /* order == 11 */
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[10] * (i64)data[i-11];
                            sum += qlp_coeff[9] * (i64)data[i-10];
                            sum += qlp_coeff[8] * (i64)data[i-9];
                            sum += qlp_coeff[7] * (i64)data[i-8];
                            sum += qlp_coeff[6] * (i64)data[i-7];
                            sum += qlp_coeff[5] * (i64)data[i-6];
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                }
                else {
                    if(order == 10) {
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[9] * (i64)data[i-10];
                            sum += qlp_coeff[8] * (i64)data[i-9];
                            sum += qlp_coeff[7] * (i64)data[i-8];
                            sum += qlp_coeff[6] * (i64)data[i-7];
                            sum += qlp_coeff[5] * (i64)data[i-6];
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                    else { /* order == 9 */
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[8] * (i64)data[i-9];
                            sum += qlp_coeff[7] * (i64)data[i-8];
                            sum += qlp_coeff[6] * (i64)data[i-7];
                            sum += qlp_coeff[5] * (i64)data[i-6];
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                }
            }
            else if(order > 4) {
                if(order > 6) {
                    if(order == 8) {
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[7] * (i64)data[i-8];
                            sum += qlp_coeff[6] * (i64)data[i-7];
                            sum += qlp_coeff[5] * (i64)data[i-6];
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                    else { /* order == 7 */
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[6] * (i64)data[i-7];
                            sum += qlp_coeff[5] * (i64)data[i-6];
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                }
                else {
                    if(order == 6) {
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[5] * (i64)data[i-6];
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                    else { /* order == 5 */
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[4] * (i64)data[i-5];
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                }
            }
            else {
                if(order > 2) {
                    if(order == 4) {
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[3] * (i64)data[i-4];
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                    else { /* order == 3 */
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[2] * (i64)data[i-3];
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                }
                else {
                    if(order == 2) {
                        for(i = 0; i < (int)data_len; i++) {
                            sum = 0;
                            sum += qlp_coeff[1] * (i64)data[i-2];
                            sum += qlp_coeff[0] * (i64)data[i-1];
                            data[i] = residual[i] + (i32)(sum >> lp_quantization);
                        }
                    }
                    else { /* order == 1 */
                        for(i = 0; i < (int)data_len; i++)
                            data[i] = residual[i] + (i32)((qlp_coeff[0] * (i64)data[i-1]) >> lp_quantization);
                    }
                }
            }
        }
        else { /* order > 12 */
            for(i = 0; i < (int)data_len; i++) {
                sum = 0;
                switch(order) {
                    case 32: sum += qlp_coeff[31] * (i64)data[i-32];
                    case 31: sum += qlp_coeff[30] * (i64)data[i-31];
                    case 30: sum += qlp_coeff[29] * (i64)data[i-30];
                    case 29: sum += qlp_coeff[28] * (i64)data[i-29];
                    case 28: sum += qlp_coeff[27] * (i64)data[i-28];
                    case 27: sum += qlp_coeff[26] * (i64)data[i-27];
                    case 26: sum += qlp_coeff[25] * (i64)data[i-26];
                    case 25: sum += qlp_coeff[24] * (i64)data[i-25];
                    case 24: sum += qlp_coeff[23] * (i64)data[i-24];
                    case 23: sum += qlp_coeff[22] * (i64)data[i-23];
                    case 22: sum += qlp_coeff[21] * (i64)data[i-22];
                    case 21: sum += qlp_coeff[20] * (i64)data[i-21];
                    case 20: sum += qlp_coeff[19] * (i64)data[i-20];
                    case 19: sum += qlp_coeff[18] * (i64)data[i-19];
                    case 18: sum += qlp_coeff[17] * (i64)data[i-18];
                    case 17: sum += qlp_coeff[16] * (i64)data[i-17];
                    case 16: sum += qlp_coeff[15] * (i64)data[i-16];
                    case 15: sum += qlp_coeff[14] * (i64)data[i-15];
                    case 14: sum += qlp_coeff[13] * (i64)data[i-14];
                    case 13: sum += qlp_coeff[12] * (i64)data[i-13];
                             sum += qlp_coeff[11] * (i64)data[i-12];
                             sum += qlp_coeff[10] * (i64)data[i-11];
                             sum += qlp_coeff[ 9] * (i64)data[i-10];
                             sum += qlp_coeff[ 8] * (i64)data[i- 9];
                             sum += qlp_coeff[ 7] * (i64)data[i- 8];
                             sum += qlp_coeff[ 6] * (i64)data[i- 7];
                             sum += qlp_coeff[ 5] * (i64)data[i- 6];
                             sum += qlp_coeff[ 4] * (i64)data[i- 5];
                             sum += qlp_coeff[ 3] * (i64)data[i- 4];
                             sum += qlp_coeff[ 2] * (i64)data[i- 3];
                             sum += qlp_coeff[ 1] * (i64)data[i- 2];
                             sum += qlp_coeff[ 0] * (i64)data[i- 1];
                }
                data[i] = residual[i] + (i32)(sum >> lp_quantization);
            }
        }
    }
    #endif
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_expected_bits_per_residual_sample(
        lpc_error:     f64,
        total_samples: u32) -> f64 {
    
    todo!();
        /*
            double error_scale;

        ASSERT(total_samples > 0);

        error_scale = 0.5 * M_LN2 * M_LN2 / (double)total_samples;

        return lpc_compute_expected_bits_per_residual_sample_with_error_scale(lpc_error, error_scale);
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_expected_bits_per_residual_sample_with_error_scale(
        lpc_error:   f64,
        error_scale: f64) -> f64 {
    
    todo!();
        /*
            if(lpc_error > 0.0) {
            double bps = (double)0.5 * log(error_scale * lpc_error) / M_LN2;
            if(bps >= 0.0)
                return bps;
            else
                return 0.0;
        }
        else if(lpc_error < 0.0) { /* error should not be negative but can happen due to inadequate floating-point resolution */
            return 1e32;
        }
        else {
            return 0.0;
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_lpc_compute_best_order(
        lpc_error:               &[f64],
        max_order:               u32,
        total_samples:           u32,
        overhead_bits_per_order: u32) -> u32 {
    
    todo!();
        /*
            unsigned order, indx, best_index; /* 'index' the index into lpc_error; index==order-1 since lpc_error[0] is for order==1, lpc_error[1] is for order==2, etc */
        double bits, best_bits, error_scale;

        ASSERT(max_order > 0);
        ASSERT(total_samples > 0);

        error_scale = 0.5 * M_LN2 * M_LN2 / (double)total_samples;

        best_index = 0;
        best_bits = (unsigned)(-1);

        for(indx = 0, order = 1; indx < max_order; indx++, order++) {
            bits = lpc_compute_expected_bits_per_residual_sample_with_error_scale(lpc_error[indx], error_scale) * (double)(total_samples - order) + (double)(order * overhead_bits_per_order);
            if(bits < best_bits) {
                best_index = indx;
                best_bits = bits;
            }
        }

        return best_index+1; /* +1 since indx of lpc_error[] is order-1 */
        */
}
