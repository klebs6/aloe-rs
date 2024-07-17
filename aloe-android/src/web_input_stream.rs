crate::ix!();

#[no_copy]
#[leak_detector]
#[cfg(target_os="android")]
pub struct WebInputStreamPimpl {
    status_code:                    i32, // default = 0
    url:                            Url,
    is_contenturl:                  bool,
    add_parameters_to_request_body: bool,
    has_body_data_to_send:          bool,
    eof_stream_reached:             bool, // default = false
    num_redirects_to_follow:        i32, // default = 5
    time_out_ms:                    i32, // default = 0
    http_request:                   String,
    headers:                        String,
    response_headers:               Vec<(String,String)>,
    create_stream_lock:             CriticalSection,
    has_been_cancelled:             bool, // default = false
    read_position:                  i32, // default = 0
    stream:                         GlobalRef,
}

#[cfg(target_os="android")]
impl Drop for WebInputStreamPimpl {

    fn drop(&mut self) {
        todo!();
        /* 
            cancel();
         */
    }
}

#[cfg(target_os="android")]
impl WebInputStreamPimpl {

    pub const contentStreamCacheSize: usize = 1024;

    pub fn new(
        _0:                     &mut WebInputStream,
        url_to_copy:            &Url,
        add_parameters_to_body: bool) -> Self {
    
        todo!();
        /*


            : url (urlToCopy),
              isContentURL (urlToCopy.getScheme() == "content"),
              addParametersToRequestBody (addParametersToBody),
              hasBodyDataToSend (addParametersToRequestBody || url.hasBodyDataToSend()),
              httpRequest (hasBodyDataToSend ? "POST" : "GET")
        */
    }
    
    pub fn cancel(&mut self)  {
        
        todo!();
        /*
            if (isContentURL)
            {
                stream.callVoidMethod (AndroidInputStream.close);
                return;
            }

            const ScopedLock lock (createStreamLock);

            if (stream != nullptr)
            {
                stream.callVoidMethod (HTTPStream.release);
                stream.clear();
            }

            hasBeenCancelled = true;
        */
    }
    
    pub fn connect(&mut self, listener: *mut dyn WebInputStreamListener) -> bool {
        
        todo!();
        /*
            auto* env = getEnv();

            if (isContentURL)
            {
                auto inputStream = AndroidContentUriResolver::getStreamForContentUri (url, true);

                if (inputStream != nullptr)
                {
                    stream = GlobalRef (inputStream);
                    statusCode = 200;

                    return true;
                }
            }
            else
            {
                String address = url.toString (! addParametersToRequestBody);

                if (! address.contains ("://"))
                    address = "http://" + address;

                MemoryBlock postData;

                if (hasBodyDataToSend)
                    WebInputStream::createHeadersAndPostData (url,
                                                              headers,
                                                              postData,
                                                              addParametersToRequestBody);

                jbyteArray postDataArray = nullptr;

                if (! postData.isEmpty())
                {
                    postDataArray = env->NewByteArray (static_cast<jsize> (postData.getSize()));
                    env->SetByteArrayRegion (postDataArray, 0, static_cast<jsize> (postData.getSize()), (const jbyte*) postData.getData());
                }

                LocalRef<jobject> responseHeaderBuffer (env->NewObject (StringBuffer, StringBuffer.constructor));

                // Annoyingly, the android HTTP functions will choke on this call if you try to do it on the message
                // thread. You'll need to move your networking code to a background thread to keep it happy..
                jassert (Thread::getCurrentThread() != nullptr);

                jintArray statusCodeArray = env->NewIntArray (1);
                jassert (statusCodeArray != nullptr);

                {
                    const ScopedLock lock (createStreamLock);

                    if (! hasBeenCancelled)
                        stream = GlobalRef (LocalRef<jobject> (env->CallStaticObjectMethod (HTTPStream,
                                                                                            HTTPStream.createHTTPStream,
                                                                                            javaString (address).get(),
                                                                                            (jboolean) addParametersToRequestBody,
                                                                                            postDataArray,
                                                                                            javaString (headers).get(),
                                                                                            (jint) timeOutMs,
                                                                                            statusCodeArray,
                                                                                            responseHeaderBuffer.get(),
                                                                                            (jint) numRedirectsToFollow,
                                                                                            javaString (httpRequest).get())));
                }

                if (stream != nullptr && ! stream.callBooleanMethod (HTTPStream.connect))
                    stream.clear();

                jint* const statusCodeElements = env->GetIntArrayElements (statusCodeArray, nullptr);
                statusCode = statusCodeElements[0];
                env->ReleaseIntArrayElements (statusCodeArray, statusCodeElements, 0);
                env->DeleteLocalRef (statusCodeArray);

                if (postDataArray != nullptr)
                    env->DeleteLocalRef (postDataArray);

                if (stream != nullptr)
                {
                    Vec<String> headerLines;

                    {
                        LocalRef<jstring> headersString ((jstring) env->CallObjectMethod (responseHeaderBuffer.get(),
                                                                                          StringBuffer.toString));
                        headerLines.addLines (aloeString (env, headersString));
                    }

                    for (int i = 0; i < headerLines.size(); ++i)
                    {
                        const String& header = headerLines[i];
                        const String key (header.upToFirstOccurrenceOf (": ", false, false));
                        const String value (header.fromFirstOccurrenceOf (": ", false, false));
                        const String previousValue (responseHeaders[key]);

                        responseHeaders.set (key, previousValue.isEmpty() ? value : (previousValue + "," + value));
                    }

                    return true;
                }
            }

            return false;
        */
    }

