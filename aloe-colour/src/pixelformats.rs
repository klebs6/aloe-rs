crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_PixelFormats.h]

#[inline] pub fn mask_pixel_components(x: u32) -> u32 {
    
    todo!();
    /*
        return (x >> 8) & 0x00ff00ff;
    */
}

#[inline] pub fn clamp_pixel_components(x: u32) -> u32 {
    
    todo!();
    /*
        return (x | (0x01000100 - maskPixelComponents (x))) & 0x00ff00ff;
    */
}

/**
  | Represents a 32-bit INTERNAL pixel
  | with premultiplied alpha, and can perform
  | compositing operations with it.
  | 
  | This is used internally by the imaging
  | classes.
  | 
  | @see PixelRGB
  | 
  | @tags{Graphics}
  |
  */
pub struct PixelARGB {
    internal: u32,
}

impl PixelARGB {

    pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {

        // Convert the a, r, g, b values to the internal representation
        let internal = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

        Self {
            internal,
        }
    }

    pub fn a(&self) -> u8 {
        let bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_a]
    }

    pub fn r(&self) -> u8 {
        let bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_r]
    }

    pub fn g(&self) -> u8 {
        let bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_g]
    }

    pub fn b(&self) -> u8 {
        let bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_b]
    }

    pub fn set_a(&mut self, value: u8) {
        let mut bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_a] = value;
        self.internal = unsafe { std::mem::transmute(bytes) };
    }

    pub fn set_r(&mut self, value: u8) {
        let mut bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_r] = value;
        self.internal = unsafe { std::mem::transmute(bytes) };
    }

    pub fn set_g(&mut self, value: u8) {
        let mut bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_g] = value;
        self.internal = unsafe { std::mem::transmute(bytes) };
    }

    pub fn set_b(&mut self, value: u8) {
        let mut bytes: [u8; 4] = unsafe { std::mem::transmute(self.internal) };
        bytes[pixel_argb::index_b] = value;
        self.internal = unsafe { std::mem::transmute(bytes) };
    }
}

impl Default for PixelARGB {
    
    /**
      | Creates a pixel without defining its
      | colour.
      |
      */
    fn default() -> Self {
        Self {
            internal: 0x00000000, // sets ARGB to 0 (transparent pixel)
        }
    }
}

pub mod pixel_argb {

    /**
      | The indexes of the different components
      | in the byte layout of this type of colour.
      |
      */
    #[cfg(all(target_os="android",ALOE_BIG_ENDIAN))]           pub const index_a: usize = 0;
    #[cfg(all(target_os="android",ALOE_BIG_ENDIAN))]           pub const index_r: usize = 3;
    #[cfg(all(target_os="android",ALOE_BIG_ENDIAN))]           pub const index_g: usize = 2;
    #[cfg(all(target_os="android",ALOE_BIG_ENDIAN))]           pub const index_b: usize = 1;

    #[cfg(all(target_os="android",not(ALOE_BIG_ENDIAN)))]      pub const index_a: usize = 3;
    #[cfg(all(target_os="android",not(ALOE_BIG_ENDIAN)))]      pub const index_r: usize = 0;
    #[cfg(all(target_os="android",not(ALOE_BIG_ENDIAN)))]      pub const index_g: usize = 1;
    #[cfg(all(target_os="android",not(ALOE_BIG_ENDIAN)))]      pub const index_b: usize = 2;

    #[cfg(all(not(target_os="android"),ALOE_BIG_ENDIAN))]      pub const index_a: usize = 0;
    #[cfg(all(not(target_os="android"),ALOE_BIG_ENDIAN))]      pub const index_r: usize = 1;
    #[cfg(all(not(target_os="android"),ALOE_BIG_ENDIAN))]      pub const index_g: usize = 2;
    #[cfg(all(not(target_os="android"),ALOE_BIG_ENDIAN))]      pub const index_b: usize = 3;

    #[cfg(all(not(target_os="android"),not(ALOE_BIG_ENDIAN)))] pub const index_a: usize = 3;
    #[cfg(all(not(target_os="android"),not(ALOE_BIG_ENDIAN)))] pub const index_r: usize = 2;
    #[cfg(all(not(target_os="android"),not(ALOE_BIG_ENDIAN)))] pub const index_g: usize = 1;
    #[cfg(all(not(target_os="android"),not(ALOE_BIG_ENDIAN)))] pub const index_b: usize = 0;

