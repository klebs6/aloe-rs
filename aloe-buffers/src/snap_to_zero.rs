crate::ix!();

#[cfg(not(ALOE_SNAP_TO_ZERO))]
#[cfg(ALOE_INTEL)]
macro_rules! aloe_snap_to_zero {
    ($n:ident) => {
        /*
                if (! (n < -1.0e-8f || n > 1.0e-8f)) n = 0;
        */
    }
}

#[cfg(not(ALOE_SNAP_TO_ZERO))]
#[cfg(not(ALOE_INTEL))]
macro_rules! aloe_snap_to_zero {
    ($n:ident) => {
        /*
                ignoreUnused (n)
        */
    }
}


