crate::ix!();

pub struct CodeDocumentLine {
    line:                          String,
    line_start_in_file:            i32,
    line_length:                   i32,
    line_length_without_new_lines: i32,
}

impl CodeDocumentLine {

    pub fn new(
        start_of_line:      CharPointerType,
        end_of_line:        CharPointerType,
        line_len:           i32,
        num_new_line_chars: i32,
        start_in_file:      i32) -> Self {
    
        todo!();
        /*

            : line (startOfLine, endOfLine),
              lineStartInFile (startInFile),
              lineLength (lineLen),
              lineLengthWithoutNewLines (lineLen - numNewLineChars)
        */
    }
    
    pub fn create_lines(
        new_lines: &mut Vec<*mut CodeDocumentLine>,
        text:      &str)  {
        
        todo!();
        /*
            auto t = text.text;
            int charNumInFile = 0;
            bool finished = false;

            while (! (finished || t.isEmpty()))
            {
                auto startOfLine = t;
                auto startOfLineInFile = charNumInFile;
                int lineLength = 0;
                int numNewLineChars = 0;

                for (;;)
                {
                    auto c = t.getAndAdvance();

                    if (c == 0)
                    {
                        finished = true;
                        break;
                    }

                    ++charNumInFile;
                    ++lineLength;

                    if (c == '\r')
                    {
                        ++numNewLineChars;

                        if (*t == '\n')
                        {
                            ++t;
                            ++charNumInFile;
                            ++lineLength;
                            ++numNewLineChars;
                        }

                        break;
                    }

                    if (c == '\n')
                    {
                        ++numNewLineChars;
                        break;
                    }
                }

                newLines.add (new CodeDocumentLine (startOfLine, t, lineLength,
                                                    numNewLineChars, startOfLineInFile));
            }

            jassert (charNumInFile == text.length());
        */
    }
    
    pub fn ends_with_line_break(&self) -> bool {
        
        todo!();
        /*
            return lineLengthWithoutNewLines != lineLength;
        */
    }
    
    pub fn update_length(&mut self)  {
        
        todo!();
        /*
            lineLength = 0;
            lineLengthWithoutNewLines = 0;

            for (auto t = line.getCharPointer();;)
            {
                auto c = t.getAndAdvance();

                if (c == 0)
                    break;

                ++lineLength;

                if (c != '\n' && c != '\r')
                    lineLengthWithoutNewLines = lineLength;
            }
        */
    }
}
