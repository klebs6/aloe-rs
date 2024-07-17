crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/marketplace/aloe_OnlineUnlockStatus.h]

/**
  | A base class for online unlocking systems.
  | 
  | This class stores information about
  | whether your app has been unlocked for
  | the current machine, and handles communication
  | with a web-store to perform the unlock
  | procedure.
  | 
  | You probably won't ever use this base
  | class directly, but rather a store-specific
  | subclass such as TracktionMarketplaceStatus,
  | which knows how to talk to the particular
  | online store that you're using.
  | 
  | To use it, you create a subclass which
  | implements all the pure virtual methods
  | (see their comments to find out what
  | you'll need to make them do).
  | 
  | Then you can create an instance of your
  | subclass which will hold the registration
  | state. Typically, you'll want to just
  | keep a single instance of the class around
  | for the duration of your app. You can
  | then call its methods to handle the various
  | registration tasks.
  | 
  | Areas of your code that need to know whether
  | the user is registered (e.g. to decide
  | whether a particular feature is available)
  | should call isUnlocked() to find out.
  | 
  | If you want to create a GUI that allows
  | your users to enter their details and
  | register, see the OnlineUnlockForm
  | class.
  | 
  | @see OnlineUnlockForm, KeyGeneration
  | 
  | @tags{ProductUnlocking}
  |
  */
#[no_copy]
#[leak_detector]
pub struct OnlineUnlockStatus {
    status: ValueTree,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/marketplace/aloe_OnlineUnlockStatus.cpp]
impl OnlineUnlockStatus {

    /*
       The following methods can be called by your
       app:
      */
    
    /**
      | Returns true if the product has been
      | successfully authorised for this machine.
      | 
      | The reason it returns a variant rather
      | than a bool is just to make it marginally
      | more tedious for crackers to work around.
      | Hopefully if this method gets inlined
      | they'll need to hack all the places where
      | you call it, rather than just the function
      | itself.
      | 
      | Bear in mind that each place where you
      | check this return value will need to
      | be changed by a cracker in order to unlock
      | your app, so the more places you call
      | this method, the more hassle it will
      | be for them to find and crack them all.
      |
      */
    #[inline] pub fn is_unlocked(&self) -> Var {
        
