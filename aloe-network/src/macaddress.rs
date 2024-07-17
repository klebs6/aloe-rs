crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_MACAddress.h]

/**
  | Represents a MAC network card adapter
  | address ID.
  | 
  | @tags{Core}
  |
  */
pub struct MACAddress {
    address: [u8; 6],
}

impl PartialEq<MACAddress> for MACAddress {
    
    #[inline] fn eq(&self, other: &MACAddress) -> bool {
        todo!();
        /*
            return memcmp (address, other.address, sizeof (address)) == 0;
        */
    }
}

impl Eq for MACAddress {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_MACAddress.cpp]
impl Default for MACAddress {

    /**
      | Creates a null address (00-00-00-00-00-00).
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            zeromem (address, sizeof (address));
        */
    }
}

impl MACAddress {
    
    /**
      | Populates a list of the MAC addresses
      | of all the available network cards.
      |
      */
    pub fn find_all_addresses(&mut self, result: &mut Vec<MACAddress>)  {
        
        todo!();
        /*
            #if ALOE_BSD
        struct ifaddrs* addrs = nullptr;

        if (getifaddrs (&addrs) != -1)
        {
            for (auto* i = addrs; i != nullptr; i = i->ifa_next)
            {
                if (i->ifa_addr->sa_family == AF_LINK)
                {
                    struct sockaddr_dl* sdl = (struct sockaddr_dl*) i->ifa_addr;
                    MACAddress ma ((const uint8*) (sdl->sdl_data + sdl->sdl_nlen));

                    if (! ma.isNull())
                        result.addIfNotAlreadyThere (ma);
                }
            }

            freeifaddrs (addrs);
        }
       #else
        auto s = socket (AF_INET, SOCK_DGRAM, 0);

        if (s != -1)
        {
            struct ifaddrs* addrs = nullptr;

            if (getifaddrs (&addrs) != -1)
            {
                for (auto* i = addrs; i != nullptr; i = i->ifa_next)
                {
                    struct ifreq ifr;
                    strcpy (ifr.ifr_name, i->ifa_name);
                    ifr.ifr_addr.sa_family = AF_INET;

                    if (ioctl (s, SIOCGIFHWADDR, &ifr) == 0)
                    {
                        MACAddress ma ((const uint8*) ifr.ifr_hwaddr.sa_data);

                        if (! ma.isNull())
                            result.addIfNotAlreadyThere (ma);
                    }
                }

                freeifaddrs (addrs);
            }

            ::close (s);
        }
       #endif
        */
    }

    /**
      | Returns a pointer to the 6 bytes that
      | make up this address.
      |
      */
    pub fn get_bytes(&self) -> *const u8 {
        
        todo!();
        /*
            return address;
        */
    }

    /**
      | Creates a copy of another address.
      |
      */
    pub fn new_from_other(other: &MACAddress) -> Self {
    
        todo!();
        /*
            memcpy (address, other.address, sizeof (address));
        */
    }
    
    /**
      | Creates a copy of another address.
      |
      */
    pub fn assign_from(&mut self, other: &MACAddress) -> &mut MACAddress {
        
        todo!();
        /*
            memcpy (address, other.address, sizeof (address));
        return *this;
        */
    }
    
    /**
      | Creates an address from 6 bytes.
      |
      */
    pub fn new_from_bytes(bytes: [u8; 6]) -> Self {
    
        todo!();
        /*
            memcpy (address, bytes, sizeof (address));
        */
    }
    
    /**
      | Creates an address from a hex string.
      | 
      | If the string isn't a 6-byte hex value,
      | this will just default-initialise
      | the object.
      |
      */
    pub fn new_from_address_string(address_string: &str) -> Self {
    
        todo!();
        /*


            MemoryBlock hex;
        hex.loadFromHexString (addressString);

        if (hex.getSize() == sizeof (address))
            memcpy (address, hex.getData(), sizeof (address));
        else
            zeromem (address, sizeof (address));
        */
    }

    /**
      | Returns a dash-separated string in
      | the form "11-22-33-44-55-66"
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return toString ("-");
        */
    }

    /**
      | Returns a hex string of this address,
      | using a custom separator between each
      | byte.
      |
      */
    pub fn to_string_with_separator(&self, separator: &str) -> String {
        
        todo!();
        /*
            String s;

        for (size_t i = 0; i < sizeof (address); ++i)
        {
            s << String::toHexString ((int) address[i]).paddedLeft ('0', 2);

            if (i < sizeof (address) - 1)
                s << separator;
        }

        return s;
        */
    }

    /**
      | Returns the address in the lower 6 bytes
      | of an int64.
      | 
      | This uses a little-endian arrangement,
      | with the first byte of the address being
      | stored in the least-significant byte
      | of the result value.
      |
      */
    pub fn to_int64(&self) -> i64 {
        
        todo!();
        /*
            int64 n = 0;

        for (int i = (int) sizeof (address); --i >= 0;)
            n = (n << 8) | address[i];

        return n;
        */
    }

    /**
      | Returns a list of the MAC addresses of
      | all the available network cards.
      |
      */
    pub fn get_all_addresses(&mut self) -> Vec<MACAddress> {
        
        todo!();
        /*
            Vec<MACAddress> addresses;
        findAllAddresses (addresses);
        return addresses;
        */
    }

    /**
      | Returns true if this address is null
      | (00-00-00-00-00-00).
      |
      */
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return toInt64() == 0;
        */
    }
}
