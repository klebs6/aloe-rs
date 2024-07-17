crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/native/aloe_linux_EventLoop.h]

pub mod linux_event_loop {
    use super::*;

    /**
      | Registers a callback that will be called
      | when a file descriptor is ready for I/O.
      | 
      | This will add the given file descriptor
      | to the internal set of file descriptors
      | that will be passed to the poll() call.
      | When this file descriptor has data to
      | read the readCallback will be called.
      | 
      | -----------
      | @param fd
      | 
      | the file descriptor to be monitored
      | ----------
      | @param readCallback
      | 
      | a callback that will be called when the
      | file descriptor has data to read. The
      | file descriptor will be passed as an
      | argument
      | ----------
      | @param eventMask
      | 
      | a bit mask specifying the events you
      | are interested in for the file descriptor.
      | The possible values for this are defined
      | in <poll.h>
      |
      */
    pub fn register_fd_callback(
        fd:            i32,
        read_callback: fn(_0: i32) -> (),
        event_mask:    Option<i16>

    ) {
        let event_mask: i16 = event_mask.unwrap_or(1 /*POLLIN*/);

        todo!();
        /*

        */
    }

    /**
      | Unregisters a previously registered
      | file descriptor.
      | 
      | @see registerFdCallback
      |
      */
    pub fn unregister_fd_callback(fd: i32)  {

        todo!();
        /*

        */
    }
}
