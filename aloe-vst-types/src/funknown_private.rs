crate::ix!();

#[inline(always)] pub fn _funknown_iid_equal(
        iid1: *const c_void,
        iid2: *const c_void) -> bool {
    
    todo!();
        /*
            const uint64* p1 = reinterpret_cast<const uint64*> (iid1);
        const uint64* p2 = reinterpret_cast<const uint64*> (iid2);
        return p1[0] == p2[0] && p1[1] == p2[1];
        */
}

#[PLUGIN_API]
pub fn _funknown_atomic_add(
        value:  &mut i32,
        amount: i32) -> i32 {
    
    todo!();
        /*
            #if SMTG_USE_STDATOMIC_H
        return atomic_fetch_add (reinterpret_cast<atomic_int_least32_t*> (&var), d) + d;
    #else
    #if SMTG_OS_WINDOWS
    #ifdef __MINGW32__
        return InterlockedExchangeAdd (reinterpret_cast<long volatile*>(&var), d) + d;
    #else
        return InterlockedExchangeAdd ((LONG*)&var, d) + d;
    #endif
    #elif SMTG_OS_MACOS
        return OSAtomicAdd32Barrier (d, (int32_t*)&var);
    #elif defined(__ANDROID__)
        return atomic_fetch_add ((atomic_int*)&var, d) + d;
    #elif SMTG_OS_LINUX
        __gnu_cxx::__atomic_add (&var, d);
        return var;
    #else
    #warning implement me!
        var += d;
        return var;
    #endif
    #endif
        */
}

#[cfg(SMTG_CPP11_STDLIBSUPPORT)]
lazy_static!{
    /*
    template <typename T>
    struct Void : std::false_type
    {
        using Type = void;
    };

    template <typename T>
    using VoidT = typename Void<T>::Type;
    */
}

/**
  | This type trait detects if a class has
  | an @c iid member variable. It is used
  | to detect if the FUID and DECLARE_CLASS_IID
  | method or the SKI::UID method is used.
  |
  */
#[cfg(SMTG_CPP11_STDLIBSUPPORT)]
lazy_static!{
    /*
    template <typename T, typename U = void>
        struct HasIIDType : std::false_type
        {
        };

        template <typename T>
        struct HasIIDType<T, FUnknownPrivate::VoidT<typename T::IID>> : std::true_type
        {
        };
    */
}
