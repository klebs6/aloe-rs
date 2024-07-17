crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLPixelFormat.h]

/**
  | Represents the various properties
  | of an OpenGL pixel format.
  | 
  | @see OpenGLContext::setPixelFormat
  | 
  | @tags{OpenGL}
  |
  */
pub struct OpenGLPixelFormat {

    /**
      | The number of bits per pixel to use for
      | the red channel.
      |
      */
    red_bits:                       i32,

    /**
      | The number of bits per pixel to use for
      | the green channel.
      |
      */
    green_bits:                     i32,

    /**
      | The number of bits per pixel to use for
      | the blue channel.
      |
      */
    blue_bits:                      i32,

    /**
      | The number of bits per pixel to use for
      | the alpha channel.
      |
      */
    alpha_bits:                     i32,

    /**
      | The number of bits per pixel to use for
      | a depth buffer.
      |
      */
    depth_buffer_bits:              i32,

    /**
      | The number of bits per pixel to use for
      | a stencil buffer.
      |
      */
    stencil_buffer_bits:            i32,

    /**
      | The number of bits per pixel to use for
      | an accumulation buffer's red channel.
      |
      */
    accumulation_buffer_red_bits:   i32,

    /**
      | The number of bits per pixel to use for
      | an accumulation buffer's green channel.
      |
      */
    accumulation_buffer_green_bits: i32,

    /**
      | The number of bits per pixel to use for
      | an accumulation buffer's blue channel.
      |
      */
    accumulation_buffer_blue_bits:  i32,

    /**
      | The number of bits per pixel to use for
      | an accumulation buffer's alpha channel.
      |
      */
    accumulation_buffer_alpha_bits: i32,

    /**
      | The number of samples to use for full-scene
      | multisampled anti-aliasing (if available).
      |
      */
    multisampling_level:            u8,
}

impl PartialEq<OpenGLPixelFormat> for OpenGLPixelFormat {
    
    #[inline] fn eq(&self, other: &OpenGLPixelFormat) -> bool {
        todo!();
        /*
            return redBits == other.redBits
                && greenBits == other.greenBits
                && blueBits  == other.blueBits
                && alphaBits == other.alphaBits
                && depthBufferBits == other.depthBufferBits
                && stencilBufferBits == other.stencilBufferBits
                && accumulationBufferRedBits   == other.accumulationBufferRedBits
                && accumulationBufferGreenBits == other.accumulationBufferGreenBits
                && accumulationBufferBlueBits  == other.accumulationBufferBlueBits
                && accumulationBufferAlphaBits == other.accumulationBufferAlphaBits
                && multisamplingLevel == other.multisamplingLevel;
        */
    }
}

impl Eq for OpenGLPixelFormat {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/opengl/aloe_OpenGLPixelFormat.cpp]
impl OpenGLPixelFormat {

    /**
      | Creates an OpenGLPixelFormat.
      | 
      | The default constructor just initialises
      | the object as a simple 8-bit
      | 
      | RGBA format.
      |
      */
    pub fn new(
        bits_per_rgb_component: Option<i32>,
        alpha_bits:             Option<i32>,
        depth_buffer_bits:      Option<i32>,
        stencil_buffer_bits:    Option<i32>

    ) -> Self {

        let bits_per_rgb_component: i32 =
                 bits_per_rgb_component.unwrap_or(8);

        let alpha_bits: i32 = alpha_bits.unwrap_or(8);

        let depth_buffer_bits: i32 =
                 depth_buffer_bits.unwrap_or(16);

        let stencil_buffer_bits: i32 =
                 stencil_buffer_bits.unwrap_or(0);
    
        todo!();
        /*
        : red_bits(bitsPerRGBComponent),
        : green_bits(bitsPerRGBComponent),
        : blue_bits(bitsPerRGBComponent),
        : alpha_bits(alphaBits_),
        : depth_buffer_bits(depthBufferBits_),
        : stencil_buffer_bits(stencilBufferBits_),
        : accumulation_buffer_red_bits(0),
        : accumulation_buffer_green_bits(0),
        : accumulation_buffer_blue_bits(0),
        : accumulation_buffer_alpha_bits(0),
        : multisampling_level(0),

        
        */
    }
}
