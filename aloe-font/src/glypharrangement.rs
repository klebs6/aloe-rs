crate::ix!();

/**
  | A set of glyphs, each with a position.
  | 
  | You can create a GlyphArrangement,
  | text to it and then draw it onto a graphics
  | context. It's used internally by the
  | text methods in the Graphics class,
  | but can be used directly if more control
  | is needed.
  | 
  | @see Font, PositionedGlyph
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct GlyphArrangement {
    glyphs: Vec<PositionedGlyph>,
}

impl Default for GlyphArrangement {
    
    /**
      | Creates an empty arrangement.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/fonts/aloe_GlyphArrangement.cpp]
impl GlyphArrangement {

    /**
      | Returns the total number of glyphs in
      | the arrangement.
      |
      */
    pub fn get_num_glyphs(&self) -> i32 {
        
        todo!();
        /*
            return glyphs.size();
        */
    }

    pub fn begin(&self) -> *const PositionedGlyph {
        
        todo!();
        /*
            return glyphs.begin();
        */
    }
    
    pub fn end(&self) -> *const PositionedGlyph {
        
        todo!();
        /*
            return glyphs.end();
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
            glyphs.ensureStorageAllocated (128);
        */
    }
    
    /**
      | Clears all text from the arrangement
      | and resets it.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            glyphs.clear();
        */
    }
    
    /**
      | Returns one of the glyphs from the arrangement.
      | 
      | -----------
      | @param index
      | 
      | the glyph's index, from 0 to (getNumGlyphs()
      | - 1). Be careful not to pass an out-of-range
      | index here, as it doesn't do any bounds-checking.
      |
      */
    pub fn get_glyph(&mut self, index: i32) -> &mut PositionedGlyph {
        
        todo!();
        /*
            return glyphs.getReference (index);
        */
    }
    
    /**
      | Appends another glyph arrangement
      | to this one.
      |
      */
    pub fn add_glyph_arrangement(&mut self, other: &GlyphArrangement)  {
        
        todo!();
        /*
            glyphs.addArray (other.glyphs);
        */
    }
    
    /**
      | Appends a custom glyph to the arrangement.
      |
      */
    pub fn add_glyph(&mut self, glyph: &PositionedGlyph)  {
        
        todo!();
        /*
            glyphs.add (glyph);
        */
    }
    
    /**
      | Removes a set of glyphs from the arrangement.
      | 
      | -----------
      | @param startIndex
      | 
      | the first glyph to remove
      | ----------
      | @param numGlyphs
      | 
      | the number of glyphs to remove; if this
      | is < 0, all glyphs after startIndex will
      | be deleted
      |
      */
    pub fn remove_range_of_glyphs(&mut self, 
        start_index: i32,
        num:         i32)  {
        
        todo!();
        /*
            glyphs.removeRange (startIndex, num < 0 ? glyphs.size() : num);
        */
    }
    
    /**
      | Appends a line of text to the arrangement.
      | 
      | This will add the text as a single line,
      | where x is the left-hand edge of the first
      | character, and y is the position for
      | the text's baseline.
      | 
      | If the text contains new-lines or carriage-returns,
      | this will ignore them - use addJustifiedText()
      | to add multi-line arrangements.
      |
      */
    pub fn add_line_of_text(&mut self, 
        font:     &Font,
        text:     &String,
        x_offset: f32,
        y_offset: f32)  {
        
        todo!();
        /*
            addCurtailedLineOfText (font, text, xOffset, yOffset, 1.0e10f, false);
        */
    }
    
    /**
      | Adds a line of text, truncating it if
      | it's wider than a specified size.
      | 
      | This is the same as addLineOfText(),
      | but if the line's width exceeds the value
      | specified in maxWidthPixels, it will
      | be truncated using either ellipsis
      | (i.e. dots: "..."), if useEllipsis
      | is true, or if this is false, it will just
      | drop any subsequent characters.
      |
      */
    pub fn add_curtailed_line_of_text(&mut self, 
        font:             &Font,
        text:             &String,
        x_offset:         f32,
        y_offset:         f32,
        max_width_pixels: f32,
        use_ellipsis:     bool)  {
        
        todo!();
        /*
            if (text.isNotEmpty())
        {
            Vec<int> newGlyphs;
            Vec<float> xOffsets;
            font.getGlyphPositions (text, newGlyphs, xOffsets);
            auto textLen = newGlyphs.size();
            glyphs.ensureStorageAllocated (glyphs.size() + textLen);

            auto t = text.getCharPointer();

            for (int i = 0; i < textLen; ++i)
            {
                auto nextX = xOffsets.getUnchecked (i + 1);

                if (nextX > maxWidthPixels + 1.0f)
                {
                    // curtail the string if it's too wide..
                    if (useEllipsis && textLen > 3 && glyphs.size() >= 3)
                        insertEllipsis (font, xOffset + maxWidthPixels, 0, glyphs.size());

                    break;
                }

                auto thisX = xOffsets.getUnchecked (i);
                bool isWhitespace = t.isWhitespace();

                glyphs.add (PositionedGlyph (font, t.getAndAdvance(),
                                             newGlyphs.getUnchecked(i),
                                             xOffset + thisX, yOffset,
                                             nextX - thisX, isWhitespace));
            }
        }
        */
    }
    
    pub fn insert_ellipsis(&mut self, 
        font:        &Font,
        max_xpos:    f32,
        start_index: i32,
        end_index:   i32) -> i32 {
        
        todo!();
        /*
            int numDeleted = 0;

        if (! glyphs.isEmpty())
        {
            Vec<int> dotGlyphs;
            Vec<float> dotXs;
            font.getGlyphPositions ("..", dotGlyphs, dotXs);

            auto dx = dotXs[1];
            float xOffset = 0.0f, yOffset = 0.0f;

            while (endIndex > startIndex)
            {
                auto& pg = glyphs.getReference (--endIndex);
                xOffset = pg.x;
                yOffset = pg.y;

                glyphs.remove (endIndex);
                ++numDeleted;

                if (xOffset + dx * 3 <= maxXPos)
                    break;
            }

            for (int i = 3; --i >= 0;)
            {
                glyphs.insert (endIndex++, PositionedGlyph (font, '.', dotGlyphs.getFirst(),
                                                            xOffset, yOffset, dx, false));
                --numDeleted;
                xOffset += dx;

                if (xOffset > maxXPos)
                    break;
            }
        }

        return numDeleted;
        */
    }
    
    /**
      | Adds some multi-line text, breaking
      | lines at word-boundaries if they are
      | too wide.
      | 
      | This will add text to the arrangement,
      | breaking it into new lines either where
      | there is a new-line or carriage-return
      | character in the text, or where a line's
      | width exceeds the value set in maxLineWidth.
      | 
      | Each line that is added will be laid out
      | using the flags set in horizontalLayout,
      | so the lines can be left- or right-justified,
      | or centred horizontally in the space
      | between x and (x + maxLineWidth).
      | 
      | The y coordinate is the position of the
      | baseline of the first line of text - subsequent
      | lines will be placed below it, separated
      | by a distance of font.getHeight() +
      | leading.
      |
      */
    pub fn add_justified_text(
        &mut self, 
        font:              &Font,
        text:              &String,
        x:                 f32,
        y:                 f32,
        max_line_width:    f32,
        horizontal_layout: Justification,
        leading:           Option<f32>
    )  {

        let leading: f32 = leading.unwrap_or(0.0);
        
        todo!();
        /*
            auto lineStartIndex = glyphs.size();
        addLineOfText (font, text, x, y);

        auto originalY = y;

        while (lineStartIndex < glyphs.size())
        {
            int i = lineStartIndex;

            if (glyphs.getReference(i).getCharacter() != '\n'
                  && glyphs.getReference(i).getCharacter() != '\r')
                ++i;

            auto lineMaxX = glyphs.getReference (lineStartIndex).getLeft() + maxLineWidth;
            int lastWordBreakIndex = -1;

            while (i < glyphs.size())
            {
                auto& pg = glyphs.getReference (i);
                auto c = pg.getCharacter();

                if (c == '\r' || c == '\n')
                {
                    ++i;

                    if (c == '\r' && i < glyphs.size()
                         && glyphs.getReference(i).getCharacter() == '\n')
                        ++i;

                    break;
                }

                if (pg.isWhitespace())
                {
                    lastWordBreakIndex = i + 1;
                }
                else if (pg.getRight() - 0.0001f >= lineMaxX)
                {
                    if (lastWordBreakIndex >= 0)
                        i = lastWordBreakIndex;

                    break;
                }

                ++i;
            }

            auto currentLineStartX = glyphs.getReference (lineStartIndex).getLeft();
            auto currentLineEndX = currentLineStartX;

            for (int j = i; --j >= lineStartIndex;)
            {
                if (! glyphs.getReference (j).isWhitespace())
                {
                    currentLineEndX = glyphs.getReference (j).getRight();
                    break;
                }
            }

            float deltaX = 0.0f;

            if (horizontalLayout.testFlags (Justification::horizontallyJustified))
                spreadOutLine (lineStartIndex, i - lineStartIndex, maxLineWidth);
            else if (horizontalLayout.testFlags (Justification::horizontallyCentred))
                deltaX = (maxLineWidth - (currentLineEndX - currentLineStartX)) * 0.5f;
            else if (horizontalLayout.testFlags (Justification::right))
                deltaX = maxLineWidth - (currentLineEndX - currentLineStartX);

            moveRangeOfGlyphs (lineStartIndex, i - lineStartIndex,
                               x + deltaX - currentLineStartX, y - originalY);

            lineStartIndex = i;

            y += font.getHeight() + leading;
        }
        */
    }
    
    /**
      | Tries to fit some text within a given
      | space.
      | 
      | This does its best to make the given text
      | readable within the specified rectangle,
      | so it's useful for labelling things.
      | 
      | If the text is too big, it'll be squashed
      | horizontally or broken over multiple
      | lines if the maximumLinesToUse value
      | allows this. If the text just won't fit
      | into the space, it'll cram as much as
      | possible in there, and put some ellipsis
      | at the end to show that it's been truncated.
      | 
      | A Justification parameter lets you
      | specify how the text is laid out within
      | the rectangle, both horizontally and
      | vertically.
      | 
      | The minimumHorizontalScale parameter
      | specifies how much the text can be squashed
      | horizontally to try to squeeze it into
      | the space. If you don't want any horizontal
      | scaling to occur, you can set this value
      | to 1.0f. Pass 0 if you want it to use the
      | default value.
      | 
      | @see Graphics::drawFittedText
      |
      */
    pub fn add_fitted_text(
        &mut self, 
        f:                        &Font,
        text:                     &String,
        x:                        f32,
        y:                        f32,
        width:                    f32,
        height:                   f32,
        layout:                   Justification,
        maximum_lines:            i32,
        minimum_horizontal_scale: Option<f32>
    )  {

        let minimum_horizontal_scale: f32 = minimum_horizontal_scale.unwrap_or(0.0);
        
        todo!();
        /*
            if (minimumHorizontalScale == 0.0f)
            minimumHorizontalScale = Font::getDefaultMinimumHorizontalScaleFactor();

        // doesn't make much sense if this is outside a sensible range of 0.5 to 1.0
        jassert (minimumHorizontalScale > 0 && minimumHorizontalScale <= 1.0f);

        if (text.containsAnyOf ("\r\n"))
        {
            addLinesWithLineBreaks (text, f, x, y, width, height, layout);
        }
        else
        {
            auto startIndex = glyphs.size();
            auto trimmed = text.trim();
            addLineOfText (f, trimmed, x, y);
            auto numGlyphs = glyphs.size() - startIndex;

            if (numGlyphs > 0)
            {
                auto lineWidth = glyphs.getReference (glyphs.size() - 1).getRight()
                                    - glyphs.getReference (startIndex).getLeft();

                if (lineWidth > 0)
                {
                    if (lineWidth * minimumHorizontalScale < width)
                    {
                        if (lineWidth > width)
                            stretchRangeOfGlyphs (startIndex, numGlyphs, width / lineWidth);

                        justifyGlyphs (startIndex, numGlyphs, x, y, width, height, layout);
                    }
                    else if (maximumLines <= 1)
                    {
                        fitLineIntoSpace (startIndex, numGlyphs, x, y, width, height,
                                          f, layout, minimumHorizontalScale);
                    }
                    else
                    {
                        splitLines (trimmed, f, startIndex, x, y, width, height,
                                    maximumLines, lineWidth, layout, minimumHorizontalScale);
                    }
                }
            }
        }
        */
    }
    
    /**
      | Shifts a set of glyphs by a given amount.
      | 
      | -----------
      | @param startIndex
      | 
      | the first glyph to transform
      | ----------
      | @param numGlyphs
      | 
      | the number of glyphs to move; if this
      | is < 0, all glyphs after startIndex will
      | be used
      | ----------
      | @param deltaX
      | 
      | the amount to add to their x-positions
      | ----------
      | @param deltaY
      | 
      | the amount to add to their y-positions
      |
      */
    pub fn move_range_of_glyphs(&mut self, 
        start_index: i32,
        num:         i32,
        dx:          f32,
        dy:          f32)  {
        
        todo!();
        /*
            jassert (startIndex >= 0);

        if (dx != 0.0f || dy != 0.0f)
        {
            if (num < 0 || startIndex + num > glyphs.size())
                num = glyphs.size() - startIndex;

            while (--num >= 0)
                glyphs.getReference (startIndex++).moveBy (dx, dy);
        }
        */
    }
    
    pub fn add_lines_with_line_breaks(&mut self, 
        text:   &String,
        f:      &Font,
        x:      f32,
        y:      f32,
        width:  f32,
        height: f32,
        layout: Justification)  {
        
        todo!();
        /*
            GlyphArrangement ga;
        ga.addJustifiedText (f, text, x, y, width, layout);

        auto bb = ga.getBoundingBox (0, -1, false);
        auto dy = y - bb.getY();

        if (layout.testFlags (Justification::verticallyCentred))   dy += (height - bb.getHeight()) * 0.5f;
        else if (layout.testFlags (Justification::bottom))         dy += (height - bb.getHeight());

        ga.moveRangeOfGlyphs (0, -1, 0.0f, dy);

        glyphs.addArray (ga.glyphs);
        */
    }
    
    pub fn fit_line_into_space(&mut self, 
        start:                    i32,
        num_glyphs:               i32,
        x:                        f32,
        y:                        f32,
        w:                        f32,
        h:                        f32,
        font:                     &Font,
        justification:            Justification,
        minimum_horizontal_scale: f32) -> i32 {
        
        todo!();
        /*
            int numDeleted = 0;
        auto lineStartX = glyphs.getReference (start).getLeft();
        auto lineWidth  = glyphs.getReference (start + numGlyphs - 1).getRight() - lineStartX;

        if (lineWidth > w)
        {
            if (minimumHorizontalScale < 1.0f)
            {
                stretchRangeOfGlyphs (start, numGlyphs, jmax (minimumHorizontalScale, w / lineWidth));
                lineWidth = glyphs.getReference (start + numGlyphs - 1).getRight() - lineStartX - 0.5f;
            }

            if (lineWidth > w)
            {
                numDeleted = insertEllipsis (font, lineStartX + w, start, start + numGlyphs);
                numGlyphs -= numDeleted;
            }
        }

        justifyGlyphs (start, numGlyphs, x, y, w, h, justification);
        return numDeleted;
        */
    }
    
    /**
      | Expands or compresses a set of glyphs
      | horizontally.
      | 
      | -----------
      | @param startIndex
      | 
      | the first glyph to transform
      | ----------
      | @param numGlyphs
      | 
      | the number of glyphs to stretch; if this
      | is < 0, all glyphs after startIndex will
      | be used
      | ----------
      | @param horizontalScaleFactor
      | 
      | how much to scale their horizontal width
      | by
      |
      */
    pub fn stretch_range_of_glyphs(&mut self, 
        start_index:             i32,
        num:                     i32,
        horizontal_scale_factor: f32)  {
        
        todo!();
        /*
            jassert (startIndex >= 0);

        if (num < 0 || startIndex + num > glyphs.size())
            num = glyphs.size() - startIndex;

        if (num > 0)
        {
            auto xAnchor = glyphs.getReference (startIndex).getLeft();

            while (--num >= 0)
            {
                auto& pg = glyphs.getReference (startIndex++);

                pg.x = xAnchor + (pg.x - xAnchor) * horizontalScaleFactor;
                pg.font.setHorizontalScale (pg.font.getHorizontalScale() * horizontalScaleFactor);
                pg.w *= horizontalScaleFactor;
            }
        }
        */
    }
    
    /**
      | Finds the smallest rectangle that will
      | enclose a subset of the glyphs.
      | 
      | -----------
      | @param startIndex
      | 
      | the first glyph to test
      | ----------
      | @param numGlyphs
      | 
      | the number of glyphs to include; if this
      | is < 0, all glyphs after startIndex will
      | be included
      | ----------
      | @param includeWhitespace
      | 
      | if true, the extent of any whitespace
      | characters will also be taken into account
      |
      */
    pub fn get_bounding_box(&self, 
        start_index:        i32,
        num:                i32,
        include_whitespace: bool) -> Rectangle<f32> {
        
        todo!();
        /*
            jassert (startIndex >= 0);

        if (num < 0 || startIndex + num > glyphs.size())
            num = glyphs.size() - startIndex;

        Rectangle<float> result;

        while (--num >= 0)
        {
            auto& pg = glyphs.getReference (startIndex++);

            if (includeWhitespace || ! pg.isWhitespace())
                result = result.getUnion (pg.getBounds());
        }

        return result;
        */
    }
    
    /**
      | Justifies a set of glyphs within a given
      | space.
      | 
      | This moves the glyphs as a block so that
      | the whole thing is located within the
      | given rectangle with the specified
      | layout.
      | 
      | If the Justification::horizontallyJustified
      | flag is specified, each line will be
      | stretched out to fill the specified
      | width.
      |
      */
    pub fn justify_glyphs(&mut self, 
        start_index:   i32,
        num:           i32,
        x:             f32,
        y:             f32,
        width:         f32,
        height:        f32,
        justification: Justification)  {
        
        todo!();
        /*
            jassert (num >= 0 && startIndex >= 0);

        if (glyphs.size() > 0 && num > 0)
        {
            auto bb = getBoundingBox (startIndex, num, ! justification.testFlags (Justification::horizontallyJustified
                                                                                   | Justification::horizontallyCentred));
            float deltaX = x, deltaY = y;

            if (justification.testFlags (Justification::horizontallyJustified))     deltaX -= bb.getX();
            else if (justification.testFlags (Justification::horizontallyCentred))  deltaX += (width - bb.getWidth()) * 0.5f - bb.getX();
            else if (justification.testFlags (Justification::right))                deltaX += width - bb.getRight();
            else                                                                    deltaX -= bb.getX();

            if (justification.testFlags (Justification::top))                       deltaY -= bb.getY();
            else if (justification.testFlags (Justification::bottom))               deltaY += height - bb.getBottom();
            else                                                                    deltaY += (height - bb.getHeight()) * 0.5f - bb.getY();

            moveRangeOfGlyphs (startIndex, num, deltaX, deltaY);

            if (justification.testFlags (Justification::horizontallyJustified))
            {
                int lineStart = 0;
                auto baseY = glyphs.getReference (startIndex).getBaselineY();

                int i;
                for (i = 0; i < num; ++i)
                {
                    auto glyphY = glyphs.getReference (startIndex + i).getBaselineY();

                    if (glyphY != baseY)
                    {
                        spreadOutLine (startIndex + lineStart, i - lineStart, width);

                        lineStart = i;
                        baseY = glyphY;
                    }
                }

                if (i > lineStart)
                    spreadOutLine (startIndex + lineStart, i - lineStart, width);
            }
        }
        */
    }
    
    pub fn spread_out_line(&mut self, 
        start:        i32,
        num:          i32,
        target_width: f32)  {
        
        todo!();
        /*
            if (start + num < glyphs.size()
             && glyphs.getReference (start + num - 1).getCharacter() != '\r'
             && glyphs.getReference (start + num - 1).getCharacter() != '\n')
        {
            int numSpaces = 0;
            int spacesAtEnd = 0;

            for (int i = 0; i < num; ++i)
            {
                if (glyphs.getReference (start + i).isWhitespace())
                {
                    ++spacesAtEnd;
                    ++numSpaces;
                }
                else
                {
                    spacesAtEnd = 0;
                }
            }

            numSpaces -= spacesAtEnd;

            if (numSpaces > 0)
            {
                auto startX = glyphs.getReference (start).getLeft();
                auto endX   = glyphs.getReference (start + num - 1 - spacesAtEnd).getRight();

                auto extraPaddingBetweenWords = (targetWidth - (endX - startX)) / (float) numSpaces;
                float deltaX = 0.0f;

                for (int i = 0; i < num; ++i)
                {
                    glyphs.getReference (start + i).moveBy (deltaX, 0.0f);

                    if (glyphs.getReference (start + i).isWhitespace())
                        deltaX += extraPaddingBetweenWords;
                }
            }
        }
        */
    }
    
    pub fn split_lines(&mut self, 
        text:                     &String,
        font:                     Font,
        start_index:              i32,
        x:                        f32,
        y:                        f32,
        width:                    f32,
        height:                   f32,
        maximum_lines:            i32,
        line_width:               f32,
        layout:                   Justification,
        minimum_horizontal_scale: f32)  {
        
        todo!();
        /*
            auto length = text.length();
        auto originalStartIndex = startIndex;
        int numLines = 1;

        if (length <= 12 && ! text.containsAnyOf (" -\t\r\n"))
            maximumLines = 1;

        maximumLines = jmin (maximumLines, length);

        while (numLines < maximumLines)
        {
            ++numLines;
            auto newFontHeight = height / (float) numLines;

            if (newFontHeight < font.getHeight())
            {
                font.setHeight (jmax (8.0f, newFontHeight));

                removeRangeOfGlyphs (startIndex, -1);
                addLineOfText (font, text, x, y);

                lineWidth = glyphs.getReference (glyphs.size() - 1).getRight()
                                - glyphs.getReference (startIndex).getLeft();
            }

            // Try to estimate the point at which there are enough lines to fit the text,
            // allowing for unevenness in the lengths due to differently sized words.
            const float lineLengthUnevennessAllowance = 80.0f;

            if ((float) numLines > (lineWidth + lineLengthUnevennessAllowance) / width || newFontHeight < 8.0f)
                break;
        }

        if (numLines < 1)
            numLines = 1;

        int lineIndex = 0;
        auto lineY = y;
        auto widthPerLine = jmin (width / minimumHorizontalScale,
                                  lineWidth / (float) numLines);

        while (lineY < y + height)
        {
            auto endIndex = startIndex;
            auto lineStartX = glyphs.getReference (startIndex).getLeft();
            auto lineBottomY = lineY + font.getHeight();

            if (lineIndex++ >= numLines - 1
                 || lineBottomY >= y + height)
            {
                widthPerLine = width;
                endIndex = glyphs.size();
            }
            else
            {
                while (endIndex < glyphs.size())
                {
                    if (glyphs.getReference (endIndex).getRight() - lineStartX > widthPerLine)
                    {
                        // got to a point where the line's too long, so skip forward to find a
                        // good place to break it..
                        auto searchStartIndex = endIndex;

                        while (endIndex < glyphs.size())
                        {
                            auto& g = glyphs.getReference (endIndex);

                            if ((g.getRight() - lineStartX) * minimumHorizontalScale < width)
                            {
                                if (isBreakableGlyph (g))
                                {
                                    ++endIndex;
                                    break;
                                }
                            }
                            else
                            {
                                // can't find a suitable break, so try looking backwards..
                                endIndex = searchStartIndex;

                                for (int back = 1; back < jmin (7, endIndex - startIndex - 1); ++back)
                                {
                                    if (isBreakableGlyph (glyphs.getReference (endIndex - back)))
                                    {
                                        endIndex -= back - 1;
                                        break;
                                    }
                                }

                                break;
                            }

                            ++endIndex;
                        }

                        break;
                    }

                    ++endIndex;
                }

                auto wsStart = endIndex;
                auto wsEnd   = endIndex;

                while (wsStart > 0 && glyphs.getReference (wsStart - 1).isWhitespace())
                    --wsStart;

                while (wsEnd < glyphs.size() && glyphs.getReference (wsEnd).isWhitespace())
                    ++wsEnd;

                removeRangeOfGlyphs (wsStart, wsEnd - wsStart);
                endIndex = jmax (wsStart, startIndex + 1);
            }

            endIndex -= fitLineIntoSpace (startIndex, endIndex - startIndex,
                                          x, lineY, width, font.getHeight(), font,
                                          layout.getOnlyHorizontalFlags() | Justification::verticallyCentred,
                                          minimumHorizontalScale);

            startIndex = endIndex;
            lineY = lineBottomY;

            if (startIndex >= glyphs.size())
                break;
        }

        justifyGlyphs (originalStartIndex, glyphs.size() - originalStartIndex,
                       x, y, width, height, layout.getFlags() & ~Justification::horizontallyJustified);
        */
    }
    
    /**
      | Converts the set of glyphs into a path.
      | 
      | -----------
      | @param path
      | 
      | the glyphs' outlines will be appended
      | to this path
      |
      */
    pub fn create_path(&self, path: &mut Path)  {
        
        todo!();
        /*
            for (auto& g : glyphs)
            g.createPath (path);
        */
    }
    
    /**
      | Looks for a glyph that contains the given
      | coordinate.
      | 
      | 
      | -----------
      | @return
      | 
      | the index of the glyph, or -1 if none were
      | found.
      |
      */
    pub fn find_glyph_index_at(&self, x: f32, y: f32) -> i32 {
        
        todo!();
        /*
            for (int i = 0; i < glyphs.size(); ++i)
            if (glyphs.getReference (i).hitTest (x, y))
                return i;

        return -1;
        */
    }
}
