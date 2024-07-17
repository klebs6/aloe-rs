crate::ix!();

/**
  | This class just manages the attributes
  | that the shaders use.
  |
  */
pub struct OpenGLAppDemoAttributes {
    position:         Box<OpenGLShaderProgramAttribute>,
    normal:           Box<OpenGLShaderProgramAttribute>,
    source_colour:    Box<OpenGLShaderProgramAttribute>,
    texture_coord_in: Box<OpenGLShaderProgramAttribute>,
}

impl OpenGLAppDemoAttributes {

    pub fn new(shader: &mut OpenGLShaderProgram) -> Self {
    
        todo!();
        /*


            position      .reset (createAttribute (shader, "position"));
                normal        .reset (createAttribute (shader, "normal"));
                sourceColour  .reset (createAttribute (shader, "sourceColour"));
                textureCoordIn.reset (createAttribute (shader, "textureCoordIn"));
        */
    }
    
    pub fn create_attribute(
        shader:         &mut OpenGLShaderProgram,
        attribute_name: *const u8

    ) -> *mut OpenGLShaderProgramAttribute {
        
        todo!();
        /*
            using namespace ::gl;

                if (glGetAttribLocation (shader.getProgramID(), attributeName) < 0)
                    return nullptr;

                return new OpenGLShaderProgram::Attribute (shader, attributeName);
        */
    }
}

pub trait Enable {

    fn enable(&mut self);
}

impl Enable for OpenGLAppDemoAttributes {

    fn enable(&mut self)  {
        
        todo!();
        /*
            using namespace ::gl;

                if (position.get() != nullptr)
                {
                    glVertexAttribPointer (position->attributeID, 3, GL_FLOAT, GL_FALSE, sizeof (Vertex), nullptr);
                    glEnableVertexAttribArray (position->attributeID);
                }

                if (normal.get() != nullptr)
                {
                    glVertexAttribPointer (normal->attributeID, 3, GL_FLOAT, GL_FALSE, sizeof (Vertex), (GLvoid*) (sizeof (float) * 3));
                    glEnableVertexAttribArray (normal->attributeID);
                }

                if (sourceColour.get() != nullptr)
                {
                    glVertexAttribPointer (sourceColour->attributeID, 4, GL_FLOAT, GL_FALSE, sizeof (Vertex), (GLvoid*) (sizeof (float) * 6));
                    glEnableVertexAttribArray (sourceColour->attributeID);
                }

                if (textureCoordIn.get() != nullptr)
                {
                    glVertexAttribPointer (textureCoordIn->attributeID, 2, GL_FLOAT, GL_FALSE, sizeof (Vertex), (GLvoid*) (sizeof (float) * 10));
                    glEnableVertexAttribArray (textureCoordIn->attributeID);
                }
        */
    }
}

pub trait Disable {

    fn disable(&mut self);
}

impl Disable for OpenGLAppDemoAttributes {
    
    fn disable(&mut self)  {
        
        todo!();
        /*
            using namespace ::gl;

                if (position.get() != nullptr)        glDisableVertexAttribArray (position->attributeID);
                if (normal.get() != nullptr)          glDisableVertexAttribArray (normal->attributeID);
                if (sourceColour.get() != nullptr)    glDisableVertexAttribArray (sourceColour->attributeID);
                if (textureCoordIn.get() != nullptr)  glDisableVertexAttribArray (textureCoordIn->attributeID);
        */
    }
}
