crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_Font.h]

/**
  | Represents a particular font, including
  | its size, style, etc.
  | 
  | Apart from the typeface to be used, a
  | Font object also dictates whether the
  | font is bold, italic, underlined, how
  | big it is, and its kerning and horizontal
  | scale factor.
  | 
  | @see Typeface
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct Font {
    font: ReferenceCountedObjectPtr<SharedFontInternal>,
}

/**
  | A combination of these values is used
  | by the constructor to specify the style
  | of font to use.
  |
  */
pub enum FontStyleFlags
{
    /**
      | indicates a plain, non-bold, non-italic
      | version of the font. @see setStyleFlags
      |
      */
    plain       = 0,    

    /**
      | boldens the font. @see setStyleFlags
      |
      */
    bold        = 1,    

    /**
      | finds an italic version of the font.
      | @see setStyleFlags
      |
      */
    italic      = 2,    

    /**
      | underlines the font. @see setStyleFlags
      |
      */
    underlined  = 4,     
}

///-------------------
pub struct SharedFontInternal {
    base:             ReferenceCountedObject,
    typeface:         TypefacePtr,
    typeface_name:    String,
    typeface_style:   String,
    height:           f32,
    horizontal_scale: f32, // default = 1.0f
    kerning:          f32, // default = 0
    ascent:           f32, // default = 0
    underline:        bool, // default = false
}

impl Default for SharedFontInternal {
    
    fn default() -> Self {
        todo!();
        /*


            : typeface (TypefaceCache::getInstance()->defaultFace),
              typefaceName (Font::getDefaultSansSerifFontName()),
              typefaceStyle (Font::getDefaultStyle()),
              height (FontValues::defaultFontHeight
        */
    }
}

impl PartialEq<SharedFontInternal> for SharedFontInternal {
    
    #[inline] fn eq(&self, other: &SharedFontInternal) -> bool {
        todo!();
        /*
            return height == other.height
                    && underline == other.underline
                    && horizontalScale == other.horizontalScale
                    && kerning == other.kerning
                    && typefaceName == other.typefaceName
                    && typefaceStyle == other.typefaceStyle;
        */
    }
}

impl Eq for SharedFontInternal {}

impl SharedFontInternal {

    pub fn new_from_style_flags_and_font_height(
        style_flags: i32,
        font_height: f32) -> Self {
    
        todo!();
        /*


            : typefaceName (Font::getDefaultSansSerifFontName()),
              typefaceStyle (FontStyleHelpers::getStyleName (styleFlags)),
              height (fontHeight),
              underline ((styleFlags & underlined) != 0)

            if (styleFlags == plain)
                typeface = TypefaceCache::getInstance()->defaultFace;
        */
    }
    
    pub fn new_from_name_style_flags_and_font_height(
        name:        &String,
        style_flags: i32,
        font_height: f32) -> Self {
    
        todo!();
        /*


            : typefaceName (name),
              typefaceStyle (FontStyleHelpers::getStyleName (styleFlags)),
              height (fontHeight),
              underline ((styleFlags & underlined) != 0)

            if (styleFlags == plain && typefaceName.isEmpty())
                typeface = TypefaceCache::getInstance()->defaultFace;
        */
    }
    
    pub fn new_from_name_style_and_font_height(
        name:        &String,
        style:       &String,
        font_height: f32) -> Self {
    
        todo!();
        /*
        : typeface_name(name),
        : typeface_style(style),
        : height(fontHeight),

            if (typefaceName.isEmpty())
                typefaceName = Font::getDefaultSansSerifFontName();
        */
    }
    
    pub fn new_from_typeface_ref(face: &TypefacePtr) -> Self {
    
        todo!();
        /*


            : typeface (face),
              typefaceName (face->getName()),
              typefaceStyle (face->getStyle()),
              height (FontValues::defaultFontHeight)
            jassert (typefaceName.isNotEmpty());
        */
    }
    
    pub fn new_from_shared_font_internal(other: &SharedFontInternal) -> Self {
    
        todo!();
        /*


            : ReferenceCountedObject(),
              typeface (other.typeface),
              typefaceName (other.typefaceName),
              typefaceStyle (other.typefaceStyle),
              height (other.height),
              horizontalScale (other.horizontalScale),
              kerning (other.kerning),
              ascent (other.ascent),
              underline (other.underline)
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_Font.cpp]

pub mod font_values {

    use super::*;

    pub fn limit_font_height(height: f32) -> f32 {
        
        todo!();
        /*
            return jlimit (0.1f, 10000.0f, height);
        */
    }

    pub const default_font_height: f32 = 14.0;

    lazy_static!{
        /*
        float minimumHorizontalScale = 0.7f;
            String fallbackFont;
            String fallbackFontStyle;
        */
    }
}

pub type get_typeface_for_font = fn(_0: &Font) -> TypefacePtr;

