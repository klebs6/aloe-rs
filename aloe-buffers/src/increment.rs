crate::ix!();

macro_rules! aloe_increment_src_dest {
    () => {
        /*
                dest += (16 / sizeof (*dest)); src += (16 / sizeof (*dest));
        */
    }
}

macro_rules! aloe_increment_src1_src2_dest {
    () => {
        /*
                dest += (16 / sizeof (*dest)); src1 += (16 / sizeof (*dest)); src2 += (16 / sizeof (*dest));
        */
    }
}

macro_rules! aloe_increment_dest {
    () => {
        /*
                dest += (16 / sizeof (*dest));
        */
    }
}
