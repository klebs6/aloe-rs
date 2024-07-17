crate::ix!();
    
/**
  | Context Menu Item Target interface:
  | Vst::IContextMenuTarget \ingroup
  | vstIHost vstIPlug vst350
  | 
  | - [host imp]
  | 
  | - [plug imp]
  | 
  | - [released: 3.5.0]
  | 
  | - [optional]
  | 
  | A receiver of a menu item should implement
  | this interface, which will be called
  | after the user has selected this menu
  | item.
  | 
  | \see IComponentHandler3 for more information.
  |
  */
pub trait IContextMenuTarget: FUnknown {

    /**
      | Called when an menu item was executed.
      |
      */
    #[PLUGIN_API]
    fn execute_menu_item(&mut self, tag: i32) -> tresult;
}

lazy_static!{
    /*
    static const FUID icontext_menu_target_iid;
    */
}

declare_class_iid!{
    IContextMenuTarget, 
    0x3CDF2E75, 
    0x85D34144, 
    0xBF86D36B, 
    0xD7C4894D
}