    /*
    #[bitfield]
    #[cfg(all(target_os="android",ALOE_BIG_ENDIAN))]
    pub struct Components
    {
        a: B8,
        b: B8,
        g: B8,
        r: B8,
    }

    #[bitfield]
    #[cfg(all(target_os="android",not(ALOE_BIG_ENDIAN)))]
    pub struct Components
    {
        r: B8,
        g: B8,
        b: B8,
        a: B8,
    }
    
    #[bitfield]
    #[cfg(all(not(target_os="android"),ALOE_BIG_ENDIAN))]
    pub struct Components
    {
        a: B8,
        r: B8,
        g: B8,
        b: B8,
    }

    #[bitfield]
    #[cfg(all(not(target_os="android"),not(ALOE_BIG_ENDIAN)))]
    pub struct Components
    {
        b: B8,
        g: B8,
        r: B8,
        a: B8,
    }
    */
}

impl PixelARGB {
    
    pub const fn new_from_argb(
        a: u8,
        r: u8,
        g: u8,
        b: u8) -> Self {
    
        todo!();
        /*


            components.b = b;
            components.g = g;
            components.r = r;
            components.a = a;
        */
    }
    
    /**
      | Returns a uint32 which represents the
      | pixel in a platform dependent format.
      |
      */
    #[inline(always)] pub fn get_nativeargb(&self) -> u32 {
        
