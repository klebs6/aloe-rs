crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/OpenGLDemo.h]

pub struct BackgroundStar
{
    x:     SlowerBouncingNumber,
    y:     SlowerBouncingNumber,
    hue:   SlowerBouncingNumber,
    angle: SlowerBouncingNumber,
}

/**
  | This is the main demo component - the
  | GL context gets attached to it, and it
  | implements the OpenGLRenderer callback
  | so that it can do real GL work.
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLDemo<'a> {
    base:                  Component<'a>,
    base3:                 AsyncUpdater<'a>,
    draggable_orientation: Draggable3DOrientation,
    do_background_drawing: bool, // default = false
    scale:                 f32, // default = 0.5
    rotation_speed:        f32, // default = 0.0
    bouncing_number:       BouncingNumber,
    open_gl_context:       OpenGLContext<'a>,
    controls_overlay:      Box<DemoControlsOverlay<'a>>,
    rotation:              f32, // default = 0.0f
    shader:                Box<OpenGLShaderProgram<'a>>,
    shape:                 Box<OpenGLAppDemoShape>,
    attributes:            Box<OpenGLAppDemoAttributes>,
    uniforms:              Box<OpenGLAppDemoUniforms<'a>>,
    texture:               OpenGLTexture<'a>,
    texture_to_use:        *mut OpenGLAppDemoTexture, // default = nullptr
    last_texture:          *mut OpenGLAppDemoTexture, // default = nullptr
    new_vertex_shader:     String,
    new_fragment_shader:   String,
    status_text:           String,
    stars:                 [BackgroundStar; 3],
}

impl<'a> OpenGLRenderer for OpenGLDemo<'a> {

    fn open_gl_context_closing(&mut self)  {
        
        todo!();
        /*
            // When the context is about to close, you must use this callback to delete
            // any GPU resources while the context is still current.
            freeAllContextObjects();

            if (lastTexture != nullptr)
                setTexture (lastTexture);
        */
    }

    fn new_open_gl_context_created(&mut self)  {
        
        todo!();
        /*
            // nothing to do in this case - we'll initialise our shaders + textures
            // on demand, during the render callback.
            freeAllContextObjects();

            if (controlsOverlay.get() != nullptr)
                controlsOverlay->updateShader();
        */
    }

    /**
      | This is a virtual method in OpenGLRenderer,
      | and is called when it's time to do your GL
      | rendering.
      */
    fn render_opengl(&mut self)  {
        
        todo!();
        /*
            using namespace ::gl;

            jassert (OpenGLHelpers::isContextActive());

            auto desktopScale = (float) openGLContext.getRenderingScale();

            OpenGLHelpers::clear (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground,
                                                          Colours::lightblue));

            if (textureToUse != nullptr)
                if (! textureToUse->applyTo (texture))
                    textureToUse = nullptr;

            // First draw our background graphics to demonstrate the OpenGLGraphicsContext class
            if (doBackgroundDrawing)
                drawBackground2DStuff (desktopScale);

            updateShader();   // Check whether we need to compile a new shader

            if (shader.get() == nullptr)
                return;

            // Having used the aloe 2D renderer, it will have messed-up a whole load of GL state, so
            // we need to initialise some important settings before doing our normal GL 3D drawing..
            glEnable (GL_DEPTH_TEST);
            glDepthFunc (GL_LESS);
            glEnable (GL_BLEND);
            glBlendFunc (GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
            glActiveTexture (GL_TEXTURE0);
            glEnable (GL_TEXTURE_2D);

            glViewport (0, 0, roundToInt (desktopScale * (float) getWidth()), roundToInt (desktopScale * (float) getHeight()));

            texture.bind();

            glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
            glTexParameteri (GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);

            shader->use();

            if (uniforms->projectionMatrix.get() != nullptr)
                uniforms->projectionMatrix->setMatrix4 (getProjectionMatrix().mat, 1, false);

            if (uniforms->viewMatrix.get() != nullptr)
                uniforms->viewMatrix->setMatrix4 (getViewMatrix().mat, 1, false);

            if (uniforms->texture.get() != nullptr)
                uniforms->texture->set ((GLint) 0);

            if (uniforms->lightPosition.get() != nullptr)
                uniforms->lightPosition->set (-15.0f, 10.0f, 15.0f, 0.0f);

            if (uniforms->bouncingNumber.get() != nullptr)
                uniforms->bouncingNumber->set (bouncingNumber.getValue());

            shape->draw (*attributes);

            // Reset the element buffers so child Components draw correctly
            glBindBuffer (GL_ARRAY_BUFFER, 0);
            glBindBuffer (GL_ELEMENT_ARRAY_BUFFER, 0);

            if (! controlsOverlay->isMouseButtonDown())
                rotation += (float) rotationSpeed;
        */
    }

}

impl<'a> Default for OpenGLDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            if (auto* peer = getPeer())
                peer->setCurrentRenderingEngine (0);

            setOpaque (true);
            controlsOverlay.reset (new DemoControlsOverlay (*this));
            addAndMakeVisible (controlsOverlay.get());

            openGLContext.setRenderer (this);
            openGLContext.attachTo (*this);
            openGLContext.setContinuousRepainting (true);

            controlsOverlay->initialise();

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for OpenGLDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            openGLContext.detach();
         */
    }
}

pub trait FreeAllContextObjects {

    fn free_all_context_objects(&mut self);
}

impl<'a> FreeAllContextObjects for OpenGLDemo<'a> {

