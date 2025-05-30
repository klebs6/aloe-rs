crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_posix_IPAddress.h]

pub struct InterfaceInfo
{
    interface_address: IPAddress,
    broadcast_address: IPAddress,
}

impl PartialEq<InterfaceInfo> for InterfaceInfo {
    
    fn eq(&self, other: &InterfaceInfo) -> bool {
        todo!();
        /*
            return lhs.interfaceAddress == rhs.interfaceAddress
            && lhs.broadcastAddress == rhs.broadcastAddress;
        */
    }
}

impl Eq for InterfaceInfo {}

#[cfg(not(ALOE_WASM))]
pub fn make_address6(addr_in: *const libc::sockaddr_in6) -> IPAddress {
    
    todo!();
    /*
        if (addr_in == nullptr)
            return {};

        auto addr = addr_in->sin6_addr;

        IPAddressByteUnion temp;
        uint16 arr[8];

        for (int i = 0; i < 8; ++i) // Swap bytes from network to host order
        {
            temp.split[0] = addr.s6_addr[i * 2 + 1];
            temp.split[1] = addr.s6_addr[i * 2];

            arr[i] = temp.combined;
        }

        return IPAddress (arr);
    */
}

#[cfg(not(ALOE_WASM))]
pub fn make_address(addr_in: *const libc::sockaddr_in) -> IPAddress {
    
    todo!();
    /*
        if (addr_in->sin_addr.s_addr == INADDR_NONE)
            return {};

        return IPAddress (ntohl (addr_in->sin_addr.s_addr));
    */
}

#[cfg(not(ALOE_WASM))]
pub fn populate_interface_info(
        ifa:            *mut ifaddrs,
        interface_info: &mut InterfaceInfo) -> bool {
    
    todo!();
    /*
        if (ifa->ifa_addr != nullptr)
        {
            if (ifa->ifa_addr->sa_family == AF_INET)
            {
                auto interfaceAddressInfo = unalignedPointerCast<sockaddr_in*> (ifa->ifa_addr);
                auto broadcastAddressInfo = unalignedPointerCast<sockaddr_in*> (ifa->ifa_dstaddr);

                if (interfaceAddressInfo->sin_addr.s_addr != INADDR_NONE)
                {
                    interfaceInfo.interfaceAddress = makeAddress (interfaceAddressInfo);
                    interfaceInfo.broadcastAddress = makeAddress (broadcastAddressInfo);
                    return true;
                }
            }
            else if (ifa->ifa_addr->sa_family == AF_INET6)
            {
                interfaceInfo.interfaceAddress = makeAddress (unalignedPointerCast<sockaddr_in6*> (ifa->ifa_addr));
                interfaceInfo.broadcastAddress = makeAddress (unalignedPointerCast<sockaddr_in6*> (ifa->ifa_dstaddr));
                return true;
            }
        }

        return false;
    */
}

pub fn get_all_interface_info() -> Vec<InterfaceInfo> {
    
    todo!();
    /*
        Vec<InterfaceInfo> interfaces;

       #if ALOE_WASM
        // TODO
       #else
        struct ifaddrs* ifaddr = nullptr;

        if (getifaddrs (&ifaddr) != -1)
        {
            for (auto* ifa = ifaddr; ifa != nullptr; ifa = ifa->ifa_next)
            {
                InterfaceInfo i;

                if (populateInterfaceInfo (ifa, i))
                    interfaces.addIfNotAlreadyThere (i);
            }

            freeifaddrs (ifaddr);
        }
       #endif

        return interfaces;
    */
}
