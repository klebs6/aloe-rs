crate::ix!();

pub trait Update {

    /**
      | Called periodically, at the frequency
      | specified by setFramesPerSecond().
      | 
      | This is a the best place to do things like
      | advancing animation parameters, checking
      | the mouse position, etc.
      |
      */
    fn update(&mut self);
}

pub trait AnimatedAppComponentInterface: Update {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_AnimatedAppComponent.h]

/**
  | A base class for writing simple one-page
  | graphical apps.
  | 
  | A subclass can inherit from this and
  | implement just a few methods such as
  | paint() and mouse-handling. The base
  | class provides some simple abstractions
  | to take care of continuously repainting
  | itself.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AnimatedAppComponent<'a> {
    base:             Component<'a>,
    base2:            Timer,
    last_update_time: Time,
    total_updates:    i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_AnimatedAppComponent.cpp]
impl<'a> Default for AnimatedAppComponent<'a> {

    fn default() -> Self {
    
        todo!();
        /*

            : lastUpdateTime (Time::getCurrentTime()), totalUpdates (0)

        setOpaque (true);
        */
    }
}

impl<'a> AnimatedAppComponent<'a> {

    /**
      | Returns the number of times that update()
      | has been called since the component
      | started running.
      |
      */
    pub fn get_frame_counter(&self) -> i32 {
        
        todo!();
        /*
            return totalUpdates;
        */
    }
    
    /**
      | Your subclass can call this to start
      | a timer running which will call update()
      | and repaint the component at the given
      | frequency.
      |
      */
    pub fn set_frames_per_second(&mut self, frames_per_second: i32)  {
        
        todo!();
        /*
            jassert (framesPerSecond > 0 && framesPerSecond < 1000);
        startTimerHz (framesPerSecond);
        */
    }
    
    /**
      | When called from update(), this returns
      | the number of milliseconds since the
      | last update call.
      | 
      | This might be useful for accurately
      | timing animations, etc.
      |
      */
    pub fn get_milliseconds_since_last_update(&self) -> i32 {
        
        todo!();
        /*
            return (int) (Time::getCurrentTime() - lastUpdateTime).inMilliseconds();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            ++totalUpdates;
        update();
        repaint();
        lastUpdateTime = Time::getCurrentTime();
        */
    }
}
