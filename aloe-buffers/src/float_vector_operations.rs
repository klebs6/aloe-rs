crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/buffers/aloe_FloatVectorOperations.h]

pub struct FloatVectorOperations {

}

/**
  | A collection of simple vector operations on
  | arrays of floats, accelerated with SIMD
  | instructions where possible.
  |
  | @tags{Audio}
  */
impl FloatVectorOperations {

    /**
      | Clears a vector of floats.
      |
      */
    pub fn clear_f32(
        &mut self, 
        dest: *mut f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vclr (dest, 1, (size_t) num);
       #else
        zeromem (dest, (size_t) num * sizeof (float));
       #endif
        */
    }
    
    /**
      | Clears a vector of doubles.
      |
      */
    pub fn clear_f64(
        &mut self, 
        dest: *mut f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vclrD (dest, 1, (size_t) num);
       #else
        zeromem (dest, (size_t) num * sizeof (double));
       #endif
        */
    }
    
    /**
      | Copies a repeated value into a vector
      | of floats.
      |
      */
    pub fn fill_f32(
        &mut self, 
        dest:          *mut f32,
        value_to_fill: f32,
        num:           i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vfill (&valueToFill, dest, 1, (size_t) num);
       #else
        ALOE_PERFORM_VEC_OP_DEST (dest[i] = valueToFill, val, ALOE_LOAD_NONE,
                                  const Mode::ParallelType val = Mode::load1 (valueToFill);)
       #endif
        */
    }
    
    /**
      | Copies a repeated value into a vector
      | of doubles.
      |
      */
    pub fn fill_f64(
        &mut self, 
        dest:          *mut f64,
        value_to_fill: f64,
        num:           i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vfillD (&valueToFill, dest, 1, (size_t) num);
       #else
        ALOE_PERFORM_VEC_OP_DEST (dest[i] = valueToFill, val, ALOE_LOAD_NONE,
                                  const Mode::ParallelType val = Mode::load1 (valueToFill);)
       #endif
        */
    }
    
