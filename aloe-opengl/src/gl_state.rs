crate::ix!();

pub struct GLState<'a> {
    target:                       RenderingTarget<'a>,
    blend_mode:                   BlendingMode,
    active_textures:              ActiveTextures<'a>,
    texture_cache:                TextureCache<'a>,
    current_shader:               CurrentShader<'a>,
    shader_quad_queue:            ShaderQuadQueue<'a>,
    cached_image_list:            CachedImageListPtr<'a>,
    previous_frame_buffer_target: u32,
}

impl<'a> Drop for GLState<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            flush();
            target.context.extensions.glBindFramebuffer (GL_FRAMEBUFFER, previousFrameBufferTarget);
        */
    }
}

impl<'a> GLState<'a> {

    pub fn new(t: &RenderingTarget) -> Self {
    
        todo!();
        /*
            : target (t),
              activeTextures (t.context),
              currentShader (t.context),
              shaderQuadQueue (t.context),
              previousFrameBufferTarget (OpenGLFrameBuffer::getCurrentFrameBufferTarget())
            // This object can only be created and used when the current thread has an active OpenGL context.
            jassert (OpenGLHelpers::isContextActive());

            ALOE_CHECK_OPENGL_ERROR
            target.makeActive();
            blendMode.resync();
            ALOE_CHECK_OPENGL_ERROR
            activeTextures.clear();
            shaderQuadQueue.initialise();
            cachedImageList = CachedImageList::get (t.context);
            ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            shaderQuadQueue.flush();
            currentShader.clearShader (shaderQuadQueue);
            ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn set_shader(&mut self, shader: &mut ShaderBase)  {
        
        todo!();
        /*
            currentShader.setShader (target, shaderQuadQueue, shader);
            ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn set_shader_for_gradient_fill(&mut self, 
        g:              &ColourGradient,
        transform:      &AffineTransform,
        mask_textureid: i32,
        mask_area:      *const Rectangle<i32>)  {
        
        todo!();
        /*
            ALOE_CHECK_OPENGL_ERROR
            activeTextures.disableTextures (shaderQuadQueue);
            blendMode.setPremultipliedBlendingMode (shaderQuadQueue);
            ALOE_CHECK_OPENGL_ERROR

            if (maskArea != nullptr)
            {
                activeTextures.setTexturesEnabled (shaderQuadQueue, 3);
                activeTextures.setActiveTexture (1);
                activeTextures.bindTexture ((GLuint) maskTextureID);
                activeTextures.setActiveTexture (0);
                textureCache.bindTextureForGradient (activeTextures, g);
            }
            else
            {
                activeTextures.setSingleTextureMode (shaderQuadQueue);
                textureCache.bindTextureForGradient (activeTextures, g);
            }

            auto t = transform.translated (0.5f - (float) target.bounds.getX(),
                                           0.5f - (float) target.bounds.getY());
            auto p1 = g.point1.transformedBy (t);
            auto p2 = g.point2.transformedBy (t);
            auto p3 = Point<float> (g.point1.x + (g.point2.y - g.point1.y),
                                    g.point1.y - (g.point2.x - g.point1.x)).transformedBy (t);

            auto programs = currentShader.programs;
            const ShaderPrograms::MaskedShaderParams* maskParams = nullptr;

            if (g.isRadial)
            {
                ShaderPrograms::RadialGradientParams* gradientParams;

                if (maskArea == nullptr)
                {
                    setShader (programs->radialGradient);
                    gradientParams = &programs->radialGradient.gradientParams;
                }
                else
                {
                    setShader (programs->radialGradientMasked);
                    gradientParams = &programs->radialGradientMasked.gradientParams;
                    maskParams = &programs->radialGradientMasked.maskParams;
                }

                gradientParams->setMatrix (p1, p2, p3);
            }
            else
            {
                p1 = Line<float> (p1, p3).findNearestPointTo (p2);
                const Point<float> delta (p2.x - p1.x, p1.y - p2.y);
                const ShaderPrograms::LinearGradientParams* gradientParams;
                float grad, length;

                if (std::abs (delta.x) < std::abs (delta.y))
                {
                    if (maskArea == nullptr)
                    {
                        setShader (programs->linearGradient1);
                        gradientParams = &(programs->linearGradient1.gradientParams);
                    }
                    else
                    {
                        setShader (programs->linearGradient1Masked);
                        gradientParams = &(programs->linearGradient1Masked.gradientParams);
                        maskParams = &programs->linearGradient1Masked.maskParams;
                    }

                    grad = delta.x / delta.y;
                    length = (p2.y - grad * p2.x) - (p1.y - grad * p1.x);
                }
                else
                {
                    if (maskArea == nullptr)
                    {
                        setShader (programs->linearGradient2);
                        gradientParams = &(programs->linearGradient2.gradientParams);
                    }
                    else
                    {
                        setShader (programs->linearGradient2Masked);
                        gradientParams = &(programs->linearGradient2Masked.gradientParams);
                        maskParams = &programs->linearGradient2Masked.maskParams;
                    }

                    grad = delta.y / delta.x;
                    length = (p2.x - grad * p2.y) - (p1.x - grad * p1.y);
                }

                gradientParams->gradientInfo.set (p1.x, p1.y, grad, length);
            }

            if (maskParams != nullptr)
                maskParams->setBounds (*maskArea, target, 1);

            ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn set_shader_for_tiled_image_fill(&mut self, 
        texture_info:   &TextureInfo,
        transform:      &AffineTransform,
        mask_textureid: i32,
        mask_area:      *const Rectangle<i32>,
        is_tiled_fill:  bool)  {
        
        todo!();
        /*
            blendMode.setPremultipliedBlendingMode (shaderQuadQueue);

            auto programs = currentShader.programs;

            const ShaderPrograms::MaskedShaderParams* maskParams = nullptr;
            const ShaderPrograms::ImageParams* imageParams;

            if (maskArea != nullptr)
            {
                activeTextures.setTwoTextureMode (shaderQuadQueue, textureInfo.textureID, (GLuint) maskTextureID);

                if (isTiledFill)
                {
                    setShader (programs->tiledImageMasked);
                    imageParams = &programs->tiledImageMasked.imageParams;
                    maskParams  = &programs->tiledImageMasked.maskParams;
                }
                else
                {
                    setShader (programs->imageMasked);
                    imageParams = &programs->imageMasked.imageParams;
                    maskParams  = &programs->imageMasked.maskParams;
                }
            }
            else
            {
                activeTextures.setSingleTextureMode (shaderQuadQueue);
                activeTextures.bindTexture (textureInfo.textureID);

                if (isTiledFill)
                {
                    setShader (programs->tiledImage);
                    imageParams = &programs->tiledImage.imageParams;
                }
                else
                {
                    setShader (programs->image);
                    imageParams = &programs->image.imageParams;
                }
            }

            imageParams->setMatrix (transform, textureInfo, (float) target.bounds.getX(), (float) target.bounds.getY(), isTiledFill);

            if (maskParams != nullptr)
                maskParams->setBounds (*maskArea, target, 1);
        */
    }
}
