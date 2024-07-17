crate::ix!();

pub const XWINDOW_SYSTEM_DND_VERSION: u64 = 3;

pub enum XWindowSystemAtomsProtocolItems {
    TAKE_FOCUS    = 0,
    DELETE_WINDOW = 1,
    PING          = 2
}

/**
  | Initialises and stores some atoms for
  | the display.
  | 
  | @tags{GUI}
  |
  */
#[derive(Default)]
pub struct XWindowSystemAtoms {
    protocols:               Atom,
    protocol_list:           [Atom; 3],
    change_state:            Atom,
    state:                   Atom,
    user_time:               Atom,
    active_win:              Atom,
    pid:                     Atom,
    window_type:             Atom,
    window_state:            Atom,
    window_state_hidden:     Atom,
    xdnd_aware:              Atom,
    xdnd_enter:              Atom,
    xdnd_leave:              Atom,
    xdnd_position:           Atom,
    xdnd_status:             Atom,
    xdnd_drop:               Atom,
    xdnd_finished:           Atom,
    xdnd_selection:          Atom,
    xdnd_type_list:          Atom,
    xdnd_action_list:        Atom,
    xdnd_action_description: Atom,
    xdnd_action_copy:        Atom,
    xdnd_action_private:     Atom,
    xembed_msg_type:         Atom,
    xembed_info:             Atom,
    allowed_actions:         [Atom; 5],
    allowed_mime_types:      [Atom; 4],
    utf_8string:             Atom,
    clipboard:               Atom,
    targets:                 Atom,
}

impl XWindowSystemAtoms {

    pub fn new(display: *mut Display) -> Self {
    
        todo!();
        /*


            protocols                    = getIfExists (display, "WM_PROTOCOLS");
        protocolList [TAKE_FOCUS]    = getIfExists (display, "WM_TAKE_FOCUS");
        protocolList [DELETE_WINDOW] = getIfExists (display, "WM_DELETE_WINDOW");
        protocolList [PING]          = getIfExists (display, "_NET_WM_PING");
        changeState                  = getIfExists (display, "WM_CHANGE_STATE");
        state                        = getIfExists (display, "WM_STATE");
        userTime                     = getCreating (display, "_NET_WM_USER_TIME");
        activeWin                    = getCreating (display, "_NET_ACTIVE_WINDOW");
        pid                          = getCreating (display, "_NET_WM_PID");
        windowType                   = getIfExists (display, "_NET_WM_WINDOW_TYPE");
        windowState                  = getIfExists (display, "_NET_WM_STATE");
        windowStateHidden            = getIfExists (display, "_NET_WM_STATE_HIDDEN");

        XdndAware                    = getCreating (display, "XdndAware");
        XdndEnter                    = getCreating (display, "XdndEnter");
        XdndLeave                    = getCreating (display, "XdndLeave");
        XdndPosition                 = getCreating (display, "XdndPosition");
        XdndStatus                   = getCreating (display, "XdndStatus");
        XdndDrop                     = getCreating (display, "XdndDrop");
        XdndFinished                 = getCreating (display, "XdndFinished");
        XdndSelection                = getCreating (display, "XdndSelection");

        XdndTypeList                 = getCreating (display, "XdndTypeList");
        XdndActionList               = getCreating (display, "XdndActionList");
        XdndActionCopy               = getCreating (display, "XdndActionCopy");
        XdndActionPrivate            = getCreating (display, "XdndActionPrivate");
        XdndActionDescription        = getCreating (display, "XdndActionDescription");

        XembedMsgType                = getCreating (display, "_XEMBED");
        XembedInfo                   = getCreating (display, "_XEMBED_INFO");

        allowedMimeTypes[0]          = getCreating (display, "UTF8_STRING");
        allowedMimeTypes[1]          = getCreating (display, "text/plain;charset=utf-8");
        allowedMimeTypes[2]          = getCreating (display, "text/plain");
        allowedMimeTypes[3]          = getCreating (display, "text/uri-list");

        allowedActions[0]            = getCreating (display, "XdndActionMove");
        allowedActions[1]            = XdndActionCopy;
        allowedActions[2]            = getCreating (display, "XdndActionLink");
        allowedActions[3]            = getCreating (display, "XdndActionAsk");
        allowedActions[4]            = XdndActionPrivate;

        utf8String                   = getCreating (display, "UTF8_STRING");
        clipboard                    = getCreating (display, "CLIPBOARD");
        targets                      = getCreating (display, "TARGETS");
        */
    }
    
    pub fn get_if_exists(&mut self, 
        display: *mut Display,
        name:    *const u8) -> Atom {
        
        todo!();
        /*
            return X11Symbols::getInstance()->xInternAtom (display, name, True);
        */
    }
    
    pub fn get_creating(&mut self, 
        display: *mut Display,
        name:    *const u8) -> Atom {
        
        todo!();
        /*
            return X11Symbols::getInstance()->xInternAtom (display, name, False);
        */
    }
    
    pub fn get_name(&mut self, 
        display: *mut Display,
        atom:    Atom) -> String {
        
        todo!();
        /*
            if (atom == None)
            return "None";

        return X11Symbols::getInstance()->xGetAtomName (display, atom);
        */
    }
    
    pub fn is_mime_type_file(&mut self, 
        display: *mut Display,
        atom:    Atom) -> bool {
        
        todo!();
        /*
            return getName (display, atom).equalsIgnoreCase ("text/uri-list");
        */
    }
}
