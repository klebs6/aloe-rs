crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_SplashScreen.h]

/**
  | A component for showing a splash screen
  | while your app starts up.
  | 
  | This will automatically position itself,
  | and can be told to delete itself after
  | being on-screen for a minimum length
  | of time.
  | 
  | To use it, just create one of these in
  | your ALOEApplicationBase::initialise()
  | method, and when your initialisation
  | tasks have finished running, call its
  | deleteAfterDelay() method to make
  | it automatically get rid of itself.
  | 
  | -----------
  | @note
  | 
  | although you could call deleteAfterDelay()
  | as soon as you create the
  | 
  | SplashScreen object, if you've got
  | a long initialisation procedure, you
  | probably don't want the splash to time-out
  | and disappear before the initialisation
  | has finished, which is why it makes sense
  | to not call this method until the end
  | of your init tasks.
  | 
  | -----------
  | @code
  | 
  | void MyApp::initialise (const String& commandLine)
  | {
  |     splash = new SplashScreen ("Welcome to my app!",
  |                                ImageFileFormat::loadFrom (File ("/foobar/splash.jpg")),
  |                                true);
  | 
  |     // now kick off your initialisation work on some kind of thread or task, and
  |     launchBackgroundInitialisationThread();
  | }
  | 
  | void MyApp::myInitialisationWorkFinished()
  | {
  |     // ..assuming this is some kind of callback method that is triggered when
  |     // your background initialisation threads have finished, and it's time to open
  |     // your main window, etc..
  | 
  |     splash->deleteAfterDelay (RelativeTime::seconds (4), false);
  | 
  |     ...etc...
  | }
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct SplashScreen<'a> {
    base:                  Component<'a>,
    base2:                 Timer,
    base3:                 DeletedAtShutdown,
    background_image:      Image,
    creation_time:         Time,
    minimum_visible_time:  RelativeTime,
    click_count_to_delete: i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_SplashScreen.cpp]
impl<'a> SplashScreen<'a> {

    /**
      | Creates a SplashScreen object.
      | 
      | When called, the constructor will position
      | the SplashScreen in the centre of the
      | display, and after the time specified,
      | it will automatically delete itself.
      | 
      | Bear in mind that if you call this during
      | your ALOEApplicationBase::initialise()
      | method and then block the message thread
      | by performing some kind of task, then
      | obviously neither your splash screen
      | nor any other GUI will appear until you
      | allow the message thread to resume and
      | do its work. So if you have time-consuming
      | tasks to do during startup, use a background
      | thread for them.
      | 
      | After creating one of these (or your
      | subclass of it), you should do your app's
      | initialisation work, and then call
      | the deleteAfterDelay() method to tell
      | this object to delete itself after the
      | user has had chance to get a good look
      | at it.
      | 
      | If you're writing a custom splash screen
      | class, there's another protected constructor
      | that your subclass can call, which doesn't
      | take an image.
      | 
      | -----------
      | @param title
      | 
      | the name to give the component
      | ----------
      | @param backgroundImage
      | 
      | an image to draw on the component. The
      | component's size will be set to the size
      | of this image, and if the image is semi-transparent,
      | the component will be made non-opaque
      | ----------
      | @param useDropShadow
      | 
      | if true, the window will have a drop shadow
      |
      */
    pub fn new(
        title:           &String,
        image:           &Image,
        use_drop_shadow: bool) -> Self {
    
        todo!();
        /*
        : component(title),
        : background_image(image),
        : click_count_to_delete(0),

            // You must supply a valid image here!
        jassert (backgroundImage.isValid());

        setOpaque (! backgroundImage.hasAlphaChannel());

       #if ALOE_IOS || ALOE_ANDROID
        const bool useFullScreen = true;
       #else
        const bool useFullScreen = false;
       #endif

        makeVisible (image.getWidth(), image.getHeight(), useDropShadow, useFullScreen);
        */
    }
    
    /**
      | This constructor is for use by custom
      | sub-classes that don't want to provide
      | an image.
      |
      */
    pub fn new_without_image(
        title:           &String,
        width:           i32,
        height:          i32,
        use_drop_shadow: bool) -> Self {
    
        todo!();
        /*
        : component(title),
        : click_count_to_delete(0),

            makeVisible (width, height, useDropShadow, false);
        */
    }
    
    pub fn make_visible(&mut self, 
        w:               i32,
        h:               i32,
        use_drop_shadow: bool,
        fullscreen:      bool)  {
        
        todo!();
        /*
            clickCountToDelete = Desktop::getInstance().getMouseButtonClickCounter();
        creationTime = Time::getCurrentTime();

        const Rectangle<int> screenSize = Desktop::getInstance().getDisplays().getPrimaryDisplay()->userArea;
        const int width  = (fullscreen ? screenSize.getWidth()   : w);
        const int height = (fullscreen ? screenSize.getHeight()  : h);

        setAlwaysOnTop (true);
        setVisible (true);
        centreWithSize (width, height);
        addToDesktop (useDropShadow ? ComponentPeer::windowHasDropShadow : 0);

        if (fullscreen)
            getPeer()->setFullScreen (true);

        toFront (false);
        */
    }
    
    /**
      | Tells the component to auto-delete
      | itself after a timeout period, or when
      | the mouse is clicked.
      | 
      | You should call this after finishing
      | your app's initialisation work.
      | 
      | -----------
      | @note
      | 
      | although you could call deleteAfterDelay()
      | as soon as you create the
      | 
      | SplashScreen object, if you've got
      | a long initialisation procedure, you
      | probably don't want the splash to time-out
      | and disappear before your initialisation
      | has finished, which is why it makes sense
      | to not call this method and start the
      | self-delete timer until you're ready.
      | 
      | It's safe to call this method from a non-GUI
      | thread as long as there's no danger that
      | the object may be being deleted at the
      | same time.
      | ----------
      | @note
      | 
      | this time is measured from the construction-time
      | of this object, not from the time that
      | the deleteAfterDelay() method is called,
      | so if you call this method after a long
      | initialisation period, it may be deleted
      | without any further delay.
      | 
      | -----------
      | @param minimumTotalTimeToDisplayFor
      | 
      | how long the splash screen should stay
      | visible for.
      | ----------
      | @param removeOnMouseClick
      | 
      | if true, the window will be deleted as
      | soon as the user clicks the mouse (anywhere)
      |
      */
    pub fn delete_after_delay(&mut self, 
        timeout:               RelativeTime,
        remove_on_mouse_click: bool)  {
        
        todo!();
        /*
            // Note that this method must be safe to call from non-GUI threads
        if (! removeOnMouseClick)
            clickCountToDelete = std::numeric_limits<int>::max();

        minimumVisibleTime = timeout;

        startTimer (50);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setOpacity (1.0f);
        g.drawImage (backgroundImage, getLocalBounds().toFloat(), RectanglePlacement (RectanglePlacement::fillDestination));
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (Time::getCurrentTime() > creationTime + minimumVisibleTime
             || Desktop::getInstance().getMouseButtonClickCounter() > clickCountToDelete)
            delete this;
        */
    }
}
