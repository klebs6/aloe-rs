/*!
  | \defgroup pluginGUI Graphical User
  | Interface
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/gui/iplugview.h]

/**
  | Plug-in definition of a view. \ingroup
  | pluginGUI vstIPlug vst300
  | 
  | - [plug imp]
  | 
  | - [released: 3.0.0]
  | 
  | \par Sizing of a view
  | 
  | Usually, the size of a plug-in view is
  | fixed. But both the host and the plug-in
  | can cause a view to be resized: \n
  | 
  | - \b Host: If IPlugView::canResize()
  | returns kResultTrue the host will set
  | up the window so that the user can resize
  | it. While the user resizes the window,
  | 
  | IPlugView::checkSizeConstraint()
  | is called, allowing the plug-in to change
  | the size to a valid a valid supported
  | rectangle size. The host then resizes
  | the window to this rect and has to call
  | IPlugView::onSize(). \n \n
  | 
  | - \b Plug-in: The plug-in can call IPlugFrame::resizeView()
  | and cause the host to resize the window.\n\n
  | 
  | Afterwards, in the same callstack,
  | the host has to call IPlugView::onSize()
  | if a resize is needed (size was changed).
  | 
  | -----------
  | @note
  | 
  | if the host calls IPlugView::getSize()
  | before calling IPlugView::onSize()
  | (if needed), it will get the current
  | (old) size not the wanted one!!\n
  | 
  | Here the calling sequence:\n
  | 
  | - plug-in->host: IPlugFrame::resizeView
  | (newSize)
  | 
  | - host->plug-in (optional): IPlugView::getSize()
  | returns the currentSize (not the newSize!)
  | 
  | - host->plug-in: if newSize is different
  | from the current size: IPlugView::onSize
  | (newSize)
  | 
  | - host->plug-in (optional): IPlugView::getSize()
  | returns the newSize \n <b>Please only
  | resize the platform representation
  | of the view when IPlugView::onSize()
  | is called.</b>
  | 
  | \par Keyboard handling
  | 
  | The plug-in view receives keyboard
  | events from the host. A view implementation
  | must not handle keyboard events by the
  | means of platform callbacks, but let
  | the host pass them to the view. The host
  | depends on a proper return value when
  | IPlugView::onKeyDown is called, otherwise
  | the plug-in view may cause a malfunction
  | of the host's key command handling.
  | 
  | \see IPlugFrame, \ref platformUIType
  |
  */
pub trait IPlugView: FUnknown {

    /**
      | Is Platform UI Type supported
      | 
      | -----------
      | @param type
      | 
      | : IDString of \ref platformUIType
      |
      */
    #[PLUGIN_API]
    fn is_platform_type_supported(&mut self, ty: FIDString) -> tresult;

    /**
      | The parent window of the view has been
      | created, the (platform) representation
      | of the view should now be created as well.
      | 
      | -----------
      | @note
      | 
      | the parent is owned by the caller and
      | you are not allowed to alter it in any
      | way other than adding your own views.
      | ----------
      | @note
      | 
      | in this call the plug-in could call a
      | IPlugFrame::resizeView ()!
      | 
      | -----------
      | @param parent
      | 
      | : platform handle of the parent window
      | or view
      | ----------
      | @param type
      | 
      | : \ref platformUIType which should
      | be created
      |
      */
    #[PLUGIN_API]
    fn attached(&mut self, 
            parent: *mut c_void,
            ty:     FIDString) -> tresult;

    /**
      | The parent window of the view is about
      | to be destroyed.
      | 
      | You have to remove all your own views
      | from the parent window or view.
      |
      */
    #[PLUGIN_API]
    fn removed(&mut self) -> tresult;

    /**
      | Handling of mouse wheel.
      |
      */
    #[PLUGIN_API]
    fn on_wheel(&mut self, distance: f32) -> tresult;

    /**
      | Handling of keyboard events : Key Down.
      | 
      | -----------
      | @param key
      | 
      | : unicode code of key
      | ----------
      | @param keyCode
      | 
      | : virtual keycode for non ascii keys
      | - see \ref VirtualKeyCodes in keycodes.h
      | ----------
      | @param modifiers
      | 
      | : any combination of modifiers - see
      | \ref KeyModifier in keycodes.h
      | 
      | -----------
      | @return
      | 
      | kResultTrue if the key is handled, otherwise
      | kResultFalse. \n <b> Please note that
      | kResultTrue must only be returned if
      | the key has really been handled. </b>
      | Otherwise key command handling of the
      | host might be blocked!
      |
      */
    #[PLUGIN_API]
    fn on_key_down(&mut self, 
            key:       u16,
            key_code:  i16,
            modifiers: i16) -> tresult;

    /**
      | Handling of keyboard events : Key Up.
      | 
      | -----------
      | @param key
      | 
      | : unicode code of key
      | ----------
      | @param keyCode
      | 
      | : virtual keycode for non ascii keys
      | - see \ref VirtualKeyCodes in keycodes.h
      | ----------
      | @param modifiers
      | 
      | : any combination of KeyModifier - see
      | \ref KeyModifier in keycodes.h
      | 
      | -----------
      | @return
      | 
      | kResultTrue if the key is handled, otherwise
      | return kResultFalse.
      |
      */
    #[PLUGIN_API]
    fn on_key_up(&mut self, 
            key:       u16,
            key_code:  i16,
            modifiers: i16) -> tresult;

    /**
      | Returns the size of the platform representation
      | of the view.
      |
      */
    #[PLUGIN_API]
    fn get_size(&mut self, size: *mut ViewRect) -> tresult;

    /**
      | Resizes the platform representation
      | of the view to the given rect. Note that
      | if the plug-in requests a resize (IPlugFrame::resizeView
      | ()) onSize has to be called afterward.
      |
      */
    #[PLUGIN_API]
    fn on_size(&mut self, new_size: *mut ViewRect) -> tresult;

    /**
      | Focus changed message.
      |
      */
    #[PLUGIN_API]
    fn on_focus(&mut self, state: TBool) -> tresult;

    /**
      | Sets IPlugFrame object to allow the
      | plug-in to inform the host about resizing.
      |
      */
    #[PLUGIN_API]
    fn set_frame(&mut self, frame: *mut dyn IPlugFrame) -> tresult;

    /**
      | Is view sizable by user.
      |
      */
    #[PLUGIN_API]
    fn can_resize(&mut self) -> tresult;

    /**
      | On live resize this is called to check
      | if the view can be resized to the given
      | rect, if not adjust the rect to the allowed
      | size.
      |
      */
    #[PLUGIN_API]
    fn check_size_constraint(&mut self, rect: *mut ViewRect) -> tresult;
}

lazy_static!{
    /*
    static const FUID iplug_view_iid;
    */
}

declare_class_iid!{
    IPlugView, 
    0x5BC32507, 
    0xD06049EA, 
    0xA6151B52, 
    0x2B755B29
}
