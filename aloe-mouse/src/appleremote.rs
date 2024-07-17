crate::ix!();

pub trait ButtonPressed {

    /**
      | Override this method to receive the
      | callback about a button press.
      | 
      | The callback will happen on the application's
      | message thread.
      | 
      | Some buttons trigger matching up and
      | down events, in which the isDown parameter
      | will be true and then false. Others only
      | send a single event when the button is
      | pressed.
      |
      */
    fn button_pressed(&mut self, 
            button_id: AppleRemoteDeviceButtonType,
            is_down:   bool);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_AppleRemote.h]

/**
  | Receives events from an Apple IR remote
  | control device (Only available in OSX!).
  | 
  | To use it, just create a subclass of this
  | class, implementing the buttonPressed()
  | callback, then call start() and stop()
  | to start or stop receiving events.
  | 
  | @tags{GUI}
  |
  */
#[cfg(target_os="macos")]
#[no_copy]
#[leak_detector]
pub struct AppleRemoteDevice {
    device:    *mut c_void,
    queue:     *mut c_void,
    remote_id: i32,
}

#[cfg(target_os="macos")]
pub trait AppleRemoteDeviceInterface: ButtonPressed {}

/**
  | The set of buttons that may be pressed.
  | @see buttonPressed
  |
  */
#[cfg(target_os="macos")]
pub enum AppleRemoteDeviceButtonType
{
    /**
      | The menu button (if it's held for a short
      | time).
      |
      */
    menuButton = 0,     

    /**
      | The play button.
      |
      */
    playButton,         

    /**
      | The plus or volume-up button.
      |
      */
    plusButton,         

    /**
      | The minus or volume-down button.
      |
      */
    minusButton,        

    /**
      | The right button (if it's held for a short
      | time).
      |
      */
    rightButton,        

    /**
      | The left button (if it's held for a short
      | time).
      |
      */
    leftButton,         

    /**
      | The right button (if it's held for a long
      | time).
      |
      */
    rightButton_Long,   

    /**
      | The menu button (if it's held for a long
      | time).
      |
      */
    leftButton_Long,    

    /**
      | The menu button (if it's held for a long
      | time).
      |
      */
    menuButton_Long,    

    playButtonSleepMode,

    switched
}

#[cfg(target_os="macos")]
impl AppleRemoteDevice {

    /**
      | Starts the device running and responding
      | to events.
      | 
      | Returns true if it managed to open the
      | device.
      | 
      | -----------
      | @param inExclusiveMode
      | 
      | if true, the remote will be grabbed exclusively
      | for this app, and will not be available
      | to any other part of the system. If false,
      | it will be shared with other apps. @see
      | stop
      |
      */
    pub fn start(&mut self, in_exclusive_mode: bool) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Stops the device running. @see start
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if the device has been started
      | successfully.
      |
      */
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the ID number of the remote,
      | if it has sent one.
      |
      */
    pub fn get_remote_id(&self) -> i32 {
        
        todo!();
        /*
            return remoteId;
        */
    }
    
    pub fn handle_callback_internal(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn open(&mut self, open_in_exclusive_mode: bool) -> bool {
        
        todo!();
        /*
        
        */
    }
}
