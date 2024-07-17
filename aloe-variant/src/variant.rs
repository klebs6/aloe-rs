crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_Variant.h]

pub union VarValueUnion
{
    int_value:    i32,
    int_64value:  i64,
    bool_value:   bool,
    double_value: f64,
    string_value: [u8; std::mem::size_of::<String>()],
    object_value: *mut ReferenceCountedObject,
    binary_value: *mut MemoryBlock,
    method_value: *mut VarNativeFunction,
}

/**
  | This structure is passed to a NativeFunction
  | callback, and contains invocation
  | details about the function's arguments
  | and context.
  |
  */
pub struct VarNativeFunctionArgs<'a>
{
    this_object:   &'a Var,
    arguments:     *const Var,
    num_arguments: i32,
}

pub type VarNativeFunction = fn(_0: &VarNativeFunctionArgs) -> Var;

/**
  | A variant class, that can be used to hold
  | a range of primitive values.
  | 
  | A Var object can hold a range of simple
  | primitive values, strings, or any kind
  | of ReferenceCountedObject. The Var
  | class is intended to act like the kind
  | of values used in dynamic scripting
  | languages.
  | 
  | You can save/load Var objects either
  | in a small, proprietary binary format
  | using writeToStream()/readFromStream(),
  | or as JSON by using the JSON class.
  | 
  | @see JSON, DynamicObject
  | 
  | @tags{Core}
  |
  */
pub struct Var {
    ty:    *const VariantType,
    value: VarValueUnion,
}

pub enum VariantType {
    Todo,
}

impl Default for Var {
    
