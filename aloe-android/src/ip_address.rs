crate::ix!();

#[cfg(target_os="android")]
impl IPAddress {
    
    #[cfg(__ANDROID_API___LT_24)]
    pub fn find_all_addresses(&mut self, 
        result:       &mut Vec<IPAddress>,
        include_ipv6: bool)  {
        
        todo!();
        /*
            for (auto& a : findIPAddresses())
            result.add (a.interfaceAddress);
        */
    }
    
    #[cfg(__ANDROID_API___LT_24)]
    pub fn get_interface_broadcast_address(&mut self, address: &IPAddress) -> IPAddress {
        
        todo!();
        /*
            for (auto& a : findIPAddresses())
            if (a.interfaceAddress == address)
                return a.broadcastAddress;

        return {};
        */
    }
}

#[cfg(__ANDROID_API___LT_24)]
pub fn find_ip_addresses() -> Vec<InterfaceInfo> {
    
    todo!();
    /*
        auto dummySocket = socket (AF_INET, SOCK_DGRAM, 0); // a dummy socket to execute the IO control

        if (dummySocket < 0)
            return {};

        auto result = findIPAddresses (dummySocket);
        ::close (dummySocket);
        return result;
    */
}

#[cfg(__ANDROID_API___LT_24)]
pub struct InterfaceInfo {
    interface_address: IPAddress,
    broadcast_address: IPAddress,
}

#[cfg(__ANDROID_API___LT_24)]
pub fn find_ip_addresses(dummy_socket: i32) -> Vec<InterfaceInfo> {
    
    todo!();
    /*
        ifconf cfg;
        HeapBlock<char> buffer;
        int bufferSize = 1024;

        do
        {
            bufferSize *= 2;
            buffer.calloc (bufferSize);

            cfg.ifc_len = bufferSize;
            cfg.ifc_buf = buffer;

            if (ioctl (dummySocket, SIOCGIFCONF, &cfg) < 0 && errno != EINVAL)
                return {};

        } while (bufferSize < cfg.ifc_len + 2 * (int) (IFNAMSIZ + sizeof (struct sockaddr_in6)));

        Vec<InterfaceInfo> result;

        for (size_t i = 0; i < (size_t) cfg.ifc_len / (size_t) sizeof (struct ifreq); ++i)
        {
            auto& item = cfg.ifc_req[i];

            if (item.ifr_addr.sa_family == AF_INET)
            {
                InterfaceInfo info;
                info.interfaceAddress = makeAddress (reinterpret_cast<const sockaddr_in*> (&item.ifr_addr));

                if (! info.interfaceAddress.isNull())
                {
                    if (ioctl (dummySocket, SIOCGIFBRDADDR, &item) == 0)
                        info.broadcastAddress = makeAddress (reinterpret_cast<const sockaddr_in*> (&item.ifr_broadaddr));

                    result.add (info);
                }
            }
            else if (item.ifr_addr.sa_family == AF_INET6)
            {
                // TODO: IPv6
            }
        }

        return result;
    */
}

/**
  | Android support for getifadds was added
  | in Android 7.0 (API 24) so the posix implementation
  | does not apply
  |
  */
#[cfg(__ANDROID_API___LT_24)]
pub fn make_address(addr_in: *const libc::sockaddr_in) -> IPAddress {
    
    todo!();
    /*
        if (addr_in->sin_addr.s_addr == INADDR_NONE)
            return {};

        return IPAddress (ntohl (addr_in->sin_addr.s_addr));
    */
}
