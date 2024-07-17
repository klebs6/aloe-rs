crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/x11/aloe_linux_X11_Symbols.h]

pub mod return_helpers {

    use super::*;

    lazy_static!{
        /*
        template <typename Type>
            Type returnDefaultConstructedAnyType()               { return {}; }

            template <>
            inline void returnDefaultConstructedAnyType<void>()  {}
        */
    }
}

#[macro_export]
macro_rules! aloe_generate_function_with_default {
    ($functionName:ident, 
     $objectName:ident, 
     ($($args:ty),*), 
     $returnType:ty) => {
        /*
        
            using functionName      = returnType (*) args; 
            functionName objectName = [] args -> returnType  { return ReturnHelpers::returnDefaultConstructedAnyType<returnType>(); };
        */
    }
}

pub mod x11_symbol_helpers {
    use super::*;

    pub struct SymbolBinding<'a, FuncPtr> {
        func: &'a mut FuncPtr,
        name: *const u8,
    }

    pub fn make_symbol_binding<FuncPtr>(
            func: &mut FuncPtr,
            name: *const u8) -> SymbolBinding<FuncPtr> {

        todo!();
            /*
                return { func, name };
            */

    }

    pub fn load_symbols<FuncPtr>(
            lib:     &mut DynamicLibrary,
            binding: SymbolBinding<FuncPtr>) -> bool {

        todo!();
            /*
                if (auto* func = lib.getFunction (binding.name))
               {
                   binding.func = reinterpret_cast<FuncPtr> (func);
                   return true;
               }

               return false;
            */

    }

    pub fn load_symbols2<FuncPtr, Args>(
            lib1:    &mut DynamicLibrary,
            lib2:    &mut DynamicLibrary,
            binding: SymbolBinding<FuncPtr>) -> bool {

        todo!();
            /*
                return loadSymbols (lib1, binding) || loadSymbols (lib2, binding);
            */

    }

    pub fn load_symbols_with_args<FuncPtr, Args>(
            lib:     &mut DynamicLibrary,
            binding: SymbolBinding<FuncPtr>,
            args:    Args) -> bool {

        todo!();
            /*
                return loadSymbols (lib, binding) && loadSymbols (lib, args...);
            */

    }

    pub fn load_symbols2_with_args<FuncPtr, Args>(
            lib1:    &mut DynamicLibrary,
            lib2:    &mut DynamicLibrary,
            binding: SymbolBinding<FuncPtr>,
            args:    Args) -> bool {

        todo!();
            /*
                return loadSymbols (lib1, lib2, binding) && loadSymbols (lib1, lib2, args...);
            */

    }
}
