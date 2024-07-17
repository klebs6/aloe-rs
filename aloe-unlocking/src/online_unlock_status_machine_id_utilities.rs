crate::ix!();

/**
  | This class contains some utility functions
  | that might help with machine ID generation.
  |
  */
pub struct OnlineUnlockStatusMachineIDUtilities {

}

impl OnlineUnlockStatusMachineIDUtilities {

    /**
      | Returns a character that represents
      | the current OS.
      | 
      | E.g. 'M' for Mac, 'W' for windows, etc
      |
      */
    pub fn get_platform_prefix(&mut self) -> u8 {
        
        todo!();
        /*
            #if ALOE_MAC
        return 'M';
       #elif ALOE_WINDOWS
        return 'W';
       #elif ALOE_LINUX
        return 'L';
       #elif ALOE_BSD
        return 'B';
       #elif ALOE_IOS
        return 'I';
       #elif ALOE_ANDROID
        return 'A';
       #endif
        */
    }
    
    /**
      | Returns an encoded hash string from
      | the given input string, prefixing it
      | with a letter to represent the current
      | OS type.
      |
      */
    pub fn get_encoded_id_string(&mut self, input: &String) -> String {
        
        todo!();
        /*
            auto platform = String::charToString (static_cast<aloe_wchar> (getPlatformPrefix()));

        return platform + MD5 ((input + "salt_1" + platform).toUTF8())
                            .toHexString().substring (0, 9).toUpperCase();
        */
    }
    
    /**
      | Utility function that you may want to
      | use in your machine-ID generation code.
      | 
      | This adds an ID string to the given array
      | which is a hash of the filesystem ID of
      | the given file.
      |
      */
    pub fn add_file_id_to_list(&mut self, 
        ids: &mut StringArray,
        f:   &File) -> bool {
        
        todo!();
        /*
            if (auto num = f.getFileIdentifier())
        {
            ids.add (getEncodedIDString (String::toHexString ((int64) num)));
            return true;
        }

        return false;
        */
    }
    
    /**
      | Utility function that you may want to
      | use in your machine-ID generation code.
      | 
      | This adds some ID strings to the given
      | array which represent each MAC address
      | of the machine.
      |
      */
    pub fn add_mac_addresses_to_list(&mut self, ids: &mut StringArray)  {
        
        todo!();
        /*
            for (auto& address : MACAddress::getAllAddresses())
            ids.add (getEncodedIDString (address.toString()));
        */
    }
    
    /**
      | This method calculates some machine
      | IDs based on things like network
      | 
      | MAC addresses, hard-disk IDs, etc,
      | but if you want, you can overload it to
      | generate your own list of IDs.
      | 
      | The IDs that are returned should be short
      | alphanumeric strings without any punctuation
      | characters. Since users may need to
      | type them, case is ignored when comparing
      | them.
      | 
      | -----------
      | @note
      | 
      | the first item in the list is considered
      | to be the "main" ID, and this will be the
      | one that is displayed to the user and
      | registered with the marketplace webserver.
      | Subsequent IDs are just used as fallback
      | to avoid false negatives when checking
      | for registration on machines which
      | have had hardware added/removed since
      | the product was first registered.
      |
      */
    pub fn get_local_machine_ids(&mut self) -> StringArray {
        
        todo!();
        /*
            auto identifiers = SystemStats::getDeviceIdentifiers();

        for (auto& identifier : identifiers)
            identifier = getEncodedIDString (identifier);

        return identifiers;
        */
    }
}
