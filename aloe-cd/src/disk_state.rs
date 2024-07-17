crate::ix!();

pub enum AudioCDBurnerDiskState
{
    /**
      | An error condition, if the device isn't
      | responding
      |
      */
    unknown,                

    /**
      | The drive is currently open. Note that
      | a slot-loading drive may seem to be permanently
      | open.
      |
      */
    trayOpen,               

    /**
      | The drive has no disk in it
      |
      */
    noDisc,                 

    /**
      | The drive contains a writeable disk
      |
      */
    writableDiskPresent,    

    /**
      | The drive contains a read-only disk
      |
      */
    readOnlyDiskPresent     
}
