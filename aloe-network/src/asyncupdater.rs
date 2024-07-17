crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_CallbackMessage.h]

/**
  | Internal class used as the base class
  | for all message objects.
  | 
  | You shouldn't need to use this directly
  | - see the CallbackMessage or Message
  | classes instead.
  |
  */
pub trait MessageBaseInterface
/* 
: Default 
+ ReferenceCountedObjectInterface
*/ 
{
    fn message_callback(&mut self);

    fn post(&mut self) -> bool {
        
        todo!();
        /*
            auto* mm = MessageManager::instance;

        if (mm == nullptr || mm->quitMessagePosted.get() != 0 || ! postMessageToSystemQueue (this))
        {
            Ptr deleter (this); // (this will delete messages that were just created with a 0 ref count)
            return false;
        }

        return true;
        */
    }

    fn as_ptr(&self) 
    -> ReferenceCountedObjectPtr<Self> where Self: Sized {

        todo!();
    }
}

/**
  | A message that invokes a callback method
  | when it gets delivered.
  | 
  | You can use this class to fire off actions
  | that you want to be performed later on
  | the message thread.
  | 
  | To use it, create a subclass of CallbackMessage
  | which implements the messageCallback()
  | method, then call post() to dispatch
  | it. The event thread will then invoke
  | your messageCallback() method later
  | on, and will automatically delete the
  | message object afterwards.
  | 
  | Always create a new instance of a CallbackMessage
  | on the heap, as it will be deleted automatically
  | after the message has been delivered.
  | 
  | Note that this class was essential back
  | in the days before C++11, but in modern
  | times you may prefer to use MessageManager::callAsync()
  | with a lambda.
  | 
  | @see MessageManager::callAsync,
  | MessageListener, ActionListener,
  | ChangeListener
  | 
  | @tags{Events}
  |
  */
#[no_copy]
pub trait CallbackMessage /* : Default */ {

    /*
      | Avoid the leak-detector because for
      | plugins, the host can unload our DLL with
      | undelivered messages still in the system
      | event queue. These aren't harmful, but can
      | cause annoying assertions.
      */

    /**
      | Called when the message is delivered.
      | 
      | You should implement this method and
      | make it do whatever action you want to
      | perform.
      | 
      | Note that like all other messages, this
      | object will be deleted immediately
      | after this method has been invoked.
      |
      */
    fn message_callback(&mut self);
}

impl<T: MessageBaseInterface> CallbackMessage for T {

    fn message_callback(&mut self) {
        <Self as MessageBaseInterface>::message_callback(self)
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/broadcasters/aloe_AsyncUpdater.h]

pub trait AsyncUpdaterInterface {

    /**
      | Called back to do whatever your class
      | needs to do.
      | 
      | This method is called by the message
      | thread at the next convenient time after
      | the triggerAsyncUpdate() method has
      | been called.
      |
      */
    fn handle_async_update(&mut self);
}

#[no_copy]
pub struct AsyncUpdaterMessage<'a> {
    owner:          &'a mut AsyncUpdater<'a>,
    should_deliver: AtomicI32,
}

impl<'a> Default for AsyncUpdaterMessage<'a> {

    fn default() -> Self {
        todo!();
    }
}

impl<'a> CallbackMessage for AsyncUpdaterMessage<'a> {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
            if (shouldDeliver.compareAndSetBool (0, 1))
                    owner.handleAsyncUpdate();
        */
    }
}

impl<'a> AsyncUpdaterMessage<'a> {

    pub fn new(au: &mut AsyncUpdater) -> Self {
    
        todo!();
        /*
        : owner(au),
        */
    }
}

