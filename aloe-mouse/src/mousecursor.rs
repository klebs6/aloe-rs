crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseCursor.h]

/**
  | The set of available standard mouse
  | cursors.
  |
  */
pub enum MouseCursorStandardCursorType
{
    /**
      | Indicates that the component's parent's
      | cursor should be used.
      |
      */
    ParentCursor = 0,               

    /**
      | An invisible cursor.
      |
      */
    NoCursor,                       

    /**
      | The standard arrow cursor.
      |
      */
    NormalCursor,                   

    /**
      | The normal hourglass or spinning-beachball
      | 'busy' cursor.
      |
      */
    WaitCursor,                     

    /**
      | A vertical I-beam for positioning within
      | text.
      |
      */
    IBeamCursor,                    

    /**
      | A pair of crosshairs.
      |
      */
    CrosshairCursor,                

    /**
      | The normal arrow cursor, but with a "+"
      | on it to indicate that you're dragging
      | a copy of something.
      |
      */
    CopyingCursor,                  

    /**
      | A hand with a pointing finger, for clicking
      | on web-links.
      |
      */
    PointingHandCursor,             

    /**
      | An open flat hand for dragging heavy
      | objects around.
      |
      */
    DraggingHandCursor,             

    /**
      | An arrow pointing left and right.
      |
      */
    LeftRightResizeCursor,          

    /**
      | an arrow pointing up and down.
      |
      */
    UpDownResizeCursor,             

    /**
      | An arrow pointing up, down, left and
      | right.
      |
      */
    UpDownLeftRightResizeCursor,    

    /**
      | A platform-specific cursor for resizing
      | the top-edge of a window.
      |
      */
    TopEdgeResizeCursor,            

    /**
      | A platform-specific cursor for resizing
      | the bottom-edge of a window.
      |
      */
    BottomEdgeResizeCursor,         

    /**
      | A platform-specific cursor for resizing
      | the left-edge of a window.
      |
      */
    LeftEdgeResizeCursor,           

    /**
      | A platform-specific cursor for resizing
      | the right-edge of a window.
      |
      */
    RightEdgeResizeCursor,          

    /**
      | A platform-specific cursor for resizing
      | the top-left-corner of a window.
      |
      */
    TopLeftCornerResizeCursor,      

    /**
      | A platform-specific cursor for resizing
      | the top-right-corner of a window.
      |
      */
    TopRightCornerResizeCursor,     

    /**
      | A platform-specific cursor for resizing
      | the bottom-left-corner of a window.
      |
      */
    BottomLeftCornerResizeCursor,   

    /**
      | A platform-specific cursor for resizing
      | the bottom-right-corner of a window.
      |
      */
    BottomRightCornerResizeCursor,  

    NumStandardCursorTypes
}

///----------------------
#[no_copy]
#[leak_detector]
pub struct MouseCursorSharedCursorHandle {
    info:          Box<CustomMouseCursorInfo>,
    handle:        *mut c_void,
    ref_count:     Atomic<i32>, // default = { 1  }
    standard_type: MouseCursorStandardCursorType,
    is_standard:   bool,
}

pub mod shared_cursor_handle {

    use super::*;

    lazy_static!{
        /*
        static SpinLock lock;
        SpinLock MouseCursorSharedCursorHandle::lock;
        */
    }
}

impl Drop for MouseCursorSharedCursorHandle {

    fn drop(&mut self) {
        todo!();
        /* 
                deleteMouseCursor (handle, isStandard);
             */
    }
}

impl MouseCursorSharedCursorHandle {

    /**
      | Creates one of the standard mouse cursor
      |
      */
    pub fn new_from_cursor_type(ty: MouseCursorStandardCursorType) -> Self {
    
        todo!();
        /*
        : handle(createStandardMouseCursor (type)),
        : standard_type(type),
        : is_standard(true),

        
        */
    }
    
    pub fn new(
        image:        &Image,
        hot_spot:     Point<i32>,
        scale_factor: f32) -> Self {
    
        todo!();
        /*


            : info (new CustomMouseCursorInfo (image, hotSpot, scaleFactor)),
                  handle (info->create()),
                  standardType (NormalCursor),
                  isStandard (false)

                // your hotspot needs to be within the bounds of the image!
                jassert (image.getBounds().contains (hotSpot));
        */
    }
    
    pub fn create_standard(ty: MouseCursorStandardCursorType) -> *mut MouseCursorSharedCursorHandle {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (type, NumStandardCursorTypes));

                const SpinLock::ScopedLockType sl (lock);
                auto& c = getSharedCursor (type);

