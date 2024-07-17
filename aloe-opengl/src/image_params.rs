crate::ix!();

pub struct ImageParams<'a> {
    image_texture: OpenGLShaderProgramUniform<'a>,
    matrix:        OpenGLShaderProgramUniform<'a>,
    image_limits:  OpenGLShaderProgramUniform<'a>,
}

impl<'a> ImageParams<'a> {

    pub fn new(program: &mut OpenGLShaderProgram) -> Self {
    
        todo!();
        /*


            : imageTexture (program, "imageTexture"),
                  matrix (program, "matrix"),
                  imageLimits (program, "imageLimits")
        */
    }
    
    pub fn set_matrix(&self, 
        trans:                  &AffineTransform,
        image_width:            i32,
        image_height:           i32,
        full_width_proportion:  f32,
        full_height_proportion: f32,
        targetx:                f32,
        targety:                f32,
        is_for_tiling:          bool)  {
        
        todo!();
        /*
            auto t = trans.translated (-targetX, -targetY)
                              .inverted().scaled (fullWidthProportion  / (float) imageWidth,
                                                  fullHeightProportion / (float) imageHeight);

                const GLfloat m[] = { t.mat00, t.mat01, t.mat02, t.mat10, t.mat11, t.mat12 };
                matrix.set (m, 6);

                if (isForTiling)
                {
                    fullWidthProportion  -= 0.5f / (float) imageWidth;
                    fullHeightProportion -= 0.5f / (float) imageHeight;
                }

                imageLimits.set (fullWidthProportion, fullHeightProportion);
        */
    }
    
    pub fn set_matrix_with_texture_info(
        &self, 
        trans:         &AffineTransform,
        texture_info:  &TextureInfo,
        targetx:       f32,
        targety:       f32,
        is_for_tiling: bool)  {
        
        todo!();
        /*
            setMatrix (trans,
                           textureInfo.imageWidth, textureInfo.imageHeight,
                           textureInfo.fullWidthProportion, textureInfo.fullHeightProportion,
                           targetX, targetY, isForTiling);
        */
    }
}
