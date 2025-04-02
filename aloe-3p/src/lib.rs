#![feature(test)]
#![feature(adt_const_params)]

#[macro_export] macro_rules! x { 
    ($x:ident) => { 
        mod $x; 
        pub use $x::*; 
    }
}

#[macro_export] macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
        use crate::*;
    } 
}

#[macro_export] macro_rules! ternary {
    ($condition:expr,$if_true:expr,$if_false:expr) => {
        match $condition {
            true => $if_true,
            false => $if_false,
        }
    }
}

//TODO: compile time error message?
#[macro_export] macro_rules! static_assert {
    ($b:expr) => {  
        const_assert!( $b );
    }
}

#[macro_export] macro_rules! declare_jni_class_with_min_sdk {
    ($CppClassName:ident, 
     $javaPath:expr, 
     $minSDK:expr) => {
        /*
        
            DECLARE_JNI_CLASS_WITH_BYTECODE (CppClassName, javaPath, minSDK, nullptr, 0)
        */
    }
}

#[macro_export] macro_rules! declare_jni_class {
    ($CppClassName:ty, 
     $javaPath:expr) => {
        /*
        
            DECLARE_JNI_CLASS_WITH_MIN_SDK (CppClassName, javaPath, 16)
        */
    }
}

#[macro_export] macro_rules! create_jni_method {
    ($methodID:ident, 
     $stringName:ident, 
     $params:ident) => {
        /*
                methodID = resolveMethod (env, stringName, params);
        */
    }
}

#[macro_export] macro_rules! create_jni_staticmethod {
    ($methodID:ident, 
     $stringName:ident, 
     $params:ident) => {
        /*
                methodID = resolveStaticMethod (env, stringName, params);
        */
    }
}

#[macro_export] macro_rules! create_jni_field {
    ($fieldID:ident, 
     $stringName:ident, 
     $signature:ident) => {
        /*
                fieldID  = resolveField (env, stringName, signature);
        */
    }
}

#[macro_export] macro_rules! create_jni_staticfield {
    ($fieldID:ident, 
     $stringName:ident, 
     $signature:ident) => {
        /*
                fieldID  = resolveStaticField (env, stringName, signature);
        */
    }
}

#[macro_export] macro_rules! create_jni_callback {
    ($callbackName:ident, 
     $stringName:ident, 
     $signature:ident) => {
        /*
                callbacks.add ({stringName, signature, (void*) callbackName});
        */
    }
}

#[macro_export] macro_rules! declare_jni_method {
    ($methodID:ident, 
     $stringName:ident, 
     $params:ident) => {
        /*
                jmethodID methodID;
        */
    }
}

#[macro_export] macro_rules! declare_jni_field {
    ($fieldID:ident, 
     $stringName:ident, 
     $signature:ident) => {
        /*
                jfieldID  fieldID;
        */
    }
}

#[macro_export] macro_rules! declare_jni_callback {
    ($fieldID:ident, 
     $stringName:ident, 
     $signature:ident) => {
    }
}

#[macro_export] macro_rules! declare_jni_class_with_bytecode {
    ($CppClassName:ty, 
     $javaPath:expr, 
     $minSDK:expr, 
     $byteCodeData:expr, 
     $byteCodeSize:expr) => {
        /*
        
            class CppClassName ## _Class   : public JNIClassBase 
            { 
             
                CppClassName ## _Class() : JNIClassBase (javaPath, minSDK, byteCodeData, byteCodeSize) {} 
            
                void initialiseFields (JNIEnv* env) 
                { 
                    Vec<JNINativeMethod> callbacks; 
                    JNI_CLASS_MEMBERS (CREATE_JNI_METHOD, CREATE_JNI_STATICMETHOD, CREATE_JNI_FIELD, CREATE_JNI_STATICFIELD, CREATE_JNI_CALLBACK); 
                    resolveCallbacks (env, callbacks); 
                } 
            
                JNI_CLASS_MEMBERS (DECLARE_JNI_METHOD, DECLARE_JNI_METHOD, DECLARE_JNI_FIELD, DECLARE_JNI_FIELD, DECLARE_JNI_CALLBACK) 
            }; 
            static CppClassName ## _Class CppClassName;
        */
    }
}

#[macro_export] macro_rules! generic_float_const {

    ($trait_name:ident, $fn_name:ident, $value:expr) => {

        pub trait $trait_name {

            #[inline(always)] fn $fn_name() -> Self;
        }

        impl<T: num::Float> $trait_name for T { 

            #[inline(always)] fn $fn_name() -> Self { 
                T::from($value).unwrap() 
            }
        }

        #[inline(always)] pub fn $fn_name<F: num::Float>() -> F {
            <F as $trait_name>::$fn_name()
        }
    }
}

pub trait GetName {

    fn get_name(&self) -> String {
        
        todo!();
        /*
            jassertfalse; // You shouldn't call this for an expression that's not actually a function!
            return {};
        */
    }
}

extern crate atomic;
extern crate bitfield;
extern crate libc;
extern crate test;
extern crate rand;

pub fn no_op() {}

pub trait InputIteratorTag    {}
pub trait HasValueType        { type ValueType; }
pub trait HasReference        { type Reference; }
pub trait HasDifferenceType   { type DifferenceType; }
pub trait HasPointer          { type Pointer; }
pub trait HasIteratorCategory { type IteratorCategory; }

#[macro_use] extern crate static_assertions;
#[macro_use] extern crate lazy_static;

pub type aloe_wchar = libc::wchar_t;

pub use jni::sys::{jstring,jint,jintArray,jobject,jobjectArray,JNIEnv};

//pub use gl::types::GLuint;
//use jni::JNIEnv;
pub use atomic::Atomic;
pub use std::mem::ManuallyDrop;
pub use cocoa::appkit::{NSImage};
pub use cocoa::foundation::{NSRect,NSPoint};
pub use derive_error::*;
pub use jni::objects::{JClass, JFieldID, JObject};
pub use lazy_static::*;
pub use libc::*;
pub use modular_bitfield::*;
pub use num::*;
pub use num_traits::Zero;
pub use parking_lot::*;
pub use parking_lot::RawRwLock;
pub use rand::*;
pub use rand::rngs::ThreadRng;
pub use spin;
pub use std::any::TypeId;
pub use std::cell::RefCell;
pub use std::collections::*;
pub use std::error::Error;
pub use std::fmt;
pub use std::io::{Write,Read};
pub use std::marker::{PhantomData,ConstParamTy};
pub use std::mem::MaybeUninit;
pub use std::mem::{size_of,size_of_val};
pub use std::ops::*;
pub use std::ops::Add;
pub use std::path::{Path as StdPath, PathBuf};
pub use std::ptr::{null,null_mut};
pub use std::rc::{Rc,Weak};
pub use std::sync::Arc;
pub use std::sync::Condvar;
pub use std::sync::atomic::{AtomicBool,AtomicI32};
pub use std::time::Duration;
pub use std::time::Instant;
