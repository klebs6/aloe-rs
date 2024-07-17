crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/misc/aloe_Functional.h]

/// Some helper methods for checking a callable object before invoking with the specified
/// arguments.
/// 
/// If the object is an Option with a function, it will check for None before calling. For
/// a callable object, it will invoke the function call operator.
///
pub struct NullCheckedInvocation;

impl NullCheckedInvocation {
    pub fn invoke<Callable, Args>(func: Option<Callable>, args: Args)
    where
        Callable: FnOnce(Args),
    {
        if let Some(func) = func {
            func(args);
        }
    }
    
    // This is just an example if you also wanted a version
    // that doesn't handle the "null" check, similar to your second 
    // `invoke` in C++
    pub fn just_invoke<Callable, Args>(func: Callable, args: Args)
    where
        Callable: FnOnce(Args),
    {
        func(args);
    }
}
