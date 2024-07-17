crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/marketplace/aloe_TracktionMarketplaceStatus.h]

/**
  | An implementation of the OnlineUnlockStatus
  | class which talks to the
  | 
  | Tracktion Marketplace server.
  | 
  | For details about how to use this class,
  | see the docs for the base class: OnlineUnlockStatus.
  | Basically, you need to inherit from
  | it, and implement all the pure virtual
  | methods to tell it about your product.
  | 
  | @see OnlineUnlockStatus, OnlineUnlockForm,
  | KeyGeneration
  | 
  | @tags{ProductUnlocking}
  |
  */
#[no_copy]
#[leak_detector]
pub struct TracktionMarketplaceStatus<'a> {
    base:                 OnlineUnlockStatus,
    stream_creation_lock: CriticalSection,
    stream:               Box<WebInputStream<'a>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/marketplace/aloe_TracktionMarketplaceStatus.cpp]
impl<'a> TracktionMarketplaceStatus<'a> {
    
    pub fn get_server_authenticationurl(&mut self) -> Url {
        
        todo!();
        /*
            return Url ("https://www.tracktion.com/marketplace/authenticate.php");
        */
    }
    
    pub fn get_website_name(&mut self) -> String {
        
        todo!();
        /*
            return "tracktion.com";
        */
    }
    
    pub fn does_product_id_match(&mut self, returned_id_from_server: &String) -> bool {
        
        todo!();
        /*
            return getProductID() == returnedIDFromServer;
        */
    }
    
    pub fn read_reply_from_webserver(&mut self, 
        email:    &String,
        password: &String) -> String {
        
        todo!();
        /*
            Url url (getServerAuthenticationURL()
                    .withParameter ("product", getProductID())
                    .withParameter ("email", email)
                    .withParameter ("pw", password)
                    .withParameter ("os", SystemStats::getOperatingSystemName())
                    .withParameter ("mach", getLocalMachineIDs()[0]));

        DBG ("Trying to unlock via Url: " << url.toString (true));

        {
            ScopedLock lock (streamCreationLock);
            stream.reset (new WebInputStream (url, true));
        }

        if (stream->connect (nullptr))
        {
            auto thread = Thread::getCurrentThread();
            MemoryOutputStream result;

            while (! (stream->isExhausted() || stream->isError()
                        || (thread != nullptr && thread->threadShouldExit())))
            {
                auto bytesRead = result.writeFromInputStream (*stream, 8192);

                if (bytesRead < 0)
                    break;
            }

            return result.toString();
        }

        return {};
        */
    }
    
    pub fn user_cancelled(&mut self)  {
        
        todo!();
        /*
            ScopedLock lock (streamCreationLock);

        if (stream != nullptr)
            stream->cancel();
        */
    }
}
