crate::ix!();

/**
  | comp instance, parameters in forward
  | order
  |
  */
#[cfg(TARGET_OS_MAC)]
#[cfg(__LP64__)]
macro_rules! param {
    ($_typ:ident, 
     $_name:ident, 
     $_index:ident, 
     $_nparams:ident) => {
        /*
        
                    _typ _name = *(_typ *)&params->params[_index + 1];
        */
    }
}

/**
  | parameters in reverse order, then comp
  | instance
  |
  */
#[cfg(TARGET_OS_MAC)]
#[cfg(not(__LP64__))]
macro_rules! param {
    ($_typ:ident, 
     $_name:ident, 
     $_index:ident, 
     $_nparams:ident) => {
        /*
        
                    _typ _name = *(_typ *)&params->params[_nparams - 1 - _index];
        */
    }
}

/**
  | (no comp instance), parameters in forward
  | order
  |
  */
#[cfg(TARGET_OS_WIN32)]
macro_rules! param {
    ($_typ:ident, 
     $_name:ident, 
     $_index:ident, 
     $_nparams:ident) => {
        /*
        
                    _typ _name = *(_typ *)&params->params[_index];
        */
    }
}

