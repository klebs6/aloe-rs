crate::ix!();

#[leak_detector]
pub struct TextEditorIterator<'a> {
    index_in_text:        i32, // default = 0
    liney:                f32, // default = 0
    line_height:          f32, // default = 0
    max_descent:          f32, // default = 0
    atomx:                f32, // default = 0
    atom_right:           f32, // default = 0
    atom:                 *const TextAtom, // default = nullptr
    sections:             &'a Vec<TextEditorUniformTextSection>,
    current_section:      *const TextEditorUniformTextSection, // default = nullptr
    section_index:        i32, // default = 0
    atom_index:           i32, // default = 0
    justification:        Justification,
    bottom_right:         Point<f32>,
    word_wrap_width:      f32,
    password_character:   wchar_t,
    line_spacing:         f32,
    underline_whitespace: bool,
    long_atom:            TextAtom,
}

impl<'a> TextEditorIterator<'a> {

    pub fn new(ed: &TextEditor) -> Self {
    
        todo!();
        /*


            : sections (ed.sections),
            justification (ed.justification),
            bottomRight ((float) ed.getMaximumTextWidth(), (float) ed.getMaximumTextHeight()),
            wordWrapWidth ((float) ed.getWordWrapWidth()),
            passwordCharacter (ed.passwordCharacter),
            lineSpacing (ed.lineSpacing),
            underlineWhitespace (ed.underlineWhitespace)

            jassert (wordWrapWidth > 0);

            if (! sections.isEmpty())
            {
                currentSection = sections.getUnchecked (sectionIndex);

                if (currentSection != nullptr)
                    beginNewLine();
            }

            lineHeight = ed.currentFont.getHeight();
        */
    }
    
    pub fn next(&mut self) -> bool {
        
        todo!();
        /*
            if (atom == &longAtom && chunkLongAtom (true))
                return true;

            if (sectionIndex >= sections.size())
            {
                moveToEndOfLastAtom();
                return false;
            }

            bool forceNewLine = false;

            if (atomIndex >= currentSection->atoms.size() - 1)
            {
                if (atomIndex >= currentSection->atoms.size())
                {
                    if (++sectionIndex >= sections.size())
                    {
                        moveToEndOfLastAtom();
                        return false;
                    }

                    atomIndex = 0;
                    currentSection = sections.getUnchecked (sectionIndex);
                }
                else
                {
                    auto& lastAtom = currentSection->atoms.getReference (atomIndex);

                    if (! lastAtom.isWhitespace())
                    {
                        // handle the case where the last atom in a section is actually part of the same
                        // word as the first atom of the next section...
                        float right = atomRight + lastAtom.width;
                        float lineHeight2 = lineHeight;
                        float maxDescent2 = maxDescent;

                        for (int section = sectionIndex + 1; section < sections.size(); ++section)
                        {
                            auto* s = sections.getUnchecked (section);

                            if (s->atoms.size() == 0)
                                break;

                            auto& nextAtom = s->atoms.getReference (0);

                            if (nextAtom.isWhitespace())
                                break;

                            right += nextAtom.width;

                            lineHeight2 = jmax (lineHeight2, s->font.getHeight());
                            maxDescent2 = jmax (maxDescent2, s->font.getDescent());

                            if (shouldWrap (right))
                            {
                                lineHeight = lineHeight2;
                                maxDescent = maxDescent2;

                                forceNewLine = true;
                                break;
                            }

                            if (s->atoms.size() > 1)
                                break;
                        }
                    }
                }
            }

            bool isInPreviousAtom = false;

            if (atom != nullptr)
            {
                atomX = atomRight;
                indexInText += atom->numChars;

                if (atom->isNewLine())
                    beginNewLine();
                else
                    isInPreviousAtom = true;
            }

            atom = &(currentSection->atoms.getReference (atomIndex));
            atomRight = atomX + atom->width;
            ++atomIndex;

            if (shouldWrap (atomRight) || forceNewLine)
            {
                if (atom->isWhitespace())
                {
                    // leave whitespace at the end of a line, but truncate it to avoid scrolling
                    atomRight = jmin (atomRight, wordWrapWidth);
                }
                else if (shouldWrap (atom->width))  // atom too big to fit on a line, so break it up..
                {
                    longAtom = *atom;
                    longAtom.numChars = 0;
                    atom = &longAtom;
                    chunkLongAtom (isInPreviousAtom);
                }
                else
                {
                    beginNewLine();
                    atomRight = atomX + atom->width;
                }
            }

            return true;
        */
    }
    
