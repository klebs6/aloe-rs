crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/futils.h]

/**
   min/max/etc. template functions
  */
#[inline] pub fn min<'a, T>(a: &'a T, b: &'a T) -> &'a T {

    todo!();
        /*
            return b < a ? b : a;
        */
}

#[inline] pub fn max<'a, T>(a: &'a T, b: &'a T) -> &'a T {

    todo!();
        /*
            return a < b ? b : a;
        */
}

#[inline] pub fn abs<'a, T>(value: &'a T) -> T {

    todo!();
        /*
            return (value >= (T)0) ? value : -value;
        */
}

#[inline] pub fn sign<'a, T>(value: &'a T) -> T {

    todo!();
        /*
            return (value == (T)0) ? 0 : ((value >= (T)0) ? 1 : -1);
        */
}

#[inline] pub fn bound<T>(
    minval: T,
    maxval: T,
    x:      T

) -> T {

    todo!();
        /*
            if (x < minval)
            return minval;
        else if (x > maxval)
            return maxval;
        return x;
        */
}

pub fn swap<T>(
    t1: &mut T,
    t2: &mut T

) {

    todo!();
        /*
            T tmp = t1;
        t1 = t2;
        t2 = tmp;
        */
}

pub fn is_approximate_equal<T>(
    t1:      T,
    t2:      T,
    epsilon: T

) -> bool {

    todo!();
        /*
            if (t1 == t2)
            return true;
        T diff = t1 - t2;
        if (diff < 0.0)
            diff = -diff;
        if (diff < epsilon)
            return true;
        return false;
        */
}

#[inline] pub fn to_normalized<T>(
    value:     &T,
    num_steps: i32

) -> T {

    todo!();
        /*
            return value / T (numSteps);
        */
}

#[inline] pub fn from_normalized<T>(
    norm:      &T,
    num_steps: i32

) -> i32 {

    todo!();
        /*
            return Min<int32> (numSteps, int32 (norm * (numSteps + 1)));
        */
}

/**
   Four character constant
  */
#[cfg(not(CCONST))]
macro_rules! cconst {
    ($a:ident, 
     $b:ident, 
     $c:ident, 
     $d:ident) => {
        /*
        
             ((((int32) (a)) << 24) | (((int32) (b)) << 16) | (((int32) (c)) << 8) | (((int32) (d)) << 0))
        */
    }
}
