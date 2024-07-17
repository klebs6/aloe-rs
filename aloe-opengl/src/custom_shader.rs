crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLGraphicsContext.h]

/**
  | Used to create custom shaders for use
  | with an openGL 2D rendering context.
  | 
  | Given a GL-based rendering context,
  | you can write a fragment shader that
  | applies some kind of per-pixel effect.
  | 
  | @tags{OpenGL}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OpenGLGraphicsContextCustomShader {

    /**
      | Optional lambda that will be called
      | when the shader is activated, to allow
      | user code to do setup tasks.
      |
      */
    on_shader_activated: fn(_0: &mut OpenGLShaderProgram) -> (),

    code:                String,
    hash_name:           String,
}

lazy_static!{
    /*
    extern void (*clearOpenGLGlyphCache)(); // declared in aloe_graphics
    */
}

impl Drop for OpenGLGraphicsContextCustomShader {

    fn drop(&mut self) {
        todo!();
        /*
            if (OpenGLContext* context = OpenGLContext::getCurrentContext())
            context->setAssociatedObject (hashName.toRawUTF8(), nullptr);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLGraphicsContext.cpp]
impl OpenGLGraphicsContextCustomShader {

    /**
      | Returns the code that was used to create
      | this object.
      |
      */
    pub fn get_fragment_shader_code(&self) -> &String {
        
        todo!();
        /*
            return code;
        */
    }

    /**
      | Creates a custom shader.
      | 
      | The shader code will not be compiled
      | until actually needed, so it's OK to
      | call this constructor when no GL context
      | is active.
      | 
      | The code should be a normal fragment
      | shader. As well as the usual GLSL variables,
      | there is also an automatically declared
      | varying vec2 called "pixelPos", which
      | indicates the pixel position within
      | the graphics context of the pixel being
      | drawn. There is also a varying value
      | "pixelAlpha", which indicates the
      | alpha by which the pixel should be multiplied,
      | so that the edges of any clip-region
      | masks are anti-aliased correctly.
      |
      */
    pub fn new(fragment_shader_code: &String) -> Self {
    
        todo!();
        /*
            : code (String (ALOE_DECLARE_VARYING_COLOUR
                        ALOE_DECLARE_VARYING_PIXELPOS
                        "\n#define pixelAlpha frontColour.a\n") + fragmentShaderCode),
          hashName (String::toHexString (fragmentShaderCode.hashCode64()) + "_shader")
        */
    }
    
    /**
      | Returns the program, if it has been linked
      | and is active.
      | 
      | This can be called when you're about
      | to use fillRect, to set up any uniforms/textures
      | that the program may require.
      |
      */
    pub fn get_program(&self, gc: &mut dyn LowLevelGraphicsContext) -> *mut OpenGLShaderProgram {
        
        todo!();
        /*
            String errorMessage;

        if (auto c = CustomProgram::getOrCreate (gc, hashName, code, errorMessage))
            return &(c->program);

        return {};
        */
    }
    
    /**
      | Applies the shader to a rectangle within
      | the graphics context.
      |
      */
    pub fn fill_rect(
        &self, 
        gc:   &mut dyn LowLevelGraphicsContext,
        area: Rectangle<i32>

    ) {
        
        todo!();
        /*
            String errorMessage;

        if (auto sc = dynamic_cast<OpenGLRendering::ShaderContext*> (&gc))
        {
            if (auto c = CustomProgram::getOrCreate (gc, hashName, code, errorMessage))
            {
                c->onShaderActivated = onShaderActivated;
                sc->fillRectWithCustomShader (*c, area);
            }
        }
        */
    }
    
    /**
      | Attempts to compile the program if necessary,
      | and returns an error message if it fails.
      |
      */
    pub fn check_compilation(&mut self, gc: &mut dyn LowLevelGraphicsContext) -> Result<(),()> {
        
        todo!();
        /*
            String errorMessage;

        if (CustomProgram::getOrCreate (gc, hashName, code, errorMessage) != nullptr)
            return Result::ok();

        return Result::fail (errorMessage);
        */
    }
}
