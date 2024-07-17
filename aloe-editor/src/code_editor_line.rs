crate::ix!();

#[derive(Default)]
pub struct CodeEditorLine {
    tokens:                 Vec<CodeEditorLineSyntaxToken>,
    highlight_column_start: i32, // default = 0
    highlight_column_end:   i32, // default = 0
}

impl CodeEditorLine {

    pub fn update(
        &mut self, 
        code_doc:   &mut CodeDocument,
        line_num:   i32,
        source:     &mut CodeDocumentIterator,
        tokeniser:  *mut dyn CodeTokeniser,
        tab_spaces: i32,
        sel_start:  &CodeDocumentPosition,
        sel_end:    &CodeDocumentPosition

    ) -> bool {
        
        todo!();
        /*
            Vec<CodeEditorLineSyntaxToken> newTokens;
            newTokens.ensureStorageAllocated (8);

            if (tokeniser == nullptr)
            {
                auto line = codeDoc.getLine (lineNum);
                addToken (newTokens, line, line.length(), -1);
            }
            else if (lineNum < codeDoc.getNumLines())
            {
                const CodeDocument::Position pos (codeDoc, lineNum, 0);
                createTokens (pos.getPosition(), pos.getLineText(),
                              source, *tokeniser, newTokens);
            }

            replaceTabsWithSpaces (newTokens, tabSpaces);

            int newHighlightStart = 0;
            int newHighlightEnd = 0;

            if (selStart.getLineNumber() <= lineNum && selEnd.getLineNumber() >= lineNum)
            {
                auto line = codeDoc.getLine (lineNum);

                CodeDocument::Position lineStart (codeDoc, lineNum, 0), lineEnd (codeDoc, lineNum + 1, 0);
                newHighlightStart = indexToColumn (jmax (0, selStart.getPosition() - lineStart.getPosition()),
                                                   line, tabSpaces);
                newHighlightEnd = indexToColumn (jmin (lineEnd.getPosition() - lineStart.getPosition(), selEnd.getPosition() - lineStart.getPosition()),
                                                 line, tabSpaces);
            }

            if (newHighlightStart != highlightColumnStart || newHighlightEnd != highlightColumnEnd)
            {
                highlightColumnStart = newHighlightStart;
                highlightColumnEnd = newHighlightEnd;
            }
            else if (tokens == newTokens)
            {
                return false;
            }

            tokens.swapWith (newTokens);
            return true;
        */
    }
    
    pub fn get_highlight_area(
        &self, 
        area:            &mut RectangleList<f32>,
        x:               f32,
        y:               i32,
        lineh:           i32,
        character_width: f32

    ) {
        
        todo!();
        /*
            if (highlightColumnStart < highlightColumnEnd)
                area.add (Rectangle<float> (x + (float) highlightColumnStart * characterWidth - 1.0f, (float) y - 0.5f,
                                            (float) (highlightColumnEnd - highlightColumnStart) * characterWidth + 1.5f, (float) lineH + 1.0f));
        */
    }
    
    pub fn draw(
        &self, 
        owner:           &mut CodeEditorComponent,
        g:               &mut Graphics,
        font_to_use:     &Font,
        right_clip:      f32,
        x:               f32,
        y:               i32,
        lineh:           i32,
        character_width: f32

    ) {
        
        todo!();
        /*
            AttributedString as;
            as.setJustification (Justification::centredLeft);

            int column = 0;

            for (auto& token : tokens)
            {
                const float tokenX = x + (float) column * characterWidth;
                if (tokenX > rightClip)
                    break;

                as.append (token.text.initialSectionNotContaining ("\r\n"), fontToUse, owner.getColourForTokenType (token.tokenType));
                column += token.length;
            }

            as.draw (g, { x, (float) y, (float) column * characterWidth + 10.0f, (float) lineH });
        */
    }
    
    pub fn create_tokens(
        start_position: i32,
        line_text:      &String,
        source:         &mut CodeDocumentIterator,
        tokeniser:      &mut dyn CodeTokeniser,
        new_tokens:     &mut Vec<CodeEditorLineSyntaxToken>

    ) {
        
        todo!();
        /*
            CodeDocument::Iterator lastIterator (source);
            const int lineLength = lineText.length();

            for (;;)
            {
                int tokenType = tokeniser.readNextToken (source);
                int tokenStart = lastIterator.getPosition();
                int tokenEnd = source.getPosition();

                if (tokenEnd <= tokenStart)
                    break;

                tokenEnd -= startPosition;

                if (tokenEnd > 0)
                {
                    tokenStart -= startPosition;
                    const int start = jmax (0, tokenStart);
                    addToken (newTokens, lineText.substring (start, tokenEnd),
                              tokenEnd - start, tokenType);

                    if (tokenEnd >= lineLength)
                        break;
                }

                lastIterator = source;
            }

            source = lastIterator;
        */
    }
    
    pub fn replace_tabs_with_spaces(
        tokens:         &mut Vec<CodeEditorLineSyntaxToken>,
        spaces_per_tab: i32

    ) {
        
        todo!();
        /*
            int x = 0;

            for (auto& t : tokens)
            {
                for (;;)
                {
                    const int tabPos = t.text.indexOfChar ('\t');
                    if (tabPos < 0)
                        break;

                    const int spacesNeeded = spacesPerTab - ((tabPos + x) % spacesPerTab);
                    t.text = t.text.replaceSection (tabPos, 1, String::repeatedString (" ", spacesNeeded));
                    t.length = t.text.length();
                }

                x += t.length;
            }
        */
    }
    
    pub fn index_to_column(
        &self, 
        index:      i32,
        line:       &String,
        tab_spaces: i32

    ) -> i32 {
        
        todo!();
        /*
            jassert (index <= line.length());

            auto t = line.getCharPointer();
            int col = 0;

            for (int i = 0; i < index; ++i)
            {
                if (t.getAndAdvance() != '\t')
                    ++col;
                else
                    col += tabSpaces - (col % tabSpaces);
            }

            return col;
        */
    }
    
    pub fn add_token(
        dest:   &mut Vec<CodeEditorLineSyntaxToken>,
        text:   &String,
        length: i32,
        ty:     i32

    ) {
        
        todo!();
        /*
            if (length > 1000)
            {
                // subdivide very long tokens to avoid unwieldy glyph sequences
                addToken (dest, text.substring (0, length / 2), length / 2, type);
                addToken (dest, text.substring (length / 2), length - length / 2, type);
            }
            else
            {
                dest.add (CodeEditorLineSyntaxToken (text, length, type));
            }
        */
    }
}
