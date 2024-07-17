crate::ix!();

/**
  | Connect a component with another one:
  | Vst::IConnectionPoint \ingroup vstIPlug
  | vst300
  | 
  | - [plug imp]
  | 
  | - [host imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | This interface is used for the communication
  | of separate components.
  | 
  | -----------
  | @note
  | 
  | some hosts will place a proxy object
  | between the components so that they
  | are not directly connected.
  | 
  | \see \ref vst3Communication
  |
  */
pub trait IConnectionPoint: FUnknown {

    /**
      | Connects this instance with another
      | connection point.
      |
      */
    #[PLUGIN_API]
    fn connect(&mut self, other: *mut dyn IConnectionPoint) -> tresult;

    /**
      | Disconnects a given connection point
      | from this.
      |
      */
    #[PLUGIN_API]
    fn disconnect(&mut self, other: *mut dyn IConnectionPoint) -> tresult;

    /**
      | Called when a message has been sent from
      | the connection point to this.
      |
      */
    #[PLUGIN_API]
    fn notify(&mut self, message: *mut dyn IMessage) -> tresult;
}

lazy_static!{
    /*
    static const FUID iconnection_point_iid;
    */
}

declare_class_iid!{
    IConnectionPoint, 
    0x70A4156F, 
    0x6E6E4026, 
    0x989148BF, 
    0xAA60D8D1
}
