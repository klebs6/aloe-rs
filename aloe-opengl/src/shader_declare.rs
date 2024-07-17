crate::ix!();

macro_rules! aloe_declare_varying_colour {
    () => {
        /*
                "varying " ALOE_MEDIUMP " vec4 frontColour;"
        */
    }
}

macro_rules! aloe_declare_varying_pixelpos {
    () => {
        /*
                "varying " ALOE_HIGHP " vec2 pixelPos;"
        */
    }
}

macro_rules! aloe_declare_mask_uniforms {
    () => {
        /*
                "uniform sampler2D maskTexture;" 
                                                "uniform ivec4 maskBounds;"
        */
    }
}

macro_rules! aloe_fragcoord_to_mask_pos {
    () => {
        /*
                "vec2 ((pixelPos.x - float (maskBounds.x)) / float (maskBounds.z)," 
                                                      "1.0 - (pixelPos.y - float (maskBounds.y)) / float (maskBounds.w))"
        */
    }
}

macro_rules! aloe_get_mask_alpha {
    () => {
        /*
                "texture2D (maskTexture, " ALOE_FRAGCOORD_TO_MASK_POS ").a"
        */
    }
}

macro_rules! aloe_declare_matrix_uniform {
    () => {
        /*
                "uniform " ALOE_HIGHP " float matrix[6];"
        */
    }
}

macro_rules! aloe_declare_radial_uniforms {
    () => {
        /*
                "uniform sampler2D gradientTexture;" ALOE_DECLARE_MATRIX_UNIFORM
        */
    }
}

macro_rules! aloe_matrix_times_fragcoord {
    () => {
        /*
                "(mat2 (matrix[0], matrix[3], matrix[1], matrix[4]) * pixelPos" 
                                                  " + vec2 (matrix[2], matrix[5]))"
        */
    }
}

macro_rules! aloe_get_texture_colour {
    () => {
        /*
                "(frontColour.a * texture2D (gradientTexture, vec2 (gradientPos, 0.5)))"
        */
    }
}

macro_rules! aloe_declare_linear_uniforms {
    () => {
        /*
                "uniform sampler2D gradientTexture;" 
                                                  "uniform " ALOE_MEDIUMP " vec4 gradientInfo;" 
                                                  ALOE_DECLARE_VARYING_COLOUR ALOE_DECLARE_VARYING_PIXELPOS
        */
    }
}

macro_rules! aloe_calc_linear_grad_pos1 {
    () => {
        /*
                ALOE_MEDIUMP " float gradientPos = (pixelPos.y - (gradientInfo.y + (gradientInfo.z * (pixelPos.x - gradientInfo.x)))) / gradientInfo.w;"
        */
    }
}

macro_rules! aloe_calc_linear_grad_pos2 {
    () => {
        /*
                ALOE_MEDIUMP " float gradientPos = (pixelPos.x - (gradientInfo.x + (gradientInfo.z * (pixelPos.y - gradientInfo.y)))) / gradientInfo.w;"
        */
    }
}

macro_rules! aloe_declare_image_uniforms {
    () => {
        /*
                "uniform sampler2D imageTexture;" 
                                                "uniform " ALOE_MEDIUMP " vec2 imageLimits;" 
                                                ALOE_DECLARE_MATRIX_UNIFORM ALOE_DECLARE_VARYING_COLOUR ALOE_DECLARE_VARYING_PIXELPOS
        */
    }
}

macro_rules! aloe_get_image_pixel {
    () => {
        /*
                "texture2D (imageTexture, vec2 (texturePos.x, 1.0 - texturePos.y))"
        */
    }
}

macro_rules! aloe_clamp_texture_coord {
    () => {
        /*
                ALOE_HIGHP " vec2 texturePos = clamp (" ALOE_MATRIX_TIMES_FRAGCOORD ", vec2 (0, 0), imageLimits);"
        */
    }
}

macro_rules! aloe_mod_texture_coord {
    () => {
        /*
                ALOE_HIGHP " vec2 texturePos = mod (" ALOE_MATRIX_TIMES_FRAGCOORD ", imageLimits);"
        */
    }
}
