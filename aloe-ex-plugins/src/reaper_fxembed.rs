/*!
 | to support via Vst2: 
 |
 | canDo("hasCockosEmbeddedUI") should return 0xbeef0000
 |
 | dispatcher will be called with 
 | opcode=effVendorSpecific, 
 | index=effEditDraw, 
 | value=parm2, 
 | ptr=(void*)(INT_PTR)parm3, 
 | opt=message (REAPER_FXEMBED_WM_*)
 |
 | to support via Vst3: 
 |
 | IController should support IReaperUIEmbedInterface, 
 |
 | see reaper_vst3_interfaces.h
 |
 | to support via LV2: todo
 */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/extern/reaper_plugin_fx_embed.h]

/* ---------- these alias to win32's WM_*  ---------- */

/** 
  | return 1 if embedding is supported and available
  | return -1 if embedding is supported and unavailable
  | return 0 if embedding is not supported
  */
pub const REAPER_FXEMBED_WM_IS_SUPPORTED: usize = 0x0000;

/**
   called when embedding begins (return value ignored)
  */
pub const REAPER_FXEMBED_WM_CREATE: usize = 0x0001;

/**
   called when embedding ends (return value ignored)
  */
pub const REAPER_FXEMBED_WM_DESTROY: usize = 0x0002;

/**
   alias of REAPER_inline_positioninfo
  */
pub struct REAPER_FXEMBED_DrawInfo 
 {
     /**
       0=unknown (v6.23 and earlier), 1=TCP, 2=MCP
       */
     context:        i32,

     /**
       0=unknown (v6.23 and earlier), otherwise 24.8 fixed point (256=100%)
       */
     dpi:            i32,

     /**
       for REAPER_FXEMBED_WM_MOUSEWHEEL, 120 = step, typically
       */
     mousewheel_amt: i32,

     res2:           f64,
     width:          i32,
     height:         i32,
     mouse_x:        i32,
     mouse_y:        i32,

     /**
       REAPER_FXEMBED_DRAWINFO_FLAG_PAINT_OPTIONAL etc
       */
     flags:          i32,

     res3:           i32,
     spare:          [*mut c_void; 6],
 }

pub const REAPER_FXEMBED_DRAWINFO_FLAG_PAINT_OPTIONAL:   usize = 1;
pub const REAPER_FXEMBED_DRAWINFO_FLAG_LBUTTON_CAPTURED: usize = 0x10000;
pub const REAPER_FXEMBED_DRAWINFO_FLAG_RBUTTON_CAPTURED: usize = 0x20000;

/**
 | draw embedded UI.
 | parm2: REAPER_FXEMBED_IBitmap * to draw into. note
 | parm3: REAPER_FXEMBED_DrawInfo *
 |
 | if flags has
 | REAPER_FXEMBED_DRAWINFO_FLAG_PAINT_OPTIONAL
 | set, update is optional. if no change since
 | last draw, return 0.
 |
 | if flags has
 | REAPER_FXEMBED_DRAWINFO_FLAG_LBUTTON_CAPTURED
 | set, left mouse button is down and captured
 |
 | if flags has
 | REAPER_FXEMBED_DRAWINFO_FLAG_RBUTTON_CAPTURED
 | set, right mouse button is down and captured
 |
 | HiDPI:
 |
 | if
 | REAPER_FXEMBED_IBitmap::Extended(REAPER_FXEMBED_EXT_GET_ADVISORY_SCALING,NULL)
 | returns nonzero, then it is a 24.8 scalefactor
 | for UI drawing
 |
 | return 1 if drawing occurred, 0 otherwise.
 |
 */
pub const REAPER_FXEMBED_WM_PAINT: usize = 0x000F;

/**
  | parm3: REAPER_FXEMBED_DrawInfo*. set mouse
  | cursor and return
  | REAPER_FXEMBED_RETNOTIFY_HANDLED, or return 0.
  */
pub const REAPER_FXEMBED_WM_SETCURSOR: usize = 0x0020;

pub const REAPER_FXEMBED_WM_GETMINMAXINFO: usize = 0x0024;

/**
  | get size hints. parm3 = (REAPER_FXEMBED_SizeHints*).
  | return 1 if supported note that these
  | are just hints, the actual size may vary
  |
  */
