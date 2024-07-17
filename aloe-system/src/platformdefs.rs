crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/system/aloe_PlatformDefs.h]

/**
  | Writes a string to the standard error
  | stream. Note that as well as a single
  | string, you can use this to write multiple
  | items as a stream, e.g.
  | 
  | @code
  | 
  | DBG ("foo = " << foo << "bar = " << bar);
  | 
  | @endcode
  | 
  | The macro is only enabled in a debug build,
  | so be careful not to use it with expressions
  | that have important side-effects!
  | 
  | @see Logger::outputDebugString
  |
  */
#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
macro_rules! dbg {
    ($textToWrite:ident) => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (aloe::String tempDbgBuf; tempDbgBuf << textToWrite; aloe::Logger::outputDebugString (tempDbgBuf);)
        */
    }
}

/**
  | This will always cause an assertion
  | failure.
  | 
  | It is only compiled in a debug build,
  | (unless ALOE_LOG_ASSERTIONS is enabled
  | for your build). @see jassert
  |
  */
#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
macro_rules! jassertfalse {
    () => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (ALOE_LOG_CURRENT_ASSERTION; if (aloe::aloe_isRunningUnderDebugger()) ALOE_BREAK_IN_DEBUGGER; ALOE_ANALYZER_NORETURN)
        */
    }
}

/**
  | Platform-independent assertion macro.
  | 
  | This macro gets turned into a no-op when
  | you're building with debugging turned
  | off, so be careful that the expression
  | you pass to it doesn't perform any actions
  | that are vital for the correct behaviour
  | of your program!
  | 
  | @see jassertfalse
  |
  */
#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
macro_rules! jassert {
    ($expression:ident) => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (if (! (expression)) jassertfalse;)
        */
    }
}

/**
  | Platform-independent assertion macro
  | which suppresses ignored-variable
  | warnings in all build modes. You should
  | probably use a plain jassert() by default,
  | and only replace it with jassertquiet()
  | once you've convinced yourself that
  | any unused-variable warnings emitted
  | by the compiler are harmless.
  |
  */
#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
macro_rules! jassertquiet {
    ($expression:ident) => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (if (! (expression)) jassertfalse;)
        */
    }
}

#[cfg(not(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS)))]
macro_rules! dbg { ($textToWrite:ident) => { } }

#[cfg(not(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS)))]
macro_rules! jassertfalse {
    () => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (ALOE_LOG_CURRENT_ASSERTION)
        */
    }
}

///------------------------
#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
#[cfg(ALOE_LOG_ASSERTIONS)]
macro_rules! jassert {
    ($expression:ident) => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (if (! (expression)) jassertfalse;)
        */
    }
}

#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
#[cfg(ALOE_LOG_ASSERTIONS)]
macro_rules! jassertquiet {
    ($expression:ident) => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (if (! (expression)) jassertfalse;)
        */
    }
}

#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
#[cfg(not(ALOE_LOG_ASSERTIONS))]
macro_rules! jassert {
    ($expression:ident) => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON ( ; )
        */
    }
}

#[cfg(all(ALOE_DEBUG,ALOE_ENABLE_ASSERTIONS))]
#[cfg(not(ALOE_LOG_ASSERTIONS))]
macro_rules! jassertquiet {
    ($expression:ident) => {
        /*
                ALOE_BLOCK_WITH_FORCED_SEMICOLON (if (false) (void) (expression);)
        */
    }
}

