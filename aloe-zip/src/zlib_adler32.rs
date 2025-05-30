/*!
  | adler32.c -- compute the Adler-32 checksum
  | of a data stream Copyright (C) 1995-2004
  | Mark Adler For conditions of distribution
  | and use, see copyright notice in zlib.h
  | 
  | @(#) $Id: adler32.c,v 1.1 2007/06/07
  | 17:54:37 jules_rms Exp $
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/zlib/adler32.c]

/**
   largest prime smaller than 65536
  */
pub const BASE: usize = 65521;

/**
   NMAX is the largest n such that 255n(n+1)/2
   + (n+1)(BASE-1) <= 2^32-1
  */
pub const NMAX: usize = 5552;

macro_rules! do1 {
    ($buf:ident, $i:ident) => {
        /*
                {adler += (buf)[i]; sum2 += adler;}
        */
    }
}

macro_rules! do2 {
    ($buf:ident, $i:ident) => {
        /*
                DO1(buf,i); DO1(buf,i+1);
        */
    }
}

macro_rules! do4 {
    ($buf:ident, $i:ident) => {
        /*
                DO2(buf,i); DO2(buf,i+2);
        */
    }
}

macro_rules! do8 {
    ($buf:ident, $i:ident) => {
        /*
                DO4(buf,i); DO4(buf,i+4);
        */
    }
}

macro_rules! do16 {
    ($buf:ident) => {
        /*
                DO8(buf,0); DO8(buf,8);
        */
    }
}

/**
  | use NO_DIVIDE if your processor does
  | not do division in hardware
  |
  */
#[cfg(NO_DIVIDE)]
macro_rules! mod_ {
    ($a:ident) => {
        /*
        
            do { 
                if (a >= (BASE << 16)) a -= (BASE << 16); 
                if (a >= (BASE << 15)) a -= (BASE << 15); 
                if (a >= (BASE << 14)) a -= (BASE << 14); 
                if (a >= (BASE << 13)) a -= (BASE << 13); 
                if (a >= (BASE << 12)) a -= (BASE << 12); 
                if (a >= (BASE << 11)) a -= (BASE << 11); 
                if (a >= (BASE << 10)) a -= (BASE << 10); 
                if (a >= (BASE << 9)) a -= (BASE << 9); 
                if (a >= (BASE << 8)) a -= (BASE << 8); 
                if (a >= (BASE << 7)) a -= (BASE << 7); 
                if (a >= (BASE << 6)) a -= (BASE << 6); 
                if (a >= (BASE << 5)) a -= (BASE << 5); 
                if (a >= (BASE << 4)) a -= (BASE << 4); 
                if (a >= (BASE << 3)) a -= (BASE << 3); 
                if (a >= (BASE << 2)) a -= (BASE << 2); 
                if (a >= (BASE << 1)) a -= (BASE << 1); 
                if (a >= BASE) a -= BASE; 
            } while (0)
        */
    }
}

#[cfg(NO_DIVIDE)]
macro_rules! mod4 {
    ($a:ident) => {
        /*
        
            do { 
                if (a >= (BASE << 4)) a -= (BASE << 4); 
                if (a >= (BASE << 3)) a -= (BASE << 3); 
                if (a >= (BASE << 2)) a -= (BASE << 2); 
                if (a >= (BASE << 1)) a -= (BASE << 1); 
                if (a >= BASE) a -= BASE; 
            } while (0)
        */
    }
}

#[cfg(not(NO_DIVIDE))]
macro_rules! mod_ {
    ($a:ident) => {
        /*
                a %= BASE
        */
    }
}

#[cfg(not(NO_DIVIDE))]
macro_rules! mod4 {
    ($a:ident) => {
        /*
                a %= BASE
        */
    }
}

pub fn adler32(
        adler: u64,
        buf:   *const Bytef,
        len:   u32) -> u64 {
    
    todo!();
    /*
        unsigned long sum2;
        unsigned n;

        /* split Adler-32 into component sums */
        sum2 = (adler >> 16) & 0xffff;
        adler &= 0xffff;

        /* in case user likes doing a byte at a time, keep it fast */
        if (len == 1) {
            adler += buf[0];
            if (adler >= BASE)
                adler -= BASE;
            sum2 += adler;
            if (sum2 >= BASE)
                sum2 -= BASE;
            return adler | (sum2 << 16);
        }

        /* initial Adler-32 value (deferred check for len == 1 speed) */
        if (buf == Z_NULL)
            return 1L;

        /* in case short lengths are provided, keep it somewhat fast */
        if (len < 16) {
            while (len--) {
                adler += *buf++;
                sum2 += adler;
            }
            if (adler >= BASE)
                adler -= BASE;
            MOD4(sum2);             /* only added so many BASE's */
            return adler | (sum2 << 16);
        }

        /* do length NMAX blocks -- requires just one modulo operation */
        while (len >= NMAX) {
            len -= NMAX;
            n = NMAX / 16;          /* NMAX is divisible by 16 */
            do {
                DO16(buf);          /* 16 sums unrolled */
                buf += 16;
            } while (--n);
            MOD(adler);
            MOD(sum2);
        }

        /* do remaining bytes (less than NMAX, still just one modulo) */
        if (len) {                  /* avoid modulos if none remaining */
            while (len >= 16) {
                len -= 16;
                DO16(buf);
                buf += 16;
            }
            while (len--) {
                adler += *buf++;
                sum2 += adler;
            }
            MOD(adler);
            MOD(sum2);
        }

        /* return recombined sums */
        return adler | (sum2 << 16);
    */
}

pub fn adler32_combine(
        adler1: u64,
        adler2: u64,
        len2:   ZOff) -> u64 {
    
    todo!();
    /*
        unsigned long sum1;
        unsigned long sum2;
        unsigned rem;

        /* the derivation of this formula is left as an exercise for the reader */
        rem = (unsigned)(len2 % BASE);
        sum1 = adler1 & 0xffff;
        sum2 = rem * sum1;
        MOD(sum2);
        sum1 += (adler2 & 0xffff) + BASE - 1;
        sum2 += ((adler1 >> 16) & 0xffff) + ((adler2 >> 16) & 0xffff) + BASE - rem;
        if (sum1 > BASE) sum1 -= BASE;
        if (sum1 > BASE) sum1 -= BASE;
        if (sum2 > (BASE << 1)) sum2 -= (BASE << 1);
        if (sum2 > BASE) sum2 -= BASE;
        return sum1 | (sum2 << 16);
    */
}