// alias to MINMAXINFO
pub struct REAPER_FXEMBED_SizeHints { 

    /**
       16.16 fixed point (65536 = 1:1, 32768 = 1:2, etc)
      */
    preferred_aspect: i32,

    /**
       16.16 fixed point
      */
    minimum_aspect:   i32,

    res1:             i32,
    res2:             i32,
    res3:             i32,
    res4:             i32,
    min_width:        i32,
    min_height:       i32,
    max_width:        i32,
    max_height:       i32,
}

/**
 | mouse messages
 |
 | parm3 = (REAPER_FXEMBED_DrawInfo*)
 |
 | capture is automatically set on mouse down,
 | released on mouse up when not captured, will
 | always receive a mousemove when exiting the
 | window
 */
pub const REAPER_FXEMBED_WM_MOUSEMOVE:     usize = 0x0200;
pub const REAPER_FXEMBED_WM_LBUTTONDOWN:   usize = 0x0201;
pub const REAPER_FXEMBED_WM_LBUTTONUP:     usize = 0x0202;
pub const REAPER_FXEMBED_WM_LBUTTONDBLCLK: usize = 0x0203;
pub const REAPER_FXEMBED_WM_RBUTTONDOWN:   usize = 0x0204;
pub const REAPER_FXEMBED_WM_RBUTTONUP:     usize = 0x0205;
pub const REAPER_FXEMBED_WM_RBUTTONDBLCLK: usize = 0x0206;
pub const REAPER_FXEMBED_WM_MOUSEWHEEL:    usize = 0x020A;

/**
  | REAPER_FXEMBED_WM_SETCURSOR should
  | return REAPER_FXEMBED_RETNOTIFY_HANDLED
  | if a cursor was set
  |
  */
pub const REAPER_FXEMBED_RETNOTIFY_HANDLED: usize = 0x0000001;

/**
  | if the mouse messages return with REAPER_FXEMBED_RETNOTIFY_INVALIDATE
  | set, a non-optional redraw is initiated
  | (generally sooner than the next timer-based
  | redraw)
  |
  */
pub const REAPER_FXEMBED_RETNOTIFY_INVALIDATE: usize = 0x1000000;

/**
 | bitmap interface
 |
 | this is an alias of LICE_IBitmap etc from
 | WDL/lice/lice.h
 |
 */
macro_rules! reaper_fxembed_rgba {
    ($r:ident, 
     $g:ident, 
     $b:ident, 
     $a:ident) => {
        /*
                (((b)&0xff)|(((g)&0xff)<<8)|(((r)&0xff)<<16)|(((a)&0xff)<<24))
        */
    }
}

macro_rules! reaper_fxembed_getb {
    ($v:ident) => {
        /*
                ((v)&0xff)
        */
    }
}

macro_rules! reaper_fxembed_getg {
    ($v:ident) => {
        /*
                (((v)>>8)&0xff)
        */
    }
}

macro_rules! reaper_fxembed_getr {
    ($v:ident) => {
        /*
                (((v)>>16)&0xff)
        */
    }
}

macro_rules! reaper_fxembed_geta {
    ($v:ident) => {
        /*
                (((v)>>24)&0xff)
        */
    }
}

/**
  | alias of LICE_IBitmap
  |
  */
pub trait REAPER_FXEMBED_IBitmap {

    fn get_bits(&mut self) -> *mut u32;

    fn get_width(&mut self) -> i32;

    fn get_height(&mut self) -> i32;

    /**
      | includes any off-bitmap data. this
      | is in sizeof(unsigned int) units, not
      | bytes.
      |
      */
    fn get_row_span(&mut self) -> i32;

    fn is_flipped(&mut self) -> bool { false }

    fn resize(&mut self, w: i32, h: i32) -> bool;

    /**
      | do not use
      |
      */
    fn getdc(&mut self)  {
        
        todo!();
        /*
            return 0;
        */
    }

    fn extended(
        &mut self, 
        id:   i32,
        data: *mut c_void

    ) -> usize {
        
        todo!();
        /*
            return 0;
        */
    }
}

/**
   data ignored, returns .8 fixed point. returns
   0 if unscaled
  */
pub const REAPER_FXEMBED_EXT_GET_ADVISORY_SCALING: usize = 0x2003;


