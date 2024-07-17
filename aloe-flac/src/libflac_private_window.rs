crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/include/private/window.h]

#[cfg(not(INTEGER_ONLY_LIBRARY))]
pub mod flac_integer_only_library {

    use super::*;

    /**
      | window_*()
      | 
      | -----------------------------------------
      | 
      | Calculates window coefficients according
      | to different apodization functions.
      | 
      | OUT window[0,L-1]
      | 
      | IN L (number of points in window)
      |
      */
    pub fn flac_window_bartlett(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_bartlett_hann(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_blackman(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_blackman_harris_4term_92db_sidelobe(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_connes(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_flattop(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    /**
      | 0.0 < stddev <= 0.5
      |
      */
    pub fn flac_window_gauss(
            window: *mut real,
            l:      i32,
            stddev: real)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_hamming(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_hann(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_kaiser_bessel(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_nuttall(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_rectangle(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_triangle(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_tukey(
            window: *mut real,
            l:      i32,
            p:      real)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_partial_tukey(
            window: *mut real,
            l:      i32,
            p:      real,
            start:  real,
            end:    real)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_punchout_tukey(
            window: *mut real,
            l:      i32,
            p:      real,
            start:  real,
            end:    real)  {
        
        todo!();
            /*
            
            */
    }

    pub fn flac_window_welch(
            window: *mut real,
            l:      i32)  {
        
        todo!();
            /*
            
            */
    }
}

