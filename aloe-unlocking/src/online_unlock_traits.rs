crate::ix!();

pub trait GetProductID {

    /**
      | This must return your product's ID,
      | as allocated by the store.
      |
      */
    fn get_productid(&mut self) -> String;
}

pub trait DoesProductIDMatch {

    /**
      | This must check whether a product ID
      | string that the server returned is OK
      | for unlocking the current app.
      |
      */
    fn does_product_id_match(&mut self, returned_id_from_server: &String) -> bool;
}

pub trait GetPublicKey {

    /**
      | This must return the RSA public key for
      | authenticating responses from the
      | server for this app. You can get this
      | key from your marketplace account page.
      |
      */
    fn get_public_key(&mut self) -> RSAKey;
}

pub trait SaveState {

    /**
      | This method must store the given string
      | somewhere in your app's persistent
      | properties, so it can be retrieved later
      | by getState().
      |
      */
    fn save_state(&mut self, _0: &String);
}

pub trait GetState {

    /**
      | This method must retrieve the last state
      | that was provided by the saveState method.
      | 
      | On first-run, it should just return
      | an empty string.
      |
      */
    fn get_state(&mut self) -> String;
}

pub trait GetWebsiteName {

    /**
      | Returns the name of the web-store website,
      | not for communication, but for presenting
      | to the user.
      |
      */
    fn get_website_name(&mut self) -> String;
}

pub trait GetServerAuthenticationURL {

    /**
      | Returns the Url of the authentication
      | API.
      |
      */
    fn get_server_authenticationurl(&mut self) -> Url;
}

pub trait ReadReplyFromWebserver {

    /**
      | Subclasses that talk to a particular
      | web-store will implement this method
      | to contact their webserver and attempt
      | to unlock the current machine for the
      | given username and password. The return
      | value is the XML text from the server
      | which contains error information and/or
      | the encrypted keyfile.
      |
      */
    fn read_reply_from_webserver(&mut self, 
            email:    &String,
            password: &String) -> String;
}

pub trait GetLocalMachineIDs {

    /**
      | Returns a list of strings, any of which
      | should be unique to this physical computer.
      | 
      | When testing whether the user is allowed
      | to use the product on this machine, this
      | list of tokens is compared to the ones
      | that were stored on the webserver.
      | 
      | The default implementation of this
      | method will simply call
      | 
      | MachineIDUtilities::getLocalMachineIDs(),
      | which provides a default version of
      | this functionality.
      |
      */
    fn get_local_machine_ids(&mut self) -> StringArray;
}

pub trait UserCancelled {

    /**
      | This method will be called if the user
      | cancels the connection to the webserver
      | by clicking the cancel button in OnlineUnlockForm::OverlayComp.
      | 
      | The default implementation of this
      | method does nothing but you should use
      | it to cancel any WebInputStreams that
      | may be connecting.
      |
      */
    fn user_cancelled(&mut self);
}

pub trait GetMessageForConnectionFailure {
    fn get_message_for_connection_failure(&mut self, is_internet_connection_working: bool) -> String;
}

pub trait GetMessageForUnexpectedReply {
    fn get_message_for_unexpected_reply(&mut self) -> String;
}

