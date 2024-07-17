crate::ix!();

/**
  | This loads a 3D model from an OBJ file
  | and converts it into some vertex buffers
  | that we can draw.
  |
  */
pub struct OpenGLAppDemoShape {
    shape_file:     WavefrontObjFile,
    vertex_buffers: Vec<Box<OpenGLAppDemoShapeVertexBuffer>>,
}

impl Default for OpenGLAppDemoShape {
    
    fn default() -> Self {
        todo!();
        /*


            if (shapeFile.load (loadEntireAssetIntoString ("teapot.obj")).wasOk())
                    for (auto* s : shapeFile.shapes)
                        vertexBuffers.add (new OpenGLAppDemoShapeVertexBuffer (*s))
        */
    }
}

pub trait DrawWithAttributes {

    type DrawConfig = OpenGLAppDemoAttributes;

    fn draw(&mut self, attributes: &mut Self::DrawConfig);
}

impl DrawWithAttributes for OpenGLAppDemoShape {

    type DrawConfig = OpenGLAppDemoAttributes;

    fn draw(&mut self, attributes: &mut Self::DrawConfig)  {
        
        todo!();
        /*
            using namespace ::gl;

                for (auto* vertexBuffer : vertexBuffers)
                {
                    vertexBuffer->bind();

                    attributes.enable();
                    glDrawElements (GL_TRIANGLES, vertexBuffer->numIndices, GL_UNSIGNED_INT, nullptr);
                    attributes.disable();
                }
        */
    }
}

impl OpenGLAppDemoShape {

    pub fn create_vertex_list_from_mesh(
        mesh:   &WavefrontObjFileMesh,
        list:   &mut Vec<Vertex>,
        colour: Colour)  {
        
        todo!();
        /*
            auto scale = 0.2f;
                WavefrontObjFile::TextureCoord defaultTexCoord = { 0.5f, 0.5f };
                WavefrontObjFile::Vertex defaultNormal = { 0.5f, 0.5f, 0.5f };

                for (int i = 0; i < mesh.vertices.size(); ++i)
                {
                    auto& v = mesh.vertices.getReference (i);

                    auto& n = (i < mesh.normals.size() ? mesh.normals.getReference (i)
                                                       : defaultNormal);

                    auto& tc = (i < mesh.textureCoords.size() ? mesh.textureCoords.getReference (i)
                                                              : defaultTexCoord);

                    list.add ({ { scale * v.x, scale * v.y, scale * v.z, },
                                { scale * n.x, scale * n.y, scale * n.z, },
                                { colour.getFloatRed(), colour.getFloatGreen(), colour.getFloatBlue(), colour.getFloatAlpha() },
                                { tc.x, tc.y } });
                }
        */
    }
}
