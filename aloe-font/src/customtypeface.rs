crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_CustomTypeface.h]

pub trait CustomTypefaceInterface {

    /**
      | If a subclass overrides this, it can
      | load glyphs into the font on-demand.
      | 
      | When methods such as getGlyphPositions()
      | or getOutlineForGlyph() are asked
      | for a particular character and there's
      | no corresponding glyph, they'll call
      | this method so that a subclass can try
      | to add that glyph, returning true if
      | it manages to do so.
      |
      */
    fn load_glyph_if_possible(&mut self, character_needed: wchar_t) -> bool;
}

///-----------------------
#[no_copy]
#[leak_detector]
pub struct CustomTypefaceGlyphInfo {
    character:     wchar_t,
    path:          Path,
    width:         f32,
    kerning_pairs: Vec<custom_typeface_glyph_info::KerningPair>,
}

pub mod custom_typeface_glyph_info {

    use super::*;

    pub struct KerningPair
    {
        character2:     wchar_t,
        kerning_amount: f32,
    }
}

impl CustomTypefaceGlyphInfo {

    pub fn new(
        c: wchar_t,
        p: &Path,
        w: f32) -> Self {
    
        todo!();
        /*
        : character(c),
        : path(p),
        : width(w),

        
        */
    }
    
    pub fn add_kerning_pair(&mut self, 
        subsequent_character: wchar_t,
        extra_kerning_amount: f32)  {
        
        todo!();
        /*
            kerningPairs.add ({ subsequentCharacter, extraKerningAmount });
        */
    }
    
    pub fn get_horizontal_spacing(&self, subsequent_character: wchar_t) -> f32 {
        
        todo!();
        /*
            if (subsequentCharacter != 0)
                for (auto& kp : kerningPairs)
                    if (kp.character2 == subsequentCharacter)
                        return width + kp.kerningAmount;

            return width;
        */
    }
}


/**
  | A typeface that can be populated with
  | custom glyphs.
  | 
  | You can create a CustomTypeface if you
  | need one that contains your own glyphs,
  | or if you need to load a typeface from
  | a Aloe-formatted binary stream.
  | 
  | If you want to create a copy of a native
  | face, you can use addGlyphsFromOtherTypeface()
  | to copy glyphs into this face.
  | 
  | -----------
  | @note
  | 
  | For most people this class is almost
  | certainly NOT the right tool to use!
  | If what you want to do is to embed a font
  | into your exe, then your best plan is
  | probably to embed your TTF/OTF font
  | file into your binary using the Proaloer,
  | and then call Typeface::createSystemTypefaceFor()
  | to load it from memory.
  | 
  | @see Typeface, Font
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct CustomTypeface {
    base:              Typeface,
    default_character: wchar_t,
    ascent:            f32,
    glyphs:            Vec<Box<CustomTypefaceGlyphInfo>>,
    lookup_table:      [i16; 128],
}

pub mod custom_typeface {

    use super::*;


    pub fn read_char<R: Read>(in_: &mut R) -> wchar_t {
        
        todo!();
        /*
            auto n = (uint32) (uint16) in.readShort();

                if (n >= 0xd800 && n <= 0xdfff)
                {
                    auto nextWord = (uint32) (uint16) in.readShort();
                    jassert (nextWord >= 0xdc00); // illegal unicode character!

                    n = 0x10000 + (((n - 0xd800) << 10) | (nextWord - 0xdc00));
                }

                return (aloe_wchar) n;
        */
    }

    pub fn write_char<W: Write>(
            out:           &mut W,
            char_to_write: wchar_t)  {
        
        todo!();
        /*
            if (charToWrite >= 0x10000)
                {
                    charToWrite -= 0x10000;
                    out.writeShort ((short) (uint16) (0xd800 + (charToWrite >> 10)));
                    out.writeShort ((short) (uint16) (0xdc00 + (charToWrite & 0x3ff)));
                }
                else
                {
                    out.writeShort ((short) (uint16) charToWrite);
                }
        */
    }
}

impl Default for CustomTypeface {
    
