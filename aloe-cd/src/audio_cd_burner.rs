crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/audio_cd/aloe_AudioCDBurner.h]

#[no_copy]
#[leak_detector]
#[cfg(feature = "cd-burner")]
pub struct AudioCDBurner<'a> {
    base: ChangeBroadcaster<'a>,
    pimpl: Box<Pimpl>,
}

#[cfg(feature = "cd-burner")]
impl<'a> AudioCDBurner<'a> {

    /**
      | Returns a list of available optical
      | drives.
      | 
      | Use openDevice() to open one of the items
      | from this list.
      |
      */
    pub fn find_available_devices() -> Vec<String> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Tries to open one of the optical drives.
      | 
      | The deviceIndex is an index into the
      | array returned by findAvailableDevices().
      |
      */
    pub fn open_device(device_index: i32) -> *mut AudioCDBurner<'a> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the current status of the device.
      | 
      | To get informed when the drive's status
      | changes, attach a ChangeListener to
      | the AudioCDBurner.
      |
      */
    pub fn get_disk_state(&self) -> AudioCDBurnerDiskState {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if there's a writable disk
      | in the drive.
      |
      */
    pub fn is_disk_present(&self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Sends an eject signal to the drive.
      | 
      | The eject will happen asynchronously,
      | so you can use getDiskState() and waitUntilStateChange()
      | to monitor its progress.
      |
      */
    pub fn open_tray(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Blocks the current thread until the
      | drive's state changes, or until the
      | timeout expires.
      | 
      | -----------
      | @return
      | 
      | the device's new state
      |
      */
    pub fn wait_until_state_change(&mut self, time_out_milliseconds: i32) -> AudioCDBurnerDiskState {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the set of possible write speeds
      | that the device can handle. These are
      | as a multiple of 'normal' speed, so e.g.
      | '24x' returns 24, etc. Note that if there's
      | no media present in the drive, this value
      | may be unavailable! @see setWriteSpeed,
      | getWriteSpeed
      |
      */
    pub fn get_available_write_speeds(&self) -> Vec<i32> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Tries to enable or disable buffer underrun
      | safety on devices that support it.
      | 
      | -----------
      | @return
      | 
      | true if it's now enabled. If the device
      | doesn't support it, this will always
      | return false.
      |
      */
    pub fn set_buffer_underrun_protection(&mut self, should_be_enabled: bool) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the number of free blocks on
      | the disk.
      | 
      | There are 75 blocks per second, at 44100Hz.
      |
      */
    pub fn get_num_available_audio_blocks(&self) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Adds a track to be written.
      | 
      | The source passed-in here will be kept
      | by this object, and it will be used and
      | deleted at some point in the future,
      | either during the burn() method or when
      | this AudioCDBurner object is deleted.
      | Your caller method shouldn't keep a
      | reference to it or use it again after
      | passing it in here.
      |
      */
    pub fn add_audio_track(
        &mut self, 
        source:      *mut dyn AudioSource,
        num_samples: i32

    ) -> bool {
        
        todo!();
        /*
        
        */
    }


    /**
      | Runs the burn process. This method will
      | block until the operation is complete.
      | 
      | -----------
      | @param listener
      | 
      | the object to receive callbacks about
      | progress
      | ----------
      | @param ejectDiscAfterwards
      | 
      | whether to eject the disk after the burn
      | completes
      | ----------
      | @param performFakeBurnForTesting
      | 
      | if true, no data will actually be written
      | to the disk
      | ----------
      | @param writeSpeed
      | 
      | one of the write speeds from getAvailableWriteSpeeds(),
      | or 0 or less to mean the fastest speed.
      |
      */
    pub fn burn(
        &mut self, 
        listener:                      *mut dyn AudioCDBurnerBurnProgressListener,
        eject_disc_afterwards:         bool,
        perform_fake_burn_for_testing: bool,
        write_speed:                   i32

    ) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | If a burn operation is currently in progress,
      | this tells it to stop as soon as possible.
      | 
      | It's also possible to stop the burn process
      | by returning true from AudioCDBurnerBurnProgressListener::audioCDBurnProgress()
      |
      */
    pub fn abort_burn(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(device_index: i32) -> Self {
    
        todo!();
        /*


        
        */
    }
}
