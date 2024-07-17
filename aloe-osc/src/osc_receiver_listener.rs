crate::ix!();

/**
  | An error handler function for OSC format
  | errors that can be called by the
  | 
  | OSCReceiver.
  | 
  | The arguments passed are the pointer
  | to and the data of the buffer that the
  | OSCReceiver has failed to parse.
  |
  */
pub type OSCReceiverFormatErrorHandler = fn(data: *const u8, data_size: i32) -> ();

/**
  | Use this struct as the template parameter
  | for OSCReceiverListener and
  | 
  | OSCReceiverListenerWithOSCAddress to receive
  | incoming OSC data on the message thread.
  | 
  | This should be used by OSC callbacks
  | that are not realtime-critical, but
  | have significant work to do, for example
  | updating Components in your app's user
  | interface.
  | 
  | This is the default type of OSC listener.
  |
  */
pub struct   OSCReceiverMessageLoopCallback {}

/**
  | Use this struct as the template parameter
  | for OSCReceiverListener and
  | 
  | OSCReceiverListenerWithOSCAddress to receive
  | incoming OSC data immediately after
  | it arrives, called directly on the network
  | thread that listens to incoming
  | 
  | OSC traffic.
  | 
  | This type can be used by OSC callbacks
  | that don't do much, but are realtime-critical,
  | for example, setting real-time audio
  | parameters.
  |
  */
pub struct   OSCReceiverRealtimeCallback {}

/**
  | A class for receiving OSC data from an
  | OSCReceiver.
  | 
  | The template argument CallbackType
  | determines how the callback will be
  | called and has to be either OSCReceiverMessageLoopCallback
  | or OSCReceiverRealtimeCallback. If not specified,
  | 
  | OSCReceiverMessageLoopCallback will be used by
  | default.
  | 
  | @see OSCReceiver::addListener, OSCReceiver::OSCReceiverListenerWithOSCAddress,
  | 
  | OSCReceiver::OSCReceiverMessageLoopCallback,
  | OSCReceiver::OSCReceiverRealtimeCallback
  |
  */
pub trait OSCReceiverListener<CallbackType = OSCReceiverMessageLoopCallback> {

    /**
      | Called when the OSCReceiver receives
      | a new OSC message.
      | 
      | You must implement this function.
      |
      */
    fn osc_message_received(&mut self, message: &OSCMessage);

    /**
      | Called when the OSCReceiver receives
      | a new OSC bundle.
      | 
      | If you are not interested in OSC bundles,
      | just ignore this method.
      | 
      | The default implementation provided
      | here will simply do nothing.
      |
      */
    fn osc_bundle_received(&mut self, bundle: &OSCBundle);
}

/**
  | A class for receiving only those OSC
  | messages from an OSCReceiver that match
  | a given OSC address.
  | 
  | Use this class if your app receives OSC
  | messages with different address patterns
  | (for example "/aloe/fader1", /aloe/knob2"
  | etc.) and you want to route those to different
  | objects. This class contains pre-build
  | functionality for that OSC address
  | routing, including wildcard pattern
  | matching (e.g. "/aloe/fader[0-9]").
  | 
  | This class implements the concept of
  | an "OSC Method" from the OpenSoundControl
  | 1.0 specification.
  | 
  | The template argument CallbackType
  | determines how the callback will be
  | called and has to be either OSCReceiverMessageLoopCallback
  | or OSCReceiverRealtimeCallback. If not specified,
  | 
  | OSCReceiverMessageLoopCallback will be used by
  | default.
  | 
  | -----------
  | @note
  | 
  | This type of listener will ignore OSC
  | bundles.
  | 
  | @see OSCReceiver::addListener, OSCReceiver::OSCReceiverListener,
  | 
  | OSCReceiver::OSCReceiverMessageLoopCallback,
  | OSCReceiver::OSCReceiverRealtimeCallback
  |
  */
pub trait OSCReceiverListenerWithOSCAddress<CallbackType = OSCReceiverMessageLoopCallback> {

    /**
      | Called when the OSCReceiver receives
      | an OSC message with an OSC address pattern
      | that matches the OSC address with which
      | this listener was added.
      |
      */
    fn osc_message_received(&mut self, message: &OSCMessage);
}
