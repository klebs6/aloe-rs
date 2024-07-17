/*!
  | This file contains a few helper functions
  | that are used internally but which need
  | to be kept away from the public headers
  | because they use obj-C symbols.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_mac_ObjCHelpers.h]

#[inline] pub fn ns_range_to_aloe(range: NSRange) -> Range<i32> {
    
    todo!();
    /*
        return { (int) range.location, (int) (range.location + range.length) };
    */
}

#[inline] pub fn aloe_range_tons(range: Range<i32>) -> NSRange {
    
    todo!();
    /*
        return NSMakeRange ((NSUInteger) range.getStart(), (NSUInteger) range.getLength());
    */
}

#[inline] pub fn ns_string_to_aloe(s: *mut NSString) -> String {
    
    todo!();
    /*
        return CharPointer_UTF8 ([s UTF8String]);
    */
}

#[inline] pub fn aloe_string_tons(s: &String) -> *mut NSString {
    
    todo!();
    /*
        return [NSString stringWithUTF8String: s.toUTF8()];
    */
}

#[inline] pub fn ns_string_literal(s: *const u8) -> *mut NSString {
    
    todo!();
    /*
        return [NSString stringWithUTF8String: s];
    */
}

#[inline] pub fn ns_empty_string() -> *mut NSString {
    
    todo!();
    /*
        return [NSString string];
    */
}

#[inline] pub fn create_nsurl_from_filename(f: &String) -> *mut NSURL {
    
    todo!();
    /*
        return [NSURL fileURLWithPath: aloeStringToNS (f)];
    */
}

#[inline] pub fn create_nsurl_from_file(f: &File) -> *mut NSURL {
    
    todo!();
    /*
        return createNSURLFromFile (f.getFullPathName());
    */
}

#[inline] pub fn create_ns_array_from_string_array<T>(
    strings: &StringArray

) -> *mut NSArray<T> {
    
    todo!();
    /*
        auto array = [[NSMutableArray alloc] init];

        for (auto string: strings)
            [array addObject:aloeStringToNS (string)];

        return [array autorelease];
    */
}

#[inline] pub fn var_object_to_ns_dictionary<K,V>(var_to_parse: &Var) 
-> *mut NSDictionary<K,V> 
{
    todo!();
    /*
        auto dictionary = [NSMutableDictionary dictionary];

        if (varToParse.isObject())
        {
            auto* dynamicObject = varToParse.getDynamicObject();

            auto& properties = dynamicObject->getProperties();

            for (int i = 0; i < properties.size(); ++i)
            {
                auto* keyString = aloeStringToNS (properties.getName (i).toString());

                const var& valueVar = properties.getValueAt (i);

                if (valueVar.isObject())
                {
                    auto* valueDictionary = varObjectToNSDictionary (valueVar);

                    [dictionary setObject: valueDictionary forKey: keyString];
                }
                else if (valueVar.isArray())
                {
                    auto* valueArray = varArrayToNSArray (valueVar);

                    [dictionary setObject: valueArray forKey: keyString];
                }
                else
                {
                    auto* valueString = aloeStringToNS (valueVar.toString());

                    [dictionary setObject: valueString forKey: keyString];
                }
            }
        }

        return dictionary;
    */
}

#[inline] pub fn var_array_to_ns_array<T>(
    var_to_parse: &Var

) -> *mut NSArray<T> {
    
    todo!();
    /*
        jassert (varToParse.isArray());

        if (! varToParse.isArray())
            return nil;

        const auto* varArray = varToParse.getArray();

        auto array = [NSMutableArray arrayWithCapacity: (NSUInteger) varArray->size()];

        for (const auto& aVar : *varArray)
        {
            if (aVar.isObject())
            {
                auto* valueDictionary = varObjectToNSDictionary (aVar);

                [array addObject: valueDictionary];
            }
            else if (aVar.isArray())
            {
                auto* valueArray = varArrayToNSArray (aVar);

                [array addObject: valueArray];
            }
            else
            {
                auto* valueString = aloeStringToNS (aVar.toString());

                [array addObject: valueString];
            }
        }

        return array;
    */
}

#[inline] pub fn ns_dictionary_to_var<K,V>(
    dictionary: *mut NSDictionary<K,V>

) -> Var {
    
    todo!();
    /*
        DynamicObject::Ptr dynamicObject (new DynamicObject());

        for (NSString* key in dictionary)
            dynamicObject->setProperty (nsStringToAloe (key), nsObjectToVar (dictionary[key]));

        return var (dynamicObject.get());
    */
}

