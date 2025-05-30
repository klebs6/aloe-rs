crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_Colour.h]

/**
  | Represents a colour, also including
  | a transparency value.
  | 
  | The colour is stored internally as unsigned
  | 8-bit red, green, blue and alpha values.
  | 
  | @tags{Graphics}
  |
  */
#[derive(Default)] // Creates a transparent black colour
pub struct Colour {
    argb: PixelARGB, // default = { 0, 0, 0, 0  }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_Colour.cpp]
impl Colour {

    /**
      | Returns the red component of this colour.
      | 
      | -----------
      | @return
      | 
      | a value between 0x00 and 0xff.
      |
      */
    pub fn get_red(&self) -> u8 {
        
        todo!();
        /*
            return argb.getRed();
        */
    }

    /**
      | Returns the green component of this
      | colour.
      | 
      | -----------
      | @return
      | 
      | a value between 0x00 and 0xff.
      |
      */
    pub fn get_green(&self) -> u8 {
        
        todo!();
        /*
            return argb.getGreen();
        */
    }

    /**
      | Returns the blue component of this colour.
      | 
      | -----------
      | @return
      | 
      | a value between 0x00 and 0xff.
      |
      */
    pub fn get_blue(&self) -> u8 {
        
        todo!();
        /*
            return argb.getBlue();
        */
    }

    /**
      | Returns the colour's alpha (opacity).
      | 
      | Alpha of 0x00 is completely transparent,
      | 0xff is completely opaque.
      |
      */
    pub fn get_alpha(&self) -> u8 {
        
        todo!();
        /*
            return argb.getAlpha();
        */
    }
}

pub mod colour_helpers {
    use super::*;

    pub fn float_to_uint8(n: f32) -> u8 {
        
        todo!();
        /*
            return n <= 0.0f ? 0 : (n >= 1.0f ? 255 : (uint8) roundToInt (n * 255.0f));
        */
    }

    pub fn get_hue(col: Colour) -> f32 {
        
        todo!();
        /*
            auto r = (int) col.getRed();
                auto g = (int) col.getGreen();
                auto b = (int) col.getBlue();

                auto hi = jmax (r, g, b);
                auto lo = jmin (r, g, b);

                float hue = 0.0f;

                if (hi > 0 && ! approximatelyEqual (hi, lo))
                {
                    auto invDiff = 1.0f / (float) (hi - lo);

                    auto red   = (float) (hi - r) * invDiff;
                    auto green = (float) (hi - g) * invDiff;
                    auto blue  = (float) (hi - b) * invDiff;

                    if      (r == hi)  hue = blue - green;
                    else if (g == hi)  hue = 2.0f + red - blue;
                    else               hue = 4.0f + green - red;

                    hue *= 1.0f / 6.0f;

                    if (hue < 0.0f)
                        hue += 1.0f;
                }

                return hue;
        */
    }

    ///-------------------
    pub struct HSL {
        hue:        f32, // default = 0.0
        saturation: f32, // default = 0.0
        lightness:  f32, // default = 0.0
    }

    impl HSL {

        pub fn new(col: Colour) -> Self {
        
            todo!();
            /*


                auto r = (int) col.getRed();
                    auto g = (int) col.getGreen();
                    auto b = (int) col.getBlue();

                    auto hi = jmax (r, g, b);
                    auto lo = jmin (r, g, b);

                    if (hi < 0)
                        return;

                    lightness = ((float) (hi + lo) / 2.0f) / 255.0f;

                    if (lightness <= 0.0f)
                        return;

                    hue = getHue (col);

                    if (1.0f <= lightness)
                        return;

                    auto denominator = 1.0f - std::abs ((2.0f * lightness) - 1.0f);
                    saturation = ((float) (hi - lo) / 255.0f) / denominator;
            */
        }
        
        pub fn to_colour(&self, original: Colour) -> Colour {
            
            todo!();
            /*
                return Colour::fromHSL (hue, saturation, lightness, original.getAlpha());
            */
        }
        
        pub fn torgb(
            h:     f32,
            s:     f32,
            l:     f32,
            alpha: u8) -> PixelARGB {
            
            todo!();
            /*
                auto v = l < 0.5f ? l * (1.0f + s) : l + s - (l * s);

                    if (approximatelyEqual (v, 0.0f))
                        return PixelARGB (alpha, 0, 0, 0);

                    auto min = (2.0f * l) - v;
                    auto sv = (v - min) / v;

                    h = ((h - std::floor (h)) * 360.0f) / 60.0f;
                    auto f = h - std::floor (h);
                    auto vsf = v * sv * f;
                    auto mid1 = min + vsf;
                    auto mid2 = v - vsf;

                    if      (h < 1.0f)  return PixelARGB (alpha, floatToUInt8 (v),    floatToUInt8 (mid1), floatToUInt8 (min));
                    else if (h < 2.0f)  return PixelARGB (alpha, floatToUInt8 (mid2), floatToUInt8 (v),    floatToUInt8 (min));
                    else if (h < 3.0f)  return PixelARGB (alpha, floatToUInt8 (min),  floatToUInt8 (v),    floatToUInt8 (mid1));
                    else if (h < 4.0f)  return PixelARGB (alpha, floatToUInt8 (min),  floatToUInt8 (mid2), floatToUInt8 (v));
                    else if (h < 5.0f)  return PixelARGB (alpha, floatToUInt8 (mid1), floatToUInt8 (min),  floatToUInt8 (v));
                    else if (h < 6.0f)  return PixelARGB (alpha, floatToUInt8 (v),    floatToUInt8 (min),  floatToUInt8 (mid2));

                    return PixelARGB (alpha, 0, 0, 0);
            */
        }
    }
    