        todo!();
        /*
            return internal;
        */
    }

    /**
      | Returns a uint32 which will be in argb
      | order as if constructed with the following
      | mask operation ((alpha << 24) | (red
      | << 16) | (green << 8) | blue).
      |
      */
    #[inline(always)] pub fn get_in_argb_mask_order(&self) -> u32 {
        
        todo!();
        /*
            #if ALOE_ANDROID
            return (uint32) ((components.a << 24) | (components.r << 16) | (components.g << 8) | (components.b << 0));
           #else
            return getNativeARGB();
           #endif
        */
    }

    /**
      | Returns a uint32 which when written
      | to memory, will be in the order a, r, g,
      | b. In other words, if the return-value
      | is read as a uint8 array then the elements
      | will be in the order of a, r, g, b
      |
      */
    #[inline] pub fn get_in_argb_memory_order(&self) -> u32 {
        
        todo!();
        /*
            #if ALOE_BIG_ENDIAN
            return getInARGBMaskOrder();
           #else
            return (uint32) ((components.b << 24) | (components.g << 16) | (components.r << 8) | components.a);
           #endif
        */
    }

    /**
      | Return channels with an even index and
      | insert zero bytes between them. This
      | is useful for blending operations.
      | The exact channels which are returned
      | is platform dependent.
      |
      */
    #[inline(always)] pub fn get_even_bytes(&self) -> u32 {
        
        todo!();
        /*
            return 0x00ff00ff & internal;
        */
    }

    /**
      | Return channels with an odd index and
      | insert zero bytes between them. This
      | is useful for blending operations.
      | The exact channels which are returned
      | is platform dependent.
      |
      */
    #[inline(always)] pub fn get_odd_bytes(&self) -> u32 {
        
        todo!();
        /*
            return 0x00ff00ff & (internal >> 8);
        */
    }
    
    #[inline(always)] pub fn get_alpha(&self) -> u8 {
        
        todo!();
        /*
            return components.a;
        */
    }
    
    #[inline(always)] pub fn get_red(&self) -> u8 {
        
        todo!();
        /*
            return components.r;
        */
    }
    
    #[inline(always)] pub fn get_green(&self) -> u8 {
        
        todo!();
        /*
            return components.g;
        */
    }
    
    #[inline(always)] pub fn get_blue(&self) -> u8 {
        
        todo!();
        /*
            return components.b;
        */
    }

    /**
      | Copies another pixel colour over this
      | one.
      | 
      | This doesn't blend it - this colour is
      | simply replaced by the other one.
      |
      */
    #[inline(always)] pub fn set<Pixel>(&mut self, src: &Pixel)  {
    
        todo!();
        /*
            internal = src.getNativeARGB();
        */
    }
    
    /**
      | Sets the pixel's colour from individual
      | components.
      |
      */
    pub fn setargb(&mut self, 
        a: u8,
        r: u8,
        g: u8,
        b: u8)  {
        
        todo!();
        /*
            components.b = b;
            components.g = g;
            components.r = r;
            components.a = a;
        */
    }

    /**
      | Blends another pixel onto this one.
      | 
      | This takes into account the opacity
      | of the pixel being overlaid, and blends
      | it accordingly.
      |
      */
    #[inline(always)] pub fn blend<Pixel>(&mut self, src: &Pixel)  {
    
        todo!();
        /*
            auto rb = src.getEvenBytes();
            auto ag = src.getOddBytes();

            const auto alpha = 0x100 - (ag >> 16);

            rb += maskPixelComponents (getEvenBytes() * alpha);
            ag += maskPixelComponents (getOddBytes() * alpha);

            internal = clampPixelComponents (rb) | (clampPixelComponents (ag) << 8);
        */
    }

    /**
      | Blends another pixel onto this one.
      | 
      | This takes into account the opacity
      | of the pixel being overlaid, and blends
      | it accordingly.
      |
      */
    #[inline(always)] pub fn blend_rgb(&mut self, src: PixelRGB)  {
        
        todo!();
        /*
            set (src);
        */
    }

    /**
      | Blends another pixel onto this one,
      | applying an extra multiplier to its
      | opacity.
      | 
      | The opacity of the pixel being overlaid
      | is scaled by the extraAlpha factor before
      | being used, so this can blend semi-transparently
      | from a PixelRGB argument.
      |
      */
    #[inline(always)] pub fn blend_with_alpha<Pixel>(&mut self, 
        src:         &Pixel,
        extra_alpha: u32)  {
    
        todo!();
        /*
            auto rb = maskPixelComponents (extraAlpha * src.getEvenBytes());
            auto ag = maskPixelComponents (extraAlpha * src.getOddBytes());

            const auto alpha = 0x100 - (ag >> 16);

            rb += maskPixelComponents (getEvenBytes() * alpha);
            ag += maskPixelComponents (getOddBytes() * alpha);

            internal = clampPixelComponents (rb) | (clampPixelComponents (ag) << 8);
        */
    }

    /**
      | Blends another pixel with this one,
      | creating a colour that is somewhere
      | between the two, as specified by the
      | amount.
      |
      */
    #[inline(always)] pub fn tween<Pixel>(&mut self, 
        src:    &Pixel,
        amount: u32)  {
    
        todo!();
        /*
            auto dEvenBytes = getEvenBytes();
            dEvenBytes += (((src.getEvenBytes() - dEvenBytes) * amount) >> 8);
            dEvenBytes &= 0x00ff00ff;

            auto dOddBytes = getOddBytes();
            dOddBytes += (((src.getOddBytes() - dOddBytes) * amount) >> 8);
            dOddBytes &= 0x00ff00ff;
            dOddBytes <<= 8;

            dOddBytes |= dEvenBytes;
            internal = dOddBytes;
        */
    }
    
    /**
      | Replaces the colour's alpha value with
      | another one.
      |
      */
    #[inline(always)] pub fn set_alpha(&mut self, new_alpha: u8)  {
        
        todo!();
        /*
            components.a = newAlpha;
        */
    }

    /**
      | Multiplies the colour's alpha value
      | with another one.
      |
      */
    #[inline(always)] pub fn multiply_alpha_i32(&mut self, multiplier: i32)  {
        
        todo!();
        /*
            // increment alpha by 1, so that if multiplier == 255 (full alpha),
            // this function will not change the values.
            ++multiplier;

            internal = ((((uint32) multiplier) * getOddBytes()) & 0xff00ff00)
                    | (((((uint32) multiplier) * getEvenBytes()) >> 8) & 0x00ff00ff);
        */
    }
    
    #[inline(always)] pub fn multiply_alpha_f32(&mut self, multiplier: f32)  {
        
        todo!();
        /*
            multiplyAlpha ((int) (multiplier * 255.0f));
        */
    }
    
    #[inline] pub fn get_unpremultiplied(&self) -> PixelARGB {
        
        todo!();
        /*
            PixelARGB p (internal);
            p.unpremultiply();
            return p;
        */
    }

    /**
      | Premultiplies the pixel's RGB values
      | by its alpha.
      |
      */
    #[inline(always)] pub fn premultiply(&mut self)  {
        
        todo!();
        /*
            const auto alpha = components.a;

            if (alpha < 0xff)
            {
                if (alpha == 0)
                {
                    components.b = 0;
                    components.g = 0;
                    components.r = 0;
                }
                else
                {
                    components.b = (uint8) ((components.b * alpha + 0x7f) >> 8);
                    components.g = (uint8) ((components.g * alpha + 0x7f) >> 8);
                    components.r = (uint8) ((components.r * alpha + 0x7f) >> 8);
                }
            }
        */
    }

    /**
      | Unpremultiplies the pixel's RGB values.
      |
      */
    #[inline(always)] pub fn unpremultiply(&mut self)  {
        
        todo!();
        /*
            const auto alpha = components.a;

            if (alpha < 0xff)
            {
                if (alpha == 0)
                {
                    components.b = 0;
                    components.g = 0;
                    components.r = 0;
                }
                else
                {
                    components.b = (uint8) jmin ((uint32) 0xffu, (components.b * 0xffu) / alpha);
                    components.g = (uint8) jmin ((uint32) 0xffu, (components.g * 0xffu) / alpha);
                    components.r = (uint8) jmin ((uint32) 0xffu, (components.r * 0xffu) / alpha);
                }
            }
        */
    }
    
    #[inline(always)] pub fn desaturate(&mut self)  {
        
        todo!();
        /*
            if (components.a < 0xff && components.a > 0)
            {
                const auto newUnpremultipliedLevel = (0xff * ((int) components.r + (int) components.g + (int) components.b) / (3 * components.a));

                components.r = components.g = components.b
                    = (uint8) ((newUnpremultipliedLevel * components.a + 0x7f) >> 8);
            }
            else
            {
                components.r = components.g = components.b
                    = (uint8) (((int) components.r + (int) components.g + (int) components.b) / 3);
            }
        */
    }
    
    pub fn new_with_internal_value(internal_value: u32) -> Self {
    
        todo!();
        /*
        : internal(internalValue),

        
        */
    }
}

