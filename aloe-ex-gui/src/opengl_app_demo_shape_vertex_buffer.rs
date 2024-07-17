crate::ix!();

type GLuint = libc::c_uint;

#[no_copy]
#[leak_detector]
pub struct OpenGLAppDemoShapeVertexBuffer {
    vertex_buffer: GLuint,
    index_buffer:  GLuint,
    num_indices:   i32,
}

impl Drop for OpenGLAppDemoShapeVertexBuffer {

    fn drop(&mut self) {
        todo!();
        /* 
                    using namespace ::gl;

                    glDeleteBuffers (1, &vertexBuffer);
                    glDeleteBuffers (1, &indexBuffer);
                 */
    }
}

impl OpenGLAppDemoShapeVertexBuffer {

    pub fn new(a_shape: &mut OpenGLAppDemoShape) -> Self {
    
        todo!();
        /*


            using namespace ::gl;

                    numIndices = aShape.mesh.indices.size();

                    glGenBuffers (1, &vertexBuffer);
                    glBindBuffer (GL_ARRAY_BUFFER, vertexBuffer);

                    Vec<OpenGLAppDemoVertex> vertices;
                    createVertexListFromMesh (aShape.mesh, vertices, Colours::green);

                    glBufferData (GL_ARRAY_BUFFER,
                                  static_cast<GLsizeiptr> (static_cast<size_t> (vertices.size()) * sizeof (OpenGLAppDemoVertex)),
                                  vertices.getRawDataPointer(), GL_STATIC_DRAW);

                    glGenBuffers (1, &indexBuffer);
                    glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
                    glBufferData (GL_ELEMENT_ARRAY_BUFFER,
                                  static_cast<GLsizeiptr> (static_cast<size_t> (numIndices) * sizeof (uint32)),
                                  aShape.mesh.indices.getRawDataPointer(), GL_STATIC_DRAW);
        */
    }
    
    pub fn bind(&mut self)  {
        
        todo!();
        /*
            using namespace ::gl;

                    glBindBuffer (GL_ARRAY_BUFFER, vertexBuffer);
                    glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
        */
    }
}

#[no_copy]
#[leak_detector]
pub struct OpenGLAppDemoShapeVertexBuffer2 {
    vertex_buffer: GLuint,
    index_buffer:  GLuint,
    num_indices:   i32,
}

impl Drop for OpenGLAppDemoShapeVertexBuffer2 {

    fn drop(&mut self) {
        todo!();
        /* 
                    using namespace ::gl;

                    glDeleteBuffers (1, &vertexBuffer);
                    glDeleteBuffers (1, &indexBuffer);
                 */
    }
}

pub trait Bind {

    fn bind(&mut self);
}

impl Bind for OpenGLAppDemoShapeVertexBuffer2 {

    fn bind(&mut self)  {
        
        todo!();
        /*
            using namespace ::gl;

                    glBindBuffer (GL_ARRAY_BUFFER, vertexBuffer);
                    glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
        */
    }
}

impl OpenGLAppDemoShapeVertexBuffer2 {

    pub fn new(shape: &mut OpenGLAppDemoShape) -> Self {
    
        todo!();
        /*


            using namespace ::gl;

                    numIndices = shape.mesh.indices.size();

                    glGenBuffers (1, &vertexBuffer);
                    glBindBuffer (GL_ARRAY_BUFFER, vertexBuffer);

                    Vec<Vertex> vertices;
                    createVertexListFromMesh (shape.mesh, vertices, Colours::green);

                    glBufferData (GL_ARRAY_BUFFER, vertices.size() * (int) sizeof (Vertex),
                                  vertices.getRawDataPointer(), GL_STATIC_DRAW);

                    glGenBuffers (1, &indexBuffer);
                    glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
                    glBufferData (GL_ELEMENT_ARRAY_BUFFER, numIndices * (int) sizeof (uint32),
                                                           shape.mesh.indices.getRawDataPointer(), GL_STATIC_DRAW);
        */
    }
}
