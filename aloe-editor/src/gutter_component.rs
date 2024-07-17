crate::ix!();

#[derive(Default)]
pub struct CodeEditorComponentGutterComponent<'a> {
    base:           Component<'a>,
    first_line:     i32, // default = 0
    last_num_lines: i32, // default = 0
}

impl<'a> CodeEditorComponentGutterComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            jassert (dynamic_cast<CodeEditorComponent*> (getParentComponent()) != nullptr);
            auto& editor = *static_cast<CodeEditorComponent*> (getParentComponent());

            g.fillAll (editor.findColour (CodeEditorComponent::backgroundColourId)
                        .overlaidWith (editor.findColour (lineNumberBackgroundId)));

            auto clip = g.getClipBounds();
            const int lineH = editor.lineHeight;
            const float lineHeightFloat = (float) lineH;
            const int firstLineToDraw = jmax (0, clip.getY() / lineH);
            const int lastLineToDraw = jmin (editor.lines.size(), clip.getBottom() / lineH + 1,
                                             lastNumLines - editor.firstLineOnScreen);

            auto lineNumberFont = editor.getFont().withHeight (jmin (13.0f, lineHeightFloat * 0.8f));
            auto w = (float) getWidth() - 2.0f;
            GlyphArrangement ga;

            for (int i = firstLineToDraw; i < lastLineToDraw; ++i)
                ga.addFittedText (lineNumberFont, String (editor.firstLineOnScreen + i + 1),
                                  0, (float) (lineH * i), w, lineHeightFloat,
                                  Justification::centredRight, 1, 0.2f);

            g.setColour (editor.findColour (lineNumberTextId));
            ga.draw (g);
        */
    }
    
    pub fn document_changed(&mut self, 
        doc:            &mut CodeDocument,
        new_first_line: i32)  {
        
        todo!();
        /*
            auto newNumLines = doc.getNumLines();

            if (newNumLines != lastNumLines || firstLine != newFirstLine)
            {
                firstLine = newFirstLine;
                lastNumLines = newNumLines;
                repaint();
            }
        */
    }
}
