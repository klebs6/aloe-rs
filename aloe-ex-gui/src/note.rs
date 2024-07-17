crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/MDIDemo.h]

/**
  | The Note class contains text editor
  | used to display and edit the note's contents
  | and will also listen to changes in the
  | text and mark the FileBasedDocument
  | as 'dirty'. This 'dirty' flag is used
  | to prompt the user to save the note when
  | it is closed.
  |
  */
#[no_copy]
#[leak_detector]
pub struct Note<'a> {
    base:              Component<'a>,
    base2:             FileBasedDocument<'a>,
    text_value_object: Value<'a>,
    editor:            TextEditor<'a>,
}

impl<'a> Note<'a> {

    pub fn new(
        name:     &String,
        contents: &String) -> Self {
    
        todo!();
        /*


            : FileBasedDocument (".jnote", "*.jnote",
                                 "Browse for note to load",
                                 "Choose file to save note to"),
              textValueObject (contents)

            // we need to use an separate Value object as our text source so it doesn't get marked
            // as changed immediately
            setName (name);

            editor.setMultiLine (true);
            editor.setReturnKeyStartsNewLine (true);
            editor.getTextValue().referTo (textValueObject);
            addAndMakeVisible (editor);
            editor.onTextChange = [this] { changed(); };
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            editor.setBounds (getLocalBounds());
        */
    }
    
    pub fn get_document_title(&mut self) -> String {
        
        todo!();
        /*
            return getName();
        */
    }
    
    pub fn load_document(&mut self, file: &File) -> Result<(),()> {
        
        todo!();
        /*
            editor.setText (file.loadFileAsString());
            return Result::ok();
        */
    }
    
    pub fn save_document(&mut self, file: &File) -> Result<(),()> {
        
        todo!();
        /*
            // attempt to save the contents into the given file
            if (file.replaceWithText (editor.getText()))
                return Result::ok();

            return Result::fail ("Can't write to file");
        */
    }
    
    pub fn get_last_document_opened(&mut self) -> File {
        
        todo!();
        /*
            // not interested in this for now
            return {};
        */
    }
    
    pub fn set_last_document_opened(&mut self, file: &File)  {
        
        todo!();
        /*
            // not interested in this for now
        */
    }
    
    pub fn get_suggested_save_as_file(&mut self, _0: &File) -> File {
        
        todo!();
        /*
            return File::getSpecialLocation (File::userDesktopDirectory)
                        .getChildFile (getName())
                        .withFileExtension ("jnote");
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            editor.applyFontToAllText (editor.getFont());
        */
    }
}