lazy_static!{
    /*
    GetTypefaceForFont aloe_getTypefaceForFont = nullptr;
    */
}

impl Font {
    
    /**
      | Returns the minimum horizontal scale
      | to which fonts may be squashed when trying
      | to create a layout. @see setDefaultMinimumHorizontalScaleFactor
      |
      */
    pub fn get_default_minimum_horizontal_scale_factor(&mut self) -> f32 {
        
        todo!();
        /*
            return FontValues::minimumHorizontalScale;
        */
    }
    
    /**
      | Sets the minimum horizontal scale to
      | which fonts may be squashed when trying
      | to create a text layout. @see getDefaultMinimumHorizontalScaleFactor
      |
      */
    pub fn set_default_minimum_horizontal_scale_factor(&mut self, new_value: f32)  {
        
        todo!();
        /*
            FontValues::minimumHorizontalScale = newValue;
        */
    }
}

///--------------------
#[no_copy]
#[leak_detector]
pub struct TypefaceCache {
    base:         DeletedAtShutdown,
    default_face: TypefacePtr,
    lock:         RawRwLock,
    faces:        Vec<typeface_cache::CachedFace>,
    counter:      usize, // default = 0
}

pub mod typeface_cache {
    use super::*;

    #[derive(Default)]
    pub struct CachedFace {

        /**
          | Although it seems a bit wacky to store
          | the name here, it's because it may be
          | a placeholder rather than a real one,
          | e.g. "<Sans-Serif>" vs the actual
          | typeface name.  Since the typeface
          | itself doesn't know that it may have
          | this alias, the name under which it was
          | fetched needs to be stored separately.
          */
        typeface_name:    String,
        typeface_style:   String,
        last_usage_count: usize, // default = 0
        typeface:         TypefacePtr,
    }
}

impl Default for TypefaceCache {
    
    fn default() -> Self {
        todo!();
        /*


            setSize (10)
        */
    }
}

aloe_declare_singleton!{
    TypefaceCache, false
}

aloe_implement_singleton!{
    TypefaceCache
}

impl Drop for TypefaceCache {
    fn drop(&mut self) {
        todo!();
        /* 
            clearSingletonInstance();
         */
    }
}

impl TypefaceCache {

    pub fn set_size(&mut self, num_to_cache: i32)  {
        
        todo!();
        /*
            const ScopedWriteLock sl (lock);

            faces.clear();
            faces.insertMultiple (-1, CachedFace(), numToCache);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            const ScopedWriteLock sl (lock);

            setSize (faces.size());
            defaultFace = nullptr;
        */
    }
    
    pub fn find_typeface_for(&mut self, font: &Font) -> TypefacePtr {
        
        todo!();
        /*
            const auto faceName = font.getTypefaceName();
            const auto faceStyle = font.getTypefaceStyle();

            jassert (faceName.isNotEmpty());

            {
                const ScopedReadLock slr (lock);

                for (int i = faces.size(); --i >= 0;)
                {
                    CachedFace& face = faces.getReference(i);

                    if (face.typefaceName == faceName
                         && face.typefaceStyle == faceStyle
                         && face.typeface != nullptr
                         && face.typeface->isSuitableForFont (font))
                    {
                        face.lastUsageCount = ++counter;
                        return face.typeface;
                    }
                }
            }

            const ScopedWriteLock slw (lock);
            int replaceIndex = 0;
            auto bestLastUsageCount = std::numeric_limits<size_t>::max();

            for (int i = faces.size(); --i >= 0;)
            {
                auto lu = faces.getReference(i).lastUsageCount;

                if (bestLastUsageCount > lu)
                {
                    bestLastUsageCount = lu;
                    replaceIndex = i;
                }
            }

            auto& face = faces.getReference (replaceIndex);
            face.typefaceName = faceName;
            face.typefaceStyle = faceStyle;
            face.lastUsageCount = ++counter;

            if (aloe_getTypefaceForFont == nullptr)
                face.typeface = Font::getDefaultTypefaceForFont (font);
            else
                face.typeface = aloe_getTypefaceForFont (font);

            jassert (face.typeface != nullptr); // the look and feel must return a typeface!

            if (defaultFace == nullptr && font == Font())
                defaultFace = face.typeface;

            return face.typeface;
        */
    }
}

lazy_static!{
    /*
    void (*clearOpenGLGlyphCache)() = nullptr;
    */
}

impl Typeface {

    /**
      | Changes the number of fonts that are
      | cached in memory.
      |
      */
    pub fn set_typeface_cache_size(&mut self, num_fonts_to_cache: i32)  {
        
        todo!();
        /*
            TypefaceCache::getInstance()->setSize (numFontsToCache);
        */
    }
    
    /**
      | Clears any fonts that are currently
      | cached in memory.
      |
      */
    pub fn clear_typeface_cache(&mut self)  {
        
        todo!();
        /*
            TypefaceCache::getInstance()->clear();

        RenderingHelpers::SoftwareRendererSavedState::clearGlyphCache();

        if (clearOpenGLGlyphCache != nullptr)
            clearOpenGLGlyphCache();
        */
    }
}