#[inline] pub fn ns_array_to_var<T>(
    array: *mut NSArray<T>) 

-> Var {

    todo!();
    /*
        Vec<var> resultArray;

        for (objc_id::Id value in array)
            resultArray.add (nsObjectToVar (value));

        return var (resultArray);
    */
}

#[inline] pub fn ns_object_to_var(obj: *mut NSObject) -> Var {
    
    todo!();
    /*
        if ([obj isKindOfClass: [NSString class]])          return nsStringToAloe ((NSString*) obj);
        else if ([obj isKindOfClass: [NSNumber class]])     return nsStringToAloe ([(NSNumber*) obj stringValue]);
        else if ([obj isKindOfClass: [NSDictionary class]]) return nsDictionaryToVar ((NSDictionary*) obj);
        else if ([obj isKindOfClass: [NSArray class]])      return nsArrayToVar ((NSArray*) obj);
        else
        {
            // Unsupported yet, add here!
            jassertfalse;
        }

        return {};
    */
}

#[cfg(target_os="macos")]
pub fn make_ns_rect<RectangleType>(r: &RectangleType) -> NSRect {

    todo!();
    /*
        return NSMakeRect (static_cast<CGFloat> (r.getX()),
                           static_cast<CGFloat> (r.getY()),
                           static_cast<CGFloat> (r.getWidth()),
                           static_cast<CGFloat> (r.getHeight()));
    */
}

#[cfg(ALOE_INTEL)]
pub struct NeedsStret<T> {

}

#[cfg(ALOE_INTEL)]
impl<T> NeedsStret<T> {

    #[cfg(ALOE_32BIT)]
    pub const value: bool = sizeof (T) > 8;

    #[cfg(not(ALOE_32BIT))]
    pub const value: bool = sizeof (T) > 16;
}

#[cfg(ALOE_INTEL)]
impl NeedsStret<c_void> {
    pub const value: bool = false;
}

#[cfg(ALOE_INTEL)]
lazy_static!{
    /*
    template <typename T, bool b = NeedsStret<T>::value>
     struct MetaSuperFn { static constexpr auto value = objc_msgSendSuper_stret; };

     template <typename T>
     struct MetaSuperFn<T, false> { static constexpr auto value = objc_msgSendSuper; };
    */
}

//---------------------------------------------------
pub struct NSObjectDeleter {

}

unsafe impl Allocator for NSObjectDeleter {

    fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> { 
        todo!() 
    }

    unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) { 
        todo!() 
    }
}

impl NSObjectDeleter {

    pub fn invoke(&self, object: *mut NSObject)  {
        
        todo!();
        /*
            if (object != nullptr)
                [object release];
        */
    }
}

pub type NSUniquePtr<NSType> = Box<NSType,NSObjectDeleter>;

#[inline] pub fn get_ivar<Type>(
        self_: objc::runtime::Object,
        name:  *const u8) -> Type {

    todo!();
    /*
        void* v = nullptr;
        object_getInstanceVariable (self, name, &v);
        return static_cast<Type> (v);
    */
}

#[no_copy]
pub struct ObjCClass<SuperclassType> {
    cls:     objc::runtime::Class,
    phantom: PhantomData<SuperclassType>,
}

impl<SuperclassType> Drop for ObjCClass<SuperclassType> {

    fn drop(&mut self) {
        todo!();
        /* 
            auto kvoSubclassName = String ("NSKVONotifying_") + class_getName (cls);

            if (objc_getClass (kvoSubclassName.toUTF8()) == nullptr)
                objc_disposeClassPair (cls);
         */
    }
}

impl<SuperclassType> ObjCClass<SuperclassType> {

    pub fn new(name_root: *const u8) -> Self {
    
        todo!();
        /*


            : cls (objc_allocateClassPair ([SuperclassType class], getRandomisedName (nameRoot).toUTF8(), 0))
        */
    }

    pub fn register_class(&mut self)  {
        
        todo!();
        /*
            objc_registerClassPair (cls);
        */
    }
    
    pub fn create_instance(&self) -> *mut SuperclassType {
        
        todo!();
        /*
            return class_createInstance (cls, 0);
        */
    }
    
    pub fn add_ivar<Type>(&mut self, name: *const u8)  {
    
        todo!();
        /*
            BOOL b = class_addIvar (cls, name, sizeof (Type), (uint8_t) rint (log2 (sizeof (Type))), @encode (Type));
            jassert (b); ignoreUnused (b);
        */
    }
    