    /**
      | Creates a new, empty typeface.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : typeface(String(), String()),

            clear();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_CustomTypeface.cpp]
impl CustomTypeface {

    /**
      | Loads a typeface from a previously saved
      | stream.
      | 
      | The stream must have been created by
      | writeToStream().
      | 
      | -----------
      | @note
      | 
      | Since this class was written, support
      | was added for loading real font files
      | from memory, so for most people, using
      | Typeface::createSystemTypefaceFor()
      | to load a real font is more appropriate
      | than using this class to store it in a
      | proprietary format.
      | 
      | @see writeToStream
      |
      */
    pub fn new<R: Read>(serialised_typeface_stream: &mut R) -> Self {
    
        todo!();
        /*
        : typeface(String(), String()),

            clear();

        GZIPDecompressorInputStream gzin (serialisedTypefaceStream);
        BufferedInputStream in (gzin, 32768);

        name = in.readString();

        const bool isBold   = in.readBool();
        const bool isItalic = in.readBool();
        style = FontStyleHelpers::getStyleName (isBold, isItalic);

        ascent = in.readFloat();
        defaultCharacter = CustomTypefaceHelpers::readChar (in);

        auto numChars = in.readInt();

        for (int i = 0; i < numChars; ++i)
        {
            auto c = CustomTypefaceHelpers::readChar (in);
            auto width = in.readFloat();

            Path p;
            p.loadPathFromStream (in);
            addGlyph (c, p, width);
        }

        auto numKerningPairs = in.readInt();

        for (int i = 0; i < numKerningPairs; ++i)
        {
            auto char1 = CustomTypefaceHelpers::readChar (in);
            auto char2 = CustomTypefaceHelpers::readChar (in);

            addKerningPair (char1, char2, in.readFloat());
        }
        */
    }
    
