crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_AttributedString.h]

/**
  | Types of word-wrap behaviour. @see
  | getWordWrap, setWordWrap
  |
  */
pub enum WordWrap
{
    /**
      | No word-wrapping: lines extend indefinitely.
      |
      */
    none,   

    /**
      | Lines are wrapped on a word boundary.
      |
      */
    byWord, 

    /**
      | Lines are wrapped on a character boundary.
      |
      */
    byChar, 
}

/**
  | Types of reading direction that can
  | be used. @see getReadingDirection,
  | setReadingDirection
  |
  */
pub enum ReadingDirection
{
    natural,
    leftToRight,
    rightToLeft,
}

/**
  | An attribute that has been applied to
  | a range of characters in an AttributedString.
  |
  */
#[derive(Default)]
#[leak_detector]
pub struct AttributedStringAttribute {

    /**
      | The range of characters to which this
      | attribute will be applied.
      |
      */
    range:  Range<i32>,

    /**
      | The font for this range of characters.
      |
      */
    font:   Font,

    /**
      | The colour for this range of characters.
      |
      */
    colour: Colour, // default = { 0xff000000  }
}

impl AttributedStringAttribute {

    /**
      | Creates an attribute that specifies
      | the font and colour for a range of characters.
      |
      */
    pub fn new(
        r: Range<i32>,
        f: &Font,
        c: Colour) -> Self {
    
        todo!();
        /*
        : range(r),
        : font(f),
        : colour(c),

        
        */
    }
}

