crate::ix!();

pub struct ArrayClass {
    base: DynamicObject,
}

impl Default for ArrayClass {
    
    fn default() -> Self {
        todo!();
        /*


            setMethod ("contains", contains);
            setMethod ("remove",   remove);
            setMethod ("join",     join);
            setMethod ("push",     push);
            setMethod ("splice",   splice);
            setMethod ("indexOf",  indexOf);
        */
    }
}

impl ArrayClass {
    
    pub fn get_class_name() -> Identifier {
        
        todo!();
        /*
            static const Identifier i ("Vec"); return i;
        */
    }
    
    pub fn contains(a: Args) -> Var {
        
        todo!();
        /*
            if (auto* array = a.thisObject.getArray())
                return array->contains (get (a, 0));

            return false;
        */
    }
    
    pub fn remove(a: Args) -> Var {
        
        todo!();
        /*
            if (auto* array = a.thisObject.getArray())
                array->removeAllInstancesOf (get (a, 0));

            return Var::undefined();
        */
    }
    
    pub fn join(a: Args) -> Var {
        
        todo!();
        /*
            StringArray strings;

            if (auto* array = a.thisObject.getArray())
                for (auto& v : *array)
                    strings.add (v.toString());

            return strings.joinIntoString (getString (a, 0));
        */
    }
    
    pub fn push(a: Args) -> Var {
        
        todo!();
        /*
            if (auto* array = a.thisObject.getArray())
            {
                for (int i = 0; i < a.numArguments; ++i)
                    array->add (a.arguments[i]);

                return array->size();
            }

            return Var::undefined();
        */
    }
    
    pub fn splice(a: Args) -> Var {
        
        todo!();
        /*
            if (auto* array = a.thisObject.getArray())
            {
                auto arraySize = array->size();
                int start = get (a, 0);

                if (start < 0)
                    start = jmax (0, arraySize + start);
                else if (start > arraySize)
                    start = arraySize;

                const int num = a.numArguments > 1 ? jlimit (0, arraySize - start, getInt (a, 1))
                    : arraySize - start;

                Vec<Var> itemsRemoved;
                itemsRemoved.ensureStorageAllocated (num);

                for (int i = 0; i < num; ++i)
                    itemsRemoved.add (array->getReference (start + i));

                array->removeRange (start, num);

                for (int i = 2; i < a.numArguments; ++i)
                    array->insert (start++, get (a, i));

                // std::move() needed here for older compilers
                ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wredundant-move")
                    return std::move (itemsRemoved);
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE
            }

            return Var::undefined();
        */
    }
    
    pub fn index_of(a: Args) -> Var {
        
        todo!();
        /*
            if (auto* array = a.thisObject.getArray())
            {
                auto target = get (a, 0);

                for (int i = (a.numArguments > 1 ? getInt (a, 1) : 0); i < array->size(); ++i)
                    if (array->getReference(i) == target)
                        return i;
            }

            return -1;
        */
    }
}
