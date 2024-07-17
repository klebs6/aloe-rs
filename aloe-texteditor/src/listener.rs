crate::ix!();

/**
  | Receives callbacks from a TextEditor
  | component when it changes.
  | 
  | @see TextEditor::addListener
  |
  */
pub trait TextEditorListener {

    /**
      | Called when the user changes the text
      | in some way.
      |
      */
    fn text_editor_text_changed(&mut self, _0: &mut TextEditor);

    /**
      | Called when the user presses the return
      | key.
      |
      */
    fn text_editor_return_key_pressed(&mut self, _0: &mut TextEditor);

    /**
      | Called when the user presses the escape
      | key.
      |
      */
    fn text_editor_escape_key_pressed(&mut self, _0: &mut TextEditor);

    /**
      | Called when the text editor loses focus.
      |
      */
    fn text_editor_focus_lost(&mut self, _0: &mut TextEditor);
}
