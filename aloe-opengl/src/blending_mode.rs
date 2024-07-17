crate::ix!();

pub struct BlendingMode {
    blending_enabled: bool, // default = false
    src_function:     GLenum, // default = 0
    dst_function:     GLenum, // default = 0
}

impl BlendingMode {
    
    pub fn resync(&mut self)  {
        
        todo!();
        /*
            glDisable (GL_BLEND);
                srcFunction = dstFunction = 0;
        */
    }
    
    
    pub fn set_premultiplied_blending_mode<QuadQueueType>(&mut self, quad_queue: &mut QuadQueueType)  {
    
        todo!();
        /*
            setBlendFunc (quadQueue, GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
        */
    }
    
    
    pub fn set_blend_func<QuadQueueType>(&mut self, 
        quad_queue: &mut QuadQueueType,
        src:        GLenum,
        dst:        GLenum)  {
    
        todo!();
        /*
            if (! blendingEnabled)
                {
                    quadQueue.flush();
                    blendingEnabled = true;
                    glEnable (GL_BLEND);
                }

                if (srcFunction != src || dstFunction != dst)
                {
                    quadQueue.flush();
                    srcFunction = src;
                    dstFunction = dst;
                    glBlendFunc (src, dst);
                }
        */
    }
    
    
    pub fn disable_blend<QuadQueueType>(&mut self, quad_queue: &mut QuadQueueType)  {
    
        todo!();
        /*
            if (blendingEnabled)
                {
                    quadQueue.flush();
                    blendingEnabled = false;
                    glDisable (GL_BLEND);
                }
        */
    }
    
    
    pub fn set_blend_mode<QuadQueueType>(&mut self, 
        quad_queue:                &mut QuadQueueType,
        replace_existing_contents: bool)  {
    
        todo!();
        /*
            if (replaceExistingContents)
                    disableBlend (quadQueue);
                else
                    setPremultipliedBlendingMode (quadQueue);
        */
    }
}
