crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/libFLAC/bitmath.c]

/** 
 | An example of what bitmath_silog2() computes:
 |
 | silog2(-10) = 5
 | silog2(- 9) = 5
 | silog2(- 8) = 4
 | silog2(- 7) = 4
 | silog2(- 6) = 4
 | silog2(- 5) = 4
 | silog2(- 4) = 3
 | silog2(- 3) = 3
 | silog2(- 2) = 2
 | silog2(- 1) = 2
 | silog2(  0) = 0
 | silog2(  1) = 2
 | silog2(  2) = 3
 | silog2(  3) = 3
 | silog2(  4) = 4
 | silog2(  5) = 4
 | silog2(  6) = 4
 | silog2(  7) = 4
 | silog2(  8) = 5
 | silog2(  9) = 5
 | silog2( 10) = 5
 */
pub fn flac_bitmath_silog2(v: i32) -> u32 {
    
    todo!();
        /*
            while(1) {
            if(v == 0) {
                return 0;
            }
            else if(v > 0) {
                unsigned l = 0;
                while(v) {
                    l++;
                    v >>= 1;
                }
                return l+1;
            }
            else if(v == -1) {
                return 2;
            }
            else {
                v++;
                v = -v;
            }
        }
        */
}

pub fn flac_bitmath_silog2_wide(v: i64) -> u32 {
    
    todo!();
        /*
            while(1) {
            if(v == 0) {
                return 0;
            }
            else if(v > 0) {
                unsigned l = 0;
                while(v) {
                    l++;
                    v >>= 1;
                }
                return l+1;
            }
            else if(v == -1) {
                return 2;
            }
            else {
                v++;
                v = -v;
            }
        }
        */
}