    ///-------------------
    pub struct HSB {
        hue:        f32, // default = 0.0
        saturation: f32, // default = 0.0
        brightness: f32, // default = 0.0
    }

    impl HSB {

        pub fn new(col: Colour) -> Self {
        
            todo!();
            /*


                auto r = (int) col.getRed();
                    auto g = (int) col.getGreen();
                    auto b = (int) col.getBlue();

                    auto hi = jmax (r, g, b);
                    auto lo = jmin (r, g, b);

                    if (hi > 0)
                    {
                        saturation = (float) (hi - lo) / (float) hi;

                        if (saturation > 0.0f)
                            hue = getHue (col);

                        brightness = (float) hi / 255.0f;
                    }
            */
        }
        
        pub fn to_colour(&self, original: Colour) -> Colour {
            
            todo!();
            /*
                return Colour (hue, saturation, brightness, original.getAlpha());
            */
        }
        
        pub fn torgb(
            h:     f32,
            s:     f32,
            v:     f32,
            alpha: u8) -> PixelARGB {
            
            todo!();
            /*
                v = jlimit (0.0f, 255.0f, v * 255.0f);
                    auto intV = (uint8) roundToInt (v);

                    if (s <= 0)
                        return PixelARGB (alpha, intV, intV, intV);

                    s = jmin (1.0f, s);
                    h = ((h - std::floor (h)) * 360.0f) / 60.0f;
                    auto f = h - std::floor (h);
                    auto x = (uint8) roundToInt (v * (1.0f - s));

                    if (h < 1.0f)   return PixelARGB (alpha, intV,    (uint8) roundToInt (v * (1.0f - (s * (1.0f - f)))), x);
                    if (h < 2.0f)   return PixelARGB (alpha,          (uint8) roundToInt (v * (1.0f - s * f)), intV, x);
                    if (h < 3.0f)   return PixelARGB (alpha, x, intV, (uint8) roundToInt (v * (1.0f - (s * (1.0f - f)))));
                    if (h < 4.0f)   return PixelARGB (alpha, x,       (uint8) roundToInt (v * (1.0f - s * f)), intV);
                    if (h < 5.0f)   return PixelARGB (alpha,          (uint8) roundToInt (v * (1.0f - (s * (1.0f - f)))), x, intV);
                    return                 PixelARGB (alpha, intV, x, (uint8) roundToInt (v * (1.0f - s * f)));
            */
        }
    }
    
    ///-------------------
    pub struct yiq {
        y:     f32, // default = 0.0
        i:     f32, // default = 0.0
        q:     f32, // default = 0.0
        alpha: f32, // default = 0.0
    }

    impl yiq {

        pub fn new(c: Colour) -> Self {
        
            todo!();
            /*


                auto r = c.getFloatRed();
                    auto g = c.getFloatGreen();
                    auto b = c.getFloatBlue();

                    y = 0.2999f * r + 0.5870f * g + 0.1140f * b;
                    i = 0.5957f * r - 0.2744f * g - 0.3212f * b;
                    q = 0.2114f * r - 0.5225f * g - 0.3113f * b;
                    alpha = c.getFloatAlpha();
            */
        }
        
        pub fn to_colour(&self) -> Colour {
            
            todo!();
            /*
                return Colour::fromFloatRGBA (y + 0.9563f * i + 0.6210f * q,
                                                  y - 0.2721f * i - 0.6474f * q,
                                                  y - 1.1070f * i + 1.7046f * q,
                                                  alpha);
            */
        }
    }
}

impl PartialEq<Colour> for Colour {
    
    #[inline] fn eq(&self, other: &Colour) -> bool {
        todo!();
        /*
            return argb.getNativeARGB() == other.argb.getNativeARGB();
        */
    }
}

impl Eq for Colour {}

impl Colour {
    
    /**
      | Creates a colour from a 32-bit ARGB value.
      | 
      | The format of this number is: ((alpha
      | << 24) | (red << 16) | (green << 8) | blue).
      | 
      | All components in the range 0x00 to 0xff.
      | An alpha of 0x00 is completely transparent,
      | alpha of 0xff is opaque.
      | 
      | @see getPixelARGB
      |
      */
    pub const fn new_from_argb(col: u32) -> Self {

        let a = ((col >> 24) & 0xff) as u8;
        let r = ((col >> 16) & 0xff) as u8;
        let g = ((col >> 8) & 0xff) as u8;
        let b = (col & 0xff) as u8;

        Self {
            argb: PixelARGB::new(a, r, g, b),
        }
    }

    /**
      | Creates an opaque colour using 8-bit
      | red, green and blue values
      |
      */
    pub fn new_from_rgb(
        red:   u8,
        green: u8,
        blue:  u8) -> Self {
    
        todo!();
        /*


            argb.setARGB (0xff, red, green, blue);
        */
    }
    