                if (c == nullptr)
                    c = new MouseCursorSharedCursorHandle (type);
                else
                    c->retain();

                return c;
        */
    }
    
    pub fn is_standard_type(&self, ty: MouseCursorStandardCursorType) -> bool {
        
        todo!();
        /*
            return type == standardType && isStandard;
        */
    }
    
    pub fn retain(&mut self) -> *mut MouseCursorSharedCursorHandle {
        
        todo!();
        /*
            ++refCount;
                return this;
        */
    }
    
    pub fn release(&mut self)  {
        
        todo!();
        /*
            if (--refCount == 0)
                {
                    if (isStandard)
                    {
                        const SpinLock::ScopedLockType sl (lock);
                        getSharedCursor (standardType) = nullptr;
                    }

                    delete this;
                }
        */
    }
    
    pub fn get_handle(&self)  {
        
        todo!();
        /*
            return handle;
        */
    }
    
    pub fn set_handle(&mut self, new_handle: *mut c_void)  {
        
        todo!();
        /*
            handle = newHandle;
        */
    }
    
    pub fn get_type(&self) -> MouseCursorStandardCursorType {
        
        todo!();
        /*
            return standardType;
        */
    }
    
    pub fn get_custom_info(&self) -> *mut CustomMouseCursorInfo {
        
        todo!();
        /*
            return info.get();
        */
    }
    
    pub fn get_shared_cursor(ty: MouseCursorStandardCursorType) -> &'static mut *mut MouseCursorSharedCursorHandle {
        
        todo!();
        /*
            static MouseCursorSharedCursorHandle* cursors[NumStandardCursorTypes] = {};
                return cursors[type];
        */
    }
}

/**
  | Represents a mouse cursor image.
  | 
  | This object can either be used to represent
  | one of the standard mouse cursor shapes,
  | or a custom one generated from an image.
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
pub struct MouseCursor {
    cursor_handle: *mut MouseCursorSharedCursorHandle, // default = nullptr
}

impl Default for MouseCursor {
    
    /**
      | Creates the standard arrow cursor.
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl Eq for MouseCursor {}

impl Drop for MouseCursor {

    fn drop(&mut self) {
        todo!();
        /* 
        if (cursorHandle != nullptr)
            cursorHandle->release();
         */
    }
}

impl PartialEq<MouseCursor> for MouseCursor {
    
    /**
      | Checks whether two mouse cursors are
      | the same.
      | 
      | For custom cursors, two cursors created
      | from the same image won't be recognised
      | as the same, only MouseCursor objects
      | that have been copied from the same object.
      |
      */
    #[inline] fn eq(&self, other: &MouseCursor) -> bool {
        todo!();
        /*
            return getHandle() == other.getHandle();
        */
    }
}

impl PartialEq<MouseCursorStandardCursorType> for MouseCursor {
    
