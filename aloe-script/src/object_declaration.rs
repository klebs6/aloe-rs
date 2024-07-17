crate::ix!();

pub struct ObjectDeclaration {
    base:         Expression,
    names:        Vec<Identifier>,
    initialisers: Vec<Box<Expression>>,
}

impl ObjectDeclaration {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : expression(l),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            DynamicObject::Ptr newObject (new DynamicObject());

            for (int i = 0; i < names.size(); ++i)
                newObject->setProperty (names.getUnchecked(i), initialisers.getUnchecked(i)->getResult (s));

            return newObject.get();
        */
    }
}