    /**
      | WebInputStream methods
      |
      */
    pub fn with_extra_headers(&mut self, extra_headers: &String)  {
        
        todo!();
        /*
            if (! headers.endsWithChar ('\n') && headers.isNotEmpty())
                headers << "\r\n";

            headers << extraHeaders;

            if (! headers.endsWithChar ('\n') && headers.isNotEmpty())
                headers << "\r\n";
        */
    }
    
    pub fn with_custom_request_command(&mut self, custom_request_command: &String)  {
        
        todo!();
        /*
            httpRequest = customRequestCommand;
        */
    }
    
    pub fn with_connection_timeout(&mut self, timeout_in_ms: i32)  {
        
        todo!();
        /*
            timeOutMs = timeoutInMs;
        */
    }
    
    pub fn with_num_redirects_to_follow(&mut self, max_redirects_to_follow: i32)  {
        
        todo!();
        /*
            numRedirectsToFollow = maxRedirectsToFollow;
        */
    }
    
    pub fn get_request_headers(&self) -> Vec<(String,String)> {
        
        todo!();
        /*
            return WebInputStream::parseHttpHeaders (headers);
        */
    }
    
    pub fn get_response_headers(&self) -> Vec<(String,String)> {
        
        todo!();
        /*
            return responseHeaders;
        */
    }
    
    pub fn get_status_code(&self) -> i32 {
        
        todo!();
        /*
            return statusCode;
        */
    }
    
    pub fn is_error(&self) -> bool {
        
        todo!();
        /*
            return stream == nullptr;
        */
    }
    
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return (isContentURL ? eofStreamReached : stream != nullptr && stream.callBooleanMethod (HTTPStream.isExhausted));
        */
    }
    
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return (isContentURL ? -1           : (stream != nullptr ? stream.callLongMethod (HTTPStream.getTotalLength) : 0));
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return (isContentURL ? readPosition : (stream != nullptr ? stream.callLongMethod (HTTPStream.getPosition)    : 0));
        */
    }
    
    pub fn set_position(&mut self, wanted_pos: i64) -> bool {
        
        todo!();
        /*
            if (isContentURL)
            {
                if (wantedPos < readPosition)
                    return false;

                auto bytesToSkip = wantedPos - readPosition;

                if (bytesToSkip == 0)
                    return true;

                HeapBlock<char> buffer (bytesToSkip);

                return (read (buffer.getData(), (int) bytesToSkip) > 0);
            }

            return stream != nullptr && stream.callBooleanMethod (HTTPStream.setPosition, (jlong) wantedPos);
        */
    }
    
    pub fn read(&mut self, 
        buffer:        *mut c_void,
        bytes_to_read: i32) -> i32 {
        
        todo!();
        /*
            jassert (buffer != nullptr && bytesToRead >= 0);

            const ScopedLock lock (createStreamLock);

            if (stream == nullptr)
                return 0;

            JNIEnv* env = getEnv();

            jbyteArray javaArray = env->NewByteArray (bytesToRead);

            auto numBytes = (isContentURL ? stream.callIntMethod (AndroidInputStream.read, javaArray, 0, (jint) bytesToRead)
                                          : stream.callIntMethod (HTTPStream.read, javaArray, (jint) bytesToRead));

            if (numBytes > 0)
                env->GetByteArrayRegion (javaArray, 0, numBytes, static_cast<jbyte*> (buffer));

            env->DeleteLocalRef (javaArray);

            readPosition += jmax (0, numBytes);

            if (numBytes == -1)
                eofStreamReached = true;

            return numBytes;
        */
    }
}
