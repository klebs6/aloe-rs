/*!
  | \defgroup platformUIType Platform
  | UI Types \ingroup pluginGUI
  | 
  | List of Platform UI types for IPlugView.
  | This list is used to match the GUI-System
  | between the host and a plug-in in case
  | that an OS provides multiple GUI-APIs.
  |
  */
crate::ix!();

/**
  | The parent parameter in IPlugView::attached()
  | is a HWND handle.
  | 
  | You should attach a child window to it.
  |
  */
pub const PLATFORM_TYPEHWND: FIDString = "HWND"; // /< HWND handle. (Microsoft Windows)

/**
  | The parent parameter in IPlugView::attached()
  | is a WindowRef.
  | 
  | You should attach a HIViewRef to the
  | content view of the window.
  |
  */
pub const PLATFORM_TYPE_HI_VIEW: FIDString = "HIView"; // /< HIViewRef. (Mac OS X)

/**
  | The parent parameter in IPlugView::attached()
  | is a NSView pointer.
  | 
  | You should attach a NSView to it.
  |
  */
pub const PLATFORM_TYPE_NS_VIEW: FIDString = "NSView"; // /< NSView pointer. (Mac OS X)

/**
  | The parent parameter in IPlugView::attached()
  | is a UIView pointer.
  | 
  | You should attach an UIView to it.
  |
  */
pub const PLATFORM_TYPE_UI_VIEW: FIDString = "UIView"; // /< UIView pointer. (iOS)

/**
  | The parent parameter in IPlugView::attached()
  | is a X11 Window supporting XEmbed.
  | 
  | You should attach a Window to it that
  | supports the XEmbed extension.
  | 
  | See https://standards.freedesktop.org/xembed-spec/xembed-spec-latest.html
  |
  */
pub const PLATFORM_TYPE_X11EMBED_WINDOWID: FIDString = "X11EmbedWindowID"; // /< X11 Window ID. (X11)