    /**
      | Creates a void variant.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

/*
impl Var {

    /**
      | Returns a Var object that can be used
      | where you need the javascript "undefined"
      | value.
      |
      */
    pub fn undefined() -> Var {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | If this variant holds an array, this
      | provides access to it.
      | 
      | -----------
      | @note
      | 
      | Beware when you use this - the array pointer
      | is only valid for the lifetime of the
      | variant that returned it, so be very
      | careful not to call this method on temporary
      | Var objects that are the return-value
      | of a function, and which may go out of
      | scope before you use the array!
      |
      */
    pub fn get_array(&self) -> *mut Vec<Var> {
        
        todo!();
        /*
        
        */
    }

    /**
      | If this variant holds a memory block,
      | this provides access to it.
      | 
      | -----------
      | @note
      | 
      | Beware when you use this - the MemoryBlock
      | pointer is only valid for the lifetime
      | of the variant that returned it, so be
      | very careful not to call this method
      | on temporary Var objects that are the
      | return-value of a function, and which
      | may go out of scope before you use the
      | MemoryBlock!
      |
      */
    pub fn get_binary_data(&self) -> *mut MemoryBlock {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this Var has the same
      | value as the one supplied.
      | 
      | -----------
      | @note
      | 
      | that this ignores the type, so a string
      | Var "123" and an integer Var with the
      | value 123 are considered to be equal.
      | @see equalsWithSameType
      |
      */
    pub fn equals(&self, other: &Var) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this Var has the same
      | value and type as the one supplied.
      | 
      | This differs from equals() because
      | e.g. "123" and 123 will be considered
      | different. @see equals
      |
      */
    pub fn equals_with_same_type(&self, other: &Var) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this Var has the same
      | type as the one supplied.
      |
      */
    pub fn has_same_type_as(&self, other: &Var) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns a deep copy of this object.
      | 
      | For simple types this just returns a
      | copy, but if the object contains any
      | arrays or DynamicObjects, they will
      | be cloned (recursively).
      |
      */
    pub fn clone(&self) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | If the Var is an array, this returns the
      | number of elements. If the Var isn't
      | actually an array, this will return
      | 0.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Appends an element to the Var, converting
      | it to an array if it isn't already one.
      | 
      | If the Var isn't an array, it will be converted
      | to one, and if its value was non-void,
      | this value will be kept as the first element
      | of the new array. The parameter value
      | will then be appended to it.
      | 
      | For more control over the array's contents,
      | you can call getArray() and manipulate
      | it directly as an Vec\<Var\>.
      |
      */
    pub fn append(&mut self, value_to_append: &Var)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Inserts an element to the Var, converting
      | it to an array if it isn't already one.
      | 
      | If the Var isn't an array, it will be converted
      | to one, and if its value was non-void,
      | this value will be kept as the first element
      | of the new array. The parameter value
      | will then be inserted into it.
      | 
      | For more control over the array's contents,
      | you can call getArray() and manipulate
      | it directly as an Vec\<Var\>.
      |
      */
    pub fn insert(&mut self, 
        index: i32,
        value: &Var)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | If the Var is an array, this removes one
      | of its elements.
      | 
      | If the index is out-of-range or the Var
      | isn't an array, nothing will be done.
      | 
      | For more control over the array's contents,
      | you can call getArray() and manipulate
      | it directly as an Vec\<Var\>.
      |
      */
    pub fn remove(&mut self, index: i32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Treating the Var as an array, this resizes
      | it to contain the specified number of
      | elements.
      | 
      | If the Var isn't an array, it will be converted
      | to one, and if its value was non-void,
      | this value will be kept as the first element
      | of the new array before resizing.
      | 
      | For more control over the array's contents,
      | you can call getArray() and manipulate
      | it directly as an Vec\<Var\>.
      |
      */
    pub fn resize(&mut self, num_array_elements_wanted: i32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | If the Var is an array, this searches
      | it for the first occurrence of the specified
      | value, and returns its index.
      | 
      | If the Var isn't an array, or if the value
      | isn't found, this returns -1.
      |
      */
    pub fn index_of(&self, value: &Var) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | If this variant is an object, this returns
      | one of its properties, or a default fallback
      | value if the property is not set.
      |
      */
    pub fn get_property(&self, 
        property_name:        &Identifier,
        default_return_value: &Var) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this variant is an object
      | and if it has the given property.
      |
      */
    pub fn has_property(&self, property_name: &Identifier) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Invokes a named method call with no arguments.
      |
      */
    pub fn call0(&self, method: &Identifier) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | Invokes a named method call with one
      | argument.
      |
      */
    pub fn call1(&self, 
        method: &Identifier,
        arg1:   &Var) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | Invokes a named method call with 2 arguments.
      |
      */
    pub fn call2(&self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | Invokes a named method call with 3 arguments.
      |
      */
    pub fn call3(&mut self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var,
        arg3:   &Var) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | Invokes a named method call with 4 arguments.
      |
      */
    pub fn call4(&self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var,
        arg3:   &Var,
        arg4:   &Var) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | Invokes a named method call with 5 arguments.
      |
      */
    pub fn call5(&self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var,
        arg3:   &Var,
        arg4:   &Var,
        arg5:   &Var) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | Invokes a named method call with a list
      | of arguments.
      |
      */
    pub fn invoke(&self, 
        method:        &Identifier,
        arguments:     *const Var,
        num_arguments: i32) -> Var {
        
        todo!();
        /*
        
        */
    }

    /**
      | If this object is a method, this returns
      | the function pointer.
      |
      */
    pub fn get_native_function(&self) -> NativeFunction {
        
        todo!();
        /*
        
        */
    }

    /**
      | Writes a binary representation of this
      | value to a stream.
      | 
      | The data can be read back later using
      | readFromStream(). @see JSON
      |
      */
    pub fn write_to_stream(&self, output: &mut OutputStream)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Reads back a stored binary representation
      | of a value.
      | 
      | The data in the stream must have been
      | written using writeToStream(), or
      | this will have unpredictable results.
      | @see JSON
      |
      */
    pub fn read_from_stream(input: &mut InputStream) -> Var {
        
        todo!();
        /*
        
        */
    }
    
    pub fn convert_to_array(&mut self) -> *mut Vec<Var> {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(_0: &VariantType) -> Self {
    
        todo!();
        /*


        
        */
    }
}

/**
  | This template-overloaded class can
  | be used to convert between Var and custom
  | types.
  | 
  | @tags{Core}
  |
  */
pub struct VariantConverter<Type> {

}

impl<Type> VariantConverter<Type> {

    pub fn from_var(v: &Var) -> Type {
        
        todo!();
        /*
            return static_cast<Type> (v);
        */
    }
    
    pub fn to_var(t: &Type) -> Var {
        
        todo!();
        /*
            return t;
        */
    }
}

impl VariantConverter<String> {

    pub fn from_var(v: &Var) -> String {
        
        todo!();
        /*
            return v.toString();
        */
    }
    
    pub fn to_var(s: &String) -> Var {
        
        todo!();
        /*
            return s;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_Variant.cpp]

pub enum VariantStreamMarkers
{
    varMarker_Int       = 1,
    varMarker_BoolTrue  = 2,
    varMarker_BoolFalse = 3,
    varMarker_Double    = 4,
    varMarker_String    = 5,
    varMarker_Int64     = 6,
    varMarker_Array     = 7,
    varMarker_Binary    = 8,
    varMarker_Undefined = 9
}

lazy_static!{
    /*
    struct varianttype
    {
        struct voidtag      {};
        struct undefinedtag {};
        struct inttag       {};
        struct int64tag     {};
        struct doubletag    {};
        struct booltag      {};
        struct stringtag    {};
        struct objecttag    {};
        struct arraytag     {};
        struct binarytag    {};
        struct methodtag    {};

        // members =====================================================================
        bool isvoid         = false;
        bool isundefined    = false;
        bool isint          = false;
        bool isint64        = false;
        bool isbool         = false;
        bool isdouble       = false;
        bool isstring       = false;
        bool isobject       = false;
        bool isarray        = false;
        bool isbinary       = false;
        bool ismethod       = false;
        bool iscomparable   = false;

        int                     (*toint)         (const valueunion&)                 = defaulttoint;
        int64                   (*toint64)       (const valueunion&)                 = defaulttoint64;
        double                  (*todouble)      (const valueunion&)                 = defaulttodouble;
        string                  (*tostring)      (const valueunion&)                 = defaulttostring;
        bool                    (*tobool)        (const valueunion&)                 = defaulttobool;
        referencecountedobject* (*toobject)      (const valueunion&)                 = defaulttoobject;
        array<Var>*             (*toarray)       (const valueunion&)                 = defaulttoarray;
        memoryblock*            (*tobinary)      (const valueunion&)                 = defaulttobinary;
        Var                     (*clone)         (const Var&)                        = defaultclone;
        void                    (*cleanup)       (valueunion&)                       = defaultcleanup;
        void                    (*createcopy)    (valueunion&, const valueunion&)    = defaultcreatecopy;

        bool                    (*equals)        (const valueunion&, const valueunion&, const varianttype&) = nullptr;
        void                    (*writetostream) (const valueunion&, outputstream&) = nullptr;

        // defaults ====================================================================
        static int                     defaulttoint         (const valueunion&)                          { return 0; }
        static int64                   defaulttoint64       (const valueunion&)                          { return 0; }
        static double                  defaulttodouble      (const valueunion&)                          { return 0; }
        static string                  defaulttostring      (const valueunion&)                          { return {}; }
        static bool                    defaulttobool        (const valueunion&)                          { return false; }
        static referencecountedobject* defaulttoobject      (const valueunion&)                          { return nullptr; }
        static array<Var>*             defaulttoarray       (const valueunion&)                          { return nullptr; }
        static memoryblock*            defaulttobinary      (const valueunion&)                          { return nullptr; }
        static Var                     defaultclone         (const Var& other)                           { return other; }
        static void                    defaultcleanup       (valueunion&)                                {}
        static void                    defaultcreatecopy    (valueunion& dest, const valueunion& source) { dest = source; }

        // void ========================================================================
        static bool voidequals (const valueunion&, const valueunion&, const varianttype& othertype) 
        {
            return othertype.isvoid || othertype.isundefined;
        }

        static void voidwritetostream (const valueunion&, outputstream& output)
        {
            output.writecompressedint (0);
        }

        constexpr explicit varianttype (voidtag) 
            : isvoid            (true),
              iscomparable      (true),
              equals            (voidequals),
              writetostream     (voidwritetostream) {}

        // undefined ===================================================================
        static string undefinedtostring (const valueunion&) { return "undefined"; }

        static bool undefinedequals (const valueunion&, const valueunion&, const varianttype& othertype) 
        {
            return othertype.isvoid || othertype.isundefined;
        }

        static void undefinedwritetostream (const valueunion&, outputstream& output)
        {
            output.writecompressedint (1);
            output.writebyte (varmarker_undefined);
        }

        constexpr explicit varianttype (undefinedtag) 
            : isundefined   (true),
              tostring      (undefinedtostring),
              equals        (undefinedequals),
              writetostream (undefinedwritetostream) {}

        // int =========================================================================
        static int    inttoint    (const valueunion& data)    { return data.intvalue; }
        static int64  inttoint64  (const valueunion& data)    { return (int64) data.intvalue; }
        static double inttodouble (const valueunion& data)    { return (double) data.intvalue; }
        static string inttostring (const valueunion& data)            { return string (data.intvalue); }
        static bool   inttobool   (const valueunion& data)    { return data.intvalue != 0; }

        static bool intequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            if (othertype.isdouble || othertype.isint64 || othertype.isstring)
                return othertype.equals (otherdata, data, varianttype { inttag{} });

            return othertype.toint (otherdata) == data.intvalue;
        }

        static void intwritetostream (const valueunion& data, outputstream& output)
        {
            output.writecompressedint (5);
            output.writebyte (varmarker_int);
            output.writeint (data.intvalue);
        }

        constexpr explicit varianttype (inttag) 
            : isint         (true),
              iscomparable  (true),
              toint         (inttoint),
              toint64       (inttoint64),
              todouble      (inttodouble),
              tostring      (inttostring),
              tobool        (inttobool),
              equals        (intequals),
              writetostream (intwritetostream) {}

        // int64 =======================================================================
        static int    int64toint    (const valueunion& data)    { return (int) data.int64value; }
        static int64  int64toint64  (const valueunion& data)    { return data.int64value; }
        static double int64todouble (const valueunion& data)    { return (double) data.int64value; }
        static string int64tostring (const valueunion& data)            { return string (data.int64value); }
        static bool   int64tobool   (const valueunion& data)    { return data.int64value != 0; }

        static bool int64equals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            if (othertype.isdouble || othertype.isstring)
                return othertype.equals (otherdata, data, varianttype { int64tag{} });

            return othertype.toint64 (otherdata) == data.int64value;
        }

        static void int64writetostream (const valueunion& data, outputstream& output)
        {
            output.writecompressedint (9);
            output.writebyte (varmarker_int64);
            output.writeint64 (data.int64value);
        }

        constexpr explicit varianttype (int64tag) 
            : isint64       (true),
              iscomparable  (true),
              toint         (int64toint),
              toint64       (int64toint64),
              todouble      (int64todouble),
              tostring      (int64tostring),
              tobool        (int64tobool),
              equals        (int64equals),
              writetostream (int64writetostream) {}

        // double ======================================================================
        static int    doubletoint    (const valueunion& data)    { return (int) data.doublevalue; }
        static int64  doubletoint64  (const valueunion& data)    { return (int64) data.doublevalue; }
        static double doubletodouble (const valueunion& data)    { return data.doublevalue; }
        static string doubletostring (const valueunion& data)            { return serialisedouble (data.doublevalue); }
        static bool   doubletobool   (const valueunion& data)    { return data.doublevalue != 0.0; }

        static bool doubleequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            return std::abs (othertype.todouble (otherdata) - data.doublevalue) < std::numeric_limits<double>::epsilon();
        }

        static void doublewritetostream (const valueunion& data, outputstream& output)
        {
            output.writecompressedint (9);
            output.writebyte (varmarker_double);
            output.writedouble (data.doublevalue);
        }

        constexpr explicit varianttype (doubletag) 
            : isdouble      (true),
              iscomparable  (true),
              toint         (doubletoint),
              toint64       (doubletoint64),
              todouble      (doubletodouble),
              tostring      (doubletostring),
              tobool        (doubletobool),
              equals        (doubleequals),
              writetostream (doublewritetostream) {}

        // bool ========================================================================
        static int    booltoint    (const valueunion& data)    { return data.boolvalue ? 1 : 0; }
        static int64  booltoint64  (const valueunion& data)    { return data.boolvalue ? 1 : 0; }
        static double booltodouble (const valueunion& data)    { return data.boolvalue ? 1.0 : 0.0; }
        static string booltostring (const valueunion& data)            { return string::chartostring (data.boolvalue ? (aloe_wchar) '1' : (aloe_wchar) '0'); }
        static bool   booltobool   (const valueunion& data)    { return data.boolvalue; }

        static bool boolequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            return othertype.tobool (otherdata) == data.boolvalue;
        }

        static void boolwritetostream (const valueunion& data, outputstream& output)
        {
            output.writecompressedint (1);
            output.writebyte (data.boolvalue ? (char) varmarker_booltrue : (char) varmarker_boolfalse);
        }

        constexpr explicit varianttype (booltag) 
            : isbool        (true),
              iscomparable  (true),
              toint         (booltoint),
              toint64       (booltoint64),
              todouble      (booltodouble),
              tostring      (booltostring),
              tobool        (booltobool),
              equals        (boolequals),
              writetostream (boolwritetostream) {}

        // string ======================================================================
        static const string* getstring (const valueunion& data)    { return unalignedpointercast<const string*> (data.stringvalue); }
        static       string* getstring (      valueunion& data)    { return unalignedpointercast<string*> (data.stringvalue); }

        static int    stringtoint    (const valueunion& data)    { return getstring (data)->getintvalue(); }
        static int64  stringtoint64  (const valueunion& data)    { return getstring (data)->getlargeintvalue(); }
        static double stringtodouble (const valueunion& data)    { return getstring (data)->getdoublevalue(); }
        static string stringtostring (const valueunion& data)            { return *getstring (data); }
        static bool   stringtobool   (const valueunion& data) 
        {
            return getstring (data)->getintvalue() != 0
                   || getstring (data)->trim().equalsignorecase ("true")
                   || getstring (data)->trim().equalsignorecase ("yes");
        }

        static void stringcleanup    (valueunion& data)                     { getstring (data)-> ~string(); }
        static void stringcreatecopy (valueunion& dest, const valueunion& source)   { new (dest.stringvalue) string (*getstring (source)); }

        static bool stringequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            return othertype.tostring (otherdata) == *getstring (data);
        }

        static void stringwritetostream (const valueunion& data, outputstream& output)
        {
            auto* s = getstring (data);
            const size_t len = s->getnumbytesasutf8() + 1;
            heapblock<char> temp (len);
            s->copytoutf8 (temp, len);
            output.writecompressedint ((int) (len + 1));
            output.writebyte (varmarker_string);
            output.write (temp, len);
        }

        constexpr explicit varianttype (stringtag) 
            : isstring      (true),
              iscomparable  (true),
              toint         (stringtoint),
              toint64       (stringtoint64),
              todouble      (stringtodouble),
              tostring      (stringtostring),
              tobool        (stringtobool),
              cleanup       (stringcleanup),
              createcopy    (stringcreatecopy),
              equals        (stringequals),
              writetostream (stringwritetostream) {}

        // object ======================================================================
        static string objecttostring (const valueunion& data)
        {
            return "object 0x" + string::tohexstring ((int) (pointer_sized_int) data.objectvalue);
        }

        static bool                    objecttobool   (const valueunion& data)    { return data.objectvalue != nullptr; }
        static referencecountedobject* objecttoobject (const valueunion& data)    { return data.objectvalue; }

        static Var objectclone (const Var& original)
        {
            if (auto* d = original.getdynamicobject())
                return d->clone().get();

            jassertfalse; // can only clone dynamicobjects!
            return {};
        }

        static void objectcleanup (valueunion& data)    { if (data.objectvalue != nullptr) data.objectvalue->decreferencecount(); }

        static void objectcreatecopy (valueunion& dest, const valueunion& source)
        {
            dest.objectvalue = source.objectvalue;
            if (dest.objectvalue != nullptr)
                dest.objectvalue->increferencecount();
        }

        static bool objectequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            return othertype.toobject (otherdata) == data.objectvalue;
        }

        static void objectwritetostream (const valueunion&, outputstream& output)
        {
            jassertfalse; // can't write an object to a stream!
            output.writecompressedint (0);
        }

        constexpr explicit varianttype (objecttag) 
            : isobject      (true),
              tostring      (objecttostring),
              tobool        (objecttobool),
              toobject      (objecttoobject),
              clone         (objectclone),
              cleanup       (objectcleanup),
              createcopy    (objectcreatecopy),
              equals        (objectequals),
              writetostream (objectwritetostream) {}

        // array =======================================================================
        static string                  arraytostring (const valueunion&)            { return "[array]"; }
        static referencecountedobject* arraytoobject (const valueunion&)    { return nullptr; }

        static array<Var>* arraytoarray (const valueunion& data) 
        {
            if (auto* a = dynamic_cast<refcountedarray*> (data.objectvalue))
                return &(a->array);

            return nullptr;
        }

        static bool arrayequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            auto* thisarray = arraytoarray (data);
            auto* otherarray = othertype.toarray (otherdata);
            return thisarray == otherarray || (thisarray != nullptr && otherarray != nullptr && *otherarray == *thisarray);
        }

        static Var arrayclone (const Var& original)
        {
            array<Var> arraycopy;

            if (auto* array = arraytoarray (original.value))
            {
                arraycopy.ensurestorageallocated (array->size());

                for (auto& i : *array)
                    arraycopy.add (i.clone());
            }

            return Var (arraycopy);
        }

        static void arraywritetostream (const valueunion& data, outputstream& output)
        {
            if (auto* array = arraytoarray (data))
            {
                memoryoutputstream buffer (512);
                buffer.writecompressedint (array->size());

                for (auto& i : *array)
                    i.writetostream (buffer);

                output.writecompressedint (1 + (int) buffer.getdatasize());
                output.writebyte (varmarker_array);
                output << buffer;
            }
        }

        struct refcountedarray  : public referencecountedobject
        {
            refcountedarray (const array<Var>& a)  : array (a)  { increferencecount(); }
            refcountedarray (array<Var>&& a)  : array (std::move (a)) { increferencecount(); }
            array<Var> array;
        };

        constexpr explicit varianttype (arraytag) 
            : isobject      (true),
              isarray       (true),
              tostring      (arraytostring),
              tobool        (objecttobool),
              toobject      (arraytoobject),
              toarray       (arraytoarray),
              clone         (arrayclone),
              cleanup       (objectcleanup),
              createcopy    (objectcreatecopy),
              equals        (arrayequals),
              writetostream (arraywritetostream) {}

        // binary ======================================================================
        static void binarycleanup    (valueunion& data)                     { delete data.binaryvalue; }
        static void binarycreatecopy (valueunion& dest, const valueunion& source)   { dest.binaryvalue = new memoryblock (*source.binaryvalue); }

        static string       binarytostring (const valueunion& data)            { return data.binaryvalue->tobase64encoding(); }
        static memoryblock* binarytobinary (const valueunion& data)    { return data.binaryvalue; }

        static bool binaryequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            const memoryblock* const otherblock = othertype.tobinary (otherdata);
            return otherblock != nullptr && *otherblock == *data.binaryvalue;
        }

        static void binarywritetostream (const valueunion& data, outputstream& output)
        {
            output.writecompressedint (1 + (int) data.binaryvalue->getsize());
            output.writebyte (varmarker_binary);
            output << *data.binaryvalue;
        }

        constexpr explicit varianttype (binarytag) 
            : isbinary      (true),
              tostring      (binarytostring),
              tobinary      (binarytobinary),
              cleanup       (binarycleanup),
              createcopy    (binarycreatecopy),
              equals        (binaryequals),
              writetostream (binarywritetostream) {}

        // method ======================================================================
        static void methodcleanup    (valueunion& data)                     { if (data.methodvalue != nullptr ) delete data.methodvalue; }
        static void methodcreatecopy (valueunion& dest, const valueunion& source)   { dest.methodvalue = new nativefunction (*source.methodvalue); }

        static string methodtostring (const valueunion&)                 { return "method"; }
        static bool   methodtobool   (const valueunion& data)    { return data.methodvalue != nullptr; }

        static bool methodequals (const valueunion& data, const valueunion& otherdata, const varianttype& othertype) 
        {
            return othertype.ismethod && otherdata.methodvalue == data.methodvalue;
        }

        static void methodwritetostream (const valueunion&, outputstream& output)
        {
            jassertfalse; // can't write a method to a stream!
            output.writecompressedint (0);
        }

        constexpr explicit varianttype (methodtag) 
            : ismethod      (true),
              tostring      (methodtostring),
              tobool        (methodtobool),
              cleanup       (methodcleanup),
              createcopy    (methodcreatecopy),
              equals        (methodequals),
              writetostream (methodwritetostream) {}
    }

    struct instance
    {
        static constexpr varianttype attributesvoid           { varianttype::voidtag{} };
        static constexpr varianttype attributesundefined      { varianttype::undefinedtag{} };
        static constexpr varianttype attributesint            { varianttype::inttag{} };
        static constexpr varianttype attributesint64          { varianttype::int64tag{} };
        static constexpr varianttype attributesbool           { varianttype::booltag{} };
        static constexpr varianttype attributesdouble         { varianttype::doubletag{} };
        static constexpr varianttype attributesmethod         { varianttype::methodtag{} };
        static constexpr varianttype attributesarray          { varianttype::arraytag{} };
        static constexpr varianttype attributesstring         { varianttype::stringtag{} };
        static constexpr varianttype attributesbinary         { varianttype::binarytag{} };
        static constexpr VariantType attributesObject         { VariantType::ObjectTag{} };
    }

    constexpr Var::VariantType Var::Instance::attributesVoid;
    constexpr Var::VariantType Var::Instance::attributesUndefined;
    constexpr Var::VariantType Var::Instance::attributesInt;
    constexpr Var::VariantType Var::Instance::attributesInt64;
    constexpr Var::VariantType Var::Instance::attributesBool;
    constexpr Var::VariantType Var::Instance::attributesDouble;
    constexpr Var::VariantType Var::Instance::attributesMethod;
    constexpr Var::VariantType Var::Instance::attributesArray;
    constexpr Var::VariantType Var::Instance::attributesString;
    constexpr Var::VariantType Var::Instance::attributesBinary;
    constexpr Var::VariantType Var::Instance::attributesObject;

    Var::Var()  : type (&Instance::attributesVoid) {}
    Var::Var (const VariantType& t)   : type (&t) {}
    Var::~Var()   { type->cleanUp (value); }

    ALOE_DECLARE_DEPRECATED_STATIC (const Var Var::null;)

    Var::Var (const Var& valueToCopy)  : type (valueToCopy.type)
    {
        type->createCopy (value, valueToCopy.value);
    }

    Var::Var (const int v)        : type (&Instance::attributesInt)    { value.intValue = v; }
    Var::Var (const int64 v)      : type (&Instance::attributesInt64)  { value.int64Value = v; }
    Var::Var (const bool v)       : type (&Instance::attributesBool)   { value.boolValue = v; }
    Var::Var (const double v)     : type (&Instance::attributesDouble) { value.doubleValue = v; }
    Var::Var (NativeFunction m)   : type (&Instance::attributesMethod) { value.methodValue = new NativeFunction (m); }
    Var::Var (const Vec<Var>& v)        : type (&Instance::attributesArray)  { value.objectValue = new VariantType::RefCountedArray (v); }
    Var::Var (const String& v)            : type (&Instance::attributesString) { new (value.stringValue) String (v); }
    Var::Var (const char* const v)        : type (&Instance::attributesString) { new (value.stringValue) String (v); }
    Var::Var (const wchar_t* const v)     : type (&Instance::attributesString) { new (value.stringValue) String (v); }
    Var::Var (const void* v, size_t sz)   : type (&Instance::attributesBinary) { value.binaryValue = new MemoryBlock (v, sz); }
    Var::Var (const MemoryBlock& v)       : type (&Instance::attributesBinary) { value.binaryValue = new MemoryBlock (v); }

    Var::Var (const StringArray& v)       : type (&Instance::attributesArray)
    {
        Vec<Var> strings;
        strings.ensureStorageAllocated (v.size());

        for (auto& i : v)
            strings.add (Var (i));

        value.objectValue = new VariantType::RefCountedArray (strings);
    }

    Var::Var (ReferenceCountedObject* const object)  : type (&Instance::attributesObject)
    {
        value.objectValue = object;

        if (object != nullptr)
            object->incReferenceCount();
    }

    Var Var::undefined()            { return Var (Instance::attributesUndefined); }

    bool Var::isVoid() const        { return type->isVoid; }
    bool Var::isUndefined() const   { return type->isUndefined; }
    bool Var::isInt() const         { return type->isInt; }
    bool Var::isInt64() const       { return type->isInt64; }
    bool Var::isBool() const        { return type->isBool; }
    bool Var::isDouble() const      { return type->isDouble; }
    bool Var::isString() const      { return type->isString; }
    bool Var::isObject() const      { return type->isObject; }
    bool Var::isArray() const       { return type->isArray; }
    bool Var::isBinaryData() const  { return type->isBinary; }
    bool Var::isMethod() const      { return type->isMethod; }

    Var::operator int() const                       { return type->toInt (value); }
    Var::operator int64() const                     { return type->toInt64 (value); }
    Var::operator bool() const                      { return type->toBool (value); }
    Var::operator float() const                     { return (float) type->toDouble (value); }
    Var::operator double() const                    { return type->toDouble (value); }
    String Var::toString() const                            { return type->toString (value); }
    Var::operator String() const                            { return type->toString (value); }
    ReferenceCountedObject* Var::getObject() const  { return type->toObject (value); }
    Vec<Var>* Var::getArray() const               { return type->toArray (value); }
    MemoryBlock* Var::getBinaryData() const         { return type->toBinary (value); }
    DynamicObject* Var::getDynamicObject() const    { return dynamic_cast<DynamicObject*> (getObject()); }

    void Var::swapWith (Var& other) 
    {
        std::swap (type, other.type);
        std::swap (value, other.value);
    }

    Var& Var::operator= (const Var& v)               { type->cleanUp (value); type = v.type; type->createCopy (value, v.value); return *this; }
    Var& Var::operator= (const int v)                { type->cleanUp (value); type = &Instance::attributesInt; value.intValue = v; return *this; }
    Var& Var::operator= (const int64 v)              { type->cleanUp (value); type = &Instance::attributesInt64; value.int64Value = v; return *this; }
    Var& Var::operator= (const bool v)               { type->cleanUp (value); type = &Instance::attributesBool; value.boolValue = v; return *this; }
    Var& Var::operator= (const double v)             { type->cleanUp (value); type = &Instance::attributesDouble; value.doubleValue = v; return *this; }
    Var& Var::operator= (const char* const v)        { type->cleanUp (value); type = &Instance::attributesString; new (value.stringValue) String (v); return *this; }
    Var& Var::operator= (const wchar_t* const v)     { type->cleanUp (value); type = &Instance::attributesString; new (value.stringValue) String (v); return *this; }
    Var& Var::operator= (const String& v)            { type->cleanUp (value); type = &Instance::attributesString; new (value.stringValue) String (v); return *this; }
    Var& Var::operator= (const MemoryBlock& v)       { type->cleanUp (value); type = &Instance::attributesBinary; value.binaryValue = new MemoryBlock (v); return *this; }
    Var& Var::operator= (const Vec<Var>& v)        { Var v2 (v); swapWith (v2); return *this; }
    Var& Var::operator= (ReferenceCountedObject* v)  { Var v2 (v); swapWith (v2); return *this; }
    Var& Var::operator= (NativeFunction v)           { Var v2 (v); swapWith (v2); return *this; }

    Var::Var (Var&& other) 
        : type (other.type),
          value (other.value)
    {
        other.type = &Instance::attributesVoid;
    }

    Var& Var::operator= (Var&& other) 
    {
        swapWith (other);
        return *this;
    }

    Var::Var (String&& v)  : type (&Instance::attributesString)
    {
        new (value.stringValue) String (std::move (v));
    }

    Var::Var (MemoryBlock&& v)  : type (&Instance::attributesBinary)
    {
        value.binaryValue = new MemoryBlock (std::move (v));
    }

    Var::Var (Vec<Var>&& v)  : type (&Instance::attributesArray)
    {
        value.objectValue = new VariantType::RefCountedArray (std::move (v));
    }

    Var& Var::operator= (String&& v)
    {
        type->cleanUp (value);
        type = &Instance::attributesString;
        new (value.stringValue) String (std::move (v));
        return *this;
    }

    bool Var::equals (const Var& other) const 
    {
        return type->equals (value, other.value, *other.type);
    }

    bool Var::equalsWithSameType (const Var& other) const 
    {
        return hasSameTypeAs (other) && equals (other);
    }

    bool Var::hasSameTypeAs (const Var& other) const 
    {
        return type == other.type;
    }

    bool canCompare (const Var& v1, const Var& v2)
    {
        return v1.type->isComparable && v2.type->isComparable;
    }

    static int compare (const Var& v1, const Var& v2)
    {
        if (v1.isString() && v2.isString())
            return v1.toString().compare (v2.toString());

        auto diff = static_cast<double> (v1) - static_cast<double> (v2);
        return diff == 0 ? 0 : (diff < 0 ? -1 : 1);
    }

    bool operator==(const Var& v1, const Var& v2)     { return v1.equals (v2); }
    bool operator!= (const Var& v1, const Var& v2)     { return ! v1.equals (v2); }
    */
}

*/

impl Eq for Var {}

impl Ord for Var {
    
    #[inline] fn cmp(&self, other: &Var) -> std::cmp::Ordering {
        todo!();
        /*
            return canCompare (v1, v2) && compare (v1, v2) <  0;
        */
    }
}

impl PartialEq<Var> for Var { 

    #[inline] fn eq(&self, other: &Var) -> bool {
        todo!();
    }
}

impl PartialOrd<Var> for Var {
    
    #[inline] fn partial_cmp(&self, other: &Var) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

lazy_static!{
    /*
    bool operator== (const Var& v1, const String& v2)  { return v1.toString() == v2; }
    bool operator== (const Var& v1, const char* v2)    { return v1.toString() == v2; }
    */
}

impl Var {
    
    pub fn clone(&self) -> Var {
        
        todo!();
        /*
            return type->clone (*this);
        */
    }
}

impl Index<&Identifier> for Var {

    type Output = Var;
    
    /**
      | If this variant is an object, this returns
      | one of its properties.
      |
      */
    #[inline] fn index(&self, property_name: &Identifier) -> &Self::Output {
        todo!();
        /*
            if (auto* o = getDynamicObject())
            return o->getProperty (propertyName);

        return getNullVarRef();
        */
    }
}

impl Index<*const u8> for Var {

    type Output = Var;
    
    /**
      | If this variant is an object, this returns
      | one of its properties.
      |
      */
    #[inline] fn index(&self, property_name: *const u8) -> &Self::Output {
        todo!();
        /*
            return operator[] (Identifier (propertyName));
        */
    }
}


impl Var {
    
    pub fn get_property(&self, 
        property_name:        &Identifier,
        default_return_value: &Var) -> Var {
        
        todo!();
        /*
            if (auto* o = getDynamicObject())
            return o->getProperties().getWithDefault (propertyName, defaultReturnValue);

        return defaultReturnValue;
        */
    }
    
    pub fn has_property(&self, property_name: &Identifier) -> bool {
        
        todo!();
        /*
            if (auto* o = getDynamicObject())
            return o->hasProperty (propertyName);

        return false;
        */
    }
    
    pub fn get_native_function(&self) -> VarNativeFunction {
        
        todo!();
        /*
            return isMethod() && (value.methodValue != nullptr) ? *value.methodValue : nullptr;
        */
    }
    
    pub fn invoke(&self, 
        method:        &Identifier,
        arguments:     *const Var,
        num_arguments: i32) -> Var {
        
        todo!();
        /*
            if (auto* o = getDynamicObject())
            return o->invokeMethod (method, Var::NativeFunctionArgs (*this, arguments, numArguments));

        return {};
        */
    }
    
    pub fn call0(&self, method: &Identifier) -> Var {
        
        todo!();
        /*
            return invoke (method, nullptr, 0);
        */
    }
    
    pub fn call1(&self, 
        method: &Identifier,
        arg1:   &Var) -> Var {
        
        todo!();
        /*
            return invoke (method, &arg1, 1);
        */
    }
    
    pub fn call2(&self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var) -> Var {
        
        todo!();
        /*
            Var args[] = { arg1, arg2 };
        return invoke (method, args, 2);
        */
    }
    
    pub fn call3(&mut self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var,
        arg3:   &Var) -> Var {
        
        todo!();
        /*
            Var args[] = { arg1, arg2, arg3 };
        return invoke (method, args, 3);
        */
    }
    
    pub fn call4(&self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var,
        arg3:   &Var,
        arg4:   &Var) -> Var {
        
        todo!();
        /*
            Var args[] = { arg1, arg2, arg3, arg4 };
        return invoke (method, args, 4);
        */
    }
    
    pub fn call5(&self, 
        method: &Identifier,
        arg1:   &Var,
        arg2:   &Var,
        arg3:   &Var,
        arg4:   &Var,
        arg5:   &Var) -> Var {
        
        todo!();
        /*
            Var args[] = { arg1, arg2, arg3, arg4, arg5 };
        return invoke (method, args, 5);
        */
    }
    
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            if (auto array = getArray())
            return array->size();

        return 0;
        */
    }
}

/**
  | If the Var is an array, this can be used
  | to return one of its elements.
  | 
  | To call this method, you must make sure
  | that the Var is actually an array, and
  | that the index is a valid number. If these
  | conditions aren't met, behaviour is
  | undefined.
  | 
  | For more control over the array's contents,
  | you can call getArray() and manipulate
  | it directly as an Vec\<Var\>.
  |
  */
impl Index<i32> for Var {

    type Output = Var;
    
    #[inline] fn index(&self, array_index: i32) -> &Self::Output {
        todo!();
        /*
            auto array = getArray();

        // When using this method, the Var must actually be an array, and the index
        // must be in-range!
        jassert (array != nullptr && isPositiveAndBelow (arrayIndex, array->size()));

        return array->getReference (arrayIndex);
        */
    }
}

/**
  | If the Var is an array, this can be used
  | to return one of its elements.
  | 
  | To call this method, you must make sure
  | that the Var is actually an array, and
  | that the index is a valid number. If these
  | conditions aren't met, behaviour is
  | undefined.
  | 
  | For more control over the array's contents,
  | you can call getArray() and manipulate
  | it directly as an Vec\<Var\>.
  |
  */
impl IndexMut<i32> for Var {
    
    #[inline] fn index_mut(&mut self, array_index: i32) -> &mut Self::Output {
        todo!();
        /*
            auto array = getArray();

        // When using this method, the Var must actually be an array, and the index
        // must be in-range!
        jassert (array != nullptr && isPositiveAndBelow (arrayIndex, array->size()));

        return array->getReference (arrayIndex);
        */
    }
}

impl Var {
    
