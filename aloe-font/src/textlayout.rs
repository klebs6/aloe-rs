crate::ix!();

pub fn substring(
        text:  &String,
        range: Range<i32>) -> String {
    
    todo!();
    /*
        return text.substring (range.getStart(), range.getEnd());
    */
}

/**
  | A positioned glyph.
  |
  */
#[leak_detector]
#[derive(Default)]
pub struct TextLayoutGlyph {

    /**
      | The code number of this glyph.
      |
      */
    glyph_code: i32,

    /**
      | The glyph's anchor point - this is relative
      | to the line's origin. @see text_layout::Line::lineOrigin
      |
      */
    anchor:     Point<f32>,

    width:      f32,
}

impl TextLayoutGlyph {

    pub fn new(
        glyph: i32,
        anch:  Point<f32>,
        w:     f32) -> Self {
    
        todo!();
        /*
        : glyph_code(glyph),
        : anchor(anch),
        : width(w),

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_TextLayout.h]

/**
  | A Pre-formatted piece of text, which
  | may contain multiple fonts and colours.
  | 
  | A TextLayout is created from an AttributedString,
  | and once created can be quickly drawn
  | into a Graphics context.
  | 
  | @see AttributedString
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct TextLayout {
    lines:         Vec<text_layout::Line>,
    width:         f32,
    height:        f32,
    justification: Justification,
}

pub mod text_layout {

    use super::*;

    /**
      | A sequence of glyphs with a common font
      | and colour.
      |
      */
    #[leak_detector]
    #[derive(Default)]
    pub struct Run {

        /**
          | The run's font.
          |
          */
        font:         Font,

        /**
          | The run's colour.
          |
          */
        colour:       Colour, // default = { 0xff000000  }

        /**
          | The glyphs in this run.
          |
          */
        glyphs:       Vec<TextLayoutGlyph>,

        /**
          | The character range that this run represents
          | in the original string that was used to create
          | it.
          */
        string_range: Range<i32>,
    }

    impl Run {

        pub fn new(
            range:                     Range<i32>,
            num_glyphs_to_preallocate: i32) -> Self {

            todo!();
            /*


                : stringRange (range)

                glyphs.ensureStorageAllocated (numGlyphsToPreallocate);
            */
        }
        
        /**
          | Returns the X position range which contains
          | all the glyphs in this run.
          |
          */
        pub fn get_run_boundsx(&self) -> Range<f32> {
            
            todo!();
            /*
                Range<float> range;
                bool isFirst = true;

                for (auto& glyph : glyphs)
                {
                    Range<float> r (glyph.anchor.x, glyph.anchor.x + glyph.width);

                    if (isFirst)
                    {
                        isFirst = false;
                        range = r;
                    }
                    else
                    {
                        range = range.getUnionWith (r);
                    }
                }

                return range;
            */
        }
    }
    
    /**
      | A line containing a sequence of
      | glyph-runs.
      |
      */
    #[derive(Default)]
    #[leak_detector]
    pub struct Line {

        /**
          | The glyph-runs in this line.
          |
          */
        runs:         Vec<Box<Run>>,

        /**
          | The character range that this line represents
          | in the original string that was used to create
          | it.
          */
        string_range: Range<i32>,

        /**
          | The line's baseline origin.
          |
          */
        line_origin:  Point<f32>,

        ascent:       f32, // default = 0.0f
        descent:      f32, // default = 0.0f
        leading:      f32, // default = 0.0f
    }

    impl Line {

        pub fn new(
            range:                   Range<i32>,
            o:                       Point<f32>,
            asc:                     f32,
            desc:                    f32,
            lead:                    f32,
            num_runs_to_preallocate: i32) -> Self {
        
            todo!();
            /*


                : stringRange (range), lineOrigin (o),
                  ascent (asc), descent (desc), leading (lead)

                runs.ensureStorageAllocated (numRunsToPreallocate);
            */
        }
        
        pub fn new_from_line_ref(other: &Line) -> Self {
        
            todo!();
            /*


                : stringRange (other.stringRange), lineOrigin (other.lineOrigin),
                  ascent (other.ascent), descent (other.descent), leading (other.leading)

                runs.addCopiesOf (other.runs);
            */
        }
        
        pub fn assign_from_line_ref(&mut self, other: &Line) 
            -> &mut text_layout::Line 
        {
            todo!();
            /*
                auto copy = other;
                swap (copy);
                return *this;
            */
        }
        
        /**
          | Returns the X position range which contains
          | all the glyphs in this line.
          |
          */
        pub fn get_line_boundsx(&self) -> Range<f32> {
            
            todo!();
            /*
                Range<float> range;
                bool isFirst = true;

                for (auto* run : runs)
                {
                    auto runRange = run->getRunBoundsX();

                    if (isFirst)
                    {
                        isFirst = false;
                        range = runRange;
                    }
                    else
                    {
                        range = range.getUnionWith (runRange);
                    }
                }

                return range + lineOrigin.x;
            */
        }
        
        /**
          | Returns the Y position range which contains
          | all the glyphs in this line.
          |
          */
        pub fn get_line_boundsy(&self) -> Range<f32> {
            
            todo!();
            /*
                return { lineOrigin.y - ascent,
                         lineOrigin.y + descent };
            */
        }
        
        /**
          | Returns the smallest rectangle which
          | contains all the glyphs in this line.
          |
          */
        pub fn get_line_bounds(&self) -> Rectangle<f32> {
            
            todo!();
            /*
                auto x = getLineBoundsX();
                auto y = getLineBoundsY();

                return { x.getStart(), y.getStart(), x.getLength(), y.getLength() };
            */
        }
        
        pub fn swap(&mut self, other: &mut Line)  {
            
            todo!();
            /*
                std::swap (other.runs,          runs);
                std::swap (other.stringRange,   stringRange);
                std::swap (other.lineOrigin,    lineOrigin);
                std::swap (other.ascent,        ascent);
                std::swap (other.descent,       descent);
                std::swap (other.leading,       leading);
            */
        }
    }

    ///------------------------
    pub struct Token {
        text:          String,
        font:          Font,
        colour:        Colour,
        area:          Rectangle<f32>,
        line:          i32,
        line_height:   f32,
        is_whitespace: bool,
        is_new_line:   bool,
    }

    impl Token {

        pub fn new(
            t:          &String,
            f:          &Font,
            c:          Colour,
            whitespace: bool) -> Self {
        
            todo!();
            /*


                : text (t), font (f), colour (c),
                      area (font.getStringWidthFloat (t), f.getHeight()),
                      isWhitespace (whitespace),
                      isNewLine (t.containsChar ('\n') || t.containsChar ('\r'))
            */
        }
    }

    ///------------------------
    #[no_copy]
    pub struct TokenList {
        tokens:      Vec<Box<Token>>,
        total_lines: i32, // default = 0
    }

    impl TokenList {

        pub fn create_layout(
            &mut self, 
            text:   &AttributedString,
            layout: &mut TextLayout)  {
            
            todo!();
            /*
                layout.ensureStorageAllocated (totalLines);

                    addTextRuns (text);
                    layoutRuns (layout.getWidth(), text.getLineSpacing(), text.getWordWrap());

                    int charPosition = 0;
                    int lineStartPosition = 0;
                    int runStartPosition = 0;

                    std::unique_ptr<text_layout::Line> currentLine;
                    std::unique_ptr<text_layout::Run> currentRun;

                    bool needToSetLineOrigin = true;

                    for (int i = 0; i < tokens.size(); ++i)
                    {
                        auto& t = *tokens.getUnchecked (i);

                        Vec<int> newGlyphs;
                        Vec<float> xOffsets;
                        t.font.getGlyphPositions (getTrimmedEndIfNotAllWhitespace (t.text), newGlyphs, xOffsets);

                        if (currentRun == nullptr)  currentRun  = std::make_unique<text_layout::Run>();
                        if (currentLine == nullptr) currentLine = std::make_unique<text_layout::Line>();

                        const auto numGlyphs = newGlyphs.size();
                        charPosition += numGlyphs;

                        if (numGlyphs > 0
                            && (! (t.isWhitespace || t.isNewLine) || needToSetLineOrigin))
                        {
                            currentRun->glyphs.ensureStorageAllocated (currentRun->glyphs.size() + newGlyphs.size());
                            auto tokenOrigin = t.area.getPosition().translated (0, t.font.getAscent());

                            if (needToSetLineOrigin)
                            {
                                needToSetLineOrigin = false;
                                currentLine->lineOrigin = tokenOrigin;
                            }

                            auto glyphOffset = tokenOrigin - currentLine->lineOrigin;

                            for (int j = 0; j < newGlyphs.size(); ++j)
                            {
                                auto x = xOffsets.getUnchecked (j);
                                currentRun->glyphs.add (text_layout::Glyph (newGlyphs.getUnchecked (j),
                                                                           glyphOffset.translated (x, 0),
                                                                           xOffsets.getUnchecked (j + 1) - x));
                            }
                        }

                        if (auto* nextToken = tokens[i + 1])
                        {
                            if (t.font != nextToken->font || t.colour != nextToken->colour)
                            {
                                addRun (*currentLine, currentRun.release(), t, runStartPosition, charPosition);
                                runStartPosition = charPosition;
                            }

                            if (t.line != nextToken->line)
                            {
                                if (currentRun == nullptr)
                                    currentRun = std::make_unique<text_layout::Run>();

                                addRun (*currentLine, currentRun.release(), t, runStartPosition, charPosition);
                                currentLine->stringRange = { lineStartPosition, charPosition };

                                if (! needToSetLineOrigin)
                                    layout.addLine (std::move (currentLine));

                                runStartPosition = charPosition;
                                lineStartPosition = charPosition;
                                needToSetLineOrigin = true;
                            }
                        }
                        else
                        {
                            addRun (*currentLine, currentRun.release(), t, runStartPosition, charPosition);
                            currentLine->stringRange = { lineStartPosition, charPosition };

                            if (! needToSetLineOrigin)
                                layout.addLine (std::move (currentLine));

                            needToSetLineOrigin = true;
                        }
                    }

                    if ((text.getJustification().getFlags() & (Justification::right | Justification::horizontallyCentred)) != 0)
                    {
                        auto totalW = layout.getWidth();
                        bool isCentred = (text.getJustification().getFlags() & Justification::horizontallyCentred) != 0;

                        for (auto& line : layout)
                        {
                            auto dx = totalW - line.getLineBoundsX().getLength();

                            if (isCentred)
                                dx /= 2.0f;

                            line.lineOrigin.x += dx;
                        }
                    }
            */
        }
        
        pub fn add_run(
            glyph_line: &mut text_layout::Line,
            glyph_run:  *mut text_layout::Run,
            t:          &Token,
            start:      i32,
            end:        i32)  {
            
            todo!();
            /*
                glyphRun->stringRange = { start, end };
                    glyphRun->font = t.font;
                    glyphRun->colour = t.colour;
                    glyphLine.ascent  = jmax (glyphLine.ascent,  t.font.getAscent());
                    glyphLine.descent = jmax (glyphLine.descent, t.font.getDescent());
                    glyphLine.runs.add (glyphRun);
            */
        }
        
        pub fn get_character_type(c: wchar_t) -> i32 {
            
            todo!();
            /*
                if (c == '\r' || c == '\n')
                        return 0;

                    return CharacterFunctions::isWhitespace (c) ? 2 : 1;
            */
        }
        
        pub fn append_text(&mut self, 
            string_text: &String,
            font:        &Font,
            colour:      Colour)  {
            
            todo!();
            /*
                auto t = stringText.getCharPointer();
                    String currentString;
                    int lastCharType = 0;

                    for (;;)
                    {
                        auto c = t.getAndAdvance();

                        if (c == 0)
                            break;

                        auto charType = getCharacterType (c);

                        if (charType == 0 || charType != lastCharType)
                        {
                            if (currentString.isNotEmpty())
                                tokens.add (new Token (currentString, font, colour,
                                                       lastCharType == 2 || lastCharType == 0));

                            currentString = String::charToString (c);

                            if (c == '\r' && *t == '\n')
                                currentString += t.getAndAdvance();
                        }
                        else
                        {
                            currentString += c;
                        }

                        lastCharType = charType;
                    }

                    if (currentString.isNotEmpty())
                        tokens.add (new Token (currentString, font, colour, lastCharType == 2));
            */
        }
        
        pub fn layout_runs(
            &mut self, 
            max_width:          f32,
            extra_line_spacing: f32,
            word_wrap:          WordWrap)  {
            
            todo!();
            /*
                float x = 0, y = 0, h = 0;
                    int i;

                    for (i = 0; i < tokens.size(); ++i)
                    {
                        auto& t = *tokens.getUnchecked (i);
                        t.area.setPosition (x, y);
                        t.line = totalLines;
                        x += t.area.getWidth();
                        h = jmax (h, t.area.getHeight() + extraLineSpacing);

                        auto* nextTok = tokens[i + 1];

                        if (nextTok == nullptr)
                            break;

                        bool tokenTooLarge = (x + nextTok->area.getWidth() > maxWidth);

                        if (t.isNewLine || ((! nextTok->isWhitespace) && (tokenTooLarge && wordWrap != AttributedString::none)))
                        {
                            setLastLineHeight (i + 1, h);
                            x = 0;
                            y += h;
                            h = 0;
                            ++totalLines;
                        }
                    }

                    setLastLineHeight (jmin (i + 1, tokens.size()), h);
                    ++totalLines;
            */
        }
        
        pub fn set_last_line_height(&mut self, 
            i:      i32,
            height: f32)  {
            
            todo!();
            /*
                while (--i >= 0)
                    {
                        auto& tok = *tokens.getUnchecked (i);

                        if (tok.line == totalLines)
                            tok.lineHeight = height;
                        else
                            break;
                    }
            */
        }
        
        pub fn add_text_runs(&mut self, text: &AttributedString)  {
            
            todo!();
            /*
                auto numAttributes = text.getNumAttributes();
                    tokens.ensureStorageAllocated (jmax (64, numAttributes));

                    for (int i = 0; i < numAttributes; ++i)
                    {
                        auto& attr = text.getAttribute (i);

                        appendText (substring (text.getText(), attr.range),
                                    attr.font, attr.colour);
                    }
            */
        }

        pub fn get_trimmed_end_if_not_all_whitespace(s: &String) -> String {
            
            todo!();
            /*
                auto trimmed = s.trimEnd();

                    if (trimmed.isEmpty() && s.isNotEmpty())
                        trimmed = s.replaceCharacters ("\r\n\t", "   ");

                    return trimmed;
            */
        }
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_TextLayout.cpp]
impl Default for TextLayout {

    /**
      | Creates an empty layout.
      | 
      | Having created a TextLayout, you can
      | populate it using createLayout() or
      | createLayoutWithBalancedLineLengths().
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : width(0),
        : height(0),
        : justification(Justification::topLeft),

        
        */
    }
}
    
impl TextLayout {
    
