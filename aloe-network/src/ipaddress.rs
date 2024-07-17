crate::ix!();

/**
  | Union used to split a 16-bit unsigned
  | integer into 2 8-bit unsigned integers
  | or vice-versa
  |
  */
pub union IPAddressByteUnion
{
    combined: u16,
    split:    [u8; 2],
}

pub fn zero_unused_bytes(address: *mut u8)  {
    
    todo!();
    /*
        for (int i = 4; i < 16; ++i)
            address[i] = 0;
    */
}

pub fn remove_port(adr: &String) -> String {
    
    todo!();
    /*
        if (adr.containsAnyOf ("[]"))
            return adr.fromFirstOccurrenceOf ("[", false, true).upToLastOccurrenceOf ("]", false, true);

        if (adr.indexOf (":") == adr.lastIndexOf (":"))
            return adr.upToLastOccurrenceOf (":", false, true);

        return adr;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_IPAddress.h]

/**
  | Represents an IP address.
  | 
  | @tags{Core}
  |
  */
pub struct IPAddress {
    
    /**
      | The elements of the IP address.
      |
      */
    address: [u8; 16],
    is_ipv6: bool, // default = false
}

impl PartialEq<IPAddress> for IPAddress {
    
    #[inline] fn eq(&self, other: &IPAddress) -> bool {
        todo!();
        /*
            return compare (other) == 0;
        */
    }
}

impl Eq for IPAddress {}

impl Ord for IPAddress {
    
    #[inline] fn cmp(&self, other: &IPAddress) -> std::cmp::Ordering {
        todo!();
        /*
            return compare (other) <  0;
        */
    }
}

impl PartialOrd<IPAddress> for IPAddress {

    #[inline] fn partial_cmp(&self, other: &IPAddress) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_IPAddress.cpp]
impl Default for IPAddress {

    /**
      | Creates a null address - 0.0.0.0 (IPv4)
      | or ::, (IPv6)
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            for (int i = 0; i < 16; ++i)
            address[i] = 0;
        */
    }
}

impl IPAddress {

    /**
      | Populates a list of all the IP addresses
      | that this machine is using.
      |
      */
    pub fn find_all_addresses(
        &mut self, 
        result:       &mut Vec<IPAddress>,
        include_ipv6: Option<bool>

    ) {
        
        let include_ipv6: bool = include_ipv6.unwrap_or(false);

        todo!();
        /*
            for (auto& i : getAllInterfaceInfo())
            if (includeIPv6 || ! i.interfaceAddress.isIPv6)
                result.addIfNotAlreadyThere (i.interfaceAddress);
        */
    }

    /**
      | Converts an IPv4 address to an IPv4-mapped
      | IPv6 address.
      |
      */
    pub fn convert_ipv4_address_to_ipv4_mapped(address_to_map: &IPAddress) -> IPAddress {
        
        todo!();
        /*
        
        */
    }

    /**
      | If the IPAdress is the address of an interface
      | on the machine, returns the associated
      | broadcast address.
      | 
      | If the address is not an interface, it
      | will return a null address.
      |
      */
    pub fn get_interface_broadcast_address(&mut self, interface_address: &IPAddress) -> IPAddress {
        
        todo!();
        /*
            for (auto& i : getAllInterfaceInfo())
            if (i.interfaceAddress == interfaceAddress)
                return i.broadcastAddress;

        return {};
        */
    }
    
    /**
      | Creates an IPv4 or IPv6 address by reading
      | 4 or 16 bytes from an array.
      | 
      | -----------
      | @param bytes
      | 
      | The array containing the bytes to read.
      | ----------
      | @param IPv6
      | 
      | if true indicates that 16 bytes should
      | be read instead of 4.
      |
      */
    pub fn new_from_bytes(
        bytes: &[u8],
        pv6:   Option<bool>

    ) -> Self {

        let pv6: bool = pv6.unwrap_or(false);
    
        todo!();
        /*
        : isi_pv6(IPv6),

            for (int i = 0; i < (isIPv6 ? 16 : 4); ++i)
            address[i] = bytes[i];

        if (! isIPv6)
            zeroUnusedBytes (address);
        */
    }
    
    /**
      | Creates an IPv6 address from an array
      | of 8 16-bit integers
      | 
      | -----------
      | @param bytes
      | 
      | The array containing the bytes to read.
      |
      */
    pub fn new_from_8_16_bit_integer_array(bytes: [u16; 8]) -> Self {
    
        todo!();
        /*
        : isi_pv6(true),

            IPAddressByteUnion temp;

        for (int i = 0; i < 8; ++i)
        {
            temp.combined = bytes[i];

            address[i * 2]     = temp.split[0];
            address[i * 2 + 1] = temp.split[1];
        }
        */
    }
    