    /**
      | Creates an opaque colour using 8-bit
      | red, green and blue values
      |
      */
    pub fn fromrgb(&mut self, 
        red:   u8,
        green: u8,
        blue:  u8) -> Colour {
        
        todo!();
        /*
            return Colour (red, green, blue);
        */
    }
    
    /**
      | Creates a colour using 8-bit red, green,
      | blue and alpha values.
      |
      */
    pub fn new_from_rgba(
        red:   u8,
        green: u8,
        blue:  u8,
        alpha: u8) -> Self {
    
        todo!();
        /*


            argb.setARGB (alpha, red, green, blue);
        */
    }
    
    /**
      | Creates a colour using 8-bit red, green,
      | blue and alpha values.
      |
      */
    pub fn fromrgba(&mut self, 
        red:   u8,
        green: u8,
        blue:  u8,
        alpha: u8) -> Colour {
        
        todo!();
        /*
            return Colour (red, green, blue, alpha);
        */
    }
    
    /**
      | Creates a colour from 8-bit red, green,
      | and blue values, and a floating-point
      | alpha.
      | 
      | Alpha of 0.0 is transparent, alpha of
      | 1.0f is opaque. Values outside the valid
      | range will be clipped.
      |
      */
    pub fn new_from_rgba_alpha_f32(
        red:   u8,
        green: u8,
        blue:  u8,
        alpha: f32) -> Self {
    
        todo!();
        /*


            argb.setARGB (ColourHelpers::floatToUInt8 (alpha), red, green, blue);
        */
    }
    
    /**
      | Creates a colour using floating point
      | red, green, blue and alpha values.
      | 
      | Numbers outside the range 0..1 will
      | be clipped.
      |
      */
    pub fn from_floatrgba(&mut self, 
        red:   f32,
        green: f32,
        blue:  f32,
        alpha: f32) -> Colour {
        
        todo!();
        /*
            return Colour (ColourHelpers::floatToUInt8 (red),
                       ColourHelpers::floatToUInt8 (green),
                       ColourHelpers::floatToUInt8 (blue), alpha);
        */
    }
    
    /**
      | Creates a colour using floating point
      | hue, saturation and brightness values,
      | and an 8-bit alpha.
      | 
      | The floating point values must be between
      | 0.0 and 1.0.
      | 
      | An alpha of 0x00 is completely transparent,
      | alpha of 0xff is opaque.
      | 
      | Values outside the valid range will
      | be clipped.
      |
      */
    pub fn new_from_hsba(
        hue:        f32,
        saturation: f32,
        brightness: f32,
        alpha:      f32) -> Self {
    
        todo!();
        /*


            : argb (ColourHelpers::HSB::toRGB (hue, saturation, brightness, ColourHelpers::floatToUInt8 (alpha)))
        */
    }
    
    /**
      | Creates a colour using floating point
      | hue, saturation, brightness and alpha
      | values.
      | 
      | All values must be between 0.0 and 1.0.
      | 
      | Numbers outside the valid range will
      | be clipped.
      |
      */
    pub fn fromhsv(&mut self, 
        hue:        f32,
        saturation: f32,
        brightness: f32,
        alpha:      f32) -> Colour {
        
        todo!();
        /*
            return Colour (hue, saturation, brightness, alpha);
        */
    }
    
    /**
      | Creates a colour using floating point
      | hue, saturation, lightness and alpha
      | values.
      | 
      | All values must be between 0.0 and 1.0.
      | 
      | Numbers outside the valid range will
      | be clipped.
      |
      */
    pub fn fromhsl(&mut self, 
        hue:        f32,
        saturation: f32,
        lightness:  f32,
        alpha:      f32) -> Colour {
        
        todo!();
        /*
            Colour hslColour;
        hslColour.argb = ColourHelpers::HSL::toRGB (hue, saturation, lightness, ColourHelpers::floatToUInt8 (alpha));

        return hslColour;
        */
    }
    
    /**
      | Creates a colour using floating point
      | hue, saturation, brightness and alpha
      | values.
      | 
      | All values must be between 0.0 and 1.0.
      | 
      | Numbers outside the valid range will
      | be clipped.
      |
      */
    pub fn new_with_u8_alpha(
        hue:        f32,
        saturation: f32,
        brightness: f32,
        alpha:      u8) -> Self {
    
        todo!();
        /*


            : argb (ColourHelpers::HSB::toRGB (hue, saturation, brightness, alpha))
        */
    }
    
    /**
      | Creates a colour using a PixelARGB object.
      | This function assumes that the argb
      | pixel is not premultiplied.
      |
      */
    pub fn new_argb(argb: PixelARGB) -> Self {
    
        todo!();
        /*


            : argb (argb_)
        */
    }
    
    /**
      | Creates a colour using a PixelRGB object.
      |
      */
    pub fn new_rgb(rgb: PixelRGB) -> Self {
    
        todo!();
        /*


            : argb (Colour (rgb.getInARGBMaskOrder()).argb)
        */
    }
    