    pub fn add_method<FunctionType>(&mut self, 
        selector:    objc::runtime::Sel,
        callback_fn: FunctionType,
        signature:   *const u8)  {
    
        todo!();
        /*
            BOOL b = class_addMethod (cls, selector, (IMP) callbackFn, signature);
            jassert (b); ignoreUnused (b);
        */
    }
    
    pub fn add_method2<FunctionType>(&mut self, 
        selector:    objc::runtime::Sel,
        callback_fn: FunctionType,
        sig1:        *const u8,
        sig2:        *const u8)  {
    
        todo!();
        /*
            addMethod (selector, callbackFn, (String (sig1) + sig2).toUTF8());
        */
    }
    
    pub fn add_method3<FunctionType>(&mut self, 
        selector:    objc::runtime::Sel,
        callback_fn: FunctionType,
        sig1:        *const u8,
        sig2:        *const u8,
        sig3:        *const u8)  {
    
        todo!();
        /*
            addMethod (selector, callbackFn, (String (sig1) + sig2 + sig3).toUTF8());
        */
    }
    
    pub fn add_method4<FunctionType>(&mut self, 
        selector:    objc::runtime::Sel,
        callback_fn: FunctionType,
        sig1:        *const u8,
        sig2:        *const u8,
        sig3:        *const u8,
        sig4:        *const u8)  {
    
        todo!();
        /*
            addMethod (selector, callbackFn, (String (sig1) + sig2 + sig3 + sig4).toUTF8());
        */
    }
    
    pub fn add_protocol(&mut self, protocol: *mut objc::runtime::Protocol)  {
        
        todo!();
        /*
            BOOL b = class_addProtocol (cls, protocol);
            jassert (b); ignoreUnused (b);
        */
    }
    
    pub fn send_superclass_message<ReturnType, Params>(
        self_:  objc_id::Id<Self>,
        sel:    objc::runtime::Sel,
        params: Params) -> ReturnType {
    
        todo!();
        /*
            return ObjCMsgSendSuper<SuperclassType, ReturnType, Params...> (self, sel, params...);
        */
    }
    
    pub fn get_randomised_name(root: *const u8) -> String {
        
        todo!();
        /*
            return root + String::toHexString (Random::getSystemRandom().nextInt64());
        */
    }
}


///--------------------------
pub struct ObjCLifetimeManagedClass<AloeClass> {
    base:    ObjCClass<NSObject>,
    phantom: PhantomData<AloeClass>,
}

pub mod objc_lifetime_managed_class {
    use super::*;

    lazy_static!{
        /*
        static ObjCLifetimeManagedClass objCLifetimeManagedClass;

        template <typename Class>
        ObjCLifetimeManagedClass<Class> ObjCLifetimeManagedClass<Class>::objCLifetimeManagedClass;
        */
    }
}

impl<AloeClass> Default for ObjCLifetimeManagedClass<AloeClass> {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("ObjCLifetimeManagedClass_")
            addIvar<AloeClass*> ("cppObject");

            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
            addMethod (@selector (initWithAloeObject:), initWithAloeObject, "@@:@");
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE

            addMethod (@selector (dealloc),             dealloc,            "v@:");

            registerClass()
        */
    }
}

impl<AloeClass> ObjCLifetimeManagedClass<AloeClass> {
    
    pub fn init_with_aloe_object(
        self_: objc_id::Id<Self>,
        _1:    objc::runtime::Sel,
        obj:   *mut AloeClass) -> objc_id::Id<Self> {
        
        todo!();
        /*
            NSObject* self = sendSuperclassMessage<NSObject*> (_self, @selector (init));
            object_setInstanceVariable (self, "cppObject", obj);

            return self;
        */
    }
    
    pub fn dealloc(
        self_: objc_id::Id<Self>,
        _1:    objc::runtime::Sel)  {
        
        todo!();
        /*
            if (auto* obj = getIvar<AloeClass*> (_self, "cppObject"))
            {
                delete obj;
                object_setInstanceVariable (_self, "cppObject", nullptr);
            }

            sendSuperclassMessage<void> (_self, @selector (dealloc));
        */
    }
}

/**
  | this will return an NSObject which takes
  | ownership of the Aloe instance passed-in
  |
  | This is useful to tie the life-time of a aloe
  | instance to the life-time of an NSObject
  */
