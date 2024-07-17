crate::ix!();

/**
  | This class is used by the script, and
  | provides methods that the JS can call.
  |
  */
#[no_copy]
#[leak_detector]
pub struct DemoClass<'a> {
    base:  DynamicObject,
    owner: &'a mut JavaScriptDemo<'a>,
}

impl<'a> DemoClass<'a> {

    pub fn new(demo: &mut JavaScriptDemo) -> Self {
    
        todo!();
        /*
        : owner(demo),

            setMethod ("print", print);
        */
    }
    
    pub fn get_class_name() -> Identifier {
        
        todo!();
        /*
            return "Demo";
        */
    }
    
    pub fn print(args: &VarNativeFunctionArgs) -> Var {
        
        todo!();
        /*
            if (args.numArguments > 0)
                    if (auto* thisObject = dynamic_cast<DemoClass*> (args.thisObject.getObject()))
                        thisObject->owner.consoleOutput (args.arguments[0].toString());

                return var::undefined();
        */
    }
}
