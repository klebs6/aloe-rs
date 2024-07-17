crate::ix!();

lazy_static!{
    /*
       static const char* unlockedProp;
       static const char* expiryTimeProp;
       const char* OnlineUnlockStatus::unlockedProp = "u";
       const char* OnlineUnlockStatus::expiryTimeProp = "t";
    */
}

pub trait OnlineUnlockStatusInterface:
    GetProductID
    + DoesProductIDMatch
    + GetPublicKey 
    + SaveState
    + GetState
    + GetWebsiteName
    + GetServerAuthenticationURL
    + ReadReplyFromWebserver
    + GetLocalMachineIDs
    + UserCancelled
    + GetMessageForConnectionFailure
    + GetMessageForUnexpectedReply {}

