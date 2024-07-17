crate::ix!();

/**
  | Can be used to save and restore the editor's
  | caret position, selection state, etc.
  |
  */
pub struct CodeEditorComponentState {
    last_top_line:      i32,
    last_caret_pos:     i32,
    last_selection_end: i32,
}

impl CodeEditorComponentState {

    /**
      | Creates an object containing the state
      | of the given editor.
      |
      */
    pub fn new_from_editor(editor: &CodeEditorComponent) -> Self {
    
        todo!();
        /*


            : lastTopLine (editor.getFirstLineOnScreen()),
          lastCaretPos (editor.getCaretPos().getPosition()),
          lastSelectionEnd (lastCaretPos)

        auto selection = editor.getHighlightedRegion();

        if (lastCaretPos == selection.getStart())
            lastSelectionEnd = selection.getEnd();
        else
            lastSelectionEnd = selection.getStart();
        */
    }
    
    pub fn new_from_editor_component_state(other: &CodeEditorComponentState) -> Self {
    
        todo!();
        /*


            : lastTopLine (other.lastTopLine),
          lastCaretPos (other.lastCaretPos),
          lastSelectionEnd (other.lastSelectionEnd)
        */
    }
    
    /**
      | Updates the given editor with this saved
      | state.
      |
      */
    pub fn restore_state(&self, editor: &mut CodeEditorComponent)  {
        
        todo!();
        /*
            editor.selectRegion (CodeDocument::Position (editor.getDocument(), lastSelectionEnd),
                             CodeDocument::Position (editor.getDocument(), lastCaretPos));

        if (lastTopLine > 0 && lastTopLine < editor.getDocument().getNumLines())
            editor.scrollToLine (lastTopLine);
        */
    }
    
    /**
      | Creates a state object from a string
      | that was previously created with toString().
      |
      */
    pub fn new_from_string(s: &String) -> Self {
    
        todo!();
        /*


            auto tokens = StringArray::fromTokens (s, ":", {});

        lastTopLine      = tokens[0].getIntValue();
        lastCaretPos     = tokens[1].getIntValue();
        lastSelectionEnd = tokens[2].getIntValue();
        */
    }
    
    /**
      | Returns a stringified version of this
      | state that can be used to recreate it
      | later.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return String (lastTopLine) + ":" + String (lastCaretPos) + ":" + String (lastSelectionEnd);
        */
    }
}