    pub fn convert_to_array(&mut self) -> *mut Vec<Var> {
        
        todo!();
        /*
            if (auto array = getArray())
            return array;

        Vec<Var> tempVar;

        if (! isVoid())
            tempVar.add (*this);

        *this = tempVar;
        return getArray();
        */
    }
    
    pub fn append(&mut self, n: &Var)  {
        
        todo!();
        /*
            convertToArray()->add (n);
        */
    }
    
    pub fn remove(&mut self, index: i32)  {
        
        todo!();
        /*
            if (auto array = getArray())
            array->remove (index);
        */
    }
    
    pub fn insert(&mut self, 
        index: i32,
        n:     &Var)  {
        
        todo!();
        /*
            convertToArray()->insert (index, n);
        */
    }
    
    pub fn resize(&mut self, num_array_elements_wanted: i32)  {
        
        todo!();
        /*
            convertToArray()->resize (numArrayElementsWanted);
        */
    }
    
    pub fn index_of(&self, n: &Var) -> i32 {
        
        todo!();
        /*
            if (auto array = getArray())
            return array->indexOf (n);

        return -1;
        */
    }
    
    pub fn write_to_stream<W: Write>(&self, output: &mut W)  {
        
        todo!();
        /*
            type->writeToStream (value, output);
        */
    }
    