    /**
      | Creates an IPv4 address from 4 bytes.
      |
      */
    pub fn new_from_four_bytes(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8) -> Self {
    
        todo!();
        /*
        : isi_pv6(false),

            address[0] = a0;  address[1] = a1;
        address[2] = a2;  address[3] = a3;

        zeroUnusedBytes (address);
        */
    }
    
    /**
      | Creates an IPv6 address from 8 16-bit
      | integers
      |
      */
    pub fn new_from_8_16_bit_integers(
        a1: u16,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: u16,
        a6: u16,
        a7: u16,
        a8: u16) -> Self {
    
        todo!();
        /*
        : isi_pv6(true),

            uint16 array[8] = { a1, a2, a3, a4, a5, a6, a7, a8 };

        IPAddressByteUnion temp;

        for (int i = 0; i < 8; ++i)
        {
            temp.combined = array[i];
            address[i * 2]     = temp.split[0];
            address[i * 2 + 1] = temp.split[1];
        }
        */
    }
    
    /**
      | Creates an IPv4 address from a packed
      | 32-bit integer, where the MSB is the
      | first number in the address, and the
      | LSB is the last.
      |
      */
    pub fn new_from_packed_32_bit_int(n: u32) -> Self {
    
        todo!();
        /*
        : isi_pv6(false),

            address[0] = static_cast<uint8> (n >> 24);
        address[1] = static_cast<uint8> ((n >> 16) & 255);
        address[2] = static_cast<uint8> ((n >> 8) & 255);
        address[3] = static_cast<uint8> ((n & 255));

        zeroUnusedBytes (address);
        */
    }
    
    /**
      | Returns whether the address contains
      | the null address (e.g. 0.0.0.0).
      |
      */
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            for (int i = 0; i < 16; ++i)
            if (address[i] != 0)
                return false;

