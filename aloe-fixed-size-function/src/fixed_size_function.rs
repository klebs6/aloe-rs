crate::ix!();

use std::any::TypeId;
use std::mem::{align_of, size_of, MaybeUninit, transmute};
use std::ptr::NonNull;

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/containers/aloe_FixedSizeFunction.h]

const DEFAULT_LEN: usize = 128;

type Storage<const LEN: usize = DEFAULT_LEN> = [u8; LEN];

struct VTable<const LEN: usize, Ret, Args> {
    call_fn:  unsafe fn(*const Storage<LEN>, Args) -> Ret,
    clear_fn: unsafe fn(*mut Storage<LEN>),
    move_fn:  unsafe fn(*const Storage<LEN>, *mut Storage<LEN>),
}

/**
  | A type similar to `std::function` that
  | holds a callable object.
  | 
  | Unlike `std::function`, the callable
  | object will always be stored in a buffer
  | of size `len` that is internal to the
  | FixedSizeFunction instance.
  | 
  | This in turn means that creating a FixedSizeFunction
  | instance will never allocate, making
  | FixedSizeFunctions suitable for use
  | in realtime contexts.
  | 
  | @tags{DSP}
  |
  */
pub struct FixedSizeFunction<const LEN: usize, Ret, Args> {
    storage: MaybeUninit<Storage<LEN>>,
    vtable:  Option<VTable<LEN, Ret, Args>>,
}

impl<const LEN: usize, Ret, Args> Drop for FixedSizeFunction<LEN, Ret, Args> {

    fn drop(&mut self) {
        self.clear()
    }
}

impl<const LEN: usize, Ret, Args> FixedSizeFunction<LEN, Ret, Args> {

    pub fn new<F: Fn(Args) -> Ret>(func: F) -> Self {

        //The requested function must fit into this FixedSizeFunction
        assert!(std::mem::size_of::<F>() <= LEN);

        // FixedSizeFunction must accommodate the requested alignment requirements
        assert!(std::mem::align_of::<F>() <= std::mem::align_of::<Storage>());

        let mut storage = MaybeUninit::<Storage<LEN>>::uninit();

        let storage_ptr = storage.as_mut_ptr() as *mut F;

        unsafe {
            std::ptr::write(storage_ptr, func);
        }

        let vtable = VTable {
            call_fn: |storage, args| unsafe {
                let func: *const F = transmute(storage);
                (*func)(args)
            },
            clear_fn: |storage| unsafe {
                let func: *mut F = transmute(storage);
                std::ptr::drop_in_place(func);
            },
            move_fn: |src, dst| unsafe {
                let src: *const F = transmute(src);
                let dst: *mut F = transmute(dst);
                std::ptr::write(dst, std::ptr::read(src));
            },
        };

        FixedSizeFunction {
            storage,
            vtable: Some(vtable),
        }
    }

    pub fn call(&self, args: Args) -> Ret {
        match self.vtable {
            Some(ref vtable) => unsafe {
                (vtable.call_fn)(self.storage.as_ptr(), args)
            },
            None => panic!("bad_function_call"),
        }
    }

    pub fn is_callable(&self) -> bool {
        self.vtable.is_some()
    }

    pub fn clear(&mut self) {
        if let Some(ref vtable) = self.vtable {
            unsafe {
                (vtable.clear_fn)(self.storage.as_mut_ptr());
            }
        }
    }
}

#[test] fn test_fixed_size_function() {

    let f = FixedSizeFunction::new(|x: i32| x + 1);
    println!("Is callable: {}", f.is_callable());

    let result = f.call(42);
    println!("Result: {}", result);
}