    /**
      | Returns the maximum width of the content.
      |
      */
    pub fn get_width(&self) -> f32 {
        
        todo!();
        /*
            return width;
        */
    }

    /**
      | Returns the maximum height of the content.
      |
      */
    pub fn get_height(&self) -> f32 {
        
        todo!();
        /*
            return height;
        */
    }

    /**
      | Returns the number of lines in the layout.
      |
      */
    pub fn get_num_lines(&self) -> i32 {
        
        todo!();
        /*
            return lines.size();
        */
    }

    /**
      | Returns an iterator over the lines of
      | content
      |
      */
    pub fn iter(&self) -> std::slice::Iter<'_, text_layout::Line> {
        self.lines.iter()
    }

    /**
      | Returns a mutable iterator over the lines of
      | content
      |
      */
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, text_layout::Line> {
        self.lines.iter_mut()
    }

    pub fn new_from_other_ref(other: &TextLayout) -> Self {
    
        todo!();
        /*
        : width(other.width),
        : height(other.height),
        : justification(other.justification),

            lines.addCopiesOf (other.lines);
        */
    }
    
    pub fn new_from_other(other: TextLayout) -> Self {
    
        todo!();
        /*
        : lines(std::move (other.lines)),
        : width(other.width),
        : height(other.height),
        : justification(other.justification),

        
        */
    }
    