///-----------------
impl Default for Font {
    
    /**
      | Creates a basic sans-serif font at a
      | default height.
      | 
      | You should use one of the other constructors
      | for creating a font that you're planning
      | on drawing with - this constructor is
      | here to help initialise objects before
      | changing the font's settings later.
      |
      */
    fn default() -> Self {
        todo!();
        /*


            : font (new SharedFontInternal()
        */
    }
}

impl PartialEq<Font> for Font {
    
    #[inline] fn eq(&self, other: &Font) -> bool {
        todo!();
        /*
            return font == other.font
                || *font == *other.font;
        */
    }
}

impl Eq for Font {}

impl Font {

    /**
      | Creates a font for a typeface.
      |
      */
    pub fn new_from_typeface_ref(typeface: &TypefacePtr) -> Self {
    
        todo!();
        /*


            : font (new SharedFontInternal (typeface))
        */
    }
    
    /**
      | Creates a copy of another Font object.
      |
      */
    pub fn new_from_font_ref(other: &Font) -> Self {
    
        todo!();
        /*
        : font(other.font),

        
        */
    }
    
    /**
      | Creates a sans-serif font in a given
      | size.
      | 
      | -----------
      | @param fontHeight
      | 
      | the height in pixels (can be fractional)
      | ----------
      | @param styleFlags
      | 
      | the style to use - this can be a combination
      | of the Font::bold, Font::italic and
      | Font::underlined, or just Font::plain
      | for the normal style. @see FontStyleFlags,
      | getDefaultSansSerifFontName
      |
      */
    pub fn new_from_font_height_and_style_flags(
        font_height: f32,
        style_flags: Option<FontStyleFlags>

    ) -> Self {

        let style_flags = style_flags.unwrap_or(FontStyleFlags::plain);
    
        todo!();
        /*


            : font (new SharedFontInternal (styleFlags, FontValues::limitFontHeight (fontHeight)))
        */
    }
    
    /**
      | Creates a font with a given typeface
      | and parameters.
      | 
      | -----------
      | @param typefaceName
      | 
      | the font family of the typeface to use
      | ----------
      | @param fontHeight
      | 
      | the height in pixels (can be fractional)
      | ----------
      | @param styleFlags
      | 
      | the style to use - this can be a combination
      | of the Font::bold, Font::italic and
      | Font::underlined, or just Font::plain
      | for the normal style. @see FontStyleFlags,
      | getDefaultSansSerifFontName
      |
      */
    pub fn new_from_typeface_font_height_and_style_flags(
        typeface_name: &String,
        font_height:   f32,
        style_flags:   i32) -> Self {
    
        todo!();
        /*


            : font (new SharedFontInternal (typefaceName, styleFlags, FontValues::limitFontHeight (fontHeight)))
        */
    }
    
    /**
      | Creates a font with a given typeface
      | and parameters.
      | 
      | -----------
      | @param typefaceName
      | 
      | the font family of the typeface to use
      | ----------
      | @param typefaceStyle
      | 
      | the font style of the typeface to use
      | ----------
      | @param fontHeight
      | 
      | the height in pixels (can be fractional)
      |
      */
    pub fn new_from_typeface_name_typeface_style_and_font_height(
        typeface_name:  &String,
        typeface_style: &String,
        font_height:    f32) -> Self {
    
        todo!();
        /*


            : font (new SharedFontInternal (typefaceName, typefaceStyle, FontValues::limitFontHeight (fontHeight)))
        */
    }
    
    pub fn assign_from_font_ref(&mut self, other: &Font) -> &mut Font {
        
        todo!();
        /*
            font = other.font;
        return *this;
        */
    }
    
    pub fn new_from_font(other: Font) -> Self {
    
        todo!();
        /*
        : font(std::move (other.font)),

        
        */
    }
    
    pub fn assign_from_font(&mut self, other: Font) -> &mut Font {
        
        todo!();
        /*
            font = std::move (other.font);
        return *this;
        */
    }
    
    pub fn dupe_internal_if_shared(&mut self)  {
        
        todo!();
        /*
            if (font->getReferenceCount() > 1)
            font = *new SharedFontInternal (*font);
        */
    }
    
    pub fn check_typeface_suitability(&mut self)  {
        
        todo!();
        /*
            if (font->typeface != nullptr && ! font->typeface->isSuitableForFont (*this))
            font->typeface = nullptr;
        */
    }
}


pub struct FontPlaceholderNames
{
    sans:    String, // default = { "<Sans-Serif>"  }
    serif:   String, // default = { "<Serif>"  }
    mono:    String, // default = { "<Monospaced>"  }
    regular: String, // default = { "<Regular>"  }
}