/**
  | A text string with a set of colour/font
  | settings that are associated with sub-ranges
  | of the text.
  | 
  | An attributed string lets you create
  | a string with varied fonts, colours,
  | word-wrapping, layout, etc., and draw
  | it using AttributedString::draw().
  | 
  | @see TextLayout
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct AttributedString {
    text:              String,
    line_spacing:      f32, // default = 0.0f
    justification:     Justification, // default = Justification::left
    word_wrap:         WordWrap, // default = AttributedString::byWord
    reading_direction: ReadingDirection, // default = AttributedString::natural
    attributes:        Vec<AttributedStringAttribute>,
}

impl Default for AttributedString {
    
    /**
      | Creates an empty attributed string.
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_AttributedString.cpp]
impl AttributedString {

    /**
      | Creates an attributed string with the
      | given text.
      |
      */
    pub fn new(new_string: &String) -> Self {
    
        todo!();
        /*
            setText (newString);
        */
    }

    /**
      | Returns the complete text of this attributed
      | string.
      |
      */
    pub fn get_text(&self) -> &String {
        
        todo!();
        /*
            return text;
        */
    }

    /**
      | Returns the justification that should
      | be used for laying-out the text.
      | 
      | This may include both vertical and horizontal
      | flags.
      |
      */
    pub fn get_justification(&self) -> Justification {
        
        todo!();
        /*
            return justification;
        */
    }

    /**
      | Returns the word-wrapping behaviour.
      |
      */
    pub fn get_word_wrap(&self) -> WordWrap {
        
        todo!();
        /*
            return wordWrap;
        */
    }

    /**
      | Returns the reading direction for the
      | text.
      |
      */
    pub fn get_reading_direction(&self) -> ReadingDirection {
        
        todo!();
        /*
            return readingDirection;
        */
    }

    /**
      | Returns the extra line-spacing distance.
      |
      */
    pub fn get_line_spacing(&self) -> f32 {
        
        todo!();
        /*
            return lineSpacing;
        */
    }

    /**
      | Returns the number of attributes that
      | have been added to this string.
      |
      */
    pub fn get_num_attributes(&self) -> i32 {
        
        todo!();
        /*
            return attributes.size();
        */
    }

    /**
      | Returns one of the string's attributes.
      | 
      | The index provided must be less than
      | getNumAttributes(), and >= 0.
      |
      */
    pub fn get_attribute(&self, index: i32) -> &AttributedStringAttribute {
        
        todo!();
        /*
            return attributes.getReference (index);
        */
    }
    
    /**
      | Replaces all the text.
      | 
      | This will change the text, but won't
      | affect any of the colour or font attributes
      | that have been added.
      |
      */
    pub fn set_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            auto newLength = newText.length();
        auto oldLength = getLength (attributes);

        if (newLength > oldLength)
            appendRange (attributes, newLength - oldLength, nullptr, nullptr);
        else if (newLength < oldLength)
            truncate (attributes, newLength);

        text = newText;
        */
    }
    
    /**
      | Appends some text (with a default font
      | and colour).
      |
      */
    pub fn append_text(&mut self, text_to_append: &String)  {
        
        todo!();
        /*
            text += textToAppend;
        appendRange (attributes, textToAppend.length(), nullptr, nullptr);
        */
    }
    
    /**
      | Appends some text, with a specified
      | font, and the default colour (black).
      |
      */
    pub fn append_text_with_font(
        &mut self, 
        text_to_append: &String,
        font:           &Font)  {
        
        todo!();
        /*
            text += textToAppend;
        appendRange (attributes, textToAppend.length(), &font, nullptr);
        */
    }
    
    /**
      | Appends some text, with a specified
      | colour, and the default font.
      |
      */
    pub fn append_text_with_colour(
        &mut self, 
        text_to_append: &String,
        colour:         Colour)  {
        
        todo!();
        /*
            text += textToAppend;
        appendRange (attributes, textToAppend.length(), nullptr, &colour);
        */
    }
    
    /**
      | Appends some text, with a specified
      | font and colour.
      |
      */
    pub fn append_text_with_font_and_colour(
        &mut self, 
        text_to_append: &String,
        font:           &Font,
        colour:         Colour)  {
        
        todo!();
        /*
            text += textToAppend;
        appendRange (attributes, textToAppend.length(), &font, &colour);
        */
    }
    
    /**
      | Appends another AttributedString
      | to this one.
      | 
      | -----------
      | @note
      | 
      | this will only append the text, fonts,
      | and colours - it won't copy any other
      | properties such as justification,
      | line-spacing, etc from the other object.
      |
      */
    pub fn append_attributed_string_ref(
        &mut self, 
        other: &AttributedString

    ) {
        
        todo!();
        /*
            auto originalLength = getLength (attributes);
        auto originalNumAtts = attributes.size();
        text += other.text;
        attributes.addArray (other.attributes);

        for (auto i = originalNumAtts; i < attributes.size(); ++i)
            attributes.getReference (i).range += originalLength;

        mergeAdjacentRanges (attributes);
        */
    }
    
    /**
      | Resets the string, clearing all text
      | and attributes.
      | 
      | -----------
      | @note
      | 
      | this won't affect global settings like
      | the justification type, word-wrap
      | mode, etc.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            text.clear();
        attributes.clear();
        */
    }
    
    /**
      | Sets the justification that should
      | be used for laying-out the text.
      | 
      | This may include both vertical and horizontal
      | flags.
      |
      */
    pub fn set_justification(&mut self, new_justification: Justification)  {
        
        todo!();
        /*
            justification = newJustification;
        */
    }
    
    /**
      | Sets the word-wrapping behaviour.
      |
      */
    pub fn set_word_wrap(&mut self, new_word_wrap: WordWrap)  {
        
        todo!();
        /*
            wordWrap = newWordWrap;
        */
    }
    
    /**
      | Sets the reading direction that should
      | be used for the text.
      |
      */
    pub fn set_reading_direction(&mut self, new_reading_direction: ReadingDirection)  {
        
        todo!();
        /*
            readingDirection = newReadingDirection;
        */
    }
    
    /**
      | Sets an extra line-spacing distance.
      |
      */
    pub fn set_line_spacing(&mut self, new_line_spacing: f32)  {
        
        todo!();
        /*
            lineSpacing = newLineSpacing;
        */
    }
    
    /**
      | Adds a colour attribute for the specified
      | range.
      |
      */
    pub fn set_colour_with_range(
        &mut self, 
        range:  Range<i32>,
        colour: Colour)  {
        
        todo!();
        /*
            applyFontAndColour (attributes, range, nullptr, &colour);
        */
    }
    
    /**
      | Adds a font attribute for the specified
      | range.
      |
      */
    pub fn set_font_with_range(
        &mut self, 
        range: Range<i32>,
        font:  &Font)  {
        
        todo!();
        /*
            applyFontAndColour (attributes, range, &font, nullptr);
        */
    }
    
    /**
      | Removes all existing colour attributes,
      | and applies this colour to the whole
      | string.
      |
      */
    pub fn set_colour(&mut self, colour: Colour)  {
        
        todo!();
        /*
            setColour ({ 0, getLength (attributes) }, colour);
        */
    }
    
    /**
      | Removes all existing font attributes,
      | and applies this font to the whole string.
      |
      */
    pub fn set_font(&mut self, font: &Font)  {
        
        todo!();
        /*
            setFont ({ 0, getLength (attributes) }, font);
        */
    }
}