    /**
      | Creates a colour using a PixelAlpha
      | object.
      |
      */
    pub fn new_alpha(alpha: PixelAlpha) -> Self {
    
        todo!();
        /*


            : argb (Colour (alpha.getInARGBMaskOrder()).argb)
        */
    }
    
    /**
      | Returns a premultiplied ARGB pixel
      | object that represents this colour.
      |
      */
    pub fn get_pixelargb(&self) -> PixelARGB {
        
        todo!();
        /*
            PixelARGB p (argb);
        p.premultiply();
        return p;
        */
    }
    
    /**
      | Returns a 32-bit integer that represents
      | this colour.
      | 
      | The format of this number is: ((alpha
      | << 24) | (red << 16) | (green << 8) | blue).
      |
      */
    pub fn getargb(&self) -> u32 {
        
        todo!();
        /*
            return argb.getInARGBMaskOrder();
        */
    }
    
    /**
      | Returns true if this colour is completely
      | transparent.
      | 
      | Equivalent to (getAlpha() == 0x00).
      |
      */
    pub fn is_transparent(&self) -> bool {
        
        todo!();
        /*
            return getAlpha() == 0;
        */
    }
    
    /**
      | Returns true if this colour is completely
      | opaque.
      | 
      | Equivalent to (getAlpha() == 0xff).
      |
      */
    pub fn is_opaque(&self) -> bool {
        
        todo!();
        /*
            return getAlpha() == 0xff;
        */
    }
    
    /**
      | Returns a colour that's the same colour
      | as this one, but with a new alpha value.
      |
      */
    pub fn with_alpha_u8(&self, new_alpha: u8) -> Colour {
        
        todo!();
        /*
            PixelARGB newCol (argb);
        newCol.setAlpha (newAlpha);
        return Colour (newCol);
        */
    }
    
    /**
      | Returns a colour that's the same colour
      | as this one, but with a new alpha value.
      |
      */
    pub fn with_alpha_f32(&self, new_alpha: f32) -> Colour {
        
        todo!();
        /*
            jassert (newAlpha >= 0 && newAlpha <= 1.0f);

        PixelARGB newCol (argb);
        newCol.setAlpha (ColourHelpers::floatToUInt8 (newAlpha));
        return Colour (newCol);
        */
    }
    
    /**
      | Returns a colour that's the same colour
      | as this one, but with a modified alpha
      | value.
      | 
      | The new colour's alpha will be this object's
      | alpha multiplied by the value passed-in.
      |
      */
    pub fn with_multiplied_alpha(&self, alpha_multiplier: f32) -> Colour {
        
        todo!();
        /*
            jassert (alphaMultiplier >= 0);

        PixelARGB newCol (argb);
        newCol.setAlpha ((uint8) jmin (0xff, roundToInt (alphaMultiplier * newCol.getAlpha())));
        return Colour (newCol);
        */
    }
    
    /**
      | Returns a colour that is the result of
      | alpha-compositing a new colour over
      | this one.
      | 
      | If the foreground colour is semi-transparent,
      | it is blended onto this colour accordingly.
      |
      */
    pub fn overlaid_with(&self, src: Colour) -> Colour {
        
        todo!();
        /*
            auto destAlpha = getAlpha();

        if (destAlpha <= 0)
            return src;

        auto invA = 0xff - (int) src.getAlpha();
        auto resA = 0xff - (((0xff - destAlpha) * invA) >> 8);

        if (resA <= 0)
            return *this;

        auto da = (invA * destAlpha) / resA;

        return Colour ((uint8) (src.getRed()   + ((((int) getRed()   - src.getRed())   * da) >> 8)),
                       (uint8) (src.getGreen() + ((((int) getGreen() - src.getGreen()) * da) >> 8)),
                       (uint8) (src.getBlue()  + ((((int) getBlue()  - src.getBlue())  * da) >> 8)),
                       (uint8) resA);
        */
    }
    
    /**
      | Returns a colour that lies somewhere
      | between this one and another.
      | 
      | If amountOfOther is zero, the result
      | is 100% this colour, if amountOfOther
      | is 1.0, the result is 100% of the other
      | colour.
      |
      */
    pub fn interpolated_with(&self, 
        other:               Colour,
        proportion_of_other: f32) -> Colour {
        
        todo!();
        /*
            if (proportionOfOther <= 0)
            return *this;

        if (proportionOfOther >= 1.0f)
            return other;

        PixelARGB c1 (getPixelARGB());
        PixelARGB c2 (other.getPixelARGB());
        c1.tween (c2, (uint32) roundToInt (proportionOfOther * 255.0f));
        c1.unpremultiply();

        return Colour (c1);
        */
    }
    
    /**
      | Returns the red component of this colour
      | as a floating point value.
      | 
      | -----------
      | @return
      | 
      | a value between 0.0 and 1.0
      |
      */
    pub fn get_float_red(&self) -> f32 {
        
        todo!();
        /*
            return getRed()   / 255.0f;
        */
    }
    
    /**
      | Returns the green component of this
      | colour as a floating point value.
      | 
      | -----------
      | @return
      | 
      | a value between 0.0 and 1.0
      |
      */
    pub fn get_float_green(&self) -> f32 {
        
        todo!();
        /*
            return getGreen() / 255.0f;
        */
    }
    
