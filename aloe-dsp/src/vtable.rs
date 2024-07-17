/*
crate::ix!();

pub struct Vtable<Ret, Args> {
    move_fn: unsafe fn(*mut (), *mut ()),
    call_fn: unsafe fn(*mut (), Args) -> Ret,
    clear_fn: unsafe fn(*mut ()),
}

impl<Ret, Args> Vtable<Ret, Args> {
    pub const fn new(
        move_fn: unsafe fn(*mut (), *mut ()),
        call_fn: unsafe fn(*mut (), Args) -> Ret,
        clear_fn: unsafe fn(*mut ()),
    ) -> Self {
        Self {
            move_fn,
            call_fn,
            clear_fn,
        }
    }
}

pub unsafe fn move_fn<Fn>(from: *mut (), to: *mut ()) {
    let from = from as *mut Fn;
    let to = to as *mut Fn;
    std::ptr::write(to, std::ptr::read(from));
}

pub unsafe fn call_fn<Fn, Ret, Args>(storage: *mut (), args: Args) -> Ret
where
    Fn: FnOnce(Args) -> Ret,
{
    let closure = storage as *mut Fn;
    (*closure)(args)
}

pub unsafe fn clear_fn<Fn>(storage: *mut ()) {
    let closure = storage as *mut Fn;
    std::ptr::drop_in_place(closure);
}

pub fn make_vtable<Fn, Ret, Args>() -> Vtable<Ret, Args>
where
    Fn: FnOnce(Args) -> Ret,
{
    Vtable::new(move_fn::<Fn>, call_fn::<Fn, Ret, Args>, clear_fn::<Fn>)
}

#[test] fn test_vtable_creation() {
    let vtable = make_vtable::<Box<dyn FnOnce(i32) -> i32>, i32, i32>();
    // Now you have a vtable you can use for dynamic dispatch
}
*/