    pub fn assign_from_other(&mut self, other: TextLayout) -> &mut TextLayout {
        
        todo!();
        /*
            lines = std::move (other.lines);
        width = other.width;
        height = other.height;
        justification = other.justification;
        return *this;
        */
    }
    
    pub fn assign_from_other_ref(&mut self, other: &TextLayout) -> &mut TextLayout {
        
        todo!();
        /*
            width = other.width;
        height = other.height;
        justification = other.justification;
        lines.clear();
        lines.addCopiesOf (other.lines);
        return *this;
        */
    }

    /**
      | Returns one of the lines.
      |
      */
    pub fn get_line(&self, index: i32) -> &mut text_layout::Line {
        
        todo!();
        /*
            return *lines.getUnchecked (index);
        */
    }
    
    /**
      | Pre-allocates space for the specified
      | number of lines.
      |
      */
    pub fn ensure_storage_allocated(&mut self, num_lines_needed: i32)  {
        
        todo!();
        /*
            lines.ensureStorageAllocated (numLinesNeeded);
        */
    }
    
    /**
      | Adds a line to the layout. The layout
      | will take ownership of this line object
      | and will delete it when it is no longer
      | needed.
      |
      */
    pub fn add_line(&mut self, line: Box<text_layout::Line>)  {
        
        todo!();
        /*
            lines.add (line.release());
        */
    }
    
