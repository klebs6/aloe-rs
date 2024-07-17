crate::ix!();

/**
  | Extended host callback interface Vst::IComponentHandler3
  | for an edit controller. \ingroup vstIHost
  | vst350
  | 
  | - [host imp]
  | 
  | - [extends IComponentHandler]
  | 
  | - [released: 3.5.0]
  | 
  | - [optional]
  | 
  | A plug-in can ask the host to create a
  | context menu for a given exported parameter
  | ID or a generic context menu.\n
  | 
  | The host may pre-fill this context menu
  | with specific items regarding the parameter
  | ID like "Show automation for parameter",
  | "MIDI learn" etc...\n
  | 
  | The plug-in can use the context menu
  | in two ways :
  | 
  | - add its own items to the menu via the
  | IContextMenu interface and call IContextMenu::popup(..)
  | to create the pop-up. See the \ref IContextMenuExample.
  | 
  | - extract the host menu items and add
  | them to a context menu created by the
  | plug-in.
  | 
  | \b Note: You can and should use this even
  | if you do not add your own items to the
  | menu as this is considered to be a big
  | user value.
  | 
  | \sa IContextMenu \sa IContextMenuTarget
  | 
  | \section IContextMenuExample Examples
  | 
  | - For example, Cubase adds its owned
  | entries in the context menu opened with
  | right-click on an exported parameter
  | when the plug-in uses createContextMenu.
  | \image html "contextmenuexample.png"
  | \n
  | 
  | - Adding plug-in specific items to the
  | context menu:
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | class PluginContextMenuTarget : public IContextMenuTarget, public FObject
  | {
  | 
  |     PluginContextMenuTarget () {}
  | 
  |     virtual tresult PLUGIN_API executeMenuItem (i32 tag)
  |     {
  |         // this will be called if the user has executed one of the menu items of the plug-in.
  |         // It will not be called for items of the host.
  |         switch (tag)
  |         {
  |             case 1: break;
  |             case 2: break;
  |         }
  |         return kResultTrue;
  |     }
  | 
  |     OBJ_METHODS(PluginContextMenuTarget, FObject)
  |     DEFINE_INTERFACES
  |         DEF_INTERFACE (IContextMenuTarget)
  |     END_DEFINE_INTERFACES (FObject)
  |     REFCOUNT_METHODS(FObject)
  | };
  | 
  | // The following is the code to create the context menu
  | void popupContextMenu (IComponentHandler* componentHandler, IPlugView* view, const ParamID* paramID, UCoord x, UCoord y)
  | {
  |     if (componentHandler == 0 || view == 0)
  |         return;
  |     FUnknownPtr<IComponentHandler3> handler (componentHandler);
  |     if (handler == 0)
  |         return;
  |     IContextMenu* menu = handler->createContextMenu (view, paramID);
  |     if (menu)
  |     {
  |         // here you can add your entries (optional)
  |         PluginContextMenuTarget* target = new PluginContextMenuTarget ();
  | 
  |         IContextMenu::Item item = {0};
  |         UString128 ("My Item 1").copyTo (item.name, 128);
  |         item.tag = 1;
  |         menu->addItem (item, target);
  | 
  |         UString128 ("My Item 2").copyTo (item.name, 128);
  |         item.tag = 2;
  |         menu->addItem (item, target);
  |         target->release ();
  |         //--end of adding new entries
  | 
  |         // here the the context menu will be pop-up (and it waits a user interaction)
  |         menu->popup (x, y);
  |         menu->release ();
  |     }
  | }
  |
  */
pub trait IComponentHandler3: FUnknown {

    /**
      | Creates a host context menu for a plug-in:
      | 
      | - If paramID is zero, the host may create
      | a generic context menu.
      | 
      | - The IPlugView object must be valid.
      | 
      | - The return IContextMenu object needs
      | to be released afterwards by the plug-in.
      |
      */
    #[PLUGIN_API]
    fn create_context_menu(
        &mut self, 
        plug_view: *mut dyn IPlugView,
        paramid:   *const ParamID
    ) -> *mut dyn IContextMenu<Item = IContextMenuItem>;
}

lazy_static!{
    /*
    static const FUID icomponent_handler3_iid;
    */
}

declare_class_iid!{
    IComponentHandler3, 
    0x69F11617, 
    0xD26B400D, 
    0xA4B6B964, 
    0x7B6EBBAB
}
