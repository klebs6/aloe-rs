/**
   Description : Base class for Component and Edit
   Controller
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstcomponentbase.h]

/**
  | Base class for Vst 3 Component and Edit
  | Controller. \ingroup vstClasses
  |
  */
pub struct ComponentBase {
    base:            FObject,
    host_context:    IPtr<Box<dyn FUnknown>>,
    peer_connection: IPtr<Box<dyn IConnectionPoint>>,
}

pub trait ComponentBaseInterface {}

impl FUnknown for ComponentBase {

    fn query_interface(
        &mut self, 
        _: [i8; 16], 
        _: *mut *mut c_void

    ) -> i32 { 
        todo!() 
    }

    fn add_ref(&mut self) -> u32 { 
        todo!() 
    }

    fn release(&mut self) -> u32 { 
        todo!() 
    }
}

impl IPluginBase for ComponentBase {

    fn initialize(
        &mut self, 
        _: *mut (dyn aloe_vst_types::FUnknown + 'static)

    ) -> i32 { 
        todo!() 
    }

    fn terminate(&mut self) -> i32 { todo!() }
}

impl IConnectionPoint for ComponentBase {

    fn connect(
        &mut self, 
        _: *mut (dyn aloe_vst_message::IConnectionPoint + 'static)

    ) -> i32 { 
        todo!() 
    }

    fn disconnect(
        &mut self, 
        _: *mut (dyn aloe_vst_message::IConnectionPoint + 'static)

    ) -> i32 { 
        todo!() 
    }

    fn notify(
        &mut self, 
        _: *mut (dyn aloe_vst_message::IMessage + 'static)

    ) -> i32 { 
        todo!() 
    }
}

pub trait ReceiveText {

    /**
      | Receives a simple text message from
      | the peer (max 255 characters). Text
      | is UTF-8 encoded.
      |
      */
    fn receive_text(&mut self, text: *const u8) -> tresult;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstcomponentbase.cpp]
impl ComponentBase {

    /**
      | Returns the hostContext (set by the
      | host during initialize call).
      |
      */
    pub fn get_host_context(&self) -> *mut dyn FUnknown {
        
        todo!();
        /*
            return hostContext;
        */
    }

    /**
      | Returns the peer for the messaging communication
      | (you can only use IConnectionPoint::notify
      | for communicate between peers, do not
      | try to cast peerConnection.
      |
      */
    pub fn get_peer(&self) -> *mut dyn IConnectionPoint {
        
        todo!();
        /*
            return peerConnection;
        */
    }

    lazy_static!{
        /*
        OBJ_METHODS (ComponentBase, FObject)
            DEFINE_INTERFACES
                DEF_INTERFACE (IPluginBase)
                DEF_INTERFACE (IConnectionPoint)
            END_DEFINE_INTERFACES (FObject)
            REFCOUNT_METHODS (FObject)
        */
    }
    
    #[PLUGIN_API]
    pub fn initialize(&mut self, context: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            // check if already initialized
        if (hostContext)
            return kResultFalse;

        hostContext = context;

        return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    pub fn terminate(&mut self) -> tresult {
        
        todo!();
        /*
            // release host interfaces
        hostContext = nullptr;

        // in case host did not disconnect us,
        // release peer now
        if (peerConnection)
        {
            peerConnection->disconnect (this);
            peerConnection = nullptr;
        }

        return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    pub fn connect(&mut self, other: *mut dyn IConnectionPoint) -> tresult {
        
        todo!();
        /*
            if (!other)
            return kInvalidArgument;

        // check if already connected
        if (peerConnection)
            return kResultFalse;

        peerConnection = other;
        return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    pub fn disconnect(&mut self, other: *mut dyn IConnectionPoint) -> tresult {
        
        todo!();
        /*
            if (peerConnection && other == peerConnection)
        {
            peerConnection = nullptr;
            return kResultOk;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn notify(&mut self, message: *mut dyn IMessage) -> tresult {
        
        todo!();
        /*
            if (!message)
            return kInvalidArgument;

        if (FIDStringsEqual (message->getMessageID (), "TextMessage"))
        {
            TChar string[256] = {0};
            if (message->getAttributes ()->getString ("Text", string, sizeof (string)) == kResultOk)
            {
                String tmp (string);
                tmp.toMultiByte (kCP_Utf8);
                return receiveText (tmp.text8 ());
            }
        }

        return kResultFalse;
        */
    }
    
    /**
      | Allocates a message instance (do not
      | forget to release it).
      |
      */
    pub fn allocate_message(&self) -> *mut dyn IMessage {
        
        todo!();
        /*
            FUnknownPtr<IHostApplication> hostApp (hostContext);
        if (hostApp)
            return Vst::allocateMessage (hostApp);
        return nullptr;
        */
    }
    
    /**
      | Sends the given message to the peer.
      |
      */
    pub fn send_message(&self, message: *mut dyn IMessage) -> tresult {
        
        todo!();
        /*
            if (message != nullptr && getPeer () != nullptr)
            return getPeer ()->notify (message);
        return kResultFalse;
        */
    }
    
    /**
      | Sends a simple text message to the peer
      | (max 255 characters).
      | 
      | Text is interpreted as UTF-8.
      |
      */
    pub fn send_text_message(&self, text: *const u8) -> tresult {
        
        todo!();
        /*
            if (auto msg = owned (allocateMessage ()))
        {
            msg->setMessageID ("TextMessage");
            String tmp (text, kCP_Utf8);
            if (tmp.length () >= 256)
                tmp.remove (255);
            msg->getAttributes ()->setString ("Text", tmp.text16 ());
            return sendMessage (msg);
        }
        return kResultFalse;
        */
    }
    
    /**
      | Sends a message with a given ID without
      | any other payload.
      |
      */
    pub fn send_messageid(&self, messageid: *const u8) -> tresult {
        
        todo!();
        /*
            if (auto msg = owned (allocateMessage ()))
        {
            msg->setMessageID (messageID);
            return sendMessage (msg);
        }
        return kResultFalse;
        */
    }
    
    pub fn receive_text(&mut self, text: *const u8) -> tresult {
        
        todo!();
        /*
            return kResultOk;
        */
    }
}