    /**
      | Resets this typeface, deleting all
      | its glyphs and settings.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            defaultCharacter = 0;
        ascent = 1.0f;
        style = "Regular";
        zeromem (lookupTable, sizeof (lookupTable));
        glyphs.clear();
        */
    }
    
    /**
      | Sets the vital statistics for the typeface.
      | 
      | -----------
      | @param fontFamily
      | 
      | the typeface's font family
      | ----------
      | @param ascent
      | 
      | the ascent - this is normalised to a height
      | of 1.0 and this is the value that will
      | be returned by Typeface::getAscent().
      | The descent is assumed to be (1.0 - ascent)
      | ----------
      | @param isBold
      | 
      | should be true if the typeface is bold
      | ----------
      | @param isItalic
      | 
      | should be true if the typeface is italic
      | ----------
      | @param defaultCharacter
      | 
      | the character to be used as a replacement
      | if there's no glyph available for the
      | character that's being drawn
      |
      */
    pub fn set_characteristics_with_bold_and_italics(
        &mut self, 
        new_name:              &String,
        new_ascent:            f32,
        is_bold:               bool,
        is_italic:             bool,
        new_default_character: wchar_t)  {
        
        todo!();
        /*
            name = newName;
        defaultCharacter = newDefaultCharacter;
        ascent = newAscent;
        style = FontStyleHelpers::getStyleName (isBold, isItalic);
        */
    }
    
    /**
      | Sets the vital statistics for the typeface.
      | 
      | -----------
      | @param fontFamily
      | 
      | the typeface's font family
      | ----------
      | @param fontStyle
      | 
      | the typeface's font style
      | ----------
      | @param ascent
      | 
      | the ascent - this is normalised to a height
      | of 1.0 and this is the value that will
      | be returned by Typeface::getAscent().
      | The descent is assumed to be (1.0 - ascent)
      | ----------
      | @param defaultCharacter
      | 
      | the character to be used as a replacement
      | if there's no glyph available for the
      | character that's being drawn
      |
      */
    pub fn set_characteristics(&mut self, 
        new_name:              &String,
        new_style:             &String,
        new_ascent:            f32,
        new_default_character: wchar_t)  {
        
        todo!();
        /*
            name = newName;
        style = newStyle;
        defaultCharacter = newDefaultCharacter;
        ascent = newAscent;
        */
    }
    
    /**
      | Adds a glyph to the typeface.
      | 
      | The path that is passed in is normalised
      | so that the font height is 1.0, and its
      | origin is the anchor point of the character
      | on its baseline.
      | 
      | The width is the nominal width of the
      | character, and any extra kerning values
      | that are specified will be added to this
      | width.
      |
      */
    pub fn add_glyph(&mut self, 
        character: wchar_t,
        path:      &Path,
        width:     f32)  {
        
        todo!();
        /*
            // Check that you're not trying to add the same character twice..
        jassert (findGlyph (character, false) == nullptr);

        if (isPositiveAndBelow ((int) character, numElementsInArray (lookupTable)))
            lookupTable [character] = (short) glyphs.size();

        glyphs.add (new GlyphInfo (character, path, width));
        */
    }
    
    /**
      | Specifies an extra kerning amount to
      | be used between a pair of characters.
      | 
      | The amount will be added to the nominal
      | width of the first character when laying
      | out a string.
      |
      */
    pub fn add_kerning_pair(&mut self, 
        char1:        wchar_t,
        char2:        wchar_t,
        extra_amount: f32)  {
        
        todo!();
        /*
            if (extraAmount != 0.0f)
        {
            if (auto* g = findGlyph (char1, true))
                g->addKerningPair (char2, extraAmount);
            else
                jassertfalse; // can only add kerning pairs for characters that exist!
        }
        */
    }
    
    /**
      | Adds a range of glyphs from another typeface.
      | 
      | This will attempt to pull in the paths
      | and kerning information from another
      | typeface and add it to this one.
      |
      */
    pub fn find_glyph(&mut self, 
        character:      wchar_t,
        load_if_needed: bool) -> *mut CustomTypefaceGlyphInfo {
        
        todo!();
        /*
            if (isPositiveAndBelow ((int) character, numElementsInArray (lookupTable)) && lookupTable [character] > 0)
            return glyphs [(int) lookupTable [(int) character]];

        for (auto* g : glyphs)
            if (g->character == character)
                return g;

        if (loadIfNeeded && loadGlyphIfPossible (character))
            return findGlyph (character, false);

        return nullptr;
        */
    }
    
    pub fn load_glyph_if_possible(&mut self, _0: wchar_t) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn add_glyphs_from_other_typeface(&mut self, 
        typeface_to_copy:      &mut Typeface,
        character_start_index: wchar_t,
        num_characters:        i32)  {
        
        todo!();
        /*
            setCharacteristics (name, style, typefaceToCopy.getAscent(), defaultCharacter);

        for (int i = 0; i < numCharacters; ++i)
        {
            auto c = (aloe_wchar) (characterStartIndex + static_cast<aloe_wchar> (i));

            Vec<int> glyphIndexes;
            Vec<float> offsets;
            typefaceToCopy.getGlyphPositions (String::charToString (c), glyphIndexes, offsets);

            const int glyphIndex = glyphIndexes.getFirst();

            if (glyphIndex >= 0 && glyphIndexes.size() > 0)
            {
                auto glyphWidth = offsets[1];

                Path p;
                typefaceToCopy.getOutlineForGlyph (glyphIndex, p);

                addGlyph (c, p, glyphWidth);

                for (int j = glyphs.size() - 1; --j >= 0;)
                {
                    auto char2 = glyphs.getUnchecked (j)->character;
                    glyphIndexes.clearQuick();
                    offsets.clearQuick();
                    typefaceToCopy.getGlyphPositions (String::charToString (c) + String::charToString (char2), glyphIndexes, offsets);

                    if (offsets.size() > 1)
                        addKerningPair (c, char2, offsets[1] - glyphWidth);
                }
            }
        }
        */
    }
    
    /**
      | Saves this typeface as a Aloe-formatted
      | font file.
      | 
      | A CustomTypeface can be created to reload
      | the data that is written - see the CustomTypeface
      | constructor.
      | 
      | -----------
      | @note
      | 
      | Since this class was written, support
      | was added for loading real font files
      | from memory, so for most people, using
      | Typeface::createSystemTypefaceFor()
      | to load a real font is more appropriate
      | than using this class to store it in a
      | proprietary format.
      |
      */
    pub fn write_to_stream<W: Write>(&mut self, output_stream: &mut W) -> bool {
        
        todo!();
        /*
            GZIPCompressorOutputStream out (outputStream);

        out.writeString (name);
        out.writeBool (FontStyleHelpers::isBold (style));
        out.writeBool (FontStyleHelpers::isItalic (style));
        out.writeFloat (ascent);
        CustomTypefaceHelpers::writeChar (out, defaultCharacter);
        out.writeInt (glyphs.size());

        int numKerningPairs = 0;

        for (auto* g : glyphs)
        {
            CustomTypefaceHelpers::writeChar (out, g->character);
            out.writeFloat (g->width);
            g->path.writePathToStream (out);

            numKerningPairs += g->kerningPairs.size();
        }

        out.writeInt (numKerningPairs);

        for (auto* g : glyphs)
        {
            for (auto& p : g->kerningPairs)
            {
                CustomTypefaceHelpers::writeChar (out, g->character);
                CustomTypefaceHelpers::writeChar (out, p.character2);
                out.writeFloat (p.kerningAmount);
            }
        }

        return true;
        */
    }
    
    /**
      | The following methods implement the
      | basic Typeface behaviour.
      |
      */
    pub fn get_ascent(&self) -> f32 {
        
        todo!();
        /*
            return ascent;
        */
    }
    
    pub fn get_descent(&self) -> f32 {
        
        todo!();
        /*
            return 1.0f - ascent;
        */
    }
    
    pub fn get_height_to_points_factor(&self) -> f32 {
        
        todo!();
        /*
            return ascent;
        */
    }
    
    pub fn get_string_width(&mut self, text: &String) -> f32 {
        
        todo!();
        /*
            float x = 0;

        for (auto t = text.getCharPointer(); ! t.isEmpty();)
        {
            auto c = t.getAndAdvance();

            if (auto* glyph = findGlyph (c, true))
            {
                x += glyph->getHorizontalSpacing (*t);
            }
            else
            {
                if (auto fallbackTypeface = Typeface::getFallbackTypeface())
                    if (fallbackTypeface.get() != this)
                        x += fallbackTypeface->getStringWidth (String::charToString (c));
            }
        }

        return x;
        */
    }
    
    pub fn get_glyph_positions(&mut self, 
        text:          &String,
        result_glyphs: &mut Vec<i32>,
        x_offsets:     &mut Vec<f32>)  {
        
        todo!();
        /*
            xOffsets.add (0);
        float x = 0;

        for (auto t = text.getCharPointer(); ! t.isEmpty();)
        {
            float width = 0.0f;
            int glyphChar = 0;

            auto c = t.getAndAdvance();

            if (auto* glyph = findGlyph (c, true))
            {
                width = glyph->getHorizontalSpacing (*t);
                glyphChar = (int) glyph->character;
            }
            else
            {
                auto fallbackTypeface = getFallbackTypeface();

                if (fallbackTypeface != nullptr && fallbackTypeface.get() != this)
                {
                    Vec<int> subGlyphs;
                    Vec<float> subOffsets;
                    fallbackTypeface->getGlyphPositions (String::charToString (c), subGlyphs, subOffsets);

                    if (subGlyphs.size() > 0)
                    {
                        glyphChar = subGlyphs.getFirst();
                        width = subOffsets[1];
                    }
                }
            }

            x += width;
            resultGlyphs.add (glyphChar);
            xOffsets.add (x);
        }
        */
    }
    
    pub fn get_outline_for_glyph(&mut self, 
        glyph_number: i32,
        path:         &mut Path) -> bool {
        
        todo!();
        /*
            if (auto* glyph = findGlyph ((aloe_wchar) glyphNumber, true))
        {
            path = glyph->path;
            return true;
        }

        if (auto fallbackTypeface = getFallbackTypeface())
            if (fallbackTypeface.get() != this)
                return fallbackTypeface->getOutlineForGlyph (glyphNumber, path);

        return false;
        */
    }
    
    pub fn get_edge_table_for_glyph(&mut self, 
        glyph_number: i32,
        transform:    &AffineTransform,
        font_height:  f32) -> *mut EdgeTable {
        
        todo!();
        /*
            if (auto* glyph = findGlyph ((aloe_wchar) glyphNumber, true))
        {
            if (! glyph->path.isEmpty())
                return new EdgeTable (glyph->path.getBoundsTransformed (transform)
                                                 .getSmallestIntegerContainer().expanded (1, 0),
                                      glyph->path, transform);
        }
        else
        {
            if (auto fallbackTypeface = getFallbackTypeface())
                if (fallbackTypeface.get() != this)
                    return fallbackTypeface->getEdgeTableForGlyph (glyphNumber, transform, fontHeight);
        }

        return nullptr;
        */
    }
}
