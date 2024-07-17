crate::ix!();

#[cfg(target_os="android")]
impl IntfIID for SLObjectItf_ {
    lazy_static!{
        /*
        static SLInterfaceID_ iid
            { 0x79216360, 0xddd7, 0x11db, 0xac16, {0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b} };
        */
    }
}

#[cfg(target_os="android")]
impl IntfIID for SLEngineItf_ {
    lazy_static!{
        /*
        static SLInterfaceID_ iid
            { 0x8d97c260, 0xddd4, 0x11db, 0x958f, {0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b} };
        */
    }
}

#[cfg(target_os="android")]
impl IntfIID for SLOutputMixItf_ {
    lazy_static!{
        /*
        static SLInterfaceID_ iid
            { 0x97750f60, 0xddd7, 0x11db, 0x92b1, {0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b} };
        */
    }
}

#[cfg(target_os="android")]
impl IntfIID for SLPlayItf_ {
    lazy_static!{
        /*
        static SLInterfaceID_ iid
             { 0xef0bd9c0, 0xddd7, 0x11db, 0xbf49, {0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b} };
        */
    }
}

#[cfg(target_os="android")]
impl IntfIID for SLRecordItf_ {
    lazy_static!{
        /*
        static SLInterfaceID_ iid
            { 0xc5657aa0, 0xdddb, 0x11db, 0x82f7, {0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b} };
        */
    }
}

#[cfg(target_os="android")]
impl IntfIID for SLAndroidSimpleBufferQueueItf_ {
    lazy_static!{
        /*
        static SLInterfaceID_ iid
            { 0x198e4940, 0xc5d7, 0x11df, 0xa2a6, {0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b} };
        */
    }
}

#[cfg(target_os="android")]
impl IntfIID for SLAndroidConfigurationItf_ {
    lazy_static!{
        /*
        static SLInterfaceID_ iid
             { 0x89f6a7e0, 0xbeac, 0x11df, 0x8b5c, {0x00, 0x02, 0xa5, 0xd5, 0xc5, 0x1b} };
        */
    }
}

