crate::ix!();

pub struct PositionedTexture {
    textureid: GLuint,
    area:      Rectangle<i32>,
    clip:      Rectangle<i32>,
}

impl PositionedTexture {

    pub fn new_with_texture(
        texture:     &mut OpenGLTexture,
        et:          &EdgeTable,
        clip_region: Rectangle<i32>) -> Self {
    
        todo!();
        /*
            : clip (clipRegion.getIntersection (et.getMaximumBounds()))

            if (clip.contains (et.getMaximumBounds()))
            {
                createMap (texture, et);
            }
            else
            {
                EdgeTable et2 (clip);
                et2.clipToEdgeTable (et);
                createMap (texture, et2);
            }
        */
    }
    
    pub fn new(
        texture:     GLuint,
        r:           Rectangle<i32>,
        clip_region: Rectangle<i32>) -> Self {
    
        todo!();
        /*
        : textureid(texture),
        : area(r),
        : clip(clipRegion),

        
        */
    }
    
    pub fn create_map(&mut self, 
        texture: &mut OpenGLTexture,
        et:      &EdgeTable)  {
        
        todo!();
        /*
            EdgeTableAlphaMap alphaMap (et);
            texture.loadAlpha (alphaMap.data, alphaMap.area.getWidth(), alphaMap.area.getHeight());
            textureID = texture.getTextureID();
            area = alphaMap.area;
        */
    }
}