pub fn get_font_placeholder_names<'a>() -> &'a FontPlaceholderNames {
    
    todo!();
    /*
        static FontPlaceholderNames names;
        return names;
    */
}

/**
  | This is a workaround for the lack of
  | thread-safety in MSVC's handling of
  | function-local statics - if multiple threads
  | all try to create the first Font object at the
  | same time, it can cause a race-condition in
  | creating these placeholder strings.
  */
#[cfg(ALOE_MSVC)]
pub struct FontNamePreloader {

}

#[cfg(ALOE_MSVC)]
impl Default for FontNamePreloader {
    
    fn default() -> Self {
        todo!();
        /*


            getFontPlaceholderNames()
        */
    }
}

#[cfg(ALOE_MSVC)]
lazy_static!{
    /*
    static FontNamePreloader fnp;
    */
}

impl Font {
    
    /**
      | Returns a typeface font family that
      | represents the default sans-serif
      | font.
      | 
      | This is also the typeface that will be
      | used when a font is created without specifying
      | any typeface details.
      | 
      | -----------
      | @note
      | 
      | this method just returns a generic placeholder
      | string that means "the default sans-serif
      | font" - it's not the actual font family
      | of this font.
      | 
      | @see setTypefaceName, getDefaultSerifFontName,
      | getDefaultMonospacedFontName
      |
      */
    pub fn get_default_sans_serif_font_name(&mut self) -> &String {
        
        todo!();
        /*
            return getFontPlaceholderNames().sans;
        */
    }
    
    /**
      | Returns a typeface font family that
      | represents the default serif font.
      | 
      | -----------
      | @note
      | 
      | this method just returns a generic placeholder
      | string that means "the default serif
      | font" - it's not the actual font family
      | of this font.
      | 
      | @see setTypefaceName, getDefaultSansSerifFontName,
      | getDefaultMonospacedFontName
      |
      */
    pub fn get_default_serif_font_name(&mut self) -> &String {
        
        todo!();
        /*
            return getFontPlaceholderNames().serif;
        */
    }
    
    /**
      | Returns a typeface font family that
      | represents the default monospaced
      | font.
      | 
      | -----------
      | @note
      | 
      | this method just returns a generic placeholder
      | string that means "the default monospaced
      | font" - it's not the actual font family
      | of this font.
      | 
      | @see setTypefaceName, getDefaultSansSerifFontName,
      | getDefaultSerifFontName
      |
      */
    pub fn get_default_monospaced_font_name(&mut self) -> &String {
        
        todo!();
        /*
            return getFontPlaceholderNames().mono;
        */
    }
    
    /**
      | Returns a font style name that represents
      | the default style.
      | 
      | -----------
      | @note
      | 
      | this method just returns a generic placeholder
      | string that means "the default font
      | style" - it's not the actual name of the
      | font style of any particular font.
      | 
      | @see setTypefaceStyle
      |
      */
    pub fn get_default_style(&mut self) -> &String {
        
        todo!();
        /*
            return getFontPlaceholderNames().regular;
        */
    }
    
    /**
      | Returns the font family of the typeface
      | that this font uses.
      | 
      | e.g. "Arial", "Courier", etc.
      | 
      | This may also be set to Font::getDefaultSansSerifFontName(),
      | Font::getDefaultSerifFontName(),
      | or Font::getDefaultMonospacedFontName(),
      | which are not actual platform-specific
      | font family names, but are generic font
      | family names that are used to represent
      | the various default fonts.
      | 
      | If you need to know the exact typeface
      | font family being used, you can call
      | Font::getTypeface()->getName(),
      | which will give you the platform-specific
      | font family.
      |
      */
    pub fn get_typeface_name(&self) -> &String {
        
        todo!();
        /*
            return font->typefaceName;
        */
    }
    
    /**
      | Returns the font style of the typeface
      | that this font uses. @see withTypefaceStyle,
      | getAvailableStyles()
      |
      */
    pub fn get_typeface_style(&self) -> &String {
        
        todo!();
        /*
            return font->typefaceStyle;
        */
    }
    
    /**
      | Changes the font family of the typeface.
      | 
      | e.g. "Arial", "Courier", etc.
      | 
      | This may also be set to Font::getDefaultSansSerifFontName(),
      | Font::getDefaultSerifFontName(),
      | or Font::getDefaultMonospacedFontName(),
      | which are not actual platform-specific
      | font family names, but are generic font
      | family names that are used to represent
      | the various default fonts. If you need
      | to know the exact typeface font family
      | being used, you can call Font::getTypeface()->getName(),
      | which will give you the platform-specific
      | font family.
      | 
      | If a suitable font isn't found on the
      | machine, it'll just use a default instead.
      |
      */
    pub fn set_typeface_name(&mut self, face_name: &String)  {
        
        todo!();
        /*
            if (faceName != font->typefaceName)
        {
            jassert (faceName.isNotEmpty());

            dupeInternalIfShared();
            font->typefaceName = faceName;
            font->typeface = nullptr;
            font->ascent = 0;
        }
        */
    }
    