    /**
      | Copies a vector of floats.
      |
      */
    pub fn copy_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        num:  i32)  {
        
        todo!();
        /*
            memcpy (dest, src, (size_t) num * sizeof (float));
        */
    }
    
    /**
      | Copies a vector of doubles.
      |
      */
    pub fn copy_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        num:  i32)  {
        
        todo!();
        /*
            memcpy (dest, src, (size_t) num * sizeof (double));
        */
    }
    
    /**
      | Copies a vector of floats, multiplying
      | each value by a given multiplier
      |
      */
    pub fn copy_with_multiply_f32(
        &mut self, 
        dest:       *mut f32,
        src:        *const f32,
        multiplier: f32,
        num:        i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsmul (src, 1, &multiplier, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = src[i] * multiplier, Mode::mul (mult, s),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
       #endif
        */
    }
    
    /**
      | Copies a vector of doubles, multiplying
      | each value by a given multiplier
      |
      */
    pub fn copy_with_multiply_f64(
        &mut self, 
        dest:       *mut f64,
        src:        *const f64,
        multiplier: f64,
        num:        i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsmulD (src, 1, &multiplier, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = src[i] * multiplier, Mode::mul (mult, s),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
       #endif
        */
    }
    
    /**
      | Adds a fixed value to the destination
      | values.
      |
      */
    pub fn add_f32(
        &mut self, 
        dest:   *mut f32,
        amount: f32,
        num:    i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsadd (dest, 1, &amount, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_DEST (dest[i] += amount, Mode::add (d, amountToAdd), ALOE_LOAD_DEST,
                                  const Mode::ParallelType amountToAdd = Mode::load1 (amount);)
       #endif
        */
    }
    
    /**
      | Adds a fixed value to the destination
      | values.
      |
      */
    pub fn add_f64(
        &mut self, 
        dest:   *mut f64,
        amount: f64,
        num:    i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_DEST (dest[i] += amount, Mode::add (d, amountToAdd), ALOE_LOAD_DEST,
                                  const Mode::ParallelType amountToAdd = Mode::load1 (amount);)
        */
    }
    
    /**
      | Adds a fixed value to each source value
      | and stores it in the destination array.
      |
      */
    pub fn add_fixed_value_to_each_source_value_and_store_in_dest(
        &mut self, 
        dest:   *mut f32,
        src:    *const f32,
        amount: f32,
        num:    i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsadd (osx108sdkCompatibilityCast (src), 1, &amount, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = src[i] + amount, Mode::add (am, s),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType am = Mode::load1 (amount);)
       #endif
        */
    }
    
    /**
      | Adds a fixed value to each source value
      | and stores it in the destination array.
      |
      */
    pub fn add_fixed_f64_into_dest(
        &mut self, 
        dest:   *mut f64,
        src:    *const f64,
        amount: f64,
        num:    i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsaddD (osx108sdkCompatibilityCast (src), 1, &amount, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = src[i] + amount, Mode::add (am, s),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType am = Mode::load1 (amount);)
       #endif
        */
    }
    
    /**
      | Adds the source values to the destination
      | values.
      |
      */
    pub fn add_f32_into_dest(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vadd (src, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] += src[i], Mode::add (d, s), ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST, )
       #endif
        */
    }
    
    /**
      | Adds the source values to the destination
      | values.
      |
      */
    pub fn add_f64_into_dest(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vaddD (src, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] += src[i], Mode::add (d, s), ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST, )
       #endif
        */
    }
    
    /**
      | Adds each source1 value to the corresponding source2
      | value and stores the result in the destination
      | array.
      */
    pub fn add_f32_pairwise(
        &mut self, 
        dest: *mut f32,
        src1: *const f32,
        src2: *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vadd (src1, 1, src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = src1[i] + src2[i], Mode::add (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Adds each source1 value to the corresponding source2
      | value and stores the result in the destination
      | array.
      */
    pub fn add_f64_pairwise(
        &mut self, 
        dest: *mut f64,
        src1: *const f64,
        src2: *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vaddD (src1, 1, src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = src1[i] + src2[i], Mode::add (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Subtracts the source values from the
      | destination values.
      |
      */
    pub fn subtract_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsub (src, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] -= src[i], Mode::sub (d, s), ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST, )
       #endif
        */
    }
    
    /**
      | Subtracts the source values from the
      | destination values.
      |
      */
    pub fn subtract_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        num:  i32
    ) 
    {
        todo!();

        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsubD (src, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] -= src[i], Mode::sub (d, s), ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST, )
       #endif
        */
    }
    
    /**
      | Subtracts each source2 value from the corresponding
      | source1 value and stores the result in the destination
      | array.
      */
    pub fn subtract_pairwise_f32(
        &mut self, 
        dest: *mut f32,
        src1: *const f32,
        src2: *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsub (src2, 1, src1, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = src1[i] - src2[i], Mode::sub (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Subtracts each source2 value from the corresponding
      | source1 value and stores the result in the destination
      | array.
      */
    pub fn subtract_pairwise_f64(
        &mut self, 
        dest: *mut f64,
        src1: *const f64,
        src2: *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsubD (src2, 1, src1, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = src1[i] - src2[i], Mode::sub (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Multiplies each source value by the
      | given multiplier, then adds it to the
      | destination value.
      |
      */
    pub fn add_with_multiply_f32(
        &mut self, 
        dest:       *mut f32,
        src:        *const f32,
        multiplier: f32,
        num:        i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsma (src, 1, &multiplier, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] += src[i] * multiplier, Mode::add (d, Mode::mul (mult, s)),
                                      ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
       #endif
        */
    }
    
    /**
      | Multiplies each source value by the
      | given multiplier, then adds it to the
      | destination value.
      |
      */
    pub fn add_with_multiply_f64(
        &mut self, 
        dest:       *mut f64,
        src:        *const f64,
        multiplier: f64,
        num:        i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsmaD (src, 1, &multiplier, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] += src[i] * multiplier, Mode::add (d, Mode::mul (mult, s)),
                                      ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
       #endif
        */
    }
    
    /**
      | Multiplies each source1 value by the corresponding
      | source2 value, then adds it to the destination
      | value.
      */
    pub fn add_with_multiply_pairwise_f32(
        &mut self, 
        dest: *mut f32,
        src1: *const f32,
        src2: *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vma ((float*) src1, 1, (float*) src2, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST_DEST (dest[i] += src1[i] * src2[i], Mode::add (d, Mode::mul (s1, s2)),
                                                 ALOE_LOAD_SRC1_SRC2_DEST,
                                                 ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Multiplies each source1 value by the corresponding
      | source2 value, then adds it to the destination
      | value.
      */
    pub fn add_with_multiply_pairwise_f64(
        &mut self, 
        dest: *mut f64,
        src1: *const f64,
        src2: *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmaD ((double*) src1, 1, (double*) src2, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST_DEST (dest[i] += src1[i] * src2[i], Mode::add (d, Mode::mul (s1, s2)),
                                                 ALOE_LOAD_SRC1_SRC2_DEST,
                                                 ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Multiplies each source value by the
      | given multiplier, then subtracts it
      | to the destination value.
      |
      */
    pub fn subtract_with_multiply_f32(
        &mut self, 
        dest:       *mut f32,
        src:        *const f32,
        multiplier: f32,
        num:        i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] -= src[i] * multiplier, Mode::sub (d, Mode::mul (mult, s)),
                                      ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
        */
    }
    
    /**
      | Multiplies each source value by the
      | given multiplier, then subtracts it
      | to the destination value.
      |
      */
    pub fn subtract_with_multiply_f64(
        &mut self, 
        dest:       *mut f64,
        src:        *const f64,
        multiplier: f64,
        num:        i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] -= src[i] * multiplier, Mode::sub (d, Mode::mul (mult, s)),
                                      ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
        */
    }
    
    /**
      | Multiplies each source1 value by the corresponding
      | source2 value, then subtracts it to the destination
      | value.
      */
    pub fn subtract_with_multiply_pairwise_f32(
        &mut self, 
        dest: *mut f32,
        src1: *const f32,
        src2: *const f32,
        num:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST_DEST (dest[i] -= src1[i] * src2[i], Mode::sub (d, Mode::mul (s1, s2)),
                                                 ALOE_LOAD_SRC1_SRC2_DEST,
                                                 ALOE_INCREMENT_SRC1_SRC2_DEST, )
        */
    }
    
    /**
      | Multiplies each source1 value by the corresponding
      | source2 value, then subtracts it to the destination
      | value.
      */
    pub fn subtract_with_multiply_pairwise_f64(
        &mut self, 
        dest: *mut f64,
        src1: *const f64,
        src2: *const f64,
        num:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST_DEST (dest[i] -= src1[i] * src2[i], Mode::sub (d, Mode::mul (s1, s2)),
                                                 ALOE_LOAD_SRC1_SRC2_DEST,
                                                 ALOE_INCREMENT_SRC1_SRC2_DEST, )
        */
    }
    
    /**
      | Multiplies the destination values
      | by the source values.
      |
      */
    pub fn multiply_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmul (src, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] *= src[i], Mode::mul (d, s), ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST, )
       #endif
        */
    }
    
    /**
      | Multiplies the destination values
      | by the source values.
      |
      */
    pub fn multiply_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmulD (src, 1, dest, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] *= src[i], Mode::mul (d, s), ALOE_LOAD_SRC_DEST, ALOE_INCREMENT_SRC_DEST, )
       #endif
        */
    }
    
    /**
      | Multiplies each source1 value by the correspinding
      | source2 value, then stores it in the destination
      | array.
      */
    pub fn multiply_pairwise_f32(
        &mut self, 
        dest: *mut f32,
        src1: *const f32,
        src2: *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmul (src1, 1, src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = src1[i] * src2[i], Mode::mul (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Multiplies each source1 value by the correspinding
      | source2 value, then stores it in the destination
      | array.
      */
    pub fn multiply_pairwise_f64(
        &mut self, 
        dest: *mut f64,
        src1: *const f64,
        src2: *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmulD (src1, 1, src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = src1[i] * src2[i], Mode::mul (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Multiplies each of the destination
      | values by a fixed multiplier.
      |
      */
    pub fn multiply_fixed_f32(
        &mut self, 
        dest:       *mut f32,
        multiplier: f32,
        num:        i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsmul (dest, 1, &multiplier, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_DEST (dest[i] *= multiplier, Mode::mul (d, mult), ALOE_LOAD_DEST,
                                  const Mode::ParallelType mult = Mode::load1 (multiplier);)
       #endif
        */
    }
    
    /**
      | Multiplies each of the destination
      | values by a fixed multiplier.
      |
      */
    pub fn multiply_fixed_f64(
        &mut self, 
        dest:       *mut f64,
        multiplier: f64,
        num:        i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vsmulD (dest, 1, &multiplier, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_DEST (dest[i] *= multiplier, Mode::mul (d, mult), ALOE_LOAD_DEST,
                                  const Mode::ParallelType mult = Mode::load1 (multiplier);)
       #endif
        */
    }
    
    /**
      | Multiplies each of the source values by a fixed
      | multiplier and stores the result in the destination
      | array.
      */
    pub fn multiply_fixed_f32_into_dest(
        &mut self, 
        dest:       *mut f32,
        src:        *const f32,
        multiplier: f32,
        num:        i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = src[i] * multiplier, Mode::mul (mult, s),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
        */
    }
    
    /**
      | Multiplies each of the source values by a fixed
      | multiplier and stores the result in the destination
      | array.
      */
    pub fn multiply_fixed_f64_into_dest(
        &mut self, 
        dest:       *mut f64,
        src:        *const f64,
        multiplier: f64,
        num:        i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = src[i] * multiplier, Mode::mul (mult, s),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
        */
    }
    
    /**
      | Copies a source vector to a destination,
      | negating each value.
      |
      */
    pub fn negate_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vneg ((float*) src, 1, dest, 1, (vDSP_Length) num);
       #else
        copyWithMultiply (dest, src, -1.0f, num);
       #endif
        */
    }
    
    /**
      | Copies a source vector to a destination,
      | negating each value.
      |
      */
    pub fn negate_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vnegD ((double*) src, 1, dest, 1, (vDSP_Length) num);
       #else
        copyWithMultiply (dest, src, -1.0f, num);
       #endif
        */
    }
    
    /**
      | Copies a source vector to a destination,
      | taking the absolute of each value.
      |
      */
    pub fn abs_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vabs ((float*) src, 1, dest, 1, (vDSP_Length) num);
       #else
        FloatVectorHelpers::signMask32 signMask;
        signMask.i = 0x7fffffffUL;
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = std::abs (src[i]), Mode::bit_and (s, mask),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mask = Mode::load1 (signMask.f);)

        ignoreUnused (signMask);
       #endif
        */
    }
    
    /**
      | Copies a source vector to a destination,
      | taking the absolute of each value.
      |
      */
    pub fn abs_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vabsD ((double*) src, 1, dest, 1, (vDSP_Length) num);
       #else
        FloatVectorHelpers::signMask64 signMask;
        signMask.i = 0x7fffffffffffffffULL;

        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = std::abs (src[i]), Mode::bit_and (s, mask),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mask = Mode::load1 (signMask.d);)

        ignoreUnused (signMask);
       #endif
        */
    }
    
    /**
      | Converts a stream of integers to floats,
      | multiplying each one by the given multiplier.
      |
      */
    pub fn convert_fixed_to_float(&mut self, 
        dest:       *mut f32,
        src:        *const i32,
        multiplier: f32,
        num:        i32)  {
        
        todo!();
        /*
            #if ALOE_USE_ARM_NEON
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = (float) src[i] * multiplier,
                                      vmulq_n_f32 (vcvtq_f32_s32 (vld1q_s32 (src)), multiplier),
                                      ALOE_LOAD_NONE, ALOE_INCREMENT_SRC_DEST, )
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = (float) src[i] * multiplier,
                                      Mode::mul (mult, _mm_cvtepi32_ps (_mm_loadu_si128 (reinterpret_cast<const __m128i*> (src)))),
                                      ALOE_LOAD_NONE, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType mult = Mode::load1 (multiplier);)
       #endif
        */
    }
    
    /**
      | Each element of dest will be the minimum of the
      | corresponding element of the source array and the
      | given comp value.
      */
    pub fn min_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        comp: f32,
        num:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = jmin (src[i], comp), Mode::min (s, cmp),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType cmp = Mode::load1 (comp);)
        */
    }
    
    /**
      | Each element of dest will be the minimum of the
      | corresponding element of the source array and the
      | given comp value.
      */
    pub fn min_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        comp: f64,
        num:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = jmin (src[i], comp), Mode::min (s, cmp),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType cmp = Mode::load1 (comp);)
        */
    }
    
    /**
      | Each element of dest will be the minimum
      | of the corresponding source1 and source2
      | values.
      |
      */
    pub fn min_pairwise_f32(
        &mut self, 
        dest: *mut f32,
        src1: *const f32,
        src2: *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmin ((float*) src1, 1, (float*) src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = jmin (src1[i], src2[i]), Mode::min (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Each element of dest will be the minimum
      | of the corresponding source1 and source2
      | values.
      |
      */
    pub fn min_pairwise_f64(
        &mut self, 
        dest: *mut f64,
        src1: *const f64,
        src2: *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vminD ((double*) src1, 1, (double*) src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = jmin (src1[i], src2[i]), Mode::min (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Each element of dest will be the maximum of the
      | corresponding element of the source array and the
      | given comp value.
      */
    pub fn max_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        comp: f32,
        num:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = jmax (src[i], comp), Mode::max (s, cmp),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType cmp = Mode::load1 (comp);)
        */
    }
    
    /**
      | Each element of dest will be the maximum of the
      | corresponding element of the source array and the
      | given comp value.
      */
    pub fn max_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        comp: f64,
        num:  i32)  {
        
        todo!();
        /*
            ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = jmax (src[i], comp), Mode::max (s, cmp),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType cmp = Mode::load1 (comp);)
        */
    }
    
    /**
      | Each element of dest will be the maximum
      | of the corresponding source1 and source2
      | values.
      |
      */
    pub fn max_pairwise_f32(
        &mut self, 
        dest: *mut f32,
        src1: *const f32,
        src2: *const f32,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmax ((float*) src1, 1, (float*) src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = jmax (src1[i], src2[i]), Mode::max (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Each element of dest will be the maximum
      | of the corresponding source1 and source2
      | values.
      |
      */
    pub fn max_pairwise_f64(
        &mut self, 
        dest: *mut f64,
        src1: *const f64,
        src2: *const f64,
        num:  i32)  {
        
        todo!();
        /*
            #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vmaxD ((double*) src1, 1, (double*) src2, 1, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC1_SRC2_DEST (dest[i] = jmax (src1[i], src2[i]), Mode::max (s1, s2), ALOE_LOAD_SRC1_SRC2, ALOE_INCREMENT_SRC1_SRC2_DEST, )
       #endif
        */
    }
    
    /**
      | Each element of dest is calculated by hard clipping
      | the corresponding src element so that it is in
      | the range specified by the arguments low and high.
      */
    pub fn clip_f32(
        &mut self, 
        dest: *mut f32,
        src:  *const f32,
        low:  f32,
        high: f32,
        num:  i32)  {
        
        todo!();
        /*
            jassert(high >= low);

       #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vclip ((float*) src, 1, &low, &high, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = jmax (jmin (src[i], high), low), Mode::max (Mode::min (s, hi), lo),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType lo = Mode::load1 (low); const Mode::ParallelType hi = Mode::load1 (high);)
       #endif
        */
    }
    
    /**
      | Each element of dest is calculated by hard clipping
      | the corresponding src element so that it is in
      | the range specified by the arguments low and high.
      */
    pub fn clip_f64(
        &mut self, 
        dest: *mut f64,
        src:  *const f64,
        low:  f64,
        high: f64,
        num:  i32)  {
        
        todo!();
        /*
            jassert(high >= low);

       #if ALOE_USE_VDSP_FRAMEWORK
        vDSP_vclipD ((double*) src, 1, &low, &high, dest, 1, (vDSP_Length) num);
       #else
        ALOE_PERFORM_VEC_OP_SRC_DEST (dest[i] = jmax (jmin (src[i], high), low), Mode::max (Mode::min (s, hi), lo),
                                      ALOE_LOAD_SRC, ALOE_INCREMENT_SRC_DEST,
                                      const Mode::ParallelType lo = Mode::load1 (low); const Mode::ParallelType hi = Mode::load1 (high);)
       #endif
        */
    }
    
    /**
      | Finds the minimum and maximum values
      | in the given array.
      |
      */
    pub fn find_min_and_max_f32(&mut self, 
        src: *const f32,
        num: i32) -> Range<f32> {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || ALOE_USE_ARM_NEON
        return FloatVectorHelpers::MinMax<FloatVectorHelpers::BasicOps32>::findMinAndMax (src, num);
       #else
        return Range<float>::findMinAndMax (src, num);
       #endif
        */
    }
    
    /**
      | Finds the minimum and maximum values
      | in the given array.
      |
      */
    pub fn find_min_and_max_f64(&mut self, 
        src: *const f64,
        num: i32) -> Range<f64> {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || ALOE_USE_ARM_NEON
        return FloatVectorHelpers::MinMax<FloatVectorHelpers::BasicOps64>::findMinAndMax (src, num);
       #else
        return Range<double>::findMinAndMax (src, num);
       #endif
        */
    }
    
    /**
      | Finds the minimum value in the given
      | array.
      |
      */
    pub fn find_minimum_f32(
        &mut self, 
        src: *const f32,
        num: i32) -> f32 {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || ALOE_USE_ARM_NEON
        return FloatVectorHelpers::MinMax<FloatVectorHelpers::BasicOps32>::findMinOrMax (src, num, true);
       #else
        return aloe::findMinimum (src, num);
       #endif
        */
    }
    
    /**
      | Finds the minimum value in the given
      | array.
      |
      */
    pub fn find_minimum_f64(
        &mut self, 
        src: *const f64,
        num: i32) -> f64 {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || ALOE_USE_ARM_NEON
        return FloatVectorHelpers::MinMax<FloatVectorHelpers::BasicOps64>::findMinOrMax (src, num, true);
       #else
        return aloe::findMinimum (src, num);
       #endif
        */
    }
    
    /**
      | Finds the maximum value in the given
      | array.
      |
      */
    pub fn find_maximum_f32(
        &mut self, 
        src: *const f32,
        num: i32) -> f32 {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || ALOE_USE_ARM_NEON
        return FloatVectorHelpers::MinMax<FloatVectorHelpers::BasicOps32>::findMinOrMax (src, num, false);
       #else
        return aloe::findMaximum (src, num);
       #endif
        */
    }
    
    /**
      | Finds the maximum value in the given
      | array.
      |
      */
    pub fn find_maximum_f64(
        &mut self, 
        src: *const f64,
        num: i32) -> f64 {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || ALOE_USE_ARM_NEON
        return FloatVectorHelpers::MinMax<FloatVectorHelpers::BasicOps64>::findMinOrMax (src, num, false);
       #else
        return aloe::findMaximum (src, num);
       #endif
        */
    }
    
    pub fn get_fp_status_register(&mut self) -> libc::intptr_t {
        
        todo!();
        /*
            intptr_t fpsr = 0;
      #if ALOE_INTEL && ALOE_USE_SSE_INTRINSICS
        fpsr = static_cast<intptr_t> (_mm_getcsr());
      #elif defined (__arm64__) || defined (__aarch64__) || ALOE_USE_ARM_NEON
       #if defined (__arm64__) || defined (__aarch64__)
        asm volatile("mrs %0, fpcr" : "=r" (fpsr));
       #elif ALOE_USE_ARM_NEON
        asm volatile("vmrs %0, fpscr" : "=r" (fpsr));
       #endif
      #else
       #if ! (defined (ALOE_INTEL) || defined (ALOE_ARM))
        jassertfalse; // No support for getting the floating point status register for your platform
       #endif
      #endif

        return fpsr;
        */
    }
    
    pub fn set_fp_status_register(&mut self, fpsr: libc::intptr_t)  {
        
        todo!();
        /*
            #if ALOE_INTEL && ALOE_USE_SSE_INTRINSICS
        auto fpsr_w = static_cast<uint32_t> (fpsr);
        _mm_setcsr (fpsr_w);
      #elif defined (__arm64__) || defined (__aarch64__) || ALOE_USE_ARM_NEON
       #if defined (__arm64__) || defined (__aarch64__)
        asm volatile("msr fpcr, %0" : : "ri" (fpsr));
       #elif ALOE_USE_ARM_NEON
        asm volatile("vmsr fpscr, %0" : : "ri" (fpsr));
       #endif
      #else
       #if ! (defined (ALOE_INTEL) || defined (ALOE_ARM))
        jassertfalse; // No support for getting the floating point status register for your platform
       #endif
        ignoreUnused (fpsr);
      #endif
        */
    }
    
    /**
      | This method enables or disables the
      | SSE/NEON flush-to-zero mode.
      |
      */
    pub fn enable_flush_to_zero_mode(&mut self, should_enable: bool)  {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || (ALOE_USE_ARM_NEON || defined (__arm64__) || defined (__aarch64__))
       #if ALOE_USE_SSE_INTRINSICS
        intptr_t mask = _MM_FLUSH_ZERO_MASK;
       #else /*ALOE_USE_ARM_NEON*/
        intptr_t mask = (1 << 24 /* FZ */);
       #endif
        setFpStatusRegister ((getFpStatusRegister() & (~mask)) | (shouldEnable ? mask : 0));
      #else
       #if ! (defined (ALOE_INTEL) || defined (ALOE_ARM))
        jassertfalse; // No support for flush to zero mode on your platform
       #endif
        ignoreUnused (shouldEnable);
      #endif
        */
    }
    
    /**
      | On Intel CPUs, this method enables the
      | SSE flush-to-zero and denormalised-are-zero
      | modes.
      | 
      | This effectively sets the DAZ and FZ
      | bits of the MXCSR register. On arm CPUs
      | this will enable flush to zero mode.
      | 
      | It's a convenient thing to call before
      | audio processing code where you really
      | want to avoid denormalisation performance
      | hits.
      |
      */
    pub fn disable_denormalised_number_support(&mut self, should_disable: Option<bool>)  {

        let should_disable: bool = should_disable.unwrap_or(true);
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || (ALOE_USE_ARM_NEON || defined (__arm64__) || defined (__aarch64__))
       #if ALOE_USE_SSE_INTRINSICS
        intptr_t mask = 0x8040;
       #else /*ALOE_USE_ARM_NEON*/
        intptr_t mask = (1 << 24 /* FZ */);
       #endif

        setFpStatusRegister ((getFpStatusRegister() & (~mask)) | (shouldDisable ? mask : 0));
      #else
        ignoreUnused (shouldDisable);

       #if ! (defined (ALOE_INTEL) || defined (ALOE_ARM))
        jassertfalse; // No support for disable denormals mode on your platform
       #endif
      #endif
        */
    }
    
    /**
      | This method returns true if denormals
      | are currently disabled.
      |
      */
    pub fn are_denormals_disabled(&mut self) -> bool {
        
        todo!();
        /*
            #if ALOE_USE_SSE_INTRINSICS || (ALOE_USE_ARM_NEON || defined (__arm64__) || defined (__aarch64__))
       #if ALOE_USE_SSE_INTRINSICS
        intptr_t mask = 0x8040;
       #else /*ALOE_USE_ARM_NEON*/
        intptr_t mask = (1 << 24 /* FZ */);
       #endif

        return ((getFpStatusRegister() & mask) == mask);
      #else
        return false;
      #endif
        */
    }
}
