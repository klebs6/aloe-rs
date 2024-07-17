crate::ix!();

pub struct CodeEditorComponentTextInterface<'a> {
    code_editor_component: &'a mut CodeEditorComponent<'a>,
}

impl<'a> AccessibilityTextInterface for CodeEditorComponentTextInterface<'a> {

    fn is_displaying_protected_text(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    fn is_read_only(&self) -> bool {
        
        todo!();
        /*
            return codeEditorComponent.isReadOnly();
        */
    }
    
    fn get_total_num_characters(&self) -> i32 {
        
        todo!();
        /*
            return codeEditorComponent.document.getAllContent().length();
        */
    }
    
    fn get_selection(&self) -> Range<i32> {
        
        todo!();
        /*
            return { codeEditorComponent.selectionStart.getPosition(),
                         codeEditorComponent.selectionEnd.getPosition() };
        */
    }
    
    fn set_selection(&mut self, r: Range<i32>)  {
        
        todo!();
        /*
            if (r.isEmpty())
                {
                    codeEditorComponent.caretPos.setPosition (r.getStart());
                    return;
                }

                auto& doc = codeEditorComponent.document;

                codeEditorComponent.selectRegion (CodeDocument::Position (doc, r.getStart()),
                                                  CodeDocument::Position (doc, r.getEnd()));
        */
    }
    
    fn get_text(&self, r: Range<i32>) -> String {
        
        todo!();
        /*
            auto& doc = codeEditorComponent.document;

                return doc.getTextBetween (CodeDocument::Position (doc, r.getStart()),
                                           CodeDocument::Position (doc, r.getEnd()));
        */
    }
    
    fn set_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            codeEditorComponent.document.replaceAllContent (newText);
        */
    }
    
    fn get_text_insertion_offset(&self) -> i32 {
        
        todo!();
        /*
            return codeEditorComponent.caretPos.getPosition();
        */
    }
    
    fn get_text_bounds(&self, text_range: Range<i32>) -> RectangleList<i32> {
        
        todo!();
        /*
            auto& doc = codeEditorComponent.document;

                RectangleList<int> localRects;

                CodeDocument::Position startPosition (doc, textRange.getStart());
                CodeDocument::Position endPosition   (doc, textRange.getEnd());

                for (int line = startPosition.getLineNumber(); line <= endPosition.getLineNumber(); ++line)
                {
                    CodeDocument::Position lineStart (doc, line, 0);
                    CodeDocument::Position lineEnd   (doc, line, doc.getLine (line).length());

                    if (line == startPosition.getLineNumber())
                        lineStart = lineStart.movedBy (startPosition.getIndexInLine());

                    if (line == endPosition.getLineNumber())
                        lineEnd = { doc, line, endPosition.getIndexInLine() };

                    auto startPos = codeEditorComponent.getCharacterBounds (lineStart).getTopLeft();
                    auto endPos = codeEditorComponent.getCharacterBounds (lineEnd).getTopLeft();

                    localRects.add (startPos.x,
                                    startPos.y,
                                    endPos.x - startPos.x,
                                    codeEditorComponent.getLineHeight());
                }

                RectangleList<int> globalRects;

                for (auto r : localRects)
                    globalRects.add (codeEditorComponent.localAreaToGlobal (r));

                return globalRects;
        */
    }
    
    fn get_offset_at_point(&self, point: Point<i32>) -> i32 {
        
        todo!();
        /*
            return codeEditorComponent.getPositionAt (point.x, point.y).getPosition();
        */
    }
}

impl<'a> CodeEditorComponentTextInterface<'a> {

    pub fn new(code_editor_component_to_wrap: &mut CodeEditorComponent) -> Self {
    
        todo!();
        /*
        : code_editor_component(codeEditorComponentToWrap),

        
        */
    }
}