    pub fn read_from_stream<R: Read>(&mut self, input: &mut R) -> Var {
        
        todo!();
        /*
            const int numBytes = input.readCompressedInt();

        if (numBytes > 0)
        {
            switch (input.readByte())
            {
                case varMarker_Int:         return Var (input.readInt());
                case varMarker_Int64:       return Var (input.readInt64());
                case varMarker_BoolTrue:    return Var (true);
                case varMarker_BoolFalse:   return Var (false);
                case varMarker_Double:      return Var (input.readDouble());

                case varMarker_String:
                {
                    MemoryOutputStream mo;
                    mo.writeFromInputStream (input, numBytes - 1);
                    return Var (mo.toUTF8());
                }

                case varMarker_Binary:
                {
                    MemoryBlock mb ((size_t) numBytes - 1);

                    if (numBytes > 1)
                    {
                        const int numRead = input.read (mb.getData(), numBytes - 1);
                        mb.setSize ((size_t) numRead);
                    }

                    return Var (mb);
                }

                case varMarker_Array:
                {
                    Var v;
                    auto* destArray = v.convertToArray();

                    for (int i = input.readCompressedInt(); --i >= 0;)
                        destArray->add (readFromStream (input));

                    return v;
                }

                default:
                    input.skipNextBytes (numBytes - 1); break;
            }
        }

        return {};
        */
    }
}

impl<'a> VarNativeFunctionArgs<'a> {
    
    pub fn new(
        t:        &Var,
        args:     *const Var,
        num_args: i32) -> Self {
    
        todo!();
        /*
        : this_object(t),
        : arguments(args),
        : num_arguments(numArgs),

        
        */
    }
}

