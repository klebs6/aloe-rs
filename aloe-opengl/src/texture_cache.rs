crate::ix!();

pub struct TextureCache<'a> {
    textures:               Vec<Box<OpenGLTexture<'a>>>,
    gradient_textures:      Vec<Box<OpenGLTexture<'a>>>,
    active_gradient_index:  i32, // default = 0
    gradient_needs_refresh: bool, // default = true
}

pub const TEXTURE_CACHE_GRADIENT_TEXTURE_SIZE:        usize = 256;
pub const TEXTURE_CACHE_NUM_TEXTURES_TO_CACHE:         usize = 8;
pub const TEXTURE_CACHE_NUM_GRADIENT_TEXTURES_TO_CACHE: usize = 10;

impl<'a> TextureCache<'a> {

    pub fn get_texture(&mut self, 
        active_textures: &mut ActiveTextures,
        w:               i32,
        h:               i32) -> *mut OpenGLTexture {
        
        todo!();
        /*
            if (textures.size() < numTexturesToCache)
                {
                    activeTextures.clear();
                    return new OpenGLTexture();
                }

                for (int i = 0; i < numTexturesToCache - 2; ++i)
                {
                    auto* t = textures.getUnchecked(i);

                    if (t->getWidth() == w && t->getHeight() == h)
                        return textures.removeAndReturn (i);
                }

                return textures.removeAndReturn (0);
        */
    }
    
    pub fn reset_gradient(&mut self)  {
        
        todo!();
        /*
            gradientNeedsRefresh = true;
        */
    }
    
    pub fn bind_texture_for_gradient(&mut self, 
        active_textures: &mut ActiveTextures,
        gradient:        &ColourGradient)  {
        
        todo!();
        /*
            if (gradientNeedsRefresh)
                {
                    gradientNeedsRefresh = false;

                    if (gradientTextures.size() < numGradientTexturesToCache)
                    {
                        activeGradientIndex = gradientTextures.size();
                        activeTextures.clear();
                        gradientTextures.add (new OpenGLTexture());
                    }
                    else
                    {
                        activeGradientIndex = (activeGradientIndex + 1) % numGradientTexturesToCache;
                    }

                    ALOE_CHECK_OPENGL_ERROR;
                    PixelARGB lookup[gradientTextureSize];
                    gradient.createLookupTable (lookup, gradientTextureSize);
                    gradientTextures.getUnchecked (activeGradientIndex)->loadARGB (lookup, gradientTextureSize, 1);
                }

                activeTextures.bindTexture (gradientTextures.getUnchecked (activeGradientIndex)->getTextureID());
        */
    }
}
