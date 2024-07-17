crate::ix!();

pub struct WebBrowserComponentPimpl<'a> {
    base:                 Thread,
    web_kit_is_available: bool, // default = false
    owner:                &'a mut WebBrowserComponent<'a>,
    receiver:             Box<CommandReceiver>,
    child_process:        i32, // default = 0
    in_channel:           i32, // default = 0
    out_channel:          i32, // default = 0
    thread_control:       [i32; 2],

    #[cfg(target_os="linux")]
    xembed:               Box<XEmbedComponent>,
    thread_blocker:       WaitableEvent,
    pfds:                 Vec<libc::pollfd>,
}

impl<'a> CommandReceiverResponder for WebBrowserComponentPimpl<'a> {

    fn handle_command(
        &mut self, 
        cmd:    &String,
        params: &Var

    ) {
        
        todo!();
        /*
            threadBlocker.reset();

            (new WebBrowserComponentHandleOnMessageThread (this, cmd, params))->post();

            // wait until the command has executed on the message thread
            // this ensures that WebBrowserComponentPimpl can never be deleted while the
            // message has not been executed yet
            threadBlocker.wait (-1);
        */
    }
    
    fn receiver_had_error(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for WebBrowserComponentPimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            quit();
        */
    }
}

impl<'a> WebBrowserComponentPimpl<'a> {

    pub fn new(parent: &mut WebBrowserComponent) -> Self {
    
        todo!();
        /*


            : Thread ("Webview"), owner (parent)

            webKitIsAvailable = WebKitSymbols::getInstance()->isWebKitAvailable();
        */
    }
    
    pub fn init(&mut self)  {
        
        todo!();
        /*
            if (! webKitIsAvailable)
                return;

            launchChild();

            auto ret = pipe (threadControl);

            ignoreUnused (ret);
            jassert (ret == 0);

            CommandReceiver::setBlocking (inChannel,        true);
            CommandReceiver::setBlocking (outChannel,       true);
            CommandReceiver::setBlocking (threadControl[0], false);
            CommandReceiver::setBlocking (threadControl[1], true);

            unsigned long windowHandle;
            auto actual = read (inChannel, &windowHandle, sizeof (windowHandle));

            if (actual != (ssize_t) sizeof (windowHandle))
            {
                killChild();
                return;
            }

            receiver.reset (new CommandReceiver (this, inChannel));

            pfds.push_back ({ threadControl[0],  POLLIN, 0 });
            pfds.push_back ({ receiver->getFd(), POLLIN, 0 });

            startThread();

            xembed.reset (new XEmbedComponent (windowHandle));
            owner.addAndMakeVisible (xembed.get());
        */
    }
    
    pub fn quit(&mut self)  {
        
        todo!();
        /*
            if (! webKitIsAvailable)
                return;

            if (isThreadRunning())
            {
                signalThreadShouldExit();

                char ignore = 0;
                ssize_t ret;

                for (;;)
                {
                    ret = write (threadControl[1], &ignore, 1);

                    if (ret != -1 || errno != EINTR)
                        break;
                }

                waitForThreadToExit (-1);
                receiver = nullptr;
            }

            if (childProcess != 0)
            {
                CommandReceiver::sendCommand (outChannel, "quit", {});
                killChild();
            }
        */
    }
    
    pub fn go_tourl(&mut self, 
        url:       &String,
        headers:   *const StringArray,
        post_data: *const MemoryBlock)  {
        
        todo!();
        /*
            if (! webKitIsAvailable)
                return;

            DynamicObject::Ptr params = new DynamicObject;

            params->setProperty ("url", url);

            if (headers != nullptr)
                params->setProperty ("headers", var (*headers));

            if (postData != nullptr)
                params->setProperty ("postData", var (*postData));

            CommandReceiver::sendCommand (outChannel, "goToURL", var (params.get()));
        */
    }
    
    pub fn go_back(&mut self)  {
        
        todo!();
        /*
            if (webKitIsAvailable) CommandReceiver::sendCommand (outChannel, "goBack",    {});
        */
    }
    
    pub fn go_forward(&mut self)  {
        
        todo!();
        /*
            if (webKitIsAvailable) CommandReceiver::sendCommand (outChannel, "goForward", {});
        */
    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            if (webKitIsAvailable) CommandReceiver::sendCommand (outChannel, "refresh",   {});
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (webKitIsAvailable) CommandReceiver::sendCommand (outChannel, "stop",      {});
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (xembed != nullptr)
                xembed->setBounds (owner.getLocalBounds());
        */
    }
    