/**
  | Has a callback method that is triggered
  | asynchronously.
  | 
  | This object allows an asynchronous
  | callback function to be triggered,
  | for tasks such as coalescing multiple
  | updates into a single callback later
  | on.
  | 
  | Basically, one or more calls to the triggerAsyncUpdate()
  | will result in the message thread calling
  | handleAsyncUpdate() as soon as it can.
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AsyncUpdater<'a> {
    active_message: ReferenceCountedObjectPtr<AsyncUpdaterMessage<'a>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/broadcasters/aloe_AsyncUpdater.cpp]
impl<'a> Drop for AsyncUpdater<'a> {

    /**
      | Destructor. If there are any pending
      | callbacks when the object is deleted,
      | these are lost.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        // You're deleting this object with a background thread while there's an update
        // pending on the main event thread - that's pretty dodgy threading, as the callback could
        // happen after this destructor has finished. You should either use a MessageManagerLock while
        // deleting this object, or find some other way to avoid such a race condition.
        jassert ((! isUpdatePending())
                  || MessageManager::getInstanceWithoutCreating() == nullptr
                  || MessageManager::getInstanceWithoutCreating()->currentThreadHasLockedMessageManager());

        activeMessage->shouldDeliver.set (0);
     */
    }
}

impl<'a> Default for AsyncUpdater<'a> {

    fn default() -> Self {
    
        todo!();
        /*
            activeMessage = *new AsyncUpdaterMessage (*this);
        */
    }
}

impl<'a> AsyncUpdater<'a> {
    
    /**
      | Causes the callback to be triggered
      | at a later time.
      | 
      | This method returns immediately, after
      | which a callback to the handleAsyncUpdate()
      | method will be made by the message thread
      | as soon as possible.
      | 
      | If an update callback is already pending
      | but hasn't happened yet, calling this
      | method will have no effect.
      | 
      | It's thread-safe to call this method
      | from any thread, BUT beware of calling
      | it from a real-time (e.g. audio) thread,
      | because it involves posting a message
      | to the system queue, which means it may
      | block (and in general will do on most
      | OSes).
      |
      */
    pub fn trigger_async_update(&mut self)  {
        
        todo!();
        /*
            // If you're calling this before (or after) the MessageManager is
        // running, then you're not going to get any callbacks!
        ALOE_ASSERT_MESSAGE_MANAGER_EXISTS

        if (activeMessage->shouldDeliver.compareAndSetBool (1, 0))
            if (! activeMessage->post())
                cancelPendingUpdate(); // if the message queue fails, this avoids getting
                                       // trapped waiting for the message to arrive
        */
    }
    
    /**
      | This will stop any pending updates from
      | happening.
      | 
      | If called after triggerAsyncUpdate()
      | and before the handleAsyncUpdate()
      | callback happens, this will cancel
      | the handleAsyncUpdate() callback.
      | 
      | -----------
      | @note
      | 
      | this method simply cancels the next
      | callback - if a callback is already in
      | progress on a different thread, this
      | won't block until the callback finishes,
      | so there's no guarantee that the callback
      | isn't still running when the method
      | returns.
      |
      */
    pub fn cancel_pending_update(&mut self)  {
        
        todo!();
        /*
            activeMessage->shouldDeliver.set (0);
        */
    }
    
    /**
      | If an update has been triggered and is
      | pending, this will invoke it synchronously.
      | 
      | Use this as a kind of "flush" operation
      | - if an update is pending, the handleAsyncUpdate()
      | method will be called immediately;
      | if no update is pending, then nothing
      | will be done.
      | 
      | Because this may invoke the callback,
      | this method must only be called on the
      | main event thread.
      |
      */
    pub fn handle_update_now_if_needed(&mut self)  {
        
        todo!();
        /*
            // This can only be called by the event thread.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        if (activeMessage->shouldDeliver.exchange (0) != 0)
            handleAsyncUpdate();
        */
    }
    
    /**
      | Returns true if there's an update callback
      | in the pipeline.
      |
      */
    pub fn is_update_pending(&self) -> bool {
        
        todo!();
        /*
            return activeMessage->shouldDeliver.value != 0;
        */
    }
}