    /**
      | Creates a layout from the given attributed
      | string.
      | 
      | This will replace any data that is currently
      | stored in the layout.
      |
      */
    pub fn create_layout(&mut self, 
        text:      &AttributedString,
        max_width: f32)  {
        
        todo!();
        /*
            createLayout (text, maxWidth, 1.0e7f);
        */
    }
    
    /**
      | Creates a layout from the given attributed
      | string, given some size constraints.
      | 
      | This will replace any data that is currently
      | stored in the layout.
      |
      */
    pub fn create_layout_with_height(
        &mut self, 
        text:       &AttributedString,
        max_width:  f32,
        max_height: f32)  {
        
        todo!();
        /*
            lines.clear();
        width = maxWidth;
        height = maxHeight;
        justification = text.getJustification();

        if (! createNativeLayout (text))
            createStandardLayout (text);

        recalculateSize();
        */
    }
    
    /**
      | Creates a layout, attempting to choose
      | a width which results in lines of a similar
      | length.
      | 
      | This will be slower than the normal createLayout
      | method, but produces a tidier result.
      |
      */
    pub fn create_layout_with_balanced_line_lengths(&mut self, 
        text:      &AttributedString,
        max_width: f32)  {
        
        todo!();
        /*
            createLayoutWithBalancedLineLengths (text, maxWidth, 1.0e7f);
        */
    }
    