/**
  | Represents a 24-bit RGB pixel, and can
  | perform compositing operations on
  | it.
  | 
  | This is used internally by the imaging
  | classes.
  | 
  | @see PixelARGB
  | 
  | @tags{Graphics}
  |
  */
#[cfg(target_os="macos")]
#[bitfield]
pub struct PixelRGB {
    r: B8,
    g: B8,
    b: B8,
}

#[cfg(not(target_os="macos"))]
#[bitfield]
pub struct PixelRGB {
    b: B8,
    g: B8,
    r: B8,
}

impl Default for PixelRGB {
    
    /**
      | Creates a pixel without defining its
      | colour.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

pub mod pixel_rgb {

    use super::*;

    /**
      | The indexes of the different components
      | in the byte layout of this type of colour.
      |
      */
    #[cfg(target_os="macos")] pub const index_r: usize = 0;
    #[cfg(target_os="macos")] pub const index_g: usize = 1;
    #[cfg(target_os="macos")] pub const index_b: usize = 2;

    #[cfg(not(target_os="macos"))] pub const index_r: usize = 2;
    #[cfg(not(target_os="macos"))] pub const index_g: usize = 1;
    #[cfg(not(target_os="macos"))] pub const index_b: usize = 0;
}

impl PixelRGB {

    /**
      | Returns a uint32 which represents the
      | pixel in a platform dependent format
      | which is compatible with the native
      | format of a PixelARGB.
      | 
      | @see PixelARGB::getNativeARGB
      |
      */
    #[inline(always)] pub fn get_nativeargb(&self) -> u32 {
        