    fn free_all_context_objects(&mut self)  {
        
        todo!();
        /*
            shape     .reset();
            shader    .reset();
            attributes.reset();
            uniforms  .reset();
            texture   .release();
        */
    }
}

pub trait GetProjectionMatrix {

    fn get_projection_matrix(&self) -> Matrix3D<f32>;
}

impl<'a> GetProjectionMatrix for OpenGLDemo<'a> {

    fn get_projection_matrix(&self) -> Matrix3D<f32> {
        
        todo!();
        /*
            auto w = 1.0f / (scale + 0.1f);
            auto h = w * getLocalBounds().toFloat().getAspectRatio (false);

            return Matrix3D<float>::fromFrustum (-w, w, -h, h, 4.0f, 30.0f);
        */
    }
}

pub trait GetViewMatrix {

    fn get_view_matrix(&self) -> Matrix3D<f32>;
}

impl<'a> GetViewMatrix for OpenGLDemo<'a> {

    fn get_view_matrix(&self) -> Matrix3D<f32> {
        
        todo!();
        /*
            auto viewMatrix = draggableOrientation.getRotationMatrix()
                                 * Vector3D<float> (0.0f, 1.0f, -10.0f);

            auto rotationMatrix = Matrix3D<float>::rotation ({ rotation, rotation, -0.3f });

            return rotationMatrix * viewMatrix;
        */
    }
}

pub trait SetTexture {

    fn set_texture(&mut self, t: *mut OpenGLAppDemoTexture);
}

impl<'a> SetTexture for OpenGLDemo<'a> {

    fn set_texture(&mut self, t: *mut OpenGLAppDemoTexture)  {
        
        todo!();
        /*
            lastTexture = textureToUse = t;
        */
    }
}

pub trait SetShaderProgram {

    fn set_shader_program(
        &mut self, 
        vertex_shader:   &String,
        fragment_shader: &String
    );
}

impl<'a> SetShaderProgram for OpenGLDemo<'a> {

    fn set_shader_program(&mut self, 
        vertex_shader:   &String,
        fragment_shader: &String)  {
        
        todo!();
        /*
            newVertexShader = vertexShader;
            newFragmentShader = fragmentShader;
        */
    }
}

impl<'a> Paint for OpenGLDemo<'a> {

    fn paint(&mut self, _0: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Resized for OpenGLDemo<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            controlsOverlay->setBounds (getLocalBounds());
            draggableOrientation.setViewport (getLocalBounds());
        */
    }
}

impl<'a> HandleAsyncUpdate for OpenGLDemo<'a> {

    fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            controlsOverlay->statusLabel.setText (statusText, dontSendNotification);
        */
    }
}

pub trait DrawBackground2D {

    fn draw_background2d_stuff(&mut self, desktop_scale: f32);
}

impl<'a> DrawBackground2D for OpenGLDemo<'a> {

    fn draw_background2d_stuff(&mut self, desktop_scale: f32)  {
        
        todo!();
        /*
            // Create an OpenGLGraphicsContext that will draw into this GL window..
            std::unique_ptr<LowLevelGraphicsContext> glRenderer (createOpenGLGraphicsContext (openGLContext,
                                                                                              roundToInt (desktopScale * (float) getWidth()),
                                                                                              roundToInt (desktopScale * (float) getHeight())));

            if (glRenderer.get() != nullptr)
            {
                Graphics g (*glRenderer);
                g.addTransform (AffineTransform::scale (desktopScale));

                for (auto s : stars)
                {
                    auto size = 0.25f;

                    // This stuff just creates a spinning star shape and fills it..
                    Path p;
                    p.addStar ({ (float) getWidth()  * s.x.getValue(),
                                 (float) getHeight() * s.y.getValue() },
                               7,
                               (float) getHeight() * size * 0.5f,
                               (float) getHeight() * size,
                               s.angle.getValue());

                    auto hue = s.hue.getValue();

                    g.setGradientFill (ColourGradient (Colours::green.withRotatedHue (hue).withAlpha (0.8f),
                                                       0, 0,
                                                       Colours::red.withRotatedHue (hue).withAlpha (0.5f),
                                                       0, (float) getHeight(), false));
                    g.fillPath (p);
                }
            }
        */
    }
}

impl<'a> UpdateShader for OpenGLDemo<'a> {
    
    fn update_shader(&mut self)  {
        
        todo!();
        /*
            if (newVertexShader.isNotEmpty() || newFragmentShader.isNotEmpty())
            {
                std::unique_ptr<OpenGLShaderProgram> newShader (new OpenGLShaderProgram (openGLContext));

                if (newShader->addVertexShader (OpenGLHelpers::translateVertexShaderToV3 (newVertexShader))
                      && newShader->addFragmentShader (OpenGLHelpers::translateFragmentShaderToV3 (newFragmentShader))
                      && newShader->link())
                {
                    shape     .reset();
                    attributes.reset();
                    uniforms  .reset();

                    shader.reset (newShader.release());
                    shader->use();

                    shape     .reset (new OpenGLAppDemoShape      ());
                    attributes.reset (new OpenGLAppDemoAttributes (*shader));
                    uniforms  .reset (new OpenGLAppDemoUniforms   (*shader));

                    statusText = "GLSL: v" + String (OpenGLShaderProgram::getLanguageVersion(), 2);
                }
                else
                {
                    statusText = newShader->getLastError();
                }

                triggerAsyncUpdate();

                newVertexShader   = {};
                newFragmentShader = {};
            }
        */
    }
}
