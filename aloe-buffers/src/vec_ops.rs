crate::ix!();

#[cfg(ALOE_USE_SSE_INTRINSICS)]
macro_rules! aloe_begin_vec_op {
    () => {
        /*
        
                using Mode = FloatVectorHelpers::ModeType<sizeof(*dest)>::Mode; 
                { 
                    const int numLongOps = num / Mode::numParallel;
        */
    }
}

#[cfg(ALOE_USE_SSE_INTRINSICS)]
macro_rules! aloe_finish_vec_op {
    ($normalOp:ident) => {
        /*
        
                    num &= (Mode::numParallel - 1); 
                    if (num == 0) return; 
                } 
                for (int i = 0; i < num; ++i) normalOp;
        */
    }
}

#[cfg(ALOE_USE_SSE_INTRINSICS)]
macro_rules! aloe_perform_vec_op_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                if (FloatVectorHelpers::isAligned (dest))   ALOE_VEC_LOOP (vecOp, dummy, Mode::loadA, Mode::storeA, locals, ALOE_INCREMENT_DEST) 
                else                                        ALOE_VEC_LOOP (vecOp, dummy, Mode::loadU, Mode::storeU, locals, ALOE_INCREMENT_DEST) 
                ALOE_FINISH_VEC_OP (normalOp)
        */
    }
}

#[cfg(ALOE_USE_SSE_INTRINSICS)]
macro_rules! aloe_perform_vec_op_src_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                if (FloatVectorHelpers::isAligned (dest)) 
                { 
                    if (FloatVectorHelpers::isAligned (src)) ALOE_VEC_LOOP (vecOp, Mode::loadA, Mode::loadA, Mode::storeA, locals, increment) 
                    else                                     ALOE_VEC_LOOP (vecOp, Mode::loadU, Mode::loadA, Mode::storeA, locals, increment) 
                }
                else 
                { 
                    if (FloatVectorHelpers::isAligned (src)) ALOE_VEC_LOOP (vecOp, Mode::loadA, Mode::loadU, Mode::storeU, locals, increment) 
                    else                                     ALOE_VEC_LOOP (vecOp, Mode::loadU, Mode::loadU, Mode::storeU, locals, increment) 
                } 
                ALOE_FINISH_VEC_OP (normalOp)
        */
    }
}

#[cfg(ALOE_USE_SSE_INTRINSICS)]
macro_rules! aloe_perform_vec_op_src1_src2_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                if (FloatVectorHelpers::isAligned (dest)) 
                { 
                    if (FloatVectorHelpers::isAligned (src1)) 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadA, Mode::loadA, Mode::storeA, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadA, Mode::loadU, Mode::storeA, locals, increment) 
                    } 
                    else 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadU, Mode::loadA, Mode::storeA, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadU, Mode::loadU, Mode::storeA, locals, increment) 
                    } 
                } 
                else 
                { 
                    if (FloatVectorHelpers::isAligned (src1)) 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadA, Mode::loadA, Mode::storeU, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadA, Mode::loadU, Mode::storeU, locals, increment) 
                    } 
                    else 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadU, Mode::loadA, Mode::storeU, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadU, Mode::loadU, Mode::storeU, locals, increment) 
                    } 
                } 
                ALOE_FINISH_VEC_OP (normalOp)
        */
    }
}

#[cfg(ALOE_USE_SSE_INTRINSICS)]
macro_rules! aloe_perform_vec_op_src1_src2_dest_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                if (FloatVectorHelpers::isAligned (dest)) 
                { 
                    if (FloatVectorHelpers::isAligned (src1)) 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadA, Mode::loadA, Mode::loadA, Mode::storeA, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadA, Mode::loadU, Mode::loadA, Mode::storeA, locals, increment) 
                    } 
                    else 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadU, Mode::loadA, Mode::loadA, Mode::storeA, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadU, Mode::loadU, Mode::loadA, Mode::storeA, locals, increment) 
                    } 
                } 
                else 
                { 
                    if (FloatVectorHelpers::isAligned (src1)) 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadA, Mode::loadA, Mode::loadU, Mode::storeU, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadA, Mode::loadU, Mode::loadU, Mode::storeU, locals, increment) 
                    } 
                    else 
                    { 
                        if (FloatVectorHelpers::isAligned (src2))   ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadU, Mode::loadA, Mode::loadU, Mode::storeU, locals, increment) 
                        else                                        ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadU, Mode::loadU, Mode::loadU, Mode::storeU, locals, increment) 
                    } 
                } 
                ALOE_FINISH_VEC_OP (normalOp)

        */
    }
}

///----------------------------
macro_rules! aloe_vec_loop {
    ($vecOp:ident, 
     $srcLoad:ident, 
     $dstLoad:ident, 
     $dstStore:ident, 
     $locals:ident, 
     $increment:ident) => {
        /*
        
                for (int i = 0; i < numLongOps; ++i) 
                { 
                    locals (srcLoad, dstLoad); 
                    dstStore (dest, vecOp); 
                    increment; 
                }
        */
    }
}

macro_rules! aloe_vec_loop_two_sources {
    ($vecOp:ident, 
     $src1Load:ident, 
     $src2Load:ident, 
     $dstStore:ident, 
     $locals:ident, 
     $increment:ident) => {
        /*
        
                for (int i = 0; i < numLongOps; ++i) 
                { 
                    locals (src1Load, src2Load); 
                    dstStore (dest, vecOp); 
                    increment; 
                }
        */
    }
}

macro_rules! aloe_vec_loop_two_sources_with_dest_load {
    ($vecOp:ident, 
     $src1Load:ident, 
     $src2Load:ident, 
     $dstLoad:ident, 
     $dstStore:ident, 
     $locals:ident, 
     $increment:ident) => {
        /*
        
                for (int i = 0; i < numLongOps; ++i) 
                { 
                    locals (src1Load, src2Load, dstLoad); 
                    dstStore (dest, vecOp); 
                    increment; 
                }
        */
    }
}

macro_rules! aloe_load_none {
    ($srcLoad:ident, 
     $dstLoad:ident) => { }
}

macro_rules! aloe_load_dest {
    ($srcLoad:ident, 
     $dstLoad:ident) => {
        /*
                const Mode::ParallelType d = dstLoad (dest);
        */
    }
}

macro_rules! aloe_load_src {
    ($srcLoad:ident, 
     $dstLoad:ident) => {
        /*
                const Mode::ParallelType s = srcLoad (src);
        */
    }
}

macro_rules! aloe_load_src1_src2 {
    ($src1Load:ident, 
     $src2Load:ident) => {
        /*
                const Mode::ParallelType s1 = src1Load (src1), s2 = src2Load (src2);
        */
    }
}

macro_rules! aloe_load_src1_src2_dest {
    ($src1Load:ident, 
     $src2Load:ident, $dstLoad:ident) => {
        /*
                const Mode::ParallelType d = dstLoad (dest), s1 = src1Load (src1), s2 = src2Load (src2);
        */
    }
}

macro_rules! aloe_load_src_dest {
    ($srcLoad:ident, 
     $dstLoad:ident) => {
        /*
                const Mode::ParallelType d = dstLoad (dest), s = srcLoad (src);
        */
    }
}
