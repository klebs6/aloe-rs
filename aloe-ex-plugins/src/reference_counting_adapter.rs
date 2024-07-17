crate::ix!();

pub struct ReferenceCountingAdapter<Contents> {
    base:     ReferenceCountedObject,
    contents: Contents,
}

impl<Contents> ReferenceCountingAdapter<Contents> {

    pub fn new<Args>(args: Args) -> Self {
    
        todo!();
        /*


            : contents (std::forward<Args> (args)...)
        */
    }
    
    pub fn get(&self) -> &Contents {
        
        todo!();
        /*
            return contents;
        */
    }
    
    pub fn get_mut(&mut self) -> &mut Contents {
        
        todo!();
        /*
            return contents;
        */
    }
}

pub fn make_reference_counted<Contents, Args>(args: Args) -> Box<ReferenceCountingAdapter<Contents>> {

    todo!();
    /*
        auto adapter = new ReferenceCountingAdapter<Contents> (std::forward<Args> (args)...);
        return std::unique_ptr<ReferenceCountingAdapter<Contents>> (adapter);
    */
}
