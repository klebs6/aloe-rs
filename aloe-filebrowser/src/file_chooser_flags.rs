crate::ix!();

/**
  | Various options for the browser.
  | 
  | A combination of these is passed into
  | the FileBrowserComponent constructor.
  |
  */
pub enum FileChooserFlags
{
    /**
      | specifies that the component should
      | allow the user to choose an existing
      | file with the intention of opening it.
      |
      */
    openMode                        = 1,    

    /**
      | specifies that the component should
      | allow the user to specify the name of
      | a file that will be used to save something.
      |
      */
    saveMode                        = 2,    

    /**
      | specifies that the user can select files
      | (can be used in conjunction with canSelectDirectories).
      |
      */
    canSelectFiles                  = 4,    

    /**
      | specifies that the user can select directories
      | (can be used in conjunction with canSelectFiles).
      |
      */
    canSelectDirectories            = 8,    

    /**
      | specifies that the user can select multiple
      | items.
      |
      */
    canSelectMultipleItems          = 16,   

    /**
      | specifies that a tree-view should be
      | shown instead of a file list.
      |
      */
    useTreeView                     = 32,   

    /**
      | specifies that the user can't type directly
      | into the filename box.
      |
      */
    filenameBoxIsReadOnly           = 64,   

    /**
      | specifies that the dialog should warn
      | about overwriting existing files (if
      | possible).
      |
      */
    warnAboutOverwriting            = 128,  

    /**
      | specifies that the file name should
      | not be cleared upon root change.
      |
      */
    doNotClearFileNameOnRootChange  = 256,   
}