    /**
      | Changes the font style of the typeface.
      | @see getAvailableStyles()
      |
      */
    pub fn set_typeface_style(&mut self, typeface_style: &String)  {
        
        todo!();
        /*
            if (typefaceStyle != font->typefaceStyle)
        {
            dupeInternalIfShared();
            font->typefaceStyle = typefaceStyle;
            font->typeface = nullptr;
            font->ascent = 0;
        }
        */
    }
    
    /**
      | Returns a copy of this font with a new
      | typeface style. @see getAvailableStyles()
      |
      */
    pub fn with_typeface_style(&self, new_style: &String) -> Font {
        
        todo!();
        /*
            Font f (*this);
        f.setTypefaceStyle (newStyle);
        return f;
        */
    }
    
    /**
      | Returns a list of the styles that this
      | font can use.
      |
      */
    pub fn get_available_styles(&self) -> Vec<String> {
        
        todo!();
        /*
            return findAllTypefaceStyles (getTypeface()->getName());
        */
    }
    
    /**
      | Returns the typeface used by this font.
      | 
      | -----------
      | @note
      | 
      | the object returned may go out of scope
      | if this font is deleted or has its style
      | changed.
      |
      */
    pub fn get_typeface(&self) -> *mut Typeface {
        
        todo!();
        /*
            if (font->typeface == nullptr)
        {
            font->typeface = TypefaceCache::getInstance()->findTypefaceFor (*this);
            jassert (font->typeface != nullptr);
        }

        return font->typeface.get();
        */
    }
    
    /**
      | Returns the font family of the typeface
      | to be used for rendering glyphs that
      | aren't found in the requested typeface.
      |
      */
    pub fn get_fallback_font_name(&mut self) -> &String {
        
        todo!();
        /*
            return FontValues::fallbackFont;
        */
    }
    
    /**
      | Sets the (platform-specific) font
      | family of the typeface to use to find
      | glyphs that aren't available in whatever
      | font you're trying to use.
      |
      */
    pub fn set_fallback_font_name(&mut self, name: &String)  {
        
        todo!();
        /*
            FontValues::fallbackFont = name;

       #if ALOE_MAC || ALOE_IOS
        jassertfalse; // Note that use of a fallback font isn't currently implemented in OSX..
       #endif
        */
    }
    
    /**
      | Returns the font style of the typeface
      | to be used for rendering glyphs that
      | aren't found in the requested typeface.
      |
      */
    pub fn get_fallback_font_style(&mut self) -> &String {
        
        todo!();
        /*
            return FontValues::fallbackFontStyle;
        */
    }
    
    /**
      | Sets the (platform-specific) font
      | style of the typeface to use to find glyphs
      | that aren't available in whatever font
      | you're trying to use.
      |
      */
    pub fn set_fallback_font_style(&mut self, style: &String)  {
        
        todo!();
        /*
            FontValues::fallbackFontStyle = style;

       #if ALOE_MAC || ALOE_IOS
        jassertfalse; // Note that use of a fallback font isn't currently implemented in OSX..
       #endif
        */
    }
    
    /**
      | Returns a copy of this font with a new
      | height.
      |
      */
    pub fn with_height(&self, new_height: f32) -> Font {
        
        todo!();
        /*
            Font f (*this);
        f.setHeight (newHeight);
        return f;
        */
    }
    
    pub fn get_height_to_points_factor(&self) -> f32 {
        
        todo!();
        /*
            return getTypeface()->getHeightToPointsFactor();
        */
    }
    
    /**
      | Returns a copy of this font with a new
      | height, specified in points.
      |
      */
    pub fn with_point_height(&self, height_in_points: f32) -> Font {
        
        todo!();
        /*
            Font f (*this);
        f.setHeight (heightInPoints / getHeightToPointsFactor());
        return f;
        */
    }
    
    /**
      | Changes the font's height. @see getHeight,
      | withHeight, setHeightWithoutChangingWidth
      |
      */
    pub fn set_height(&mut self, new_height: f32)  {
        
        todo!();
        /*
            newHeight = FontValues::limitFontHeight (newHeight);

        if (font->height != newHeight)
        {
            dupeInternalIfShared();
            font->height = newHeight;
            checkTypefaceSuitability();
        }
        */
    }
    
    /**
      | Changes the font's height without changing
      | its width.
      | 
      | This alters the horizontal scale to
      | compensate for the change in height.
      |
      */
    pub fn set_height_without_changing_width(&mut self, new_height: f32)  {
        
        todo!();
        /*
            newHeight = FontValues::limitFontHeight (newHeight);

        if (font->height != newHeight)
        {
            dupeInternalIfShared();
            font->horizontalScale *= (font->height / newHeight);
            font->height = newHeight;
            checkTypefaceSuitability();
        }
        */
    }
    
