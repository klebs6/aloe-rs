crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_Typeface.h]

pub trait GetAscent {

    /**
      | Returns the ascent of the font, as a proportion
      | of its height.
      | 
      | The height is considered to always be
      | normalised as 1.0, so this will be a value
      | less that 1.0, indicating the proportion
      | of the font that lies above its baseline.
      |
      */
    fn get_ascent(&self) -> f32;
}

pub trait GetDescent {

    /**
      | Returns the descent of the font, as a
      | proportion of its height.
      | 
      | The height is considered to always be
      | normalised as 1.0, so this will be a value
      | less that 1.0, indicating the proportion
      | of the font that lies below its baseline.
      |
      */
    fn get_descent(&self) -> f32;
}

pub trait GetHeightToPointsFactor {

    /**
      | Returns the value by which you should
      | multiply a Aloe font-height value to
      | convert it to the equivalent point-size.
      |
      */
    fn get_height_to_points_factor(&self) -> f32;
}

pub trait GetStringWidth {

    /**
      | Measures the width of a line of text.
      | 
      | The distance returned is based on the
      | font having an normalised height of
      | 1.0.
      | 
      | You should never need to call this directly!
      | Use Font::getStringWidth() instead!
      |
      */
    fn get_string_width(&mut self, text: &String) -> f32;
}

pub trait GetGlyphPositions {

    /**
      | Converts a line of text into its glyph
      | numbers and their positions.
      | 
      | The distances returned are based on
      | the font having an normalised height
      | of 1.0.
      | 
      | You should never need to call this directly!
      | Use Font::getGlyphPositions() instead!
      |
      */
    fn get_glyph_positions(&mut self, 
        text:      &String,
        glyphs:    &mut Vec<i32>,
        x_offsets: &mut Vec<f32>);
}

pub trait GetOutlineForGlyph {

    /**
      | Returns the outline for a glyph.
      | 
      | The path returned will be normalised
      | to a font height of 1.0.
      |
      */
    fn get_outline_for_glyph(&mut self, 
        glyph_number: i32,
        path:         &mut Path) -> bool;
}

pub trait GetEdgeTableForGlyph {

    /**
      | Returns a new EdgeTable that contains
      | the path for the given glyph, with the
      | specified transform applied.
      |
      */
    fn get_edge_table_for_glyph(&mut self, 
        glyph_number: i32,
        transform:    &AffineTransform,
        font_height:  f32) -> *mut EdgeTable;
}

pub trait IsHinted {

    /**
      | Returns true if the typeface uses hinting.
      |
      */
    fn is_hinted(&self) -> bool {
        false
    }
}

pub trait TypefaceInterface: 
GetAscent 
+ GetDescent 
+ GetHeightToPointsFactor 
+ GetStringWidth 
+ GetGlyphPositions 
+ GetOutlineForGlyph 
+ GetEdgeTableForGlyph 
+ IsHinted { }

/**
  | A typeface represents a size-independent
  | font.
  | 
  | This base class is abstract, but calling
  | createSystemTypefaceFor() will return
  | a platform-specific subclass that
  | can be used.
  | 
  | The CustomTypeface subclass allow
  | you to build your own typeface, and to
  | load and save it in the Aloe typeface
  | format.
  | 
  | Normally you should never need to deal
  | directly with Typeface objects - the
  | Font class does everything you typically
  | need for rendering text.
  | 
  | @see CustomTypeface, Font
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Typeface {
    base:           ReferenceCountedObject,
    name:           String,
    style:          String,
    hinting_params: Box<TypefaceHintingParams>,
    hinting_lock:   CriticalSection,
}

pub struct TypefaceHintingParams {
    cached_size:  f32,
    cached_scale: typeface_hinting_params::Scaling,
    top:          f32,
    middle:       f32,
    bottom:       f32,
}

impl Default for TypefaceHintingParams {

    fn default() -> Self {
        Self {
            cached_size:  0.0,
            cached_scale: typeface_hinting_params::Scaling::default(),
            top:          0.0,
            middle:       0.0,
            bottom:       0.0,
        }
    }
}

pub mod typeface_hinting_params {

    pub const STANDARD_HEIGHT: usize = 100;

    pub struct Scaling {
        middle:       f32,
        upper_scale:  f32,
        upper_offset: f32,
        lower_scale:  f32,
        lower_offset: f32,
    }

    impl Default for Scaling {
        
        fn default() -> Self {
            todo!();
            /*
            : middle(),
            : upper_scale(),
            : upper_offset(),
            : lower_scale(),
            : lower_offset(),

            
            */
        }
    }

    impl Scaling {

        pub fn new(
            t:         f32,
            m:         f32,
            b:         f32,
            font_size: f32) -> Self {
        
            todo!();
            /*
            : middle(m),

                const float newT = std::floor (fontSize * t + 0.5f) / fontSize;
                        const float newB = std::floor (fontSize * b + 0.5f) / fontSize;
                        const float newM = std::floor (fontSize * m + 0.3f) / fontSize; // this is slightly biased so that lower-case letters
                                                                                        // are more likely to become taller than shorter.
                        upperScale  = jlimit (0.9f, 1.1f, (newM - newT) / (m - t));
                        lowerScale  = jlimit (0.9f, 1.1f, (newB - newM) / (b - m));

                        upperOffset = newM - m * upperScale;
                        lowerOffset = newB - b * lowerScale;
            */
        }
        
        pub fn apply(&self, y: f32) -> f32 {
            
            todo!();
            /*
                return y < middle ? (y * upperScale + upperOffset)
                                          : (y * lowerScale + lowerOffset);
            */
        }
    }
}

