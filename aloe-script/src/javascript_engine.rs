crate::ix!();

/**
  | A simple javascript interpreter!
  | 
  | It's not fully standards-compliant,
  | and won't be as fast as the fancy JIT-compiled
  | engines that you get in browsers, but
  | this is an extremely compact, low-overhead
  | javascript interpreter, which is integrated
  | with the aloe Var and DynamicObject
  | classes. If you need a few simple bits
  | of scripting in your app, and want to
  | be able to easily let the JS work with
  | native objects defined as DynamicObject
  | subclasses, then this might do the job.
  | 
  | To use, simply create an instance of
  | this class and call execute() to run
  | your code. Variables that the script
  | sets can be retrieved with evaluate(),
  | and if you need to provide native objects
  | for the script to use, you can add them
  | with registerNativeObject().
  | 
  | One caveat: Because the values and objects
  | that the engine works with are DynamicObject
  | and Var objects, they use reference-counting
  | rather than garbage-collection, so
  | if your script creates complex connections
  | between objects, you run the risk of
  | creating cyclic dependencies and hence
  | leaking.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct JavascriptEngine {

    /**
      | This value indicates how long a call
      | to one of the evaluate methods is permitted
      | to run before timing-out and failing.
      | 
      | The default value is a number of seconds,
      | but you can change this to whatever value
      | suits your application.
      |
      */
    maximum_execution_time: RelativeTime,
    root:                   ReferenceCountedObjectPtr<JavascriptEngineRootObject>,
}

impl Default for JavascriptEngine {
    
    /**
      | Creates an instance of the engine.
      | 
      | This creates a root namespace and defines
      | some basic Object, String, Vec and
      | Math library methods.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/javascript/aloe_Javascript.cpp]
impl JavascriptEngine {

    pub fn new() -> Self {
    
        todo!();
        /*


            : maximumExecutionTime (15.0), root (new JavascriptEngineRootObject())

        registerNativeObject (JavascriptEngineRootObject::ObjectClass  ::getClassName(),  new JavascriptEngineRootObject::ObjectClass());
        registerNativeObject (JavascriptEngineRootObject::ArrayClass   ::getClassName(),  new JavascriptEngineRootObject::ArrayClass());
        registerNativeObject (JavascriptEngineRootObject::StringClass  ::getClassName(),  new JavascriptEngineRootObject::StringClass());
        registerNativeObject (JavascriptEngineRootObject::MathClass    ::getClassName(),  new JavascriptEngineRootObject::MathClass());
        registerNativeObject (JavascriptEngineRootObject::JSONClass    ::getClassName(),  new JavascriptEngineRootObject::JSONClass());
        registerNativeObject (JavascriptEngineRootObject::IntegerClass ::getClassName(),  new JavascriptEngineRootObject::IntegerClass());
        */
    }
    
    pub fn prepare_timeout(&self)  {
        
        todo!();
        /*
            root->timeout = Time::getCurrentTime() + maximumExecutionTime;
        */
    }
    
    /**
      | When called from another thread, causes
      | the interpreter to time-out as soon
      | as possible
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            root->timeout = {};
        */
    }
    
    /**
      | Adds a native object to the root namespace.
      | 
      | The object passed-in is reference-counted,
      | and will be retained by the engine until
      | the engine is deleted. The name must
      | be a simple JS identifier, without any
      | dots.
      |
      */
    pub fn register_native_object(&mut self, 
        name:   &Identifier,
        object: *mut DynamicObject)  {
        
        todo!();
        /*
            root->setProperty (name, object);
        */
    }
    
    /**
      | Attempts to parse and run a block of javascript
      | code.
      | 
      | If there's a parse or execution error,
      | the error description is returned in
      | the result.
      | 
      | You can specify a maximum time for which
      | the program is allowed to run, and it'll
      | return with an error message if this
      | time is exceeded.
      |
      */
    pub fn execute(&mut self, code: &String) -> Result<(),()> {
        
        todo!();
        /*
            try
        {
            prepareTimeout();
            root->execute (code);
        }
        catch (String& error)
        {
            return Result::fail (error);
        }

        return Result::ok();
        */
    }
    
    /**
      | Attempts to parse and run a javascript
      | expression, and returns the result.
      | 
      | If there's a syntax error, or the expression
      | can't be evaluated, the return value
      | will be Var::undefined(). The errorMessage
      | parameter gives you a way to find out
      | any parsing errors.
      | 
      | You can specify a maximum time for which
      | the program is allowed to run, and it'll
      | return with an error message if this
      | time is exceeded.
      |
      */
    pub fn evaluate(
        &mut self, 
        code:   &String,
        result: *mut Result<(),()>

    ) -> Var {
        
        todo!();
        /*
            try
        {
            prepareTimeout();
            if (result != nullptr) *result = Result::ok();
            return root->evaluate (code);
        }
        catch (String& error)
        {
            if (result != nullptr) *result = Result::fail (error);
        }

        return Var::undefined();
        */
    }
    
    /**
      | Calls a function in the root namespace,
      | and returns the result.
      | 
      | The function arguments are passed in
      | the same format as used by native methods
      | in the Var class.
      |
      */
    pub fn call_function(
        &mut self, 
        function: &Identifier,
        args:     &VarNativeFunctionArgs,
        result:   *mut Result<(),()>

    ) -> Var {
        
        todo!();
        /*
            auto returnVal = Var::undefined();

        try
        {
            prepareTimeout();
            if (result != nullptr) *result = Result::ok();
            JavascriptEngineRootObject::Scope ({}, *root, *root).findAndInvokeMethod (function, args, returnVal);
        }
        catch (String& error)
        {
            if (result != nullptr) *result = Result::fail (error);
        }

        return returnVal;
        */
    }
    
    /**
      | Calls a function object in the namespace
      | of a dynamic object, and returns the
      | result.
      | 
      | The function arguments are passed in
      | the same format as used by native methods
      | in the Var class.
      |
      */
    pub fn call_function_object(
        &mut self, 
        object_scope:    *mut DynamicObject,
        function_object: &Var,
        args:            &VarNativeFunctionArgs,
        result:          *mut Result<(),()>

    ) -> Var {
        
        todo!();
        /*
            auto returnVal = Var::undefined();

        try
        {
            prepareTimeout();
            if (result != nullptr) *result = Result::ok();
            JavascriptEngineRootObject::Scope rootScope ({}, *root, *root);
            JavascriptEngineRootObject::Scope (&rootScope, *root, DynamicObject::Ptr (objectScope))
                .invokeMethod (functionObject, args, returnVal);
        }
        catch (String& error)
        {
            if (result != nullptr) *result = Result::fail (error);
        }

        return returnVal;
        */
    }
    
    /**
      | Provides access to the set of properties
      | of the root namespace object.
      |
      */
    pub fn get_root_object_properties(&self) -> &NamedValueSet {
        
        todo!();
        /*
            return root->getProperties();
        */
    }
}