pub fn get_length(atts: &[AttributedStringAttribute]) -> i32 {
    
    todo!();
    /*
        return atts.size() != 0 ? atts.getReference (atts.size() - 1).range.getEnd() : 0;
    */
}

pub fn split_attribute_ranges_with_position(
    atts:     &mut Vec<AttributedStringAttribute>,
    position: i32)  {
    
    todo!();
    /*
        for (int i = atts.size(); --i >= 0;)
            {
                const auto& att = atts.getUnchecked (i);
                auto offset = position - att.range.getStart();

                if (offset >= 0)
                {
                    if (offset > 0 && position < att.range.getEnd())
                    {
                        atts.insert (i + 1, AttributedString::Attribute (att));
                        atts.getReference (i).range.setEnd (position);
                        atts.getReference (i + 1).range.setStart (position);
                    }

                    break;
                }
            }
    */
}

pub fn split_attribute_ranges(
    atts:      &mut Vec<AttributedStringAttribute>,
    new_range: Range<i32>) -> Range<i32> {
    
    todo!();
    /*
        newRange = newRange.getIntersectionWith ({ 0, getLength (atts) });

            if (! newRange.isEmpty())
            {
                splitAttributeRanges (atts, newRange.getStart());
                splitAttributeRanges (atts, newRange.getEnd());
            }

            return newRange;
    */
}

pub fn merge_adjacent_ranges(atts: &mut Vec<AttributedStringAttribute>)  {
    
    todo!();
    /*
        for (int i = atts.size() - 1; --i >= 0;)
            {
                auto& a1 = atts.getReference (i);
                auto& a2 = atts.getReference (i + 1);

                if (a1.colour == a2.colour && a1.font == a2.font)
                {
                    a1.range.setEnd (a2.range.getEnd());
                    atts.remove (i + 1);

                    if (i < atts.size() - 1)
                        ++i;
                }
            }
    */
}

pub fn append_range(
    atts:   &mut Vec<AttributedStringAttribute>,
    length: i32,
    f:      *const Font,
    c:      *const Colour

) {
    
    todo!();
    /*
        if (atts.size() == 0)
            {
                atts.add ({ Range<int> (0, length), f != nullptr ? *f : Font(), c != nullptr ? *c : Colour (0xff000000) });
            }
            else
            {
                auto start = getLength (atts);
                atts.add ({ Range<int> (start, start + length),
                            f != nullptr ? *f : atts.getReference (atts.size() - 1).font,
                            c != nullptr ? *c : atts.getReference (atts.size() - 1).colour });

                mergeAdjacentRanges (atts);
            }
    */
}

pub fn apply_font_and_colour(
    atts:  &mut Vec<AttributedStringAttribute>,
    range: Range<i32>,
    f:     *const Font,
    c:     *const Colour

) {
    todo!();

    /*
        range = splitAttributeRanges (atts, range);

            for (auto& att : atts)
            {
                if (range.getStart() < att.range.getEnd())
                {
                    if (range.getEnd() <= att.range.getStart())
                        break;

                    if (c != nullptr) att.colour = *c;
                    if (f != nullptr) att.font = *f;
                }
            }

            mergeAdjacentRanges (atts);
    */
}

pub fn truncate(
    atts:       &mut Vec<AttributedStringAttribute>,
    new_length: i32

) {

    todo!();
    /*
        splitAttributeRanges (atts, newLength);

            for (int i = atts.size(); --i >= 0;)
                if (atts.getReference (i).range.getStart() >= newLength)
                    atts.remove (i);
    */
}
