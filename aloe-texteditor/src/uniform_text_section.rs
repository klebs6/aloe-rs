crate::ix!();

/**
  | a run of text with a single font and colour
  |
  */
#[leak_detector]
pub struct TextEditorUniformTextSection {
    font:          Font,
    colour:        Colour,
    atoms:         Vec<TextAtom>,
    password_char: wchar_t,
}

impl TextEditorUniformTextSection {

    pub fn new(
        text:                 &String,
        f:                    &Font,
        col:                  Colour,
        password_char_to_use: wchar_t) -> Self {
    
        todo!();
        /*
        : font(f),
        : colour(col),
        : password_char(passwordCharToUse),

            initialiseAtoms (text);
        */
    }
    
    pub fn append(&mut self, other: &mut TextEditorUniformTextSection)  {
        
        todo!();
        /*
            if (! other.atoms.isEmpty())
            {
                int i = 0;

                if (! atoms.isEmpty())
                {
                    auto& lastAtom = atoms.getReference (atoms.size() - 1);

                    if (! CharacterFunctions::isWhitespace (lastAtom.atomText.getLastCharacter()))
                    {
                        auto& first = other.atoms.getReference(0);

                        if (! CharacterFunctions::isWhitespace (first.atomText[0]))
                        {
                            lastAtom.atomText += first.atomText;
                            lastAtom.numChars = (uint16) (lastAtom.numChars + first.numChars);
                            lastAtom.width = font.getStringWidthFloat (lastAtom.getText (passwordChar));
                            ++i;
                        }
                    }
                }

                atoms.ensureStorageAllocated (atoms.size() + other.atoms.size() - i);

                while (i < other.atoms.size())
                {
                    atoms.add (other.atoms.getReference(i));
                    ++i;
                }
            }
        */
    }
    
    pub fn split(&mut self, index_to_break_at: i32) -> *mut TextEditorUniformTextSection {
        
        todo!();
        /*
            auto* section2 = new TextEditorUniformTextSection ({}, font, colour, passwordChar);
            int index = 0;

            for (int i = 0; i < atoms.size(); ++i)
            {
                auto& atom = atoms.getReference(i);
                auto nextIndex = index + atom.numChars;

                if (index == indexToBreakAt)
                {
                    for (int j = i; j < atoms.size(); ++j)
                        section2->atoms.add (atoms.getUnchecked (j));

                    atoms.removeRange (i, atoms.size());
                    break;
                }

                if (indexToBreakAt >= index && indexToBreakAt < nextIndex)
                {
                    TextAtom secondAtom;
                    secondAtom.atomText = atom.atomText.substring (indexToBreakAt - index);
                    secondAtom.width = font.getStringWidthFloat (secondAtom.getText (passwordChar));
                    secondAtom.numChars = (uint16) secondAtom.atomText.length();

                    section2->atoms.add (secondAtom);

                    atom.atomText = atom.atomText.substring (0, indexToBreakAt - index);
                    atom.width = font.getStringWidthFloat (atom.getText (passwordChar));
                    atom.numChars = (uint16) (indexToBreakAt - index);

                    for (int j = i + 1; j < atoms.size(); ++j)
                        section2->atoms.add (atoms.getUnchecked (j));

                    atoms.removeRange (i + 1, atoms.size());
                    break;
                }

                index = nextIndex;
            }

            return section2;
        */
    }
    
    pub fn append_all_text(&self, mo: &mut MemoryOutputStream)  {
        
        todo!();
        /*
            for (auto& atom : atoms)
                mo << atom.atomText;
        */
    }
    
    pub fn append_substring(&self, 
        mo:    &mut MemoryOutputStream,
        range: Range<i32>)  {
        
        todo!();
        /*
            int index = 0;

            for (auto& atom : atoms)
            {
                auto nextIndex = index + atom.numChars;

                if (range.getStart() < nextIndex)
                {
                    if (range.getEnd() <= index)
                        break;

                    auto r = (range - index).getIntersectionWith ({ 0, (int) atom.numChars });

                    if (! r.isEmpty())
                        mo << atom.atomText.substring (r.getStart(), r.getEnd());
                }

                index = nextIndex;
            }
        */
    }
    
    pub fn get_total_length(&self) -> i32 {
        
        todo!();
        /*
            int total = 0;

            for (auto& atom : atoms)
                total += atom.numChars;

            return total;
        */
    }
    
    pub fn set_font(&mut self, 
        new_font:             &Font,
        password_char_to_use: wchar_t)  {
        
        todo!();
        /*
            if (font != newFont || passwordChar != passwordCharToUse)
            {
                font = newFont;
                passwordChar = passwordCharToUse;

                for (auto& atom : atoms)
                    atom.width = newFont.getStringWidthFloat (atom.getText (passwordChar));
            }
        */
    }
    
    pub fn initialise_atoms(&mut self, text_to_parse: &String)  {
        
        todo!();
        /*
            auto text = textToParse.getCharPointer();

            while (! text.isEmpty())
            {
                size_t numChars = 0;
                auto start = text;

                // create a whitespace atom unless it starts with non-ws
                if (text.isWhitespace() && *text != '\r' && *text != '\n')
                {
                    do
                    {
                        ++text;
                        ++numChars;
                    }
                    while (text.isWhitespace() && *text != '\r' && *text != '\n');
                }
                else
                {
                    if (*text == '\r')
                    {
                        ++text;
                        ++numChars;

                        if (*text == '\n')
                        {
                            ++start;
                            ++text;
                        }
                    }
                    else if (*text == '\n')
                    {
                        ++text;
                        ++numChars;
                    }
                    else
                    {
                        while (! (text.isEmpty() || text.isWhitespace()))
                        {
                            ++text;
                            ++numChars;
                        }
                    }
                }

                TextAtom atom;
                atom.atomText = String (start, numChars);
                atom.width = (atom.isNewLine() ? 0.0f : font.getStringWidthFloat (atom.getText (passwordChar)));
                atom.numChars = (uint16) numChars;
                atoms.add (atom);
            }
        */
    }
}