    pub fn kill_child(&mut self)  {
        
        todo!();
        /*
            if (childProcess != 0)
            {
                xembed = nullptr;

                int status = 0, result = 0;

                result = waitpid (childProcess, &status, WNOHANG);
                for (int i = 0; i < 15 && (! WIFEXITED(status) || result != childProcess); ++i)
                {
                    Thread::sleep (100);
                    result = waitpid (childProcess, &status, WNOHANG);
                }

                // clean-up any zombies
                status = 0;
                if (! WIFEXITED(status) || result != childProcess)
                {
                    for (;;)
                    {
                        kill (childProcess, SIGTERM);
                        waitpid (childProcess, &status, 0);

                        if (WIFEXITED (status))
                            break;
                    }
                }

                childProcess = 0;
            }
        */
    }
    
    pub fn launch_child(&mut self)  {
        
        todo!();
        /*
            int inPipe[2], outPipe[2];

            auto ret = pipe (inPipe);
            ignoreUnused (ret); jassert (ret == 0);

            ret = pipe (outPipe);
            ignoreUnused (ret); jassert (ret == 0);

            auto pid = fork();
            if (pid == 0)
            {
                close (inPipe[0]);
                close (outPipe[1]);

                HeapBlock<const char*> argv (5);
                StringArray arguments;

                arguments.add (File::getSpecialLocation (File::currentExecutableFile).getFullPathName());
                arguments.add ("--aloe-gtkwebkitfork-child");
                arguments.add (String (outPipe[0]));
                arguments.add (String (inPipe [1]));

                for (int i = 0; i < arguments.size(); ++i)
                    argv[i] = arguments[i].toRawUTF8();

                argv[4] = nullptr;

               #if ALOE_STANDALONE_APPLICATION
                execv (arguments[0].toRawUTF8(), (char**) argv.getData());
               #else
                aloe_gtkWebkitMain (4, (const char**) argv.getData());
               #endif
                exit (0);
            }

            close (inPipe[1]);
            close (outPipe[0]);

            inChannel  = inPipe[0];
            outChannel = outPipe[1];

            childProcess = pid;
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! threadShouldExit())
            {
                if (shouldExit())
                    return;

                receiver->tryNextRead();

                int result = 0;

                while (result == 0 || (result < 0 && errno == EINTR))
                    result = poll (&pfds.front(), static_cast<nfds_t> (pfds.size()), 0);

                if (result < 0)
                    break;
            }
        */
    }
    
    pub fn should_exit(&mut self) -> bool {
        
        todo!();
        /*
            char ignore;
            auto result = read (threadControl[0], &ignore, 1);

            return (result != -1 || (errno != EAGAIN && errno != EWOULDBLOCK));
        */
    }
    
    pub fn handle_command_on_message_thread(&mut self, 
        cmd:    &String,
        params: &Var)  {
        
        todo!();
        /*
            auto url = params.getProperty ("url", var()).toString();

            if      (cmd == "pageAboutToLoad")           handlePageAboutToLoad (url, params);
            else if (cmd == "pageFinishedLoading")       owner.pageFinishedLoading (url);
            else if (cmd == "windowCloseRequest")        owner.windowCloseRequest();
            else if (cmd == "newWindowAttemptingToLoad") owner.newWindowAttemptingToLoad (url);
            else if (cmd == "pageLoadHadNetworkError")   handlePageLoadHadNetworkError (params);

            threadBlocker.signal();
        */
    }
    
    pub fn handle_page_about_to_load(&mut self, 
        url:          &String,
        input_params: &Var)  {
        
        todo!();
        /*
            int64 decision_id = inputParams.getProperty ("decision_id", var (0));

            if (decision_id != 0)
            {
                DynamicObject::Ptr params = new DynamicObject;

                params->setProperty ("decision_id", decision_id);
                params->setProperty ("allow", owner.pageAboutToLoad (url));

                CommandReceiver::sendCommand (outChannel, "decision", var (params.get()));
            }
        */
    }
    
    pub fn handle_page_load_had_network_error(&mut self, params: &Var)  {
        
        todo!();
        /*
            String error = params.getProperty ("error", "Unknown error");

            if (owner.pageLoadHadNetworkError (error))
                goToURL (String ("data:text/plain,") + error, nullptr, nullptr);
        */
    }
    
}
