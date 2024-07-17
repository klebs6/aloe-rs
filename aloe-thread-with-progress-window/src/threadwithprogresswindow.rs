crate::ix!();

pub trait ThreadComplete {

    /**
      | This method is called (on the message
      | thread) when the operation has finished.
      | 
      | You may choose to use this callback to
      | delete the ThreadWithProgressWindow
      | object.
      |
      */
    fn thread_complete(&mut self, user_pressed_cancel: bool);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_ThreadWithProgressWindow.h]

/**
  | A thread that automatically pops up
  | a modal dialog box with a progress bar
  | and cancel button while it's busy running.
  | 
  | These are handy for performing some
  | sort of task while giving the user feedback
  | about how long there is to go, etc.
  | 
  | -----------
  | @code
  | 
  | class MyTask  : public ThreadWithProgressWindow
  | {
  | 
  |     MyTask()    : ThreadWithProgressWindow ("busy...", true, true)
  |     {
  |     }
  | 
  |     void run()
  |     {
  |         for (int i = 0; i < thingsToDo; ++i)
  |         {
  |             // must check this as often as possible, because this is
  |             // how we know if the user's pressed 'cancel'
  |             if (threadShouldExit())
  |                 break;
  | 
  |             // this will update the progress bar on the dialog box
  |             setProgress (i / (double) thingsToDo);
  | 
  |             //   ... do the business here...
  |         }
  |     }
  | };
  | 
  | void doTheTask()
  | {
  |     MyTask m;
  | 
  |     if (m.runThread())
  |     {
  |         // thread finished normally..
  |     }
  |     else
  |     {
  |         // user pressed the cancel button..
  |     }
  | }
  | 
  | @see Thread, AlertWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ThreadWithProgressWindow<'a> {
    base:                        Thread,
    base2:                       Timer,
    progress:                    f64,
    alert_window:                Box<AlertWindow<'a>>,
    message:                     String,
    message_lock:                CriticalSection,
    time_out_ms_when_cancelling: i32,
    was_cancelled_by_user:       bool,
}

pub trait ThreadWithProgressWindowInterface: ThreadComplete { }

impl<'a> Drop for ThreadWithProgressWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            stopThread (timeOutMsWhenCancelling);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/windows/aloe_ThreadWithProgressWindow.cpp]
impl<'a> ThreadWithProgressWindow<'a> {

    /**
      | Returns the AlertWindow that is being
      | used.
      |
      */
    pub fn get_alert_window(&self) -> *mut AlertWindow {
        
        todo!();
        /*
            return alertWindow.get();
        */
    }
    
    /**
      | Creates the thread.
      | 
      | Initially, the dialog box won't be visible,
      | it'll only appear when the runThread()
      | method is called.
      | 
      | -----------
      | @param windowTitle
      | 
      | the title to go at the top of the dialog
      | box
      | ----------
      | @param hasProgressBar
      | 
      | whether the dialog box should have a
      | progress bar (see setProgress() )
      | ----------
      | @param hasCancelButton
      | 
      | whether the dialog box should have a
      | cancel button
      | ----------
      | @param timeOutMsWhenCancelling
      | 
      | when 'cancel' is pressed, this is how
      | long to wait for the thread to stop before
      | killing it forcibly (see
      | 
      | Thread::stopThread() )
      | ----------
      | @param cancelButtonText
      | 
      | the text that should be shown in the cancel
      | button (if it has one). Leave this empty
      | for the default "Cancel"
      | ----------
      | @param componentToCentreAround
      | 
      | if this is non-null, the window will
      | be positioned so that it's centred around
      | this component.
      |
      */
    pub fn new(
        title:                      &String,
        has_progress_bar:           bool,
        has_cancel_button:          bool,
        cancelling_time_out_ms:     Option<i32>,
        cancel_button_text:         Option<&String>,
        component_to_centre_around: *mut Component<'a>

    ) -> Self {

        let cancelling_time_out_ms: i32 =
                 cancelling_time_out_ms.unwrap_or(10000);

        let cancel_button_text = cancel_button_text.unwrap_or(&String::new());

        todo!();
        /*


            : Thread ("ThreadWithProgressWindow"),
         progress (0.0),
         timeOutMsWhenCancelling (cancellingTimeOutMs),
         wasCancelledByUser (false)

        alertWindow.reset (LookAndFeel::getDefaultLookAndFeel()
                               .createAlertWindow (title, {},
                                                   cancelButtonText.isEmpty() ? TRANS("Cancel")
                                                                              : cancelButtonText,
                                                   {}, {}, MessageBoxIconType::NoIcon, hasCancelButton ? 1 : 0,
                                                   componentToCentreAround));

        // if there are no buttons, we won't allow the user to interrupt the thread.
        alertWindow->setEscapeKeyCancels (false);

        if (hasProgressBar)
            alertWindow->addProgressBarComponent (progress);
        */
    }
    
    /**
      | Starts the thread and returns.
      | 
      | This will start the thread and make the
      | dialog box appear in a modal state. When
      | the thread finishes normally, or the
      | cancel button is pressed, the window
      | will be hidden and the threadComplete()
      | method will be called.
      | 
      | -----------
      | @param priority
      | 
      | the priority to use when starting the
      | thread - see
      | 
      | Thread::startThread() for values
      |
      */
    pub fn launch_thread(&mut self, priority: Option<i32>)  {

        let priority: i32 = priority.unwrap_or(5);
        
        todo!();
        /*
            ALOE_ASSERT_MESSAGE_THREAD

        startThread (priority);
        startTimer (100);

        {
            const ScopedLock sl (messageLock);
            alertWindow->setMessage (message);
        }

        alertWindow->enterModalState();
        */
    }
    
    /**
      | The thread should call this periodically
      | to update the position of the progress
      | bar.
      | 
      | -----------
      | @param newProgress
      | 
      | the progress, from 0.0 to 1.0 @see setStatusMessage
      |
      */
    pub fn set_progress(&mut self, new_progress: f64)  {
        
        todo!();
        /*
            progress = newProgress;
        */
    }
    
    /**
      | The thread can call this to change the
      | message that's displayed in the dialog
      | box.
      |
      */
    pub fn set_status_message(&mut self, new_status_message: &String)  {
        
        todo!();
        /*
            const ScopedLock sl (messageLock);
        message = newStatusMessage;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            bool threadStillRunning = isThreadRunning();

        if (! (threadStillRunning && alertWindow->isCurrentlyModal (false)))
        {
            stopTimer();
            stopThread (timeOutMsWhenCancelling);
            alertWindow->exitModalState (1);
            alertWindow->setVisible (false);

            wasCancelledByUser = threadStillRunning;
            threadComplete (threadStillRunning);
            return; // (this may be deleted now)
        }

        const ScopedLock sl (messageLock);
        alertWindow->setMessage (message);
        */
    }
    
    pub fn thread_complete(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Starts the thread and waits for it to
      | finish.
      | 
      | This will start the thread, make the
      | dialog box appear, and wait until either
      | the thread finishes normally, or until
      | the cancel button is pressed.
      | 
      | Before returning, the dialog box will
      | be hidden.
      | 
      | -----------
      | @param priority
      | 
      | the priority to use when starting the
      | thread - see
      | 
      | Thread::startThread() for values
      | 
      | -----------
      | @return
      | 
      | true if the thread finished normally;
      | false if the user pressed cancel
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn run_thread(&mut self, priority: i32) -> bool {

        let priority: i32 = priority.unwrap_or(5);
        
        todo!();
        /*
            launchThread (priority);

        while (isTimerRunning())
            MessageManager::getInstance()->runDispatchLoopUntil (5);

        return ! wasCancelledByUser;
        */
    }
}
