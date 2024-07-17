crate::ix!();

#[cfg(not(AUTRACE))]
macro_rules! autrace {
    ($code:ident, 
     $obj:ident, 
     $a:ident, 
     $b:ident, 
     $c:ident, 
     $d:ident) => { }
}
