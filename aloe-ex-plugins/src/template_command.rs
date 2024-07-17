crate::ix!();

pub struct TemplateCommand<Proc,Func> {
    base2: Func,
    _0:    PhantomData<Proc>,
}

impl<Proc,Func> Command<Proc> for TemplateCommand<Proc,Func> {
    
    fn run(&mut self, proc: &mut Proc)  {
        
        todo!();
        /*
            (*this) (proc);
        */
    }
}

impl<Proc,Func> TemplateCommand<Proc,Func> {

    pub fn new<FuncPrime>(func_prime: FuncPrime) -> Self {
    
        todo!();
        /*


            : Func (std::forward<FuncPrime> (funcPrime))
        */
    }
}