        todo!();
        /*
            #if ALOE_ANDROID
            return (uint32) ((0xffu << 24) | r | ((uint32) g << 8) | ((uint32) b << 16));
           #else
            return (uint32) ((0xffu << 24) | b | ((uint32) g << 8) | ((uint32) r << 16));
           #endif
        */
    }

    /**
      | Returns a uint32 which will be in argb
      | order as if constructed with the following
      | mask operation ((alpha << 24) | (red
      | << 16) | (green << 8) | blue).
      |
      */
    #[inline(always)] pub fn get_in_argb_mask_order(&self) -> u32 {
        
        todo!();
        /*
            #if ALOE_ANDROID
            return (uint32) ((0xffu << 24) | b | ((uint32) g << 8) | ((uint32) r << 16));
           #else
            return getNativeARGB();
           #endif
        */
    }

    /**
      | Returns a uint32 which when written
      | to memory, will be in the order a, r, g,
      | b. In other words, if the return-value
      | is read as a uint8 array then the elements
      | will be in the order of a, r, g, b
      |
      */
    #[inline] pub fn get_in_argb_memory_order(&self) -> u32 {
        
        todo!();
        /*
            #if ALOE_BIG_ENDIAN
            return getInARGBMaskOrder();
           #else
            return (uint32) ((b << 24) | (g << 16) | (r << 8) | 0xff);
           #endif
        */
    }

    /**
      | Return channels with an even index and
      | insert zero bytes between them. This
      | is useful for blending operations.
      | The exact channels which are returned
      | is platform dependent but compatible
      | with the return value of getEvenBytes
      | of the PixelARGB class.
      | 
      | @see PixelARGB::getEvenBytes
      |
      */
    #[inline(always)] pub fn get_even_bytes(&self) -> u32 {
        
        todo!();
        /*
            #if ALOE_ANDROID
            return (uint32) (r | (b << 16));
           #else
            return (uint32) (b | (r << 16));
           #endif
        */
    }

    /**
      | Return channels with an odd index and
      | insert zero bytes between them. This
      | is useful for blending operations.
      | The exact channels which are returned
      | is platform dependent but compatible
      | with the return value of getOddBytes
      | of the PixelARGB class.
      | 
      | @see PixelARGB::getOddBytes
      |
      */
    #[inline(always)] pub fn get_odd_bytes(&self) -> u32 {
        
        todo!();
        /*
            return (uint32) 0xff0000 | g;
        */
    }
    
    #[inline(always)] pub fn get_alpha(&self) -> u8 {
        
        todo!();
        /*
            return 0xff;
        */
    }
    
    #[inline(always)] pub fn get_red(&self) -> u8 {
        
        todo!();
        /*
            return r;
        */
    }
    
    #[inline(always)] pub fn get_green(&self) -> u8 {
        
        todo!();
        /*
            return g;
        */
    }
    
    #[inline(always)] pub fn get_blue(&self) -> u8 {
        
        todo!();
        /*
            return b;
        */
    }
    
    /**
      | Copies another pixel colour over this
      | one.
      | 
      | This doesn't blend it - this colour is
      | simply replaced by the other one. Because
      | PixelRGB has no alpha channel, any alpha
      | value in the source pixel is thrown away.
      |
      */
    #[inline(always)] pub fn set<Pixel>(&mut self, src: &Pixel)  {
    
        todo!();
        /*
            b = src.getBlue();
            g = src.getGreen();
            r = src.getRed();
        */
    }

    /**
      | Sets the pixel's colour from individual
      | components.
      |
      */
    pub fn setargb(&mut self, 
        _0:    u8,
        red:   u8,
        green: u8,
        blue:  u8)  {
        
        todo!();
        /*
            r = red;
            g = green;
            b = blue;
        */
    }
    
    /**
      | Blends another pixel onto this one.
      | 
      | This takes into account the opacity
      | of the pixel being overlaid, and blends
      | it accordingly.
      |
      */
    #[inline(always)] pub fn blend<Pixel>(&mut self, src: &Pixel)  {
    
        todo!();
        /*
            const auto alpha = (uint32) (0x100 - src.getAlpha());

            // getEvenBytes returns 0x00rr00bb on non-android
            const auto rb = clampPixelComponents (src.getEvenBytes() + maskPixelComponents (getEvenBytes() * alpha));
            // getOddBytes returns 0x00aa00gg on non-android
            const auto ag = clampPixelComponents (src.getOddBytes() + ((g * alpha) >> 8));

            g = (uint8) (ag & 0xff);

           #if ALOE_ANDROID
            b = (uint8) (rb >> 16);
            r = (uint8) (rb & 0xff);
           #else
            r = (uint8) (rb >> 16);
            b = (uint8) (rb & 0xff);
           #endif
        */
    }
    
    #[inline(always)] pub fn blend_with_rgb(&mut self, src: PixelRGB)  {
        
        todo!();
        /*
            set (src);
        */
    }

    /**
      | Blends another pixel onto this one,
      | applying an extra multiplier to its
      | opacity.
      | 
      | The opacity of the pixel being overlaid
      | is scaled by the extraAlpha factor before
      | being used, so this can blend semi-transparently
      | from a PixelRGB argument.
      |
      */
    #[inline(always)] pub fn blend_with_alpha<Pixel>(
        &mut self, 
        src:         &Pixel,
        extra_alpha: u32)  {
    
        todo!();
        /*
            auto ag = maskPixelComponents (extraAlpha * src.getOddBytes());
            auto rb = maskPixelComponents (extraAlpha * src.getEvenBytes());

            const auto alpha = 0x100 - (ag >> 16);

            ag = clampPixelComponents (ag + (g * alpha >> 8));
            rb = clampPixelComponents (rb + maskPixelComponents (getEvenBytes() * alpha));

            g = (uint8) (ag & 0xff);

           #if ALOE_ANDROID
            b = (uint8) (rb >> 16);
            r = (uint8) (rb & 0xff);
           #else
            r = (uint8) (rb >> 16);
            b = (uint8) (rb & 0xff);
           #endif
        */
    }

    /**
      | Blends another pixel with this one,
      | creating a colour that is somewhere
      | between the two, as specified by the
      | amount.
      |
      */
    #[inline(always)] pub fn tween<Pixel>(&mut self, 
        src:    &Pixel,
        amount: u32)  {
    
        todo!();
        /*
            auto dEvenBytes = getEvenBytes();
            dEvenBytes += (((src.getEvenBytes() - dEvenBytes) * amount) >> 8);

            auto dOddBytes = getOddBytes();
            dOddBytes += (((src.getOddBytes() - dOddBytes) * amount) >> 8);

            g = (uint8) (dOddBytes & 0xff);  // dOddBytes =  0x00aa00gg

           #if ALOE_ANDROID
            r = (uint8) (dEvenBytes & 0xff); // dEvenBytes = 0x00bb00rr
            b = (uint8) (dEvenBytes >> 16);
           #else
            b = (uint8) (dEvenBytes & 0xff); // dEvenBytes = 0x00rr00bb
            r = (uint8) (dEvenBytes >> 16);
           #endif
        */
    }
    
    /**
      | Unpremultiplies the pixel's RGB values.
      |
      */
    #[inline(always)] pub fn unpremultiply(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[inline(always)] pub fn desaturate(&mut self)  {
        
        todo!();
        /*
            r = g = b = (uint8) (((int) r + (int) g + (int) b) / 3);
        */
    }
    
    pub fn new_with_internal_value(internal: u32) -> Self {
    
        todo!();
        /*


            #if ALOE_ANDROID
            b = (uint8) (internal >> 16);
            g = (uint8) (internal >> 8);
            r = (uint8) (internal);
          #else
            r = (uint8) (internal >> 16);
            g = (uint8) (internal >> 8);
            b = (uint8) (internal);
          #endif
        */
    }
}