        return true;
        */
    }

    /**
      | Parses a string IP address of the form
      | "1.2.3.4" (IPv4) or "1:2:3:4:5:6:7:8"
      | (IPv6).
      |
      */
    pub fn new(adr: &String) -> Self {
    
        todo!();
        /*


            auto ipAddress = removePort (adr);

        isIPv6 = ipAddress.contains (":");

        if (! isIPv6)
        {
            auto tokens = StringArray::fromTokens (ipAddress, ".", {});

            for (int i = 0; i < 4; ++i)
                address[i] = (uint8) tokens[i].getIntValue();

            zeroUnusedBytes (address);
        }
        else
        {
            auto tokens = StringArray::fromTokens (ipAddress, ":", {});

            if (tokens.contains ({})) // if :: shorthand has been used
            {
                auto idx = tokens.indexOf ({});
                tokens.set (idx, "0");
                tokens.removeEmptyStrings();

                // mapped IPv4 address will be treated as a single token, so pad the end of the StringArray
                if (tokens[tokens.size() - 1].containsChar ('.'))
                    tokens.add ({});

                while (tokens.size() < 8)
                    tokens.insert (idx, "0");
            }

            for (int i = 0; i < 8; ++i)
            {
                if (i == 6 && isIPv4MappedAddress (IPAddress (address, true)))
                {
                    IPAddress v4Address (tokens[i]);

                    address[12] = v4Address.address[0];
                    address[13] = v4Address.address[1];
                    address[14] = v4Address.address[2];
                    address[15] = v4Address.address[3];

                    break;
                }

                IPAddressByteUnion temp;
                temp.combined = (uint16) CharacterFunctions::HexParser<int>::parse (tokens[i].getCharPointer());

                address[i * 2]     = temp.split[0];
                address[i * 2 + 1] = temp.split[1];
            }
        }
        */
    }

    /**
      | Returns a dot- or colon-separated string
      | in the form "1.2.3.4" (IPv4) or "1:2:3:4:5:6:7:8"
      | (IPv6).
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            if (! isIPv6)
        {
            String s ((int) address[0]);

            for (int i = 1; i < 4; ++i)
                s << '.' << (int) address[i];

            return s;
        }

        IPAddressByteUnion temp;

        temp.split[0] = address[0];
        temp.split[1] = address[1];

        auto addressString = String::toHexString (temp.combined);

        for (int i = 1; i < 8; ++i)
        {
            temp.split[0] = address[i * 2];
            temp.split[1] = address[i * 2 + 1];

            addressString << ':' << String::toHexString (temp.combined);
        }

        return getFormattedAddress (addressString);
        */
    }

    /**
      | Compares this IPAddress with another.
      | 
      | -----------
      | @return
      | 
      | 0 if the two addresses are identical,
      | negative if this address is smaller
      | than the other one, or positive if is
      | greater.
      |
      */
    pub fn compare(&self, other: &IPAddress) -> i32 {
        
        todo!();
        /*
            if (isIPv6 != other.isIPv6)
        {
            if (isIPv6)
            {
                if (isIPv4MappedAddress (*this))
                    return convertIPv4MappedAddressToIPv4 (*this).compare (other);

                return 1;
            }

            if (isIPv4MappedAddress (other))
                return compare (convertIPv4MappedAddressToIPv4 (other));

            return -1;
        }

        for (int i = 0; i < (isIPv6 ? 16 : 4); ++i)
        {
            if (address[i] > other.address[i])  return 1;
            if (address[i] < other.address[i])  return -1;
        }

        return 0;
        */
    }

    /**
      | Returns an IP address meaning "any",
      | equivalent to 0.0.0.0 (IPv4) or ::,
      | (IPv6)
      |
      */
    pub fn any(&mut self) -> IPAddress {
        
        todo!();
        /*
            return IPAddress();
        */
    }

    /**
      | Returns an IPv4 address meaning "broadcast"
      | (255.255.255.255)
      |
      */
    pub fn broadcast(&mut self) -> IPAddress {
        
        todo!();
        /*
            return IPAddress (255, 255, 255, 255);
        */
    }

    /**
      | Returns an IPv4 or IPv6 address meaning
      | "localhost", equivalent to 127.0.0.1
      | (IPv4) or ::1 (IPv6)
      |
      */
    pub fn local(&mut self, pv6: Option<bool>) -> IPAddress {

        let pv6: bool = pv6.unwrap_or(false);
        
        todo!();
        /*
            return IPv6 ? IPAddress (0, 0, 0, 0, 0, 0, 0, 1)
                                                                    : IPAddress (127, 0, 0, 1);
        */
    }

    /**
      | Returns a formatted version of the provided
      | IPv6 address conforming to RFC 5952
      | with leading zeros suppressed, lower
      | case characters, and double-colon
      | notation used to represent contiguous
      | 16-bit fields of zeros.
      | 
      | -----------
      | @param unformattedAddress
      | 
      | the IPv6 address to be formatted
      |
      */
    pub fn get_formatted_address(&mut self, unformatted_address: &String) -> String {
        
        todo!();
        /*
            jassert (unformattedAddress.contains (":") && ! unformattedAddress.contains ("::")); // needs to be an unformatted IPv6 address!

        auto portString    = unformattedAddress.fromFirstOccurrenceOf ("]", false, true);
        auto addressString = unformattedAddress.dropLastCharacters (portString.length()).removeCharacters ("[]");

        auto tokens = StringArray::fromTokens (addressString, ":", {});

        int numZeros = 0;
        int numZerosTemp = 0;
        bool isFirst = false;
        bool isLast = false;

        for (int i = 0; i < tokens.size(); ++i)
        {
            auto& t = tokens.getReference (i);

            if (t.getHexValue32() == 0x0000)
            {
                ++numZeros;

                if (i == 0)
                    isFirst = true;
                else if (i == tokens.size() - 1 && numZeros > numZerosTemp)
                    isLast = true;

                if (t.length() > 1)
                    addressString = addressString.replace (String::repeatedString ("0", t.length()), "0");

                if (isFirst && numZerosTemp != 0 && numZeros > numZerosTemp)
                    isFirst = false;
            }
            else
            {
                addressString = addressString.replace (t, t.trimCharactersAtStart ("0").toLowerCase());

                if (numZeros > 0)
                {
                    if (numZeros > numZerosTemp)
                        numZerosTemp = numZeros;

                    numZeros = 0;
                }
            }
        }

        if (numZerosTemp > numZeros)
            numZeros = numZerosTemp;

        if (numZeros > 1)
        {
            if (numZeros == tokens.size())
            {
                addressString = "::,";
            }
            else
            {
                auto zeroString = isFirst ? "0" + String::repeatedString (":0", numZeros - 1)
                                          : String::repeatedString (":0", numZeros);

                addressString = addressString.replaceFirstOccurrenceOf (zeroString, ":");

                if (isLast)
                    addressString << ':';
            }
        }

        if (portString.isNotEmpty())
            addressString = "[" + addressString + "]" + portString;

        return addressString;
        */
    }

    /**
      | Returns true if the given IP address
      | is an IPv4-mapped IPv6 address.
      |
      */
    pub fn isi_pv_4mapped_address(&mut self, mapped_address: &IPAddress) -> bool {
        
        todo!();
        /*
            if (! mappedAddress.isIPv6)
            return false;

        for (int i = 0; i < 10; ++i)
            if (mappedAddress.address[i] != 0)
                return false;

        if (mappedAddress.address[10] != 255 || mappedAddress.address[11] != 255)
            return false;

        return true;
        */
    }

    /**
      | Converts an IPv4-mapped IPv6 address
      | to an IPv4 address.
      | 
      | If the address is not IPv4-mapped, this
      | will return a null address.
      |
      */
    pub fn converti_pv_4mapped_address_toi_pv4(&mut self, mapped_address: &IPAddress) -> IPAddress {
        
        todo!();
        /*
            // The address that you're converting needs to be IPv6!
        jassert (mappedAddress.isIPv6);

        if (isIPv4MappedAddress (mappedAddress))
            return { mappedAddress.address[12], mappedAddress.address[13],
                     mappedAddress.address[14], mappedAddress.address[15] };

        return {};
        */
    }

    pub fn converti_pv_4address_toi_pv_4mapped(&mut self, address_to_map: &IPAddress) -> IPAddress {
        
        todo!();
        /*
            // The address that you're converting needs to be IPv4!
        jassert (! addressToMap.isIPv6);

        return { 0x0, 0x0, 0x0, 0x0, 0x0, 0xffff,
                static_cast<uint16> ((addressToMap.address[0] << 8) | addressToMap.address[1]),
                static_cast<uint16> ((addressToMap.address[2] << 8) | addressToMap.address[3]) };
        */
    }

    /**
      | Returns the first 'real' address for
      | the local machine.
      | 
      | Unlike local(), this will attempt to
      | find the machine's actual assigned
      | address rather than "127.0.0.1". If
      | there are multiple network cards, this
      | may return any of their addresses. If
      | it doesn't find any, then it'll return
      | local() as a fallback.
      |
      */
    pub fn get_local_address(&mut self, includei_pv6: Option<bool>) -> IPAddress {

        let includei_pv6: bool = includei_pv6.unwrap_or(false);
        
        todo!();
        /*
            auto addresses = getAllAddresses (includeIPv6);

        for (auto& a : addresses)
            if (a != local())
                return a;

        return local();
        */
    }

    /**
      | Populates a list of all the IP addresses
      | that this machine is using.
      |
      */
    pub fn get_all_addresses(&mut self, includei_pv6: Option<bool>) -> Vec<IPAddress> {

        let includei_pv6: bool = includei_pv6.unwrap_or(false);
        
        todo!();
        /*
            Vec<IPAddress> addresses;
        findAllAddresses (addresses, includeIPv6);
        return addresses;
        */
    }
}

