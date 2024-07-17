
crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/OpenGLAppDemo.h]

/**
  | This component lives inside our window,
  | and this is where you should put all your
  | controls and content.
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLAppDemo<'a> {
    base:                OpenGLAppComponent<'a>,
    vertex_shader:       *const u8,
    fragment_shader:     *const u8,
    shader:              Box<OpenGLShaderProgram<'a>>,
    shape:               Box<OpenGLAppDemoShape>,
    attributes:          Box<OpenGLAppDemoAttributes>,
    uniforms:            Box<OpenGLAppDemoUniforms<'a>>,
    new_vertex_shader:   String,
    new_fragment_shader: String,
}

impl<'a> Default for OpenGLAppDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setSize (800, 600)
        */
    }
}

impl<'a> Drop for OpenGLAppDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            shutdownOpenGL();
         */
    }
}

impl<'a> OpenGLAppDemo<'a> {

    pub fn initialise(&mut self)  {
        
        todo!();
        /*
            createShaders();
        */
    }
    
    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
            shader    .reset();
            shape     .reset();
            attributes.reset();
            uniforms  .reset();
        */
    }
    
    pub fn get_projection_matrix(&self) -> Matrix3D<f32> {
        
        todo!();
        /*
            auto w = 1.0f / (0.5f + 0.1f);
            auto h = w * getLocalBounds().toFloat().getAspectRatio (false);

            return Matrix3D<float>::fromFrustum (-w, w, -h, h, 4.0f, 30.0f);
        */
    }
    
    pub fn get_view_matrix(&self) -> Matrix3D<f32> {
        
        todo!();
        /*
            Matrix3D<float> viewMatrix ({ 0.0f, 0.0f, -10.0f });
            Matrix3D<float> rotationMatrix = viewMatrix.rotation ({ -0.3f, 5.0f * std::sin ((float) getFrameCounter() * 0.01f), 0.0f });

            return rotationMatrix * viewMatrix;
        */
    }
    
    pub fn render(&mut self)  {
        
        todo!();
        /*
            using namespace ::gl;

            jassert (OpenGLHelpers::isContextActive());

            auto desktopScale = (float) openGLContext.getRenderingScale();
            OpenGLHelpers::clear (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));

            glEnable (GL_BLEND);
            glBlendFunc (GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            glViewport (0, 0, roundToInt (desktopScale * (float) getWidth()), roundToInt (desktopScale * (float) getHeight()));

            shader->use();

            if (uniforms->projectionMatrix.get() != nullptr)
                uniforms->projectionMatrix->setMatrix4 (getProjectionMatrix().mat, 1, false);

            if (uniforms->viewMatrix.get() != nullptr)
                uniforms->viewMatrix->setMatrix4 (getViewMatrix().mat, 1, false);

            shape->draw (*attributes);

            // Reset the element buffers so child Components draw correctly
            glBindBuffer (GL_ARRAY_BUFFER, 0);
            glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, 0);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // You can add your component specific drawing code here!
            // This will draw over the top of the openGL background.

            g.setColour (getLookAndFeel().findColour (Label::textColourId));
            g.setFont (20);
            g.drawText ("OpenGL Example", 25, 20, 300, 30, Justification::left);
            g.drawLine (20, 20, 170, 20);
            g.drawLine (20, 50, 170, 50);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // This is called when this component is resized.
            // If you add any child components, this is where you should
            // update their positions.
        */
    }
    
    pub fn create_shaders(&mut self)  {
        
        todo!();
        /*
            vertexShader =
                "attribute vec4 position;\n"
                "attribute vec4 sourceColour;\n"
                "attribute vec2 textureCoordIn;\n"
                "\n"
                "uniform mat4 projectionMatrix;\n"
                "uniform mat4 viewMatrix;\n"
                "\n"
                "varying vec4 destinationColour;\n"
                "varying vec2 textureCoordOut;\n"
                "\n"
                "void main()\n"
                "{\n"
                "    destinationColour = sourceColour;\n"
                "    textureCoordOut = textureCoordIn;\n"
                "    gl_Position = projectionMatrix * viewMatrix * position;\n"
                "}\n";

            fragmentShader =
               #if ALOE_OPENGL_ES
                "varying lowp vec4 destinationColour;\n"
                "varying lowp vec2 textureCoordOut;\n"
               #else
                "varying vec4 destinationColour;\n"
                "varying vec2 textureCoordOut;\n"
               #endif
                "\n"
                "void main()\n"
                "{\n"
               #if ALOE_OPENGL_ES
                "    lowp vec4 colour = vec4(0.95, 0.57, 0.03, 0.7);\n"
               #else
                "    vec4 colour = vec4(0.95, 0.57, 0.03, 0.7);\n"
               #endif
                "    gl_FragColor = colour;\n"
                "}\n";

            std::unique_ptr<OpenGLShaderProgram> newShader (new OpenGLShaderProgram (openGLContext));
            String statusText;

            if (newShader->addVertexShader (OpenGLHelpers::translateVertexShaderToV3 (vertexShader))
                  && newShader->addFragmentShader (OpenGLHelpers::translateFragmentShaderToV3 (fragmentShader))
                  && newShader->link())
            {
                shape     .reset();
                attributes.reset();
                uniforms  .reset();

                shader.reset (newShader.release());
                shader->use();

                shape     .reset (new OpenGLAppDemoShape());
                attributes.reset (new OpenGLAppDemoAttributes (*shader));
                uniforms  .reset (new OpenGLAppDemoUniforms (*shader));

                statusText = "GLSL: v" + String (OpenGLShaderProgram::getLanguageVersion(), 2);
            }
            else
            {
                statusText = newShader->getLastError();
            }
        */
    }
}