    pub fn begin_new_line(&mut self)  {
        
        todo!();
        /*
            lineY += lineHeight * lineSpacing;
            float lineWidth = 0;

            auto tempSectionIndex = sectionIndex;
            auto tempAtomIndex = atomIndex;
            auto* section = sections.getUnchecked (tempSectionIndex);

            lineHeight = section->font.getHeight();
            maxDescent = section->font.getDescent();

            float nextLineWidth = (atom != nullptr) ? atom->width : 0.0f;

            while (! shouldWrap (nextLineWidth))
            {
                lineWidth = nextLineWidth;

                if (tempSectionIndex >= sections.size())
                    break;

                bool checkSize = false;

                if (tempAtomIndex >= section->atoms.size())
                {
                    if (++tempSectionIndex >= sections.size())
                        break;

                    tempAtomIndex = 0;
                    section = sections.getUnchecked (tempSectionIndex);
                    checkSize = true;
                }

                if (! isPositiveAndBelow (tempAtomIndex, section->atoms.size()))
                    break;

                auto& nextAtom = section->atoms.getReference (tempAtomIndex);
                nextLineWidth += nextAtom.width;

                if (shouldWrap (nextLineWidth) || nextAtom.isNewLine())
                    break;

                if (checkSize)
                {
                    lineHeight = jmax (lineHeight, section->font.getHeight());
                    maxDescent = jmax (maxDescent, section->font.getDescent());
                }

                ++tempAtomIndex;
            }

            atomX = getJustificationOffsetX (lineWidth);
        */
    }
    
    pub fn get_justification_offsetx(&self, line_width: f32) -> f32 {
        
        todo!();
        /*
            if (justification.testFlags (Justification::horizontallyCentred))    return jmax (0.0f, (bottomRight.x - lineWidth) * 0.5f);
            if (justification.testFlags (Justification::right))                  return jmax (0.0f, bottomRight.x - lineWidth);

            return 0;
        */
    }
    
    pub fn draw(&self, 
        g:            &mut Graphics,
        last_section: &mut *const TextEditorUniformTextSection,
        transform:    AffineTransform)  {
        
        todo!();
        /*
            if (atom == nullptr)
                return;

            if (passwordCharacter != 0 || (underlineWhitespace || ! atom->isWhitespace()))
            {
                if (lastSection != currentSection)
                {
                    lastSection = currentSection;
                    g.setColour (currentSection->colour);
                    g.setFont (currentSection->font);
                }

                jassert (atom->getTrimmedText (passwordCharacter).isNotEmpty());

                GlyphArrangement ga;
                ga.addLineOfText (currentSection->font,
                                  atom->getTrimmedText (passwordCharacter),
                                  atomX, (float) roundToInt (lineY + lineHeight - maxDescent));
                ga.draw (g, transform);
            }
        */
    }
    
    pub fn draw_underline(&self, 
        g:         &mut Graphics,
        underline: Range<i32>,
        colour:    Colour,
        transform: AffineTransform)  {
        
        todo!();
        /*
            auto startX    = roundToInt (indexToX (underline.getStart()));
            auto endX      = roundToInt (indexToX (underline.getEnd()));
            auto baselineY = roundToInt (lineY + currentSection->font.getAscent() + 0.5f);

            Graphics::ScopedSaveState state (g);
            g.addTransform (transform);
            g.reduceClipRegion ({ startX, baselineY, endX - startX, 1 });
            g.fillCheckerBoard ({ (float) endX, (float) baselineY + 1.0f }, 3.0f, 1.0f, colour, Colours::transparentBlack);
        */
    }
    
    pub fn draw_selected_text(&self, 
        g:                    &mut Graphics,
        selected:             Range<i32>,
        selected_text_colour: Colour,
        transform:            AffineTransform)  {
        
        todo!();
        /*
            if (atom == nullptr)
                return;

            if (passwordCharacter != 0 || ! atom->isWhitespace())
            {
                GlyphArrangement ga;
                ga.addLineOfText (currentSection->font,
                                  atom->getTrimmedText (passwordCharacter),
                                  atomX, (float) roundToInt (lineY + lineHeight - maxDescent));

                if (selected.getEnd() < indexInText + atom->numChars)
                {
                    GlyphArrangement ga2 (ga);
                    ga2.removeRangeOfGlyphs (0, selected.getEnd() - indexInText);
                    ga.removeRangeOfGlyphs (selected.getEnd() - indexInText, -1);

                    g.setColour (currentSection->colour);
                    ga2.draw (g, transform);
                }

                if (selected.getStart() > indexInText)
                {
                    GlyphArrangement ga2 (ga);
                    ga2.removeRangeOfGlyphs (selected.getStart() - indexInText, -1);
                    ga.removeRangeOfGlyphs (0, selected.getStart() - indexInText);

                    g.setColour (currentSection->colour);
                    ga2.draw (g, transform);
                }

                g.setColour (selectedTextColour);
                ga.draw (g, transform);
            }
        */
    }
    