pub fn create_ns_object_from_aloe_class<Class>(obj: *mut Class) -> *mut NSObject {

    todo!();
    /*
        ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wobjc-method-access")
        return [ObjCLifetimeManagedClass<Class>::objCLifetimeManagedClass.createInstance() initWithAloeObject:obj];
        ALOE_END_IGNORE_WARNINGS_GCC_LIKE
    */
}

/**
  | Get the Aloe class instance that was
  | tied to the life-time of an NSObject
  | with the function above
  |
  */
pub fn get_aloe_class_from_ns_object<Class>(obj: *mut NSObject) -> *mut Class {

    todo!();
    /*
        return obj != nullptr ? getIvar<Class*> (obj, "cppObject") : nullptr;
    */
}

lazy_static!{
    /*
    template <typename ReturnT, class Class, typename... Params>
    ReturnT (^CreateObjCBlock(Class* object, ReturnT (Class::*fn)(Params...))) (Params...)
    {
        __block Class* _this = object;
        __block ReturnT (Class::*_fn)(Params...) = fn;

        return [[^ReturnT (Params... params) { return (_this->*_fn) (params...); } copy] autorelease];
    }
    */
}

///------------------------
/**
  | Please be aware of the following:
  | 
  | This code is still fairly high-level
  | and might not be entirely functional
  | without additional adjustments.
  | 
  | Handling Objective-C blocks in Rust
  | is non-trivial, especially if you're
  | trying to create them from Rust functions
  | or methods.
  | 
  | ARC (Automatic Reference Counting)
  | has been assumed to be enabled. If it's
  | not, you might have to do additional
  | memory management manually.
  | 
  | A complete solution would likely require
  | a more thorough integration between
  | 
  | Rust and Objective-C and possibly some
  | work in a C or Objective-C shim to make
  | some operations more straightforward.
  |
  */
pub struct ObjCBlock<BlockType> {
    block: *mut libc::c_void,
    _phantom: PhantomData<BlockType>,
}

impl<BlockType> Default for ObjCBlock<BlockType> {
    fn default() -> Self {
        ObjCBlock {
            block: std::ptr::null_mut(),
            _phantom: PhantomData,
        }
    }
}

impl<BlockType> ObjCBlock<BlockType> {

    // TODO: Implement your templated conversion from methods to blocks.
    //
    // This is a non-trivial task and might require more details or a different approach.

    pub fn from_block(b: *mut libc::c_void) -> Self {

        // Assuming ARC is enabled, copy the block
        let copy: *mut libc::c_void = unsafe {

            msg_send![b as *mut objc::runtime::Object, copy]
        };

        ObjCBlock {
            block: copy,
            _phantom: PhantomData,
        }
    }

    pub fn assign_from(&mut self, other: &BlockType) -> &mut ObjCBlock<BlockType> {

        if !self.block.is_null() {

            unsafe {
                let _: () = msg_send![self.block as *mut objc::runtime::Object, release];
            }
        }

        // For now, assume that BlockType is represented as *mut libc::c_void.
        self.block = unsafe {
            msg_send![other as *const _ as *mut objc::runtime::Object, copy]
        };

        self
    }

    pub fn equals_ptr(&self, ptr: *mut libc::c_void) -> bool {
        self.block == ptr
    }
}

impl<BlockType> Drop for ObjCBlock<BlockType> {

    fn drop(&mut self) {

        if !self.block.is_null() {
            unsafe {
                let _: () = msg_send![self.block as *mut objc::runtime::Object, release];
            }
        }
    }
}

impl<BlockType> Into<*mut libc::c_void> for ObjCBlock<BlockType> {

    fn into(self) -> *mut libc::c_void {
        self.block
    }
}

//-------------------------------------------
pub struct NSURL {
    obj: *mut objc::runtime::Object,
}

impl NSURL {

    pub fn from_string(url_string: &str) -> Option<Self> {

        let nsurl_class = objc::runtime::Class::get("NSURL").expect("NSURL class not found");

        let url: *mut objc::runtime::Object = unsafe {

            let str_obj: *mut objc::runtime::Object = msg_send![

                objc::runtime::Class::get("NSString").unwrap(), 

                stringWithUTF8String: url_string.as_ptr() as *const _
            ];

            let url_obj: *mut objc::runtime::Object = msg_send![nsurl_class, URLWithString:str_obj];

            url_obj
        };

        if url.is_null() {
            None
        } else {
            Some(NSURL { obj: url })
        }
    }

    // Other NSURL methods can be wrapped similarly...
}
