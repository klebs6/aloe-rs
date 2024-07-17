crate::ix!();

/**
  | Editing modes.
  | 
  | These are used by setEditingMode(),
  | but will be rarely needed in user code.
  |
  */
pub enum ToolbarEditingMode
{
    /**
      | Means that the component is active,
      | inside a toolbar.
      |
      */
    normalMode = 0,     

    /**
      | Means that the component is on a toolbar,
      | but the toolbar is in customisation
      | mode, and the items can be dragged around.
      |
      */
    editableOnToolbar,  

    /**
      | Means that the component is on an new-item
      | palette, so it can be dragged onto a toolbar
      | to add it to that bar.
      |
      */
    editableOnPalette,   
}