#[test] fn ip_address_tests() {

    pub struct IPAddressTests {
        base: UnitTest,
    }

    impl Default for IPAddressTests {
        
        fn default() -> Self {
            todo!();
            /*
                : UnitTest ("IPAddress", UnitTestCategories::networking
            */
        }
    }

    impl IPAddressTests {

        pub fn run_test(&mut self)  {
            
            todo!();
            /*
                testConstructors();
                testFindAllAddresses();
                testFindBroadcastAddress();
            */
        }

        pub fn test_constructors(&mut self)  {
            
            todo!();
            /*
                beginTest ("constructors");

                // Default IPAdress should be null
                IPAddress defaultConstructed;
                expect (defaultConstructed.isNull());

                auto local = IPAddress::local();
                expect (! local.isNull());

                IPAddress ipv4{1, 2, 3, 4};
                expect (! ipv4.isNull());
                expect (! ipv4.isIPv6);
                expect (ipv4.toString() == "1.2.3.4");
            */
        }

        pub fn test_find_all_addresses(&mut self)  {
            
            todo!();
            /*
                beginTest ("find all addresses");

                Vec<IPAddress> ipv4Addresses;
                Vec<IPAddress> allAddresses;

                IPAddress::findAllAddresses (ipv4Addresses, false);
                IPAddress::findAllAddresses (allAddresses, true);

                expect (allAddresses.size() >= ipv4Addresses.size());

                for (auto& a : ipv4Addresses)
                {
                    expect (! a.isNull());
                    expect (! a.isIPv6);
                }

                for (auto& a : allAddresses)
                {
                    expect (! a.isNull());
                }
            */
        }

        pub fn test_find_broadcast_address(&mut self)  {
            
            todo!();
            /*
                beginTest ("broadcast addresses");

                Vec<IPAddress> addresses;

                // Only IPv4 interfaces have broadcast
                IPAddress::findAllAddresses (addresses, false);

                for (auto& a : addresses)
                {
                    expect (! a.isNull());

                    auto broadcastAddress = IPAddress::getInterfaceBroadcastAddress (a);

                    // If we retrieve an address, it should be an IPv4 address
                    if (! broadcastAddress.isNull())
                    {
                        expect (! a.isIPv6);
                    }
                }

                // Expect to fail to find a broadcast for this address
                IPAddress address{1, 2, 3, 4};
                expect (IPAddress::getInterfaceBroadcastAddress (address).isNull());
            */
        }
    }

    lazy_static!{
        /*
        static IPAddressTests iPAddressTests;
        */
    }
}