    /**
      | Returns the blue component of this colour
      | as a floating point value.
      | 
      | -----------
      | @return
      | 
      | a value between 0.0 and 1.0
      |
      */
    pub fn get_float_blue(&self) -> f32 {
        
        todo!();
        /*
            return getBlue()  / 255.0f;
        */
    }
    
    /**
      | Returns the colour's alpha (opacity)
      | as a floating point value.
      | 
      | Alpha of 0.0 is completely transparent,
      | 1.0 is completely opaque.
      |
      */
    pub fn get_float_alpha(&self) -> f32 {
        
        todo!();
        /*
            return getAlpha() / 255.0f;
        */
    }
    
    /**
      | Returns the colour's hue, saturation
      | and brightness components all at once.
      | 
      | The values returned are in the range
      | 0.0 to 1.0
      |
      */
    pub fn gethsb(&self, 
        h: &mut f32,
        s: &mut f32,
        v: &mut f32)  {
        
        todo!();
        /*
            ColourHelpers::HSB hsb (*this);
        h = hsb.hue;
        s = hsb.saturation;
        v = hsb.brightness;
        */
    }
    
    /**
      | Returns the colour's hue, saturation
      | and lightness components all at once.
      | 
      | The values returned are in the range
      | 0.0 to 1.0
      |
      */
    pub fn gethsl(&self, 
        h: &mut f32,
        s: &mut f32,
        l: &mut f32)  {
        
        todo!();
        /*
            ColourHelpers::HSL hsl (*this);
        h = hsl.hue;
        s = hsl.saturation;
        l = hsl.lightness;
        */
    }
    
    /**
      | Returns the colour's hue component.
      | 
      | The value returned is in the range 0.0
      | to 1.0
      |
      */
    pub fn get_hue(&self) -> f32 {
        
        todo!();
        /*
            return ColourHelpers::HSB (*this).hue;
        */
    }
    
    /**
      | Returns the colour's saturation component.
      | 
      | The value returned is in the range 0.0
      | to 1.0
      |
      */
    pub fn get_saturation(&self) -> f32 {
        
        todo!();
        /*
            return ColourHelpers::HSB (*this).saturation;
        */
    }
    
    /**
      | Returns the colour's brightness component.
      | 
      | The value returned is in the range 0.0
      | to 1.0
      |
      */
    pub fn get_brightness(&self) -> f32 {
        
        todo!();
        /*
            return ColourHelpers::HSB (*this).brightness;
        */
    }
    
    /**
      | Returns the colour's saturation component
      | as represented in the HSL colour space.
      | 
      | The value returned is in the range 0.0
      | to 1.0
      |
      */
    pub fn get_saturationhsl(&self) -> f32 {
        
        todo!();
        /*
            return ColourHelpers::HSL (*this).saturation;
        */
    }
    
    /**
      | Returns the colour's lightness component.
      | 
      | The value returned is in the range 0.0
      | to 1.0
      |
      */
    pub fn get_lightness(&self) -> f32 {
        
        todo!();
        /*
            return ColourHelpers::HSL (*this).lightness;
        */
    }
    