    /**
      | Creates a layout, attempting to choose
      | a width which results in lines of a similar
      | length.
      | 
      | This will be slower than the normal createLayout
      | method, but produces a tidier result.
      |
      */
    pub fn create_layout_with_balanced_line_lengths_with_height(
        &mut self, 
        text:       &AttributedString,
        max_width:  f32,
        max_height: f32)  {
        
        todo!();
        /*
            auto minimumWidth = maxWidth / 2.0f;
        auto bestWidth = maxWidth;
        float bestLineProportion = 0.0f;

        while (maxWidth > minimumWidth)
        {
            createLayout (text, maxWidth, maxHeight);

            if (getNumLines() < 2)
                return;

            auto line1 = lines.getUnchecked (lines.size() - 1)->getLineBoundsX().getLength();
            auto line2 = lines.getUnchecked (lines.size() - 2)->getLineBoundsX().getLength();
            auto shortest = jmin (line1, line2);
            auto longest  = jmax (line1, line2);
            auto prop = shortest > 0 ? longest / shortest : 1.0f;

            if (prop > 0.9f && prop < 1.1f)
                return;

            if (prop > bestLineProportion)
            {
                bestLineProportion = prop;
                bestWidth = maxWidth;
            }

            maxWidth -= 10.0f;
        }

        if (bestWidth != maxWidth)
            createLayout (text, bestWidth, maxHeight);
        */
    }
    
    pub fn create_standard_layout(&mut self, text: &AttributedString)  {
        
        todo!();
        /*
            TextLayoutHelpers::TokenList l;
        l.createLayout (text, *this);
        */
    }
    
    /**
      | If you modify the TextLayout after creating
      | it, call this to compute the new dimensions
      | of the content.
      |
      */
    pub fn recalculate_size(&mut self)  {
        
        todo!();
        /*
            if (! lines.isEmpty())
        {
            auto bounds = lines.getFirst()->getLineBounds();

            for (auto* line : lines)
                bounds = bounds.getUnion (line->getLineBounds());

            for (auto* line : lines)
                line->lineOrigin.x -= bounds.getX();

            width  = bounds.getWidth();
            height = bounds.getHeight();
        }
        else
        {
            width = 0;
            height = 0;
        }
        */
    }
}