    /**
      | Checks whether this cursor is of the
      | standard type mentioned.
      |
      */
    #[inline] fn eq(&self, other: &MouseCursorStandardCursorType) -> bool {
        todo!();
        /*
            return cursorHandle != nullptr ? cursorHandle->isStandardType (type)
                                       : (type == NormalCursor);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/mouse/aloe_MouseCursor.cpp]
impl MouseCursor {

    #[cfg(target_os="linux")]
    pub fn delete_mouse_cursor(
        &mut self, 
        cursor_handle: *mut c_void,
        _1:            bool

    ) {
        
        todo!();
        /*
            if (cursorHandle != nullptr)
            XWindowSystem::getInstance()->deleteMouseCursor (cursorHandle);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn create_standard_mouse_cursor(&mut self, ty: MouseCursorStandardCursorType)  {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->createStandardMouseCursor (type);
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn show_in_window(&self, peer: *mut ComponentPeer)  {
        
        todo!();
        /*
            if (peer != nullptr)
            XWindowSystem::getInstance()->showCursor ((::Window) peer->getNativeHandle(), getHandle());
        */

    }

    pub fn new_from_cursor_type(ty: MouseCursorStandardCursorType) -> Self {
    
        todo!();
        /*
            : cursorHandle (type != NormalCursor ? MouseCursorSharedCursorHandle::createStandard (type) : nullptr)
        */
    }
    
    /**
      | Creates a custom cursor from an image.
      | 
      | -----------
      | @param image
      | 
      | the image to use for the cursor - if this
      | is bigger than the system can manage,
      | it might get scaled down first, and might
      | also have to be turned to black-and-white
      | if it can't do colour cursors.
      | ----------
      | @param hotSpotX
      | 
      | the x position of the cursor's hotspot
      | within the image
      | ----------
      | @param hotSpotY
      | 
      | the y position of the cursor's hotspot
      | within the image
      |
      */
    pub fn new_from_image_and_hotspot(
        image:     &Image,
        hot_spotx: i32,
        hot_spoty: i32) -> Self {
    
        todo!();
        /*
        : mouse_cursor(image, hotSpotX, hotSpotY, 1.0f),

        
        */
    }
    
    /**
      | Creates a custom cursor from an image.
      | 
      | -----------
      | @param image
      | 
      | the image to use for the cursor - if this
      | is bigger than the system can manage,
      | it might get scaled down first, and might
      | also have to be turned to black-and-white
      | if it can't do colour cursors.
      | ----------
      | @param hotSpotX
      | 
      | the x position of the cursor's hotspot
      | within the image
      | ----------
      | @param hotSpotY
      | 
      | the y position of the cursor's hotspot
      | within the image
      | ----------
      | @param scaleFactor
      | 
      | the factor by which this image is larger
      | than the target screen size of the cursor.
      |
      */
    pub fn new_from_image_and_hotspot_with_scale_factor(
        image:        &Image,
        hot_spotx:    i32,
        hot_spoty:    i32,
        scale_factor: f32) -> Self {
    
        todo!();
        /*
            : cursorHandle (new MouseCursorSharedCursorHandle (image, { hotSpotX, hotSpotY }, scaleFactor))
        */
    }
    
    /**
      | Creates a copy of another cursor object.
      |
      */
    pub fn new_from_cursor_ref(other: &MouseCursor) -> Self {
    
        todo!();
        /*
            : cursorHandle (other.cursorHandle == nullptr ? nullptr : other.cursorHandle->retain())
        */
    }
    
    /**
      | Copies this cursor from another object.
      |
      */
    pub fn assign_from_ref(&mut self, other: &MouseCursor) -> &mut MouseCursor {
        
        todo!();
        /*
            if (other.cursorHandle != nullptr)
            other.cursorHandle->retain();

        if (cursorHandle != nullptr)
            cursorHandle->release();

        cursorHandle = other.cursorHandle;
        return *this;
        */
    }
    
    /**
      | Move constructor
      |
      */
    pub fn new_from_cursor(other: MouseCursor) -> Self {
    
        todo!();
        /*
        : cursor_handle(other.cursorHandle),

            other.cursorHandle = nullptr;
        */
    }
    
    pub fn assign_from(&mut self, other: MouseCursor) -> &mut MouseCursor {
        
        todo!();
        /*
            std::swap (cursorHandle, other.cursorHandle);
        return *this;
        */
    }
    
    pub fn get_handle(&self)  {
        
        todo!();
        /*
            return cursorHandle != nullptr ? cursorHandle->getHandle() : nullptr;
        */
    }
    
    /**
      | Makes the system show its default 'busy'
      | cursor.
      | 
      | This will turn the system cursor to an
      | hourglass or spinning beachball until
      | the next time the mouse is moved, or hideWaitCursor()
      | is called.
      | 
      | This is handy if the message loop is about
      | to block for a couple of seconds while
      | busy and you want to give the user feedback
      | about this.
      |
      */
    pub fn show_wait_cursor(&mut self)  {
        
        todo!();
        /*
            Desktop::getInstance().getMainMouseSource().showMouseCursor (WaitCursor);
        */
    }
    
    /**
      | If showWaitCursor has been called,
      | this will return the mouse to its normal
      | state.
      | 
      | This will look at what component is under
      | the mouse, and update the cursor to be
      | the correct one for that component.
      | 
      | @see showWaitCursor
      |
      */
    pub fn hide_wait_cursor(&mut self)  {
        
        todo!();
        /*
            Desktop::getInstance().getMainMouseSource().revealCursor();
        */
    }
}

///---------------------
#[no_copy]
pub struct CustomMouseCursorInfo {
    image:        Image,
    hotspot:      Point<i32>,
    scale_factor: f32,
}

impl CustomMouseCursorInfo {

    pub fn new(
        im:    &Image,
        hs:    Point<i32>,
        scale: Option<f32>

    ) -> Self {

        let scale: f32 = scale.unwrap_or(1.0);

        todo!();
        /*
        : image(im),
        : hotspot(hs),
        : scale_factor(scale),
        */
    }
    
    pub fn create(&self)  {
        
        todo!();
        /*
        
        */
    }

    #[cfg(target_os="linux")]
    pub fn create(&self)  {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->createCustomMouseCursorInfo (image, hotspot);
        */

    }
}
