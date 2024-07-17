crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/window_flac.c]

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_bartlett(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        if (L & 1) {
            for (n = 0; n <= N/2; n++)
                window[n] = 2.0f * n / (float)N;
            for (; n <= N; n++)
                window[n] = 2.0f - 2.0f * n / (float)N;
        }
        else {
            for (n = 0; n <= L/2-1; n++)
                window[n] = 2.0f * n / (float)N;
            for (; n <= N; n++)
                window[n] = 2.0f - 2.0f * n / (float)N;
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_bartlett_hann(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n < L; n++)
            window[n] = (real)(0.62f - 0.48f * fabs((float)n/(float)N-0.5f) - 0.38f * cos(2.0f * M_PI * ((float)n/(float)N)));
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_blackman(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n < L; n++)
            window[n] = (real)(0.42f - 0.5f * cos(2.0f * M_PI * n / N) + 0.08f * cos(4.0f * M_PI * n / N));
        */
}

/**
  | 4-term -92dB side-lobe
  |
  */
#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_blackman_harris_4term_92db_sidelobe(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n <= N; n++)
            window[n] = (real)(0.35875f - 0.48829f * cos(2.0f * M_PI * n / N) + 0.14128f * cos(4.0f * M_PI * n / N) - 0.01168f * cos(6.0f * M_PI * n / N));
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_connes(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        const double N2 = (double)N / 2.;
        i32 n;

        for (n = 0; n <= N; n++) {
            double k = ((double)n - N2) / N2;
            k = 1.0f - k * k;
            window[n] = (real)(k * k);
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_flattop(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n < L; n++)
            window[n] = (real)(1.0f - 1.93f * cos(2.0f * M_PI * n / N) + 1.29f * cos(4.0f * M_PI * n / N) - 0.388f * cos(6.0f * M_PI * n / N) + 0.0322f * cos(8.0f * M_PI * n / N));
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_gauss(
        window: *mut real,
        l:      i32,
        stddev: real)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        const double N2 = (double)N / 2.;
        i32 n;

        for (n = 0; n <= N; n++) {
            const double k = ((double)n - N2) / (stddev * N2);
            window[n] = (real)exp(-0.5f * k * k);
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_hamming(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n < L; n++)
            window[n] = (real)(0.54f - 0.46f * cos(2.0f * M_PI * n / N));
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_hann(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n < L; n++)
            window[n] = (real)(0.5f - 0.5f * cos(2.0f * M_PI * n / N));
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_kaiser_bessel(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n < L; n++)
            window[n] = (real)(0.402f - 0.498f * cos(2.0f * M_PI * n / N) + 0.098f * cos(4.0f * M_PI * n / N) - 0.001f * cos(6.0f * M_PI * n / N));
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_nuttall(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        i32 n;

        for (n = 0; n < L; n++)
            window[n] = (real)(0.3635819f - 0.4891775f*cos(2.0f*M_PI*n/N) + 0.1365995f*cos(4.0f*M_PI*n/N) - 0.0106411f*cos(6.0f*M_PI*n/N));
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_rectangle(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            i32 n;

        for (n = 0; n < L; n++)
            window[n] = 1.0f;
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_triangle(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            i32 n;

        if (L & 1) {
            for (n = 1; n <= (L+1)/2; n++)
                window[n-1] = 2.0f * n / ((float)L + 1.0f);
            for (; n <= L; n++)
                window[n-1] = (float)(2 * (L - n + 1)) / ((float)L + 1.0f);
        }
        else {
            for (n = 1; n <= L/2; n++)
                window[n-1] = 2.0f * n / ((float)L + 1.0f);
            for (; n <= L; n++)
                window[n-1] = (float)(2 * (L - n + 1)) / ((float)L + 1.0f);
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_tukey(
        window: *mut real,
        l:      i32,
        p:      real)  {
    
    todo!();
        /*
            if (p <= 0.0)
            window_rectangle(window, L);
        else if (p >= 1.0)
            window_hann(window, L);
        else {
            const i32 Np = (i32)(p / 2.0f * L) - 1;
            i32 n;
            /* start with rectangle... */
            window_rectangle(window, L);
            /* ...replace ends with hann */
            if (Np > 0) {
                for (n = 0; n <= Np; n++) {
                    window[n] = (real)(0.5f - 0.5f * cos(M_PI * n / Np));
                    window[L-Np-1+n] = (real)(0.5f - 0.5f * cos(M_PI * (n+Np) / Np));
                }
            }
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_partial_tukey(
        window: *mut real,
        l:      i32,
        p:      real,
        start:  real,
        end:    real)  {
    
    todo!();
        /*
            const i32 start_n = (i32)(start * L);
        const i32 end_n = (i32)(end * L);
        const i32 N = end_n - start_n;
        i32 Np, n, i;

        if (p <= 0.0f)
            window_partial_tukey(window, L, 0.05f, start, end);
        else if (p >= 1.0f)
            window_partial_tukey(window, L, 0.95f, start, end);
        else {

            Np = (i32)(p / 2.0f * N);

            for (n = 0; n < start_n && n < L; n++)
                window[n] = 0.0f;
            for (i = 1; n < (start_n+Np) && n < L; n++, i++)
                window[n] = (real)(0.5f - 0.5f * cos(M_PI * i / Np));
            for (; n < (end_n-Np) && n < L; n++)
                window[n] = 1.0f;
            for (i = Np; n < end_n && n < L; n++, i--)
                window[n] = (real)(0.5f - 0.5f * cos(M_PI * i / Np));
            for (; n < L; n++)
                window[n] = 0.0f;
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_punchout_tukey(
        window: *mut real,
        l:      i32,
        p:      real,
        start:  real,
        end:    real)  {
    
    todo!();
        /*
            const i32 start_n = (i32)(start * L);
        const i32 end_n = (i32)(end * L);
        i32 Ns, Ne, n, i;

        if (p <= 0.0f)
            window_punchout_tukey(window, L, 0.05f, start, end);
        else if (p >= 1.0f)
            window_punchout_tukey(window, L, 0.95f, start, end);
        else {

            Ns = (i32)(p / 2.0f * start_n);
            Ne = (i32)(p / 2.0f * (L - end_n));

            for (n = 0, i = 1; n < Ns && n < L; n++, i++)
                window[n] = (real)(0.5f - 0.5f * cos(M_PI * i / Ns));
            for (; n < start_n-Ns && n < L; n++)
                window[n] = 1.0f;
            for (i = Ns; n < start_n && n < L; n++, i--)
                window[n] = (real)(0.5f - 0.5f * cos(M_PI * i / Ns));
            for (; n < end_n && n < L; n++)
                window[n] = 0.0f;
            for (i = 1; n < end_n+Ne && n < L; n++, i++)
                window[n] = (real)(0.5f - 0.5f * cos(M_PI * i / Ne));
            for (; n < L - (Ne) && n < L; n++)
                window[n] = 1.0f;
            for (i = Ne; n < L; n++, i--)
                window[n] = (real)(0.5f - 0.5f * cos(M_PI * i / Ne));
        }
        */
}

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub fn flac_window_welch(
        window: *mut real,
        l:      i32)  {
    
    todo!();
        /*
            const i32 N = L - 1;
        const double N2 = (double)N / 2.;
        i32 n;

        for (n = 0; n <= N; n++) {
            const double k = ((double)n - N2) / N2;
            window[n] = (real)(1.0f - k * k);
        }
        */
}