/**
  | Represents an 8-bit single-channel
  | pixel, and can perform compositing
  | operations on it.
  | 
  | This is used internally by the imaging
  | classes.
  | 
  | @see PixelARGB, PixelRGB
  | 
  | @tags{Graphics}
  |
  */
#[bitfield]
pub struct PixelAlpha {
    a: B8,
}

impl Default for PixelAlpha {
    
    /**
      | Creates a pixel without defining its
      | colour.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

pub mod pixel_alpha {

    use super::*;

    /**
      | The indexes of the different components
      | in the byte layout of this type of colour.
      |
      */
    pub const index_a: usize = 0;
}

impl PixelAlpha {

    /**
      | Returns a uint32 which represents the
      | pixel in a platform dependent format
      | which is compatible with the native
      | format of a PixelARGB.
      | 
      | @see PixelARGB::getNativeARGB
      |
      */
    #[inline(always)] pub fn get_nativeargb(&self) -> u32 {
        
        todo!();
        /*
            return (uint32) ((a << 24) | (a << 16) | (a << 8) | a);
        */
    }

    /**
      | Returns a uint32 which will be in argb
      | order as if constructed with the following
      | mask operation ((alpha << 24) | (red
      | << 16) | (green << 8) | blue).
      |
      */
    #[inline(always)] pub fn get_in_argb_mask_order(&self) -> u32 {
        
        todo!();
        /*
            return getNativeARGB();
        */
    }

    /**
      | Returns a uint32 which when written
      | to memory, will be in the order a, r, g,
      | b. In other words, if the return-value
      | is read as a uint8 array then the elements
      | will be in the order of a, r, g, b
      |
      */
    #[inline] pub fn get_in_argb_memory_order(&self) -> u32 {
        
        todo!();
        /*
            return getNativeARGB();
        */
    }

    /**
      | Return channels with an even index and
      | insert zero bytes between them. This
      | is useful for blending operations.
      | The exact channels which are returned
      | is platform dependent but compatible
      | with the return value of getEvenBytes
      | of the PixelARGB class.
      | 
      | @see PixelARGB::getEvenBytes
      |
      */
    #[inline(always)] pub fn get_even_bytes(&self) -> u32 {
        
        todo!();
        /*
            return (uint32) ((a << 16) | a);
        */
    }

    /**
      | Return channels with an odd index and
      | insert zero bytes between them. This
      | is useful for blending operations.
      | The exact channels which are returned
      | is platform dependent but compatible
      | with the return value of getOddBytes
      | of the PixelARGB class.
      | 
      | @see PixelARGB::getOddBytes
      |
      */
    #[inline(always)] pub fn get_odd_bytes(&self) -> u32 {
        
        todo!();
        /*
            return (uint32) ((a << 16) | a);
        */
    }
    
    #[inline(always)] pub fn get_alpha(&self) -> u8 {
        
        todo!();
        /*
            return a;
        */
    }
    
    #[inline(always)] pub fn get_red(&self) -> u8 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[inline(always)] pub fn get_green(&self) -> u8 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[inline(always)] pub fn get_blue(&self) -> u8 {
        
        todo!();
        /*
            return 0;
        */
    }

    /**
      | Copies another pixel colour over this
      | one.
      | 
      | This doesn't blend it - this colour is
      | simply replaced by the other one.
      |
      */
    #[inline(always)] pub fn set<Pixel>(&mut self, src: &Pixel)  {
    
        todo!();
        /*
            a = src.getAlpha();
        */
    }

    /**
      | Sets the pixel's colour from individual
      | components.
      |
      */
    #[inline(always)] pub fn setargb(&mut self, 
        a:  u8,
        _1: u8,
        _2: u8,
        _3: u8)  {
        
        todo!();
        /*
            a = a_;
        */
    }

    /**
      | Blends another pixel onto this one.
      | 
      | This takes into account the opacity
      | of the pixel being overlaid, and blends
      | it accordingly.
      |
      */
    #[inline(always)] pub fn blend<Pixel>(&mut self, src: &Pixel)  {
    
        todo!();
        /*
            const auto srcA = src.getAlpha();
            a = (uint8) ((a * (0x100 - srcA) >> 8) + srcA);
        */
    }

    /**
      | Blends another pixel onto this one,
      | applying an extra multiplier to its
      | opacity.
      | 
      | The opacity of the pixel being overlaid
      | is scaled by the extraAlpha factor before
      | being used, so this can blend semi-transparently
      | from a PixelRGB argument.
      |
      */
    #[inline(always)] pub fn blend_with_alpha<Pixel>(
        &mut self, 
        src:         &Pixel,
        extra_alpha: u32)  {
    
        todo!();
        /*
            ++extraAlpha;
            const auto srcAlpha = (int) ((extraAlpha * src.getAlpha()) >> 8);
            a = (uint8) ((a * (0x100 - srcAlpha) >> 8) + srcAlpha);
        */
    }

    /**
      | Blends another pixel with this one,
      | creating a colour that is somewhere
      | between the two, as specified by the
      | amount.
      |
      */
    #[inline(always)] pub fn tween<Pixel>(&mut self, 
        src:    &Pixel,
        amount: u32)  {
    
        todo!();
        /*
            a += ((src.getAlpha() - a) * amount) >> 8;
        */
    }
    
    /**
      | Replaces the colour's alpha value with
      | another one.
      |
      */
    #[inline(always)] pub fn set_alpha(&mut self, new_alpha: u8)  {
        
        todo!();
        /*
            a = newAlpha;
        */
    }

    /**
      | Multiplies the colour's alpha value
      | with another one.
      |
      */
    #[inline(always)] pub fn multiply_alpha_i32(&mut self, multiplier: i32)  {
        
        todo!();
        /*
            ++multiplier;
            a = (uint8) ((a * multiplier) >> 8);
        */
    }
    
    /**
      | Multiplies the colour's alpha value
      | with another one.
      |
      */
    #[inline(always)] pub fn multiply_alpha_f32(&mut self, multiplier: f32)  {
        
        todo!();
        /*
            a = (uint8) (a * multiplier);
        */
    }

    pub fn new_with_internal_value(internal: u32) -> Self {
    
        todo!();
        /*
            : a ((uint8) (internal >> 24))
        */
    }
}
