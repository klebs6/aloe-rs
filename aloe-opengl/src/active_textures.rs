crate::ix!();

pub const NUM_ACTIVE_TEXTURES: usize = 3;

pub struct ActiveTextures<'a> {
    current_textureid:      [GLuint; NUM_ACTIVE_TEXTURES],
    textures_enabled:       i32, // default = 0
    current_active_texture: i32, // default = -1
    context:                &'a OpenGLContext<'a>,
}

impl<'a> ActiveTextures<'a> {

    pub fn new(c: &OpenGLContext) -> Self {
    
        todo!();
        /*
        : context(c),

        
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            zeromem (currentTextureID, sizeof (currentTextureID));
        */
    }
    
    
    pub fn set_textures_enabled<QuadQueueType>(&mut self, 
        quad_queue:         &mut QuadQueueType,
        texture_index_mask: i32)  {
    
        todo!();
        /*
            if (texturesEnabled != textureIndexMask)
                {
                    quadQueue.flush();

                    for (int i = 3; --i >= 0;)
                    {
                        if ((texturesEnabled & (1 << i)) != (textureIndexMask & (1 << i)))
                        {
                            setActiveTexture (i);
                            ALOE_CHECK_OPENGL_ERROR

                           #if ! ALOE_ANDROID
                            if ((textureIndexMask & (1 << i)) != 0)
                                glEnable (GL_TEXTURE_2D);
                            else
                            {
                                glDisable (GL_TEXTURE_2D);
                                currentTextureID[i] = 0;
                            }

                            clearGLError();
                           #endif
                        }
                    }

                    texturesEnabled = textureIndexMask;
                }
        */
    }
    
    pub fn disable_textures<QuadQueueType>(&mut self, quad_queue: &mut QuadQueueType)  {
    
        todo!();
        /*
            setTexturesEnabled (quadQueue, 0);
        */
    }
    
    
    pub fn set_single_texture_mode<QuadQueueType>(&mut self, quad_queue: &mut QuadQueueType)  {
    
        todo!();
        /*
            setTexturesEnabled (quadQueue, 1);
                setActiveTexture (0);
        */
    }
    
    
    pub fn set_two_texture_mode<QuadQueueType>(&mut self, 
        quad_queue: &mut QuadQueueType,
        texture1:   GLuint,
        texture2:   GLuint)  {
    
        todo!();
        /*
            ALOE_CHECK_OPENGL_ERROR
                setTexturesEnabled (quadQueue, 3);

                if (currentActiveTexture == 0)
                {
                    bindTexture (texture1);
                    setActiveTexture (1);
                    bindTexture (texture2);
                }
                else
                {
                    setActiveTexture (1);
                    bindTexture (texture2);
                    setActiveTexture (0);
                    bindTexture (texture1);
                }

                ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn set_active_texture(&mut self, index: i32)  {
        
        todo!();
        /*
            if (currentActiveTexture != index)
                {
                    currentActiveTexture = index;
                    context.extensions.glActiveTexture (GL_TEXTURE0 + (GLenum) index);
                    ALOE_CHECK_OPENGL_ERROR
                }
        */
    }
    
    pub fn bind_texture(&mut self, textureid: GLuint)  {
        
        todo!();
        /*
            if (currentActiveTexture < 0 || numTextures <= currentActiveTexture)
                {
                    jassertfalse;
                    return;
                }

                if (currentTextureID[currentActiveTexture] != textureID)
                {
                    currentTextureID[currentActiveTexture] = textureID;
                    glBindTexture (GL_TEXTURE_2D, textureID);
                    ALOE_CHECK_OPENGL_ERROR
                }
                else
                {
                   #if ALOE_DEBUG
                    GLint t = 0;
                    glGetIntegerv (GL_TEXTURE_BINDING_2D, &t);
                    jassert (t == (GLint) textureID);
                   #endif
                }
        */
    }
    
    pub fn assign_from(&mut self, _0: &ActiveTextures) -> &mut ActiveTextures {
        
        todo!();
        /*
        
        */
    }
}