impl TypefaceHintingParams {

    pub fn new(t: &mut Typeface) -> Self {
    
        todo!();
        /*


            Font font (t);
                font = font.withHeight ((float) standardHeight);

                top = getAverageY (font, "BDEFPRTZOQ", true);
                middle = getAverageY (font, "acegmnopqrsuvwxy", true);
                bottom = getAverageY (font, "BDELZOC", false);
        */
    }
    
    pub fn apply_vertical_hinting_transform(&mut self, 
        font_size: f32,
        path:      &mut Path)  {
        
        todo!();
        /*
            if (cachedSize != fontSize)
                {
                    cachedSize = fontSize;
                    cachedScale = Scaling (top, middle, bottom, fontSize);
                }

                if (bottom < top + 3.0f / fontSize)
                    return;

                Path result;

                for (Path::Iterator i (path); i.next();)
                {
                    switch (i.elementType)
                    {
                        case Path::Iterator::startNewSubPath:  result.startNewSubPath (i.x1, cachedScale.apply (i.y1)); break;
                        case Path::Iterator::lineTo:           result.lineTo (i.x1, cachedScale.apply (i.y1)); break;
                        case Path::Iterator::quadraticTo:      result.quadraticTo (i.x1, cachedScale.apply (i.y1),
                                                                                   i.x2, cachedScale.apply (i.y2)); break;
                        case Path::Iterator::cubicTo:          result.cubicTo (i.x1, cachedScale.apply (i.y1),
                                                                               i.x2, cachedScale.apply (i.y2),
                                                                               i.x3, cachedScale.apply (i.y3)); break;
                        case Path::Iterator::closePath:        result.closeSubPath(); break;
                        default:                               jassertfalse; break;
                    }
                }

                result.swapWithPath (path);
        */
    }
    
    pub fn get_averagey(
        font:    &Font,
        chars:   *const u8,
        get_top: bool) -> f32 {
        
        todo!();
        /*
            GlyphArrangement ga;
                ga.addLineOfText (font, chars, 0, 0);

                Vec<float> yValues;

                for (auto& glyph : ga)
                {
                    Path p;
                    glyph.createPath (p);
                    auto bounds = p.getBounds();

                    if (! p.isEmpty())
                        yValues.add (getTop ? bounds.getY() : bounds.getBottom());
                }

                std::sort (yValues.begin(), yValues.end());

                auto median = yValues[yValues.size() / 2];
                float total = 0;
                int num = 0;

                for (auto y : yValues)
                {
                    if (std::abs (median - y) < 0.05f * (float) standardHeight)
                    {
                        total += y;
                        ++num;
                    }
                }

                return num < 4 ? 0.0f : total / ((float) num * (float) standardHeight);
        */
    }
}

/**
  | A handy typedef for a pointer to a typeface.
  |
  */
pub type TypefacePtr = ReferenceCountedObjectPtr<Typeface>;

pub trait IsSuitableForFont {

    fn is_suitable_for_font(&self, _0: &Font) -> bool;
}

impl IsSuitableForFont for Typeface {

    /**
      | Returns true if this typeface can be
      | used to render the specified font.
      | 
      | When called, the font will already have
      | been checked to make sure that its name
      | and style flags match the typeface.
      |
      */
    fn is_suitable_for_font(&self, _0: &Font) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_Typeface.cpp]
impl Typeface {

    /**
      | Returns the font family of the typeface.
      | @see Font::getTypefaceName
      |
      */
    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }

    
    /**
      | Returns the font style of the typeface.
      | @see Font::getTypefaceStyle
      |
      */
    pub fn get_style(&self) -> &String {
        
        todo!();
        /*
            return style;
        */
    }
}

///-------------------------
pub mod font_style_helpers {