        todo!();
        /*
            return status[unlockedProp];
        */
    }

    /**
      | Returns the Time when the keyfile expires.
      | 
      | If a the key file obtained has an expiry
      | time, isUnlocked will return false
      | and this will return a non-zero time.
      | The interpretation of this is up to your
      | app but could be used for subscription
      | based models or trial periods.
      |
      */
    #[inline] pub fn get_expiry_time(&self) -> Time {
        
        todo!();
        /*
            return Time (static_cast<int64> (status[expiryTimeProp]));
        */
    }

    #[cfg(feature = "aloe-data-structures")]
    pub fn new() -> Self {
    
        todo!();
        /*
        : status(stateTagName),

        
        */
    }
    
    /**
      | Attempts to load the status from the
      | state retrieved by getState().
      | 
      | Call this somewhere in your app's startup
      | code.
      |
      */
    #[cfg(feature = "aloe-data-structures")]
    pub fn load(&mut self)  {
        
        todo!();
        /*
            MemoryBlock mb;
        mb.fromBase64Encoding (getState());

        if (! mb.isEmpty())
            status = ValueTree::readFromGZIPData (mb.getData(), mb.getSize());
        else
            status = ValueTree (stateTagName);

        StringArray localMachineNums (getLocalMachineIDs());

        if (machineNumberAllowed (StringArray ("1234"), localMachineNums))
            status.removeProperty (unlockedProp, nullptr);

        KeyFileUtils::KeyFileData data;
        data = KeyFileUtils::getDataFromKeyFile (KeyFileUtils::getXmlFromKeyFile (status[keyfileDataProp], getPublicKey()));

        if (data.keyFileExpires)
        {
            if (! doesProductIDMatch (data.appID))
                status.removeProperty (expiryTimeProp, nullptr);

            if (! machineNumberAllowed (data.machineNumbers, localMachineNums))
                status.removeProperty (expiryTimeProp, nullptr);
        }
        else
        {
            if (! doesProductIDMatch (data.appID))
                status.removeProperty (unlockedProp, nullptr);

            if (! machineNumberAllowed (data.machineNumbers, localMachineNums))
                status.removeProperty (unlockedProp, nullptr);
        }
        */
    }
    
    /**
      | Triggers a call to saveState which you
      | can use to store the current unlock status
      | in your app's settings.
      |
      */
    #[cfg(feature = "aloe-data-structures")]
    pub fn save(&mut self)  {
        
        todo!();
        /*
            MemoryOutputStream mo;

        {
            GZIPCompressorOutputStream gzipStream (mo, 9);
            status.writeToStream (gzipStream);
        }

        saveState (mo.getMemoryBlock().toBase64Encoding());
        */
    }
    
    #[cfg(feature = "aloe-data-structures")]
    pub fn get_local_machine_ids(&mut self) -> StringArray {
        
        todo!();
        /*
            return OnlineUnlockStatusMachineIDUtilities::getLocalMachineIDs();
        */
    }
    
    #[cfg(feature = "aloe-data-structures")]
    pub fn user_cancelled(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Optionally allows the app to provide
      | the user's email address if it is known.
      | 
      | You don't need to call this, but if you
      | do it may save the user typing it in.
      |
      */
    #[cfg(feature = "aloe-data-structures")]
    pub fn set_user_email(&mut self, username_or_email: &String)  {
        
        todo!();
        /*
            status.setProperty (userNameProp, usernameOrEmail, nullptr);
        */
    }
    
    /**
      | Returns the user's email address if
      | known.
      |
      */
    #[cfg(feature = "aloe-data-structures")]
    pub fn get_user_email(&self) -> String {
        
        todo!();
        /*
            return status[userNameProp].toString();
        */
    }
    
    /**
      | Attempts to perform an unlock using
      | a block of key-file data provided.
      | 
      | You may wish to use this as a way of allowing
      | a user to unlock your app by drag-and-dropping
      | a file containing the key data, or by
      | letting them select such a file. This
      | is often needed for allowing registration
      | on machines without internet access.
      |
      */
    #[cfg(feature = "aloe-data-structures")]
    pub fn apply_key_file(&mut self, key_file_content: String) -> bool {
        
        todo!();
        /*
            KeyFileUtils::KeyFileData data;
        data = KeyFileUtils::getDataFromKeyFile (KeyFileUtils::getXmlFromKeyFile (keyFileContent, getPublicKey()));

        if (data.licensee.isNotEmpty() && data.email.isNotEmpty() && doesProductIDMatch (data.appID))
        {
            setUserEmail (data.email);
            status.setProperty (keyfileDataProp, keyFileContent, nullptr);
            status.removeProperty (data.keyFileExpires ? expiryTimeProp : unlockedProp, nullptr);

            var actualResult (0), dummyResult (1.0);
            var v (machineNumberAllowed (data.machineNumbers, getLocalMachineIDs()));
            actualResult.swapWith (v);
            v = machineNumberAllowed (StringArray ("01"), getLocalMachineIDs());
            dummyResult.swapWith (v);
            jassert (! dummyResult);

            if (data.keyFileExpires)
            {
                if ((! dummyResult) && actualResult)
                    status.setProperty (expiryTimeProp, data.expiryTime.toMilliseconds(), nullptr);

                return getExpiryTime().toMilliseconds() > 0;
            }

            if ((! dummyResult) && actualResult)
                status.setProperty (unlockedProp, actualResult, nullptr);

            return isUnlocked();
        }

        return false;
        */
    }
    
    #[cfg(feature = "aloe-data-structures")]
    pub fn handle_xml_reply(&mut self, xml: XmlElement) -> OnlineUnlockStatusUnlockResult {
        
        todo!();
        /*
            OnlineUnlockStatusUnlockResult r;

        if (auto keyNode = xml.getChildByName ("KEY"))
        {
            const String keyText (keyNode->getAllSubText().trim());
            r.succeeded = keyText.length() > 10 && applyKeyFile (keyText);
        }
        else
        {
            r.succeeded = false;
        }

        if (xml.hasTagName ("MESSAGE"))
            r.informativeMessage = xml.getStringAttribute ("message").trim();

        if (xml.hasTagName ("ERROR"))
            r.errorMessage = xml.getStringAttribute ("error").trim();

        if (xml.getStringAttribute ("url").isNotEmpty())
            r.urlToLaunch = xml.getStringAttribute ("url").trim();

        if (r.errorMessage.isEmpty() && r.informativeMessage.isEmpty() && r.urlToLaunch.isEmpty() && ! r.succeeded)
            r.errorMessage = getMessageForUnexpectedReply();

        return r;
        */
    }
    
    #[cfg(feature = "aloe-data-structures")]
    pub fn handle_failed_connection(&mut self) -> OnlineUnlockStatusUnlockResult {
        
        todo!();
        /*
            OnlineUnlockStatusUnlockResult r;
        r.succeeded = false;
        r.errorMessage = getMessageForConnectionFailure (areMajorWebsitesAvailable());
        return r;
        */
    }
    
    #[cfg(feature = "aloe-data-structures")]
    pub fn get_message_for_connection_failure(&mut self, is_internet_connection_working: bool) -> String {
        
        todo!();
        /*
            String message = TRANS("Couldn't connect to XYZ").replace ("XYZ", getWebsiteName()) + "...\n\n";

        if (isInternetConnectionWorking)
            message << TRANS("Your internet connection seems to be OK, but our webserver "
                             "didn't respond... This is most likely a temporary problem, so try "
                             "again in a few minutes, but if it persists, please contact us for support!");
        else
            message << TRANS("No internet sites seem to be accessible from your computer.. Before trying again, "
                             "please check that your network is working correctly, and make sure "
                             "that any firewall/security software installed on your machine isn't "
                             "blocking your web connection.");

        return message;
        */
    }
    
    #[cfg(feature = "aloe-data-structures")]
    pub fn get_message_for_unexpected_reply(&mut self) -> String {
        
        todo!();
        /*
            return TRANS ("Unexpected or corrupted reply from XYZ").replace ("XYZ", getWebsiteName()) + "...\n\n"
                        + TRANS("Please try again in a few minutes, and contact us for support if this message appears again.");
        */
    }
    
    /**
      | Contacts the webserver and attempts
      | to perform a registration with the given
      | user details.
      | 
      | The return value will either be a success,
      | or a failure with an error message from
      | the server, so you should show this message
      | to your user.
      | 
      | Because this method blocks while it
      | contacts the server, you must run it
      | on a background thread, not on the message
      | thread. For an easier way to create a
      | GUI to do the unlocking, see OnlineUnlockForm.
      |
      */
    #[cfg(feature = "aloe-data-structures")]
    pub fn attempt_webserver_unlock(
        &mut self, 
        email:    &String,
        password: &String

    ) -> OnlineUnlockStatusUnlockResult {
        
        todo!();
        /*
            // This method will block while it contacts the server, so you must run it on a background thread!
        jassert (! MessageManager::getInstance()->isThisTheMessageThread());

        auto reply = readReplyFromWebserver (email, password);

        DBG ("Reply from server: " << reply);

        if (auto xml = parseXML (reply))
            return handleXmlReply (*xml);

        return handleFailedConnection();
        */
    }
}
