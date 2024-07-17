crate::ix!();

pub trait LabelInterface
: CreateEditorComponent 
+ TextWasEdited 
+ TextWasChanged 
+ EditorShown 
+ EditorAboutToBeHidden 
{ }

/**
  | A class for receiving events from a Label.
  | 
  | You can register a Label::Listener
  | with a Label using the Label::addListener()
  | method, and it will be called when the
  | text of the label changes, either because
  | of a call to Label::setText() or by the
  | user editing the text (if the label is
  | editable).
  | 
  | @see Label::addListener, Label::removeListener
  |
  */
pub trait LabelListener
{
      /**
        | Called when a Label's text has changed.
        |
        */
      fn label_text_changed(&mut self, label_that_has_changed: *mut Label) {}

      /**
        | Called when a Label goes into editing
        | mode and displays a TextEditor.
        |
        */
      fn editor_shown(
          &mut self, 
          _0: *mut Label,
          _1: &mut TextEditor
      ) {}

      /**
        | Called when a Label is about to delete
        | its TextEditor and exit editing mode.
        |
        */
      fn editor_hidden(
          &mut self, 
          _0: *mut Label,
          _1: &mut TextEditor
      ) {}
}

pub trait CreateEditorComponent {

    /**
      | Creates the TextEditor component that
      | will be used when the user has clicked
      | on the label.
      | 
      | Subclasses can override this if they
      | need to customise this component in
      | some way.
      |
      */
    fn create_editor_component(&mut self) -> *mut TextEditor;
}

pub trait TextWasEdited {

    /**
      | Called after the user changes the text.
      |
      */
    fn text_was_edited(&mut self);
}

pub trait TextWasChanged {

    /**
      | Called when the text has been altered.
      |
      */
    fn text_was_changed(&mut self);
}
    
pub trait EditorShown {

    /**
      | Called when the text editor has just
      | appeared, due to a user click or other
      | focus change.
      |
      */
    fn editor_shown(&mut self, _0: *mut TextEditor);
}

pub trait EditorAboutToBeHidden {

    /**
      | Called when the text editor is going
      | to be deleted, after editing has finished.
      |
      */
    fn editor_about_to_be_hidden(&mut self, _0: *mut TextEditor);
}