    /**
      | Returns a copy of this colour with a different
      | hue.
      |
      */
    pub fn with_hue(&self, h: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSB hsb (*this); hsb.hue = h;        return hsb.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with a different
      | saturation.
      |
      */
    pub fn with_saturation(&self, s: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSB hsb (*this); hsb.saturation = s; return hsb.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with a different
      | brightness. @see brighter, darker,
      | withMultipliedBrightness
      |
      */
    pub fn with_brightness(&self, v: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSB hsb (*this); hsb.brightness = v; return hsb.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with a different
      | saturation in the HSL colour space.
      |
      */
    pub fn with_saturationhsl(&self, s: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSL hsl (*this); hsl.saturation = s; return hsl.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with a different
      | lightness. @see lighter, darker, withMultipliedLightness
      |
      */
    pub fn with_lightness(&self, l: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSL hsl (*this); hsl.lightness = l;  return hsl.toColour (*this);
        */
    }
    
    /**
      | Returns a skewed brightness value,
      | adjusted to better reflect the way the
      | human eye responds to different colour
      | channels. This makes it better than
      | getBrightness() for comparing differences
      | in brightness.
      |
      */
    pub fn get_perceived_brightness(&self) -> f32 {
        
        todo!();
        /*
            return std::sqrt (0.241f * square (getFloatRed())
                        + 0.691f * square (getFloatGreen())
                        + 0.068f * square (getFloatBlue()));
        */
    }
    
    /**
      | Returns a copy of this colour with its
      | hue rotated.
      | 
      | The new colour's hue is ((this->getHue()
      | + amountToRotate) % 1.0) @see brighter,
      | darker, withMultipliedBrightness
      |
      */
    pub fn with_rotated_hue(&self, amount_to_rotate: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSB hsb (*this);
        hsb.hue += amountToRotate;
        return hsb.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with its
      | saturation multiplied by the given
      | value.
      | 
      | The new colour's saturation is (this->getSaturation()
      | * multiplier) (the result is clipped
      | to legal limits).
      |
      */
    pub fn with_multiplied_saturation(&self, amount: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSB hsb (*this);
        hsb.saturation = jmin (1.0f, hsb.saturation * amount);
        return hsb.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with its
      | saturation multiplied by the given
      | value.
      | 
      | The new colour's saturation is (this->getSaturation()
      | * multiplier) (the result is clipped
      | to legal limits).
      | 
      | This will be in the HSL colour space.
      |
      */
    pub fn with_multiplied_saturationhsl(&self, amount: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSL hsl (*this);
        hsl.saturation = jmin (1.0f, hsl.saturation * amount);
        return hsl.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with its
      | brightness multiplied by the given
      | value.
      | 
      | The new colour's brightness is (this->getBrightness()
      | * multiplier) (the result is clipped
      | to legal limits).
      |
      */
    pub fn with_multiplied_brightness(&self, amount: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSB hsb (*this);
        hsb.brightness = jmin (1.0f, hsb.brightness * amount);
        return hsb.toColour (*this);
        */
    }
    
    /**
      | Returns a copy of this colour with its
      | lightness multiplied by the given value.
      | 
      | The new colour's lightness is (this->lightness()
      | * multiplier) (the result is clipped
      | to legal limits).
      |
      */
    pub fn with_multiplied_lightness(&self, amount: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::HSL hsl (*this);
        hsl.lightness = jmin (1.0f, hsl.lightness * amount);
        return hsl.toColour (*this);
        */
    }
    
    /**
      | Returns a brighter version of this colour.
      | 
      | -----------
      | @param amountBrighter
      | 
      | how much brighter to make it - a value
      | greater than or equal to 0, where 0 is
      | unchanged, and higher values make it
      | brighter
      | 
      | @see withMultipliedBrightness
      |
      */
    pub fn brighter(&self, amount: Option<f32>) -> Colour {

        let amount: f32 = amount.unwrap_or(0.4);
        
        todo!();
        /*
            jassert (amount >= 0.0f);
        amount = 1.0f / (1.0f + amount);

        return Colour ((uint8) (255 - (amount * (255 - getRed()))),
                       (uint8) (255 - (amount * (255 - getGreen()))),
                       (uint8) (255 - (amount * (255 - getBlue()))),
                       getAlpha());
        */
    }
    
    /**
      | Returns a darker version of this colour.
      | 
      | -----------
      | @param amountDarker
      | 
      | how much darker to make it - a value greater
      | than or equal to 0, where 0 is unchanged,
      | and higher values make it darker
      | 
      | @see withMultipliedBrightness
      |
      */
    pub fn darker(&self, amount: Option<f32>) -> Colour {

        let amount: f32 = amount.unwrap_or(0.4);
        
        todo!();
        /*
            jassert (amount >= 0.0f);
        amount = 1.0f / (1.0f + amount);

        return Colour ((uint8) (amount * getRed()),
                       (uint8) (amount * getGreen()),
                       (uint8) (amount * getBlue()),
                       getAlpha());
        */
    }
    
    /**
      | Returns an opaque shade of grey.
      | 
      | -----------
      | @param brightness
      | 
      | the level of grey to return - 0 is black,
      | 1.0 is white
      |
      */
    pub fn grey_level(&mut self, brightness: f32) -> Colour {
        
        todo!();
        /*
            auto level = ColourHelpers::floatToUInt8 (brightness);
        return Colour (level, level, level);
        */
    }
    
    /**
      | Returns a colour that will be clearly
      | visible against this colour.
      | 
      | The amount parameter indicates how
      | contrasting the new colour should be,
      | so e.g. Colours::black.contrasting
      | (0.1f) will return a colour that's just
      | a little bit lighter; Colours::black.contrasting
      | (1.0f) will return white; Colours::white.contrasting
      | (1.0f) will return black, etc.
      |
      */
    pub fn contrasting_with_amount(&self, amount: Option<f32>) -> Colour {

        let amount: f32 = amount.unwrap_or(1.0);
        
        todo!();
        /*
            return overlaidWith ((getPerceivedBrightness() >= 0.5f
                               ? Colours::black
                               : Colours::white).withAlpha (amount));
        */
    }
    
    /**
      | Returns a colour that is as close as possible
      | to a target colour whilst still being
      | in contrast to this one.
      | 
      | The colour that is returned will be the
      | targetColour, but with its luminosity
      | nudged up or down so that it differs from
      | the luminosity of this colour by at least
      | the amount specified by minLuminosityDiff.
      |
      */
    pub fn contrasting_with_target_and_min_contrast(
        &self, 
        target:       Colour,
        min_contrast: f32) -> Colour {
        
        todo!();
        /*
            ColourHelpers::YIQ bg (*this);
        ColourHelpers::YIQ fg (target);

        if (std::abs (bg.y - fg.y) >= minContrast)
            return target;

        auto y1 = jmax (0.0f, bg.y - minContrast);
        auto y2 = jmin (1.0f, bg.y + minContrast);
        fg.y = (std::abs (y1 - bg.y) > std::abs (y2 - bg.y)) ? y1 : y2;

        return fg.toColour();
        */
    }
    
    /**
      | Returns a colour that contrasts against
      | two colours.
      | 
      | Looks for a colour that contrasts with
      | both of the colours passed-in.
      | 
      | Handy for things like choosing a highlight
      | colour in text editors, etc.
      |
      */
    pub fn contrasting_with_two_colours(
        &mut self, 
        colour1: Colour,
        colour2: Colour) -> Colour {
        
        todo!();
        /*
            auto b1 = colour1.getPerceivedBrightness();
        auto b2 = colour2.getPerceivedBrightness();
        float best = 0.0f, bestDist = 0.0f;

        for (float i = 0.0f; i < 1.0f; i += 0.02f)
        {
            auto d1 = std::abs (i - b1);
            auto d2 = std::abs (i - b2);
            auto dist = jmin (d1, d2, 1.0f - d1, 1.0f - d2);

            if (dist > bestDist)
            {
                best = i;
                bestDist = dist;
            }
        }

        return colour1.overlaidWith (colour2.withMultipliedAlpha (0.5f))
                      .withBrightness (best);
        */
    }
    
    /**
      | Returns a stringified version of this
      | colour.
      | 
      | The string can be turned back into a colour
      | using the fromString() method.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return String::toHexString ((int) argb.getInARGBMaskOrder());
        */
    }
    
    /**
      | Reads the colour from a string that was
      | created with toString().
      |
      */
    pub fn from_string(&mut self, encoded_colour_string: &str) -> Colour {
        
        todo!();
        /*
            return Colour ((uint32) CharacterFunctions::HexParser<int>::parse (encodedColourString.text));
        */
    }
    
    /**
      | Returns the colour as a hex string in
      | the form RRGGBB or AARRGGBB.
      |
      */
    pub fn to_display_string(&self, include_alpha_value: bool) -> String {
        
        todo!();
        /*
            return String::toHexString ((int) (argb.getInARGBMaskOrder() & (includeAlphaValue ? 0xffffffff : 0xffffff)))
                      .paddedLeft ('0', includeAlphaValue ? 8 : 6)
                      .toUpperCase();
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct ColourTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for ColourTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Colour", UnitTestCategories::graphics
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl ColourTests {
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            auto testColour = [this] (Colour colour,
                                      uint8 expectedRed, uint8 expectedGreen, uint8 expectedBlue,
                                      uint8 expectedAlpha = 255, float expectedFloatAlpha = 1.0f)
            {
                expectEquals (colour.getRed(),        expectedRed);
                expectEquals (colour.getGreen(),      expectedGreen);
                expectEquals (colour.getBlue(),       expectedBlue);
                expectEquals (colour.getAlpha(),      expectedAlpha);
                expectEquals (colour.getFloatAlpha(), expectedFloatAlpha);
            };

            beginTest ("Constructors");
            {
                Colour c1;
                testColour (c1, (uint8) 0, (uint8) 0, (uint8) 0, (uint8) 0, 0.0f);

                Colour c2 ((uint32) 0);
                testColour (c2, (uint8) 0, (uint8) 0, (uint8) 0, (uint8) 0, 0.0f);

                Colour c3 ((uint32) 0xffffffff);
                testColour (c3, (uint8) 255, (uint8) 255, (uint8) 255, (uint8) 255, 1.0f);

                Colour c4 (0, 0, 0);
                testColour (c4, (uint8) 0, (uint8) 0, (uint8) 0, (uint8) 255, 1.0f);

                Colour c5 (255, 255, 255);
                testColour (c5, (uint8) 255, (uint8) 255, (uint8) 255, (uint8) 255, 1.0f);

                Colour c6 ((uint8) 0, (uint8) 0, (uint8) 0, (uint8) 0);
                testColour (c6, (uint8) 0, (uint8) 0, (uint8) 0, (uint8) 0, 0.0f);

                Colour c7 ((uint8) 255, (uint8) 255, (uint8) 255, (uint8) 255);
                testColour (c7, (uint8) 255, (uint8) 255, (uint8) 255, (uint8) 255, 1.0f);

                Colour c8 ((uint8) 0, (uint8) 0, (uint8) 0, 0.0f);
                testColour (c8, (uint8) 0, (uint8) 0, (uint8) 0, (uint8) 0, 0.0f);

                Colour c9 ((uint8) 255, (uint8) 255, (uint8) 255, 1.0f);
                testColour (c9, (uint8) 255, (uint8) 255, (uint8) 255, (uint8) 255, 1.0f);
            }

            beginTest ("HSV");
            {
                // black
                testColour (Colour::fromHSV (0.0f, 0.0f, 0.0f, 1.0f), 0, 0, 0);
                // white
                testColour (Colour::fromHSV (0.0f, 0.0f, 1.0f, 1.0f), 255, 255, 255);
                // red
                testColour (Colour::fromHSV (0.0f, 1.0f, 1.0f, 1.0f), 255, 0, 0);
                testColour (Colour::fromHSV (1.0f, 1.0f, 1.0f, 1.0f), 255, 0, 0);
                // lime
                testColour (Colour::fromHSV (120 / 360.0f, 1.0f, 1.0f, 1.0f), 0, 255, 0);
                // blue
                testColour (Colour::fromHSV (240 / 360.0f, 1.0f, 1.0f, 1.0f), 0, 0, 255);
                // yellow
                testColour (Colour::fromHSV (60 / 360.0f, 1.0f, 1.0f, 1.0f), 255, 255, 0);
                // cyan
                testColour (Colour::fromHSV (180 / 360.0f, 1.0f, 1.0f, 1.0f), 0, 255, 255);
                // magenta
                testColour (Colour::fromHSV (300 / 360.0f, 1.0f, 1.0f, 1.0f), 255, 0, 255);
                // silver
                testColour (Colour::fromHSV (0.0f, 0.0f, 0.75f, 1.0f), 191, 191, 191);
                // grey
                testColour (Colour::fromHSV (0.0f, 0.0f, 0.5f, 1.0f), 128, 128, 128);
                // maroon
                testColour (Colour::fromHSV (0.0f, 1.0f, 0.5f, 1.0f), 128, 0, 0);
                // olive
                testColour (Colour::fromHSV (60 / 360.0f, 1.0f, 0.5f, 1.0f), 128, 128, 0);
                // green
                testColour (Colour::fromHSV (120 / 360.0f, 1.0f, 0.5f, 1.0f), 0, 128, 0);
                // purple
                testColour (Colour::fromHSV (300 / 360.0f, 1.0f, 0.5f, 1.0f), 128, 0, 128);
                // teal
                testColour (Colour::fromHSV (180 / 360.0f, 1.0f, 0.5f, 1.0f), 0, 128, 128);
                // navy
                testColour (Colour::fromHSV (240 / 360.0f, 1.0f, 0.5f, 1.0f), 0, 0, 128);
            }

            beginTest ("HSL");
            {
                // black
                testColour (Colour::fromHSL (0.0f, 0.0f, 0.0f, 1.0f), 0, 0, 0);
                // white
                testColour (Colour::fromHSL (0.0f, 0.0f, 1.0f, 1.0f), 255, 255, 255);
                // red
                testColour (Colour::fromHSL (0.0f, 1.0f, 0.5f, 1.0f), 255, 0, 0);
                testColour (Colour::fromHSL (1.0f, 1.0f, 0.5f, 1.0f), 255, 0, 0);
                // lime
                testColour (Colour::fromHSL (120 / 360.0f, 1.0f, 0.5f, 1.0f), 0, 255, 0);
                // blue
                testColour (Colour::fromHSL (240 / 360.0f, 1.0f, 0.5f, 1.0f), 0, 0, 255);
                // yellow
                testColour (Colour::fromHSL (60 / 360.0f, 1.0f, 0.5f, 1.0f), 255, 255, 0);
                // cyan
                testColour (Colour::fromHSL (180 / 360.0f, 1.0f, 0.5f, 1.0f), 0, 255, 255);
                // magenta
                testColour (Colour::fromHSL (300 / 360.0f, 1.0f, 0.5f, 1.0f), 255, 0, 255);
                // silver
                testColour (Colour::fromHSL (0.0f, 0.0f, 0.75f, 1.0f), 191, 191, 191);
                // grey
                testColour (Colour::fromHSL (0.0f, 0.0f, 0.5f, 1.0f), 128, 128, 128);
                // maroon
                testColour (Colour::fromHSL (0.0f, 1.0f, 0.25f, 1.0f), 128, 0, 0);
                // olive
                testColour (Colour::fromHSL (60 / 360.0f, 1.0f, 0.25f, 1.0f), 128, 128, 0);
                // green
                testColour (Colour::fromHSL (120 / 360.0f, 1.0f, 0.25f, 1.0f), 0, 128, 0);
                // purple
                testColour (Colour::fromHSL (300 / 360.0f, 1.0f, 0.25f, 1.0f), 128, 0, 128);
                // teal
                testColour (Colour::fromHSL (180 / 360.0f, 1.0f, 0.25f, 1.0f), 0, 128, 128);
                // navy
                testColour (Colour::fromHSL (240 / 360.0f, 1.0f, 0.25f, 1.0f), 0, 0, 128);
            }

            beginTest ("Modifiers");
            {
                Colour red (255, 0, 0);
                testColour (red, 255, 0, 0);

                testColour (red.withHue (120.0f / 360.0f), 0, 255, 0);
                testColour (red.withSaturation (0.5f), 255, 128, 128);
                testColour (red.withSaturationHSL (0.5f), 191, 64, 64);
                testColour (red.withBrightness (0.5f), 128, 0, 0);
                testColour (red.withLightness (1.0f), 255, 255, 255);
                testColour (red.withRotatedHue (120.0f / 360.0f), 0, 255, 0);
                testColour (red.withRotatedHue (480.0f / 360.0f), 0, 255, 0);
                testColour (red.withRotatedHue (-240.0f / 360.0f), 0, 255, 0);
                testColour (red.withRotatedHue (-600.0f / 360.0f), 0, 255, 0);
                testColour (red.withMultipliedSaturation (0.0f), 255, 255, 255);
                testColour (red.withMultipliedSaturationHSL (0.0f), 128, 128, 128);
                testColour (red.withMultipliedBrightness (0.5f), 128, 0, 0);
                testColour (red.withMultipliedLightness (2.0f), 255, 255, 255);
                testColour (red.withMultipliedLightness (1.0f), 255, 0, 0);
                testColour (red.withLightness (red.getLightness()), 255, 0, 0);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static ColourTests colourTests;
    */
}