    /**
      | Returns the font's style flags.
      | 
      | This will return a bitwise-or'ed combination
      | of values from the FontStyleFlags enum,
      | to describe whether the font is bold,
      | italic, etc. @see FontStyleFlags,
      | withStyle
      |
      */
    pub fn get_style_flags(&self) -> i32 {
        
        todo!();
        /*
            int styleFlags = font->underline ? underlined : plain;

        if (isBold())    styleFlags |= bold;
        if (isItalic())  styleFlags |= italic;

        return styleFlags;
        */
    }
    
    /**
      | Returns a copy of this font with the given
      | set of style flags.
      | 
      | -----------
      | @param styleFlags
      | 
      | a bitwise-or'ed combination of values
      | from the FontStyleFlags enum. @see
      | FontStyleFlags, getStyleFlags
      |
      */
    pub fn with_style(&self, new_flags: i32) -> Font {
        
        todo!();
        /*
            Font f (*this);
        f.setStyleFlags (newFlags);
        return f;
        */
    }
    
    /**
      | Changes the font's style.
      | 
      | -----------
      | @param newFlags
      | 
      | a bitwise-or'ed combination of values
      | from the FontStyleFlags enum. @see
      | FontStyleFlags, withStyle
      |
      */
    pub fn set_style_flags(&mut self, new_flags: i32)  {
        
        todo!();
        /*
            if (getStyleFlags() != newFlags)
        {
            dupeInternalIfShared();
            font->typeface = nullptr;
            font->typefaceStyle = FontStyleHelpers::getStyleName (newFlags);
            font->underline = (newFlags & underlined) != 0;
            font->ascent = 0;
        }
        */
    }
    
    /**
      | Changes all the font's characteristics
      | with one call.
      |
      */
    pub fn set_size_and_style(
        &mut self, 
        new_height:           f32,
        new_style_flags:      i32,
        new_horizontal_scale: f32,
        new_kerning_amount:   f32)  {
        
        todo!();
        /*
            newHeight = FontValues::limitFontHeight (newHeight);

        if (font->height != newHeight
             || font->horizontalScale != newHorizontalScale
             || font->kerning != newKerningAmount)
        {
            dupeInternalIfShared();
            font->height = newHeight;
            font->horizontalScale = newHorizontalScale;
            font->kerning = newKerningAmount;
            checkTypefaceSuitability();
        }

        setStyleFlags (newStyleFlags);
        */
    }
    
    /**
      | Changes all the font's characteristics
      | with one call.
      |
      */
    pub fn set_size_and_style_with_string_style(
        &mut self, 
        new_height:           f32,
        new_style:            &String,
        new_horizontal_scale: f32,
        new_kerning_amount:   f32)  {
        
        todo!();
        /*
            newHeight = FontValues::limitFontHeight (newHeight);

        if (font->height != newHeight
             || font->horizontalScale != newHorizontalScale
             || font->kerning != newKerningAmount)
        {
            dupeInternalIfShared();
            font->height = newHeight;
            font->horizontalScale = newHorizontalScale;
            font->kerning = newKerningAmount;
            checkTypefaceSuitability();
        }

        setTypefaceStyle (newStyle);
        */
    }
    
    /**
      | Returns a copy of this font with a new
      | horizontal scale.
      | 
      | -----------
      | @param scaleFactor
      | 
      | a value of 1.0 is the normal scale, less
      | than this will be narrower, greater
      | than 1.0 will be stretched out. @see
      | getHorizontalScale
      |
      */
    pub fn with_horizontal_scale(&self, new_horizontal_scale: f32) -> Font {
        
        todo!();
        /*
            Font f (*this);
        f.setHorizontalScale (newHorizontalScale);
        return f;
        */
    }
    
    /**
      | Changes the font's horizontal scale
      | factor.
      | 
      | -----------
      | @param scaleFactor
      | 
      | a value of 1.0 is the normal scale, less
      | than this will be narrower, greater
      | than 1.0 will be stretched out.
      |
      */
    pub fn set_horizontal_scale(&mut self, scale_factor: f32)  {
        
        todo!();
        /*
            dupeInternalIfShared();
        font->horizontalScale = scaleFactor;
        checkTypefaceSuitability();
        */
    }
    
    /**
      | Returns the font's horizontal scale.
      | 
      | A value of 1.0 is the normal scale, less
      | than this will be narrower, greater
      | than 1.0 will be stretched out.
      | 
      | @see withHorizontalScale
      |
      */
    pub fn get_horizontal_scale(&self) -> f32 {
        
        todo!();
        /*
            return font->horizontalScale;
        */
    }
    
    /**
      | Returns the font's kerning.
      | 
      | This is the extra space added between
      | adjacent characters, as a proportion
      | of the font's height.
      | 
      | A value of zero is normal spacing, positive
      | values will spread the letters out more,
      | and negative values make them closer
      | together.
      |
      */
    pub fn get_extra_kerning_factor(&self) -> f32 {
        
        todo!();
        /*
            return font->kerning;
        */
    }
    
