crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/SystemInfoDemo.h]

pub fn get_mac_address_list() -> String {
    
    todo!();
    /*
        String addressList;

        for (auto& addr : MACAddress::getAllAddresses())
            addressList << addr.toString() << newLine;

        return addressList;
    */
}

pub fn get_file_system_roots() -> String {
    
    todo!();
    /*
        Vec<File> roots;
        File::findFileSystemRoots (roots);

        StringArray rootList;
        for (auto& r : roots)
            rootList.add (r.getFullPathName());

        return rootList.joinIntoString (", ");
    */
}

pub fn get_ip_address_list() -> String {
    
    todo!();
    /*
        String addressList;

        for (auto& addr : IPAddress::getAllAddresses())
            addressList << "   " << addr.toString() << newLine;

        return addressList;
    */
}

