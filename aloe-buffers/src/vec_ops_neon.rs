crate::ix!();

#[cfg(ALOE_USE_ARM_NEON)]
macro_rules! aloe_begin_vec_op {
    () => {
        /*
        
                using Mode = FloatVectorHelpers::ModeType<sizeof(*dest)>::Mode; 
                if (Mode::numParallel > 1) 
                { 
                    const int numLongOps = num / Mode::numParallel;
        */
    }
}

#[cfg(ALOE_USE_ARM_NEON)]
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

#[cfg(ALOE_USE_ARM_NEON)]
macro_rules! aloe_perform_vec_op_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                ALOE_VEC_LOOP (vecOp, dummy, Mode::loadU, Mode::storeU, locals, ALOE_INCREMENT_DEST) 
                ALOE_FINISH_VEC_OP (normalOp)
        */
    }
}

#[cfg(ALOE_USE_ARM_NEON)]
macro_rules! aloe_perform_vec_op_src_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                ALOE_VEC_LOOP (vecOp, Mode::loadU, Mode::loadU, Mode::storeU, locals, increment) 
                ALOE_FINISH_VEC_OP (normalOp)
        */
    }
}

#[cfg(ALOE_USE_ARM_NEON)]
macro_rules! aloe_perform_vec_op_src1_src2_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                ALOE_VEC_LOOP_TWO_SOURCES (vecOp, Mode::loadU, Mode::loadU, Mode::storeU, locals, increment) 
                ALOE_FINISH_VEC_OP (normalOp)
        */
    }
}

#[cfg(ALOE_USE_ARM_NEON)]
macro_rules! aloe_perform_vec_op_src1_src2_dest_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                ALOE_BEGIN_VEC_OP 
                setupOp 
                ALOE_VEC_LOOP_TWO_SOURCES_WITH_DEST_LOAD (vecOp, Mode::loadU, Mode::loadU, Mode::loadU, Mode::storeU, locals, increment) 
                ALOE_FINISH_VEC_OP (normalOp)
        */
    }
}

///---------------------------------
#[cfg(not(any(ALOE_USE_ARM_NEON,ALOE_USE_SSE_INTRINSICS)))]
macro_rules! aloe_perform_vec_op_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $setupOp:ident) => {
        /*
        
                for (int i = 0; i < num; ++i) normalOp;
        */
    }
}

#[cfg(not(any(ALOE_USE_ARM_NEON,ALOE_USE_SSE_INTRINSICS)))]
macro_rules! aloe_perform_vec_op_src_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                for (int i = 0; i < num; ++i) normalOp;
        */
    }
}

#[cfg(not(any(ALOE_USE_ARM_NEON,ALOE_USE_SSE_INTRINSICS)))]
macro_rules! aloe_perform_vec_op_src1_src2_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                for (int i = 0; i < num; ++i) normalOp;
        */
    }
}

#[cfg(not(any(ALOE_USE_ARM_NEON,ALOE_USE_SSE_INTRINSICS)))]
macro_rules! aloe_perform_vec_op_src1_src2_dest_dest {
    ($normalOp:ident, 
     $vecOp:ident, 
     $locals:ident, 
     $increment:ident, 
     $setupOp:ident) => {
        /*
        
                for (int i = 0; i < num; ++i) normalOp;
        */
    }
}