    /**
      | Returns a copy of this font with a new
      | kerning factor.
      | 
      | -----------
      | @param extraKerning
      | 
      | a multiple of the font's height that
      | will be added to space between the characters.
      | So a value of zero is normal spacing,
      | positive values spread the letters
      | out, negative values make them closer
      | together.
      |
      */
    pub fn with_extra_kerning_factor(&self, extra_kerning: f32) -> Font {
        
        todo!();
        /*
            Font f (*this);
        f.setExtraKerningFactor (extraKerning);
        return f;
        */
    }
    
    /**
      | Changes the font's kerning.
      | 
      | -----------
      | @param extraKerning
      | 
      | a multiple of the font's height that
      | will be added to space between the characters.
      | So a value of zero is normal spacing,
      | positive values spread the letters
      | out, negative values make them closer
      | together.
      |
      */
    pub fn set_extra_kerning_factor(&mut self, extra_kerning: f32)  {
        
        todo!();
        /*
            dupeInternalIfShared();
        font->kerning = extraKerning;
        checkTypefaceSuitability();
        */
    }
    
    /**
      | Returns a copy of this font with the bold
      | attribute set.
      | 
      | If the font does not have a bold version,
      | this will return the default font.
      |
      */
    pub fn boldened(&self) -> Font {
        
        todo!();
        /*
            return withStyle (getStyleFlags() | bold);
        */
    }
    
    /**
      | Returns a copy of this font with the italic
      | attribute set.
      |
      */
    pub fn italicised(&self) -> Font {
        
        todo!();
        /*
            return withStyle (getStyleFlags() | italic);
        */
    }
    
    /**
      | Returns true if the font is bold.
      |
      */
    pub fn is_bold(&self) -> bool {
        
        todo!();
        /*
            return FontStyleHelpers::isBold   (font->typefaceStyle);
        */
    }
    
    /**
      | Returns true if the font is italic.
      |
      */
    pub fn is_italic(&self) -> bool {
        
        todo!();
        /*
            return FontStyleHelpers::isItalic (font->typefaceStyle);
        */
    }
    
    /**
      | Returns true if the font is underlined.
      |
      */
    pub fn is_underlined(&self) -> bool {
        
        todo!();
        /*
            return font->underline;
        */
    }
    
    /**
      | Makes the font bold or non-bold.
      |
      */
    pub fn set_bold(&mut self, should_be_bold: bool)  {
        
        todo!();
        /*
            auto flags = getStyleFlags();
        setStyleFlags (shouldBeBold ? (flags | bold)
                                    : (flags & ~bold));
        */
    }
    
    /**
      | Makes the font italic or non-italic.
      |
      */
    pub fn set_italic(&mut self, should_be_italic: bool)  {
        
        todo!();
        /*
            auto flags = getStyleFlags();
        setStyleFlags (shouldBeItalic ? (flags | italic)
                                      : (flags & ~italic));
        */
    }
    
    /**
      | Makes the font underlined or non-underlined.
      |
      */
    pub fn set_underline(&mut self, should_be_underlined: bool)  {
        
        todo!();
        /*
            dupeInternalIfShared();
        font->underline = shouldBeUnderlined;
        checkTypefaceSuitability();
        */
    }
    
    /**
      | Returns the height of the font above
      | its baseline, in pixels.
      | 
      | This is the maximum height from the baseline
      | to the top. @see getHeight, getDescent
      |
      */
    pub fn get_ascent(&self) -> f32 {
        
        todo!();
        /*
            if (font->ascent == 0.0f)
            font->ascent = getTypeface()->getAscent();

        return font->height * font->ascent;
        */
    }
    
    /**
      | Returns the total height of this font,
      | in pixels.
      | 
      | This is the maximum height, from the
      | top of the ascent to the bottom of the
      | descenders.
      | 
      | @see withHeight, setHeightWithoutChangingWidth,
      | getAscent
      |
      */
    pub fn get_height(&self) -> f32 {
        
        todo!();
        /*
            return font->height;
        */
    }
    
    /**
      | Returns the amount that the font descends
      | below its baseline, in pixels.
      | 
      | This is calculated as (getHeight()
      | - getAscent()). @see getAscent, getHeight
      |
      */
    pub fn get_descent(&self) -> f32 {
        
        todo!();
        /*
            return font->height - getAscent();
        */
    }
    
    /**
      | Returns the total height of this font,
      | in points.
      | 
      | This is the maximum height, from the
      | top of the ascent to the bottom of the
      | descenders.
      | 
      | @see withPointHeight, getHeight
      |
      */
    pub fn get_height_in_points(&self) -> f32 {
        
        todo!();
        /*
            return getHeight()  * getHeightToPointsFactor();
        */
    }
    
