crate::ix!();

#[cfg(SMTG_OS_WINDOWS)]
#[cfg(_MSC_VER)]
#[cfg(DEVELOPMENT)]
macro_rules! malloc {
    ($s:ident) => {
        /*
                _malloc_dbg(s, _NORMAL_BLOCK, __FILE__, __LINE__)
        */
    }
}

#[cfg(SMTG_OS_WINDOWS)]
#[cfg(_MSC_VER)]
#[cfg(DEVELOPMENT)]
macro_rules! realloc {
    ($p:ident, $s:ident) => {
        /*
                _realloc_dbg(p,s,  _NORMAL_BLOCK, __FILE__, __LINE__)
        */
    }
}

#[cfg(SMTG_OS_WINDOWS)]
#[cfg(_MSC_VER)]
#[cfg(DEVELOPMENT)]
macro_rules! free {
    ($p:ident) => {
        /*
                _free_dbg(p, _NORMAL_BLOCK)
        */
    }
}
