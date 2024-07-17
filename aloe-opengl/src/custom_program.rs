crate::ix!();

#[no_copy]
#[leak_detector]
pub struct CustomProgram<'a> {
    base:  ReferenceCountedObject,
    base2: ShaderBase<'a>,
}

impl<'a> CustomProgram<'a> {

    pub fn new(
        c:               &mut ShaderContext,
        fragment_shader: &String) -> Self {
    
        todo!();
        /*


            : ShaderBase (c.glState.target.context, fragmentShader.toRawUTF8())
        */
    }
    
    pub fn get(hash_name: &String) -> ReferenceCountedObjectPtr<CustomProgram> {
        
        todo!();
        /*
            if (auto* c = OpenGLContext::getCurrentContext())
                if (auto* o = c->getAssociatedObject (hashName.toRawUTF8()))
                    return *static_cast<CustomProgram*> (o);

            return {};
        */
    }
    
    pub fn get_or_create(
        gc:            &mut dyn LowLevelGraphicsContext,
        hash_name:     &String,
        code:          &String,
        error_message: &mut String

    ) -> ReferenceCountedObjectPtr<CustomProgram<'a>> {
        
        todo!();
        /*
            if (auto c = get (hashName))
                return c;

            if (auto* sc = dynamic_cast<OpenGLRendering::ShaderContext*> (&gc))
            {
                ReferenceCountedObjectPtr<CustomProgram> c (new CustomProgram (*sc, code));
                errorMessage = c->lastError;

                if (errorMessage.isEmpty())
                {
                    if (auto context = OpenGLContext::getCurrentContext())
                    {
                        context->setAssociatedObject (hashName.toRawUTF8(), c.get());
                        return c;
                    }
                }
            }

            return nullptr;
        */
    }
}