    /**
      | Returns the height of the font above
      | its baseline, in points.
      | 
      | This is the maximum height from the baseline
      | to the top. @see getHeight, getDescent
      |
      */
    pub fn get_ascent_in_points(&self) -> f32 {
        
        todo!();
        /*
            return getAscent()  * getHeightToPointsFactor();
        */
    }
    
    /**
      | Returns the amount that the font descends
      | below its baseline, in points.
      | 
      | This is calculated as (getHeight()
      | - getAscent()). @see getAscent, getHeight
      |
      */
    pub fn get_descent_in_points(&self) -> f32 {
        
        todo!();
        /*
            return getDescent() * getHeightToPointsFactor();
        */
    }
    
    /**
      | Returns the total width of a string as
      | it would be drawn using this font.
      | 
      | For a more accurate floating-point
      | result, use getStringWidthFloat().
      |
      */
    pub fn get_string_width(&self, text: &String) -> i32 {
        
        todo!();
        /*
            return (int) std::ceil (getStringWidthFloat (text));
        */
    }
    
    /**
      | Returns the total width of a string as
      | it would be drawn using this font. @see
      | getStringWidth
      |
      */
    pub fn get_string_width_float(&self, text: &String) -> f32 {
        
        todo!();
        /*
            // This call isn't thread-safe when there's a message thread running
        jassert (MessageManager::getInstanceWithoutCreating() == nullptr
                   || MessageManager::getInstanceWithoutCreating()->currentThreadHasLockedMessageManager());

        auto w = getTypeface()->getStringWidth (text);

        if (font->kerning != 0.0f)
            w += font->kerning * (float) text.length();

        return w * font->height * font->horizontalScale;
        */
    }
    
    /**
      | Returns the series of glyph numbers
      | and their x offsets needed to represent
      | a string.
      | 
      | An extra x offset is added at the end of
      | the run, to indicate where the right
      | hand edge of the last character is.
      |
      */
    pub fn get_glyph_positions(&self, 
        text:      &String,
        glyphs:    &mut Vec<i32>,
        x_offsets: &mut Vec<f32>)  {
        
        todo!();
        /*
            // This call isn't thread-safe when there's a message thread running
        jassert (MessageManager::getInstanceWithoutCreating() == nullptr
                   || MessageManager::getInstanceWithoutCreating()->currentThreadHasLockedMessageManager());

        getTypeface()->getGlyphPositions (text, glyphs, xOffsets);

        if (auto num = xOffsets.size())
        {
            auto scale = font->height * font->horizontalScale;
            auto* x = xOffsets.getRawDataPointer();

            if (font->kerning != 0.0f)
            {
                for (int i = 0; i < num; ++i)
                    x[i] = (x[i] + (float) i * font->kerning) * scale;
            }
            else
            {
                for (int i = 0; i < num; ++i)
                    x[i] *= scale;
            }
        }
        */
    }
    
    /**
      | Creates an array of Font objects to represent
      | all the fonts on the system.
      | 
      | If you just need the font family names
      | of the typefaces, you can also use findAllTypefaceNames()
      | instead.
      | 
      | -----------
      | @param results
      | 
      | the array to which new Font objects will
      | be added.
      |
      */
    pub fn find_fonts(&mut self, dest_array: &mut Vec<Font>)  {
        
        todo!();
        /*
            for (auto& name : findAllTypefaceNames())
        {
            auto styles = findAllTypefaceStyles (name);

            String style ("Regular");

            if (! styles.contains (style, true))
                style = styles[0];

            destArray.add (Font (name, style, FontValues::defaultFontHeight));
        }
        */
    }
    
    /**
      | Creates a string to describe this font.
      | 
      | The string will contain information
      | to describe the font's typeface, size,
      | and style. To recreate the font from
      | this string, use fromString().
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            String s;

        if (getTypefaceName() != getDefaultSansSerifFontName())
            s << getTypefaceName() << "; ";

        s << String (getHeight(), 1);

        if (getTypefaceStyle() != getDefaultStyle())
            s << ' ' << getTypefaceStyle();

        return s;
        */
    }
    
    /**
      | Recreates a font from its stringified
      | encoding.
      | 
      | This method takes a string that was created
      | by toString(), and recreates the original
      | font.
      |
      */
    pub fn from_string(&mut self, font_description: &String) -> Font {
        
        todo!();
        /*
            const int separator = fontDescription.indexOfChar (';');
        String name;

        if (separator > 0)
            name = fontDescription.substring (0, separator).trim();

        if (name.isEmpty())
            name = getDefaultSansSerifFontName();

        String sizeAndStyle (fontDescription.substring (separator + 1).trimStart());

        float height = sizeAndStyle.getFloatValue();
        if (height <= 0)
            height = 10.0f;

        const String style (sizeAndStyle.fromFirstOccurrenceOf (" ", false, false));

        return Font (name, style, height);
        */
    }
}