    use super::*;

    pub struct ConcreteFamilyNames {
        sans:  String,
        serif: String,
        mono:  String,
    }

    impl Default for ConcreteFamilyNames {
        
        fn default() -> Self {
            todo!();
            /*


                : sans  (findName (Font::getDefaultSansSerifFontName())),
                      serif (findName (Font::getDefaultSerifFontName())),
                      mono  (findName (Font::getDefaultMonospacedFontName())
            */
        }
    }

    impl ConcreteFamilyNames {

        pub fn look_up(&mut self, placeholder: &String) -> String {
            
            todo!();
            /*
                if (placeholder == Font::getDefaultSansSerifFontName())  return sans;
                    if (placeholder == Font::getDefaultSerifFontName())      return serif;
                    if (placeholder == Font::getDefaultMonospacedFontName()) return mono;

                    return findName (placeholder);
            */
        }
        
        pub fn find_name(placeholder: &String) -> String {
            
            todo!();
            /*
                const Font f (placeholder, Font::getDefaultStyle(), 15.0f);
                    return Font::getDefaultTypefaceForFont (f)->getName();
            */
        }
    }

    ///-------------------------
    pub fn get_style_name(
        bold:   bool,
        italic: bool) -> *const u8 {
        
        todo!();
        /*
            if (bold && italic) return "Bold Italic";
            if (bold)           return "Bold";
            if (italic)         return "Italic";
            return "Regular";
        */
    }
    
    pub fn get_style_name_with_flags(style_flags: i32) -> *const u8 {
        
        todo!();
        /*
            return getStyleName ((styleFlags & Font::bold) != 0,
                                 (styleFlags & Font::italic) != 0);
        */
    }
    
    pub fn is_bold(style: &String) -> bool {
        
        todo!();
        /*
            return style.containsWholeWordIgnoreCase ("Bold");
        */
    }
    
    pub fn is_italic(style: &String) -> bool {
        
        todo!();
        /*
            return style.containsWholeWordIgnoreCase ("Italic")
                || style.containsWholeWordIgnoreCase ("Oblique");
        */
    }
    
    pub fn is_placeholder_family_name(family: &String) -> bool {
        
        todo!();
        /*
            return family == Font::getDefaultSansSerifFontName()
                || family == Font::getDefaultSerifFontName()
                || family == Font::getDefaultMonospacedFontName();
        */
    }
    
    pub fn get_concrete_family_name_from_placeholder(placeholder: &String) -> String {
        
        todo!();
        /*
            static ConcreteFamilyNames names;
            return names.lookUp (placeholder);
        */
    }
    
    pub fn get_concrete_family_name(font: &Font) -> String {
        
        todo!();
        /*
            const String& family = font.getTypefaceName();

            return isPlaceholderFamilyName (family) ? getConcreteFamilyNameFromPlaceholder (family)
                                                    : family;
        */
    }
}

impl Typeface {
    
    pub fn new(
        face_name:  &String,
        style_name: &String) -> Self {
    
        todo!();
        /*
        : name(faceName),
        : style(styleName),

        
        */
    }
    
    pub fn get_fallback_typeface(&mut self) -> TypefacePtr {
        
        todo!();
        /*
            const Font fallbackFont (Font::getFallbackFontName(), Font::getFallbackFontStyle(), 10.0f);
        return Typeface::Ptr (fallbackFont.getTypeface());
        */
    }
    
    pub fn get_edge_table_for_glyph(&mut self, 
        glyph_number: i32,
        transform:    &AffineTransform,
        font_height:  f32) -> *mut EdgeTable {
        
        todo!();
        /*
            Path path;

        if (getOutlineForGlyph (glyphNumber, path) && ! path.isEmpty())
        {
            applyVerticalHintingTransform (fontHeight, path);

            return new EdgeTable (path.getBoundsTransformed (transform).getSmallestIntegerContainer().expanded (1, 0),
                                  path, transform);
        }

        return nullptr;
        */
    }
    
    /**
      | Makes an attempt at performing a good
      | overall distortion that will scale
      | a font of the given size to align vertically
      | with the pixel grid. The path should
      | be an unscaled (i.e. normalised to height
      | of 1.0) path for a glyph.
      |
      */
    pub fn apply_vertical_hinting_transform(&mut self, 
        font_size: f32,
        path:      &mut Path)  {
        
        todo!();
        /*
            if (fontSize > 3.0f && fontSize < 25.0f)
        {
            ScopedLock sl (hintingLock);

            if (hintingParams == nullptr)
                hintingParams.reset (new HintingParams (*this));

            return hintingParams->applyVerticalHintingTransform (fontSize, path);
        }
        */
    }
}
