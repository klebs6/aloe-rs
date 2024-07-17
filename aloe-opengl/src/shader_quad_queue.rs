crate::ix!();

#[no_copy]
pub struct ShaderQuadQueue<'a> {
    buffers:      [u32; 2],
    vertex_data:  [ShaderQuadQueueVertexInfo; SHADER_QUAD_QUEUE_MAX_NUM_QUADS * 4],
    index_data:   [u16; SHADER_QUAD_QUEUE_MAX_NUM_QUADS * 6],
    context:      &'a OpenGLContext<'a>,
    num_vertices: i32, // default = 0

    #[cfg(any(target_os="android",target_os="ios"))]
    max_vertices: i32, // default = SHADER_QUAD_QUEUE_MAX_NUM_QUADS * 4 - 4 

    #[cfg(not(any(target_os="android",target_os="ios")))]
    max_vertices: i32, // default = 0
}

impl<'a> Drop for ShaderQuadQueue<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            static_assert (sizeof (ShaderQuadQueueVertexInfo) == 8, "Sanity check ShaderQuadQueueVertexInfo size");
                context.extensions.glBindBuffer (GL_ARRAY_BUFFER, 0);
                context.extensions.glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, 0);
                context.extensions.glDeleteBuffers (2, buffers);
        */
    }
}

pub struct ShaderQuadQueueVertexInfo
{
    x:      u16,
    y:      u16,
    colour: u32,
}

pub const SHADER_QUAD_QUEUE_MAX_NUM_QUADS: usize = 256;

impl<'a> ShaderQuadQueue<'a> {

    pub fn new(c: &OpenGLContext) -> Self {
    
        todo!();
        /*
        : context(c),
        */
    }
    
    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            ALOE_CHECK_OPENGL_ERROR

               #if ALOE_ANDROID || ALOE_IOS
                int numQuads = SHADER_QUAD_QUEUE_MAX_NUM_QUADS;
               #else
                GLint maxIndices = 0;
                glGetIntegerv (GL_MAX_ELEMENTS_INDICES, &maxIndices);
                auto numQuads = jmin ((int) SHADER_QUAD_QUEUE_MAX_NUM_QUADS, (int) maxIndices / 6);
                maxVertices = numQuads * 4 - 4;
               #endif

                for (int i = 0, v = 0; i < numQuads * 6; i += 6, v += 4)
                {
                    indexData[i] = (GLushort) v;
                    indexData[i + 1] = indexData[i + 3] = (GLushort) (v + 1);
                    indexData[i + 2] = indexData[i + 4] = (GLushort) (v + 2);
                    indexData[i + 5] = (GLushort) (v + 3);
                }

                context.extensions.glGenBuffers (2, buffers);
                context.extensions.glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, buffers[0]);
                context.extensions.glBufferData (GL_ELEMENT_ARRAY_BUFFER, sizeof (indexData), indexData, GL_STATIC_DRAW);
                context.extensions.glBindBuffer (GL_ARRAY_BUFFER, buffers[1]);
                context.extensions.glBufferData (GL_ARRAY_BUFFER, sizeof (vertexData), vertexData, GL_STREAM_DRAW);
                ALOE_CHECK_OPENGL_ERROR
        */
    }
    
    pub fn add_with_xywh(
        &mut self, 
        x:      i32,
        y:      i32,
        w:      i32,
        h:      i32,
        colour: PixelARGB)  {
        
        todo!();
        /*
            jassert (w > 0 && h > 0);

                auto* v = vertexData + numVertices;
                v[0].x = v[2].x = (GLshort) x;
                v[0].y = v[1].y = (GLshort) y;
                v[1].x = v[3].x = (GLshort) (x + w);
                v[2].y = v[3].y = (GLshort) (y + h);

               #if ALOE_BIG_ENDIAN
                auto rgba = (GLuint) ((colour.getRed() << 24) | (colour.getGreen() << 16)
                                    | (colour.getBlue() << 8) |  colour.getAlpha());
               #else
                auto rgba = (GLuint) ((colour.getAlpha() << 24) | (colour.getBlue() << 16)
                                    | (colour.getGreen() << 8) |  colour.getRed());
               #endif

                v[0].colour = rgba;
                v[1].colour = rgba;
                v[2].colour = rgba;
                v[3].colour = rgba;

                numVertices += 4;

                if (numVertices > maxVertices)
                    draw();
        */
    }
    
    pub fn add_with_rect_i32(
        &mut self, 
        r:      Rectangle<i32>,
        colour: PixelARGB)  {
        
        todo!();
        /*
            add (r.getX(), r.getY(), r.getWidth(), r.getHeight(), colour);
        */
    }
    
    pub fn add_with_rect_f32(
        &mut self, 
        r:      Rectangle<f32>,
        colour: PixelARGB

    ) {
        
        todo!();
        /*
            FloatRectangleRenderer<ShaderQuadQueue> frr (*this, colour);
                RenderingHelpers::FloatRectangleRasterisingInfo (r).iterate (frr);
        */
    }
    
    pub fn add_with_rect_list_i32(
        &mut self, 
        list:   &RectangleList<i32>,
        colour: PixelARGB

    ) {
        
        todo!();
        /*
            for (auto& i : list)
                    add (i, colour);
        */
    }
    
    pub fn add_with_rect_list_i32_and_clip(
        &mut self, 
        list:   &RectangleList<i32>,
        clip:   Rectangle<i32>,
        colour: PixelARGB

    ) {
        
        todo!();
        /*
            for (auto& i : list)
                {
                    auto r = i.getIntersection (clip);

                    if (! r.isEmpty())
                        add (r, colour);
                }
        */
    }
    
    
    pub fn add<IteratorType>(&mut self, 
        et:     &IteratorType,
        colour: PixelARGB)  {
    
        todo!();
        /*
            EdgeTableRenderer<ShaderQuadQueue> etr (*this, colour);
                et.iterate (etr);
        */
    }
    
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            if (numVertices > 0)
                    draw();
        */
    }
    
    pub fn draw(&mut self)  {
        
        todo!();
        /*
            context.extensions.glBufferSubData (GL_ARRAY_BUFFER, 0, (GLsizeiptr) ((size_t) numVertices * sizeof (ShaderQuadQueueVertexInfo)), vertexData);
                // NB: If you get a random crash in here and are running in a Parallels VM, it seems to be a bug in
                // their driver.. Can't find a workaround unfortunately.
                glDrawElements (GL_TRIANGLES, (numVertices * 3) / 2, GL_UNSIGNED_SHORT, nullptr);
                ALOE_CHECK_OPENGL_ERROR
                numVertices = 0;
        */
    }
}
