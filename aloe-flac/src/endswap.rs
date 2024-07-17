/*!
  | It is assumed that this header will be
  | included after "config.h".
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/flac/endswap.h]

/**
  | GCC prior to 4.8 didn't provide bswap16
  | on x86_64
  |
  */
#[cfg(HAVE_BSWAP32)] /* GCC and Clang */
#[cfg(not(HAVE_BSWAP16))]
#[inline] pub fn builtin_bswap16(a: u16) -> u16 {
    
    todo!();
        /*
            return (a<<8)|(a>>8);
        */
}

#[cfg(HAVE_BSWAP32)] /* GCC and Clang */
macro_rules! endswap_16 {
    ($x:ident) => {
        /*
                (__builtin_bswap16 (x))
        */
    }
}

#[cfg(HAVE_BSWAP32)] /* GCC and Clang */
macro_rules! endswap_32 {
    ($x:ident) => {
        /*
                (__builtin_bswap32 (x))
        */
    }
}

#[cfg(_MSC_VER)] /* Windows. Apparently in <stdlib.h>. */
macro_rules! endswap_16 {
    ($x:ident) => {
        /*
                (_byteswap_ushort (x))
        */
    }
}

#[cfg(_MSC_VER)] /* Windows. Apparently in <stdlib.h>. */
macro_rules! endswap_32 {
    ($x:ident) => {
        /*
                (_byteswap_ulong (x))
        */
    }
}

#[cfg(HAVE_BYTESWAP_H)] /* Linux */
macro_rules! endswap_16 {
    ($x:ident) => {
        /*
                (bswap_16 (x))
        */
    }
}

#[cfg(HAVE_BYTESWAP_H)] /* Linux */
macro_rules! endswap_32 {
    ($x:ident) => {
        /*
                (bswap_32 (x))
        */
    }
}

#[cfg(not(any(HAVE_BSWAP32, _MSC_VER, HAVE_BYTESWAP_H)))]
macro_rules! endswap_16 {
    ($x:ident) => {
        /*
                ((((x) >> 8) & 0xFF) | (((x) & 0xFF) << 8))
        */
    }
}

#[cfg(not(any(HAVE_BSWAP32, _MSC_VER, HAVE_BYTESWAP_H)))]
macro_rules! endswap_32 {
    ($x:ident) => {
        /*
                ((((x) >> 24) & 0xFF) | (((x) >> 8) & 0xFF00) | (((x) & 0xFF00) << 8) | (((x) & 0xFF) << 24))
        */
    }
}

/**
  | Host to little-endian byte swapping.
  |
  */
#[cfg(CPU_IS_BIG_ENDIAN)]
macro_rules! h2le_16 {
    ($x:ident) => {
        /*
                ENDSWAP_16 (x)
        */
    }
}

#[cfg(CPU_IS_BIG_ENDIAN)]
macro_rules! h2le_32 {
    ($x:ident) => {
        /*
                ENDSWAP_32 (x)
        */
    }
}

#[cfg(not(CPU_IS_BIG_ENDIAN))]
macro_rules! h2le_16 {
    ($x:ident) => {
        /*
                (x)
        */
    }
}

#[cfg(not(CPU_IS_BIG_ENDIAN))]
macro_rules! h2le_32 {
    ($x:ident) => {
        /*
                (x)
        */
    }
}