    pub fn index_tox(&self, index_to_find: i32) -> f32 {
        
        todo!();
        /*
            if (indexToFind <= indexInText || atom == nullptr)
                return atomX;

            if (indexToFind >= indexInText + atom->numChars)
                return atomRight;

            GlyphArrangement g;
            g.addLineOfText (currentSection->font,
                             atom->getText (passwordCharacter),
                             atomX, 0.0f);

            if (indexToFind - indexInText >= g.getNumGlyphs())
                return atomRight;

            return jmin (atomRight, g.getGlyph (indexToFind - indexInText).getLeft());
        */
    }
    
    pub fn x_to_index(&self, x_to_find: f32) -> i32 {
        
        todo!();
        /*
            if (xToFind <= atomX || atom == nullptr || atom->isNewLine())
                return indexInText;

            if (xToFind >= atomRight)
                return indexInText + atom->numChars;

            GlyphArrangement g;
            g.addLineOfText (currentSection->font,
                             atom->getText (passwordCharacter),
                             atomX, 0.0f);

            auto numGlyphs = g.getNumGlyphs();

            int j;
            for (j = 0; j < numGlyphs; ++j)
            {
                auto& pg = g.getGlyph(j);

                if ((pg.getLeft() + pg.getRight()) / 2 > xToFind)
                    break;
            }

            return indexInText + j;
        */
    }
    
    pub fn get_char_position(&mut self, 
        index:             i32,
        anchor:            &mut Point<f32>,
        line_height_found: &mut f32) -> bool {
        
        todo!();
        /*
            while (next())
            {
                if (indexInText + atom->numChars > index)
                {
                    anchor = { indexToX (index), lineY };
                    lineHeightFound = lineHeight;
                    return true;
                }
            }

            anchor = { atomX, lineY };
            lineHeightFound = lineHeight;
            return false;
        */
    }
    
    pub fn get_yoffset(&mut self) -> f32 {
        
        todo!();
        /*
            if (justification.testFlags (Justification::top) || lineY >= bottomRight.y)
                return 0;

            while (next())
            {
                if (lineY >= bottomRight.y)
                    return 0;
            }

            auto bottom = jmax (0.0f, bottomRight.y - lineY - lineHeight);

            if (justification.testFlags (Justification::bottom))
                return bottom;

            return bottom * 0.5f;
        */
    }
    
    pub fn get_total_text_height(&mut self) -> i32 {
        
        todo!();
        /*
            while (next()) {}

            auto height = lineY + lineHeight + getYOffset();

            if (atom != nullptr && atom->isNewLine())
                height += lineHeight;

            return roundToInt (height);
        */
    }
    
    pub fn get_text_right(&mut self) -> i32 {
        
        todo!();
        /*
            float maxWidth = 0.0f;

            while (next())
                maxWidth = jmax (maxWidth, atomRight);

            return roundToInt (maxWidth);
        */
    }
    
    pub fn get_text_bounds(&self, range: Range<i32>) -> Rectangle<i32> {
        
        todo!();
        /*
            auto startX = indexToX (range.getStart());
            auto endX   = indexToX (range.getEnd());

            return Rectangle<float> (startX, lineY, endX - startX, lineHeight * lineSpacing).toNearestInt();
        */
    }
    
    pub fn chunk_long_atom(&mut self, should_start_new_line: bool) -> bool {
        
        todo!();
        /*
            const auto numRemaining = longAtom.atomText.length() - longAtom.numChars;

            if (numRemaining <= 0)
                return false;

            longAtom.atomText = longAtom.atomText.substring (longAtom.numChars);
            indexInText += longAtom.numChars;

            GlyphArrangement g;
            g.addLineOfText (currentSection->font, atom->getText (passwordCharacter), 0.0f, 0.0f);

            int split;
            for (split = 0; split < g.getNumGlyphs(); ++split)
                if (shouldWrap (g.getGlyph (split).getRight()))
                    break;

            const auto numChars = jmax (1, split);
            longAtom.numChars = (uint16) numChars;
            longAtom.width = g.getGlyph (numChars - 1).getRight();

            atomX = getJustificationOffsetX (longAtom.width);

            if (shouldStartNewLine)
            {
                if (split == numRemaining)
                    beginNewLine();
                else
                    lineY += lineHeight * lineSpacing;
            }

            atomRight = atomX + longAtom.width;
            return true;
        */
    }
    
    pub fn move_to_end_of_last_atom(&mut self)  {
        
        todo!();
        /*
            if (atom != nullptr)
            {
                atomX = atomRight;

                if (atom->isNewLine())
                {
                    atomX = getJustificationOffsetX (0);
                    lineY += lineHeight * lineSpacing;
                }
            }
        */
    }
    
    pub fn should_wrap(&self, x: f32) -> bool {
        
        todo!();
        /*
            return (x - 0.0001f) >= wordWrapWidth;
        */
    }
}
