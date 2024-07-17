crate::ix!();

pub struct CommandFifo<Proc> {
    buffer:        Vec<Box<dyn Command<Proc>>>,
    abstract_fifo: AbstractFifo,
}

impl<Proc> Default for CommandFifo<Proc> {
    
    fn default() -> Self {
        todo!();
        /*
        : command_fifo(1024),

        
        */
    }
}

impl<Proc> CommandFifo<Proc> {

    pub fn new(size: i32) -> Self {
    
        todo!();
        /*


            : buffer ((size_t) size),
              abstractFifo (size)
        */
    }
    
    pub fn push<Item>(&mut self, item: Item)  {
    
        todo!();
        /*
            auto command = makeCommand (std::forward<Item> (item));

            abstractFifo.write (1).forEach ([&] (int index)
            {
                buffer[size_t (index)] = std::move (command);
            });
        */
    }
    
    pub fn call(&mut self, proc: &mut Proc)  {
        
        todo!();
        /*
            abstractFifo.read (abstractFifo.getNumReady()).forEach ([&] (int index)
            {
                buffer[size_t (index)]->run (proc);
            });
        */
    }
    
    pub fn make_command<Func>(func: Func) -> Box<dyn Command<Proc>> {
    
        todo!();
        /*
            using Decayed = typename std::decay<Func>::type;
            return std::make_unique<TemplateCommand<Proc, Decayed>> (std::forward<Func> (func));
        */
    }
}
