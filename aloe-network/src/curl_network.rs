crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_curl_Network.cpp]

pub struct CURLSymbols<'a> {

    curl_easy_init:           fn() -> *mut CURL,

    curl_easy_setopt:         fn(
            curl:   *mut CURL,
            option: CURLoption,
            args:   &[&str]
    ) -> CURLcode,

    curl_easy_cleanup:        fn(curl: *mut CURL) -> c_void,

    curl_easy_getinfo:        fn(
            curl: *mut CURL,
            info: CURLINFO,
            args: &[&str]
    ) -> CURLcode,

    curl_multi_add_handle:    fn(multi_handle: *mut CURLM, curl_handle: *mut CURL) -> CURLMcode,

    curl_multi_cleanup:       fn(multi_handle: *mut CURLM) -> CURLMcode,

    curl_multi_fdset:         fn(
            multi_handle: *mut CURLM,
            read_fd_set:  *mut libc::fd_set,
            write_fd_set: *mut libc::fd_set,
            exc_fd_set:   *mut libc::fd_set,
            max_fd:       *mut i32
    ) -> CURLMcode,

    curl_multi_info_read:     fn(multi_handle: *mut CURLM, msgs_in_queue: *mut i32) -> *mut CURLMsg<'a>,

    curl_multi_init:          fn() -> *mut CURLM,

    curl_multi_perform:       fn(multi_handle: *mut CURLM, running_handles: *mut i32) -> CURLMcode,

    curl_multi_remove_handle: fn(multi_handle: *mut CURLM, curl_handle: *mut CURL) -> CURLMcode,

    curl_multi_timeout:       fn(multi_handle: *mut CURLM, milliseconds: *mut i64) -> CURLMcode,

    curl_slist_append:        fn(_0: *mut CurlSlist, _1: *const u8) -> *mut CurlSlist,

    curl_slist_free_all:      fn(_0: *mut CurlSlist) -> c_void,

    curl_version_info:        fn(_0: CURLversion) -> *mut CurlVersionInfoData,

}

//------------------------[wrap the libcurl interface]
pub type CurlSlist           = curl::easy::List;
pub type CURL                = curl::easy::Easy;
pub type CURLM               = curl::multi::Multi;
pub type CURLversion         = curl::Version;
pub type CurlVersionInfoData = curl::Version;
pub type CURLMsg<'a>         = curl::multi::Message<'a>;
pub type CURLMcode           = curl::MultiError;
pub type CURLINFO            = curl::easy::Easy;
pub type CURLcode            = curl::Error;
pub type CURLoption          = curl::easy::Easy;

impl<'a> CURLSymbols<'a> {

    pub fn create() -> Box<CURLSymbols<'a>> {
        
        todo!();
        /*
            std::unique_ptr<CURLSymbols> symbols (new CURLSymbols);

           #if ALOE_LOAD_CURL_SYMBOLS_LAZILY
            const ScopedLock sl (getLibcurlLock());
            #define ALOE_INIT_CURL_SYMBOL(name)  if (! symbols->loadSymbol (symbols->name, #name)) return nullptr;
           #else
            #define ALOE_INIT_CURL_SYMBOL(name)  symbols->name = ::name;
           #endif

            ALOE_INIT_CURL_SYMBOL (curl_easy_init)
            ALOE_INIT_CURL_SYMBOL (curl_easy_setopt)
            ALOE_INIT_CURL_SYMBOL (curl_easy_cleanup)
            ALOE_INIT_CURL_SYMBOL (curl_easy_getinfo)
            ALOE_INIT_CURL_SYMBOL (curl_multi_add_handle)
            ALOE_INIT_CURL_SYMBOL (curl_multi_cleanup)
            ALOE_INIT_CURL_SYMBOL (curl_multi_fdset)
            ALOE_INIT_CURL_SYMBOL (curl_multi_info_read)
            ALOE_INIT_CURL_SYMBOL (curl_multi_init)
            ALOE_INIT_CURL_SYMBOL (curl_multi_perform)
            ALOE_INIT_CURL_SYMBOL (curl_multi_remove_handle)
            ALOE_INIT_CURL_SYMBOL (curl_multi_timeout)
            ALOE_INIT_CURL_SYMBOL (curl_slist_append)
            ALOE_INIT_CURL_SYMBOL (curl_slist_free_all)
            ALOE_INIT_CURL_SYMBOL (curl_version_info)

            return symbols;
        */
    }

    /**
      | liburl's curl_multi_init calls
      | curl_global_init which is not thread safe
      | so we need to get a lock during calls to
      | curl_multi_init and curl_multi_cleanup
      */
    pub fn get_libcurl_lock() -> &'static mut CriticalSection {
        
        todo!();
        /*
            static CriticalSection cs;
            return cs;
        */
    }

    #[cfg(ALOE_LOAD_CURL_SYMBOLS_LAZILY)]
    pub fn get_libcurl() -> &mut DynamicLibrary {
        
        todo!();
        /*
            const ScopedLock sl (getLibcurlLock());
            static DynamicLibrary libcurl;

            if (libcurl.getNativeHandle() == nullptr)
                for (auto libName : { "libcurl.so", "libcurl.so.4", "libcurl.so.3" })
                    if (libcurl.open (libName))
                        break;

            return libcurl;
        */
    }
    
    #[cfg(ALOE_LOAD_CURL_SYMBOLS_LAZILY)]
    pub fn load_symbol<FuncPtr>(&mut self, 
        dst:  &mut FuncPtr,
        name: *const u8) -> bool {
    
        todo!();
        /*
            dst = reinterpret_cast<FuncPtr> (getLibcurl().getFunction (name));
            return (dst != nullptr);
        */
    }
}


//----------------------------------------------
#[no_copy]
#[leak_detector]
struct WebInputStreamImpl<'a> {

    owner:                          &'a mut WebInputStream<'a>,
    url:                            Url,
    symbols:                        Box<CURLSymbols<'a>>, // { CURLSymbols::create() };

    /* ------------------ curl stuff  ------------------ */

    multi:                          *mut CURLM, // default = nullptr
    curl:                           *mut CURL, // default = nullptr
    header_list:                    *mut CurlSlist, // default = nullptr
    last_error:                     i32, // default = CURLE_OK

    /* -------------------- Options  -------------------- */

    time_out_ms:                    i32, // default = 0
    max_redirects:                  i32, // default = 5
    add_parameters_to_request_body: bool,
    has_body_data_to_send:          bool,
    http_request:                   String,

    /* ----- internal buffers and buffer positions  ----- */

    content_length:                 i64, // default = -1

    /* ----- internal buffers and buffer positions  ----- */

    stream_pos:                     i64, // default = 0
    curl_buffer:                    MemoryBlock,
    headers_and_post_data:          MemoryBlock,
    response_headers:               String,
    request_headers:                String,
    status_code:                    i32, // default = -1
    finished:                       bool, // default = false
    skip_bytes:                     usize, // default = 0

    /* -------------- Http POST variables  -------------- */

    post_buffer:                    *const MemoryBlock, // default = nullptr
    post_position:                  usize, // default = 0
    listener:                       *mut dyn WebInputStreamListener, // default = nullptr
    cleanup_lock:                   CriticalSection,
}

impl<'a> Drop for WebInputStreamImpl<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            cleanup();
         */
    }
}

impl<'a> WebInputStreamImpl<'a> {

    pub fn new(
        owner_stream:           &mut WebInputStream,
        url_to_copy:            &Url,
        add_parameters_to_body: bool) -> Self {
    
        todo!();
        /*


            : owner (ownerStream),
                  url (urlToCopy),
                  addParametersToRequestBody (addParametersToBody),
                  hasBodyDataToSend (url.hasBodyDataToSend() || addParametersToRequestBody),
                  httpRequest (hasBodyDataToSend ? "POST" : "GET")

                jassert (symbols); // Unable to load libcurl!

                {
                    const ScopedLock sl (CURLSymbols::getLibcurlLock());
                    multi = symbols->curl_multi_init();
                }

                if (multi != nullptr)
                {
                    curl = symbols->curl_easy_init();

                    if (curl != nullptr)
                        if (symbols->curl_multi_add_handle (multi, curl) == CURLM_OK)
                            return;
                }

                cleanup();
        */
    }

    /**
      | Input Stream overrides
      |
      */
    pub fn is_error(&self) -> bool {
        
        todo!();
        /*
            return curl == nullptr || lastError != CURLE_OK;
        */
    }

    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return (isError() || finished) && curlBuffer.isEmpty();
        */
    }

    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return streamPos;
        */
    }

    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return contentLength;
        */
    }

    pub fn read(&mut self, 
        buffer:        *mut c_void,
        bytes_to_read: i32) -> i32 {
        
        todo!();
        /*
            return readOrSkip (buffer, bytesToRead, false);
        */
    }

    pub fn set_position(&mut self, wanted_pos: i64) -> bool {
        
        todo!();
        /*
            const int amountToSkip = static_cast<int> (wantedPos - getPosition());

                if (amountToSkip < 0)
                    return false;

                if (amountToSkip == 0)
                    return true;

                const int actuallySkipped = readOrSkip (nullptr, amountToSkip, true);

                return actuallySkipped == amountToSkip;
        */
    }

    /**
      | WebInputStream methods
      |
      */
    pub fn with_extra_headers(&mut self, extra_headers: &String)  {
        
        todo!();
        /*
            if (! requestHeaders.endsWithChar ('\n') && requestHeaders.isNotEmpty())
                    requestHeaders << "\r\n";

                requestHeaders << extraHeaders;

                if (! requestHeaders.endsWithChar ('\n') && requestHeaders.isNotEmpty())
                    requestHeaders << "\r\n";
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
            maxRedirects = maxRedirectsToFollow;
        */
    }

    pub fn get_request_headers(&self) -> StringPairArray {
        
        todo!();
        /*
            return WebInputStream::parseHttpHeaders (requestHeaders);
        */
    }

    pub fn get_response_headers(&self) -> StringPairArray {
        
        todo!();
        /*
            return WebInputStream::parseHttpHeaders (responseHeaders);
        */
    }

    pub fn get_status_code(&self) -> i32 {
        
        todo!();
        /*
            return statusCode;
        */
    }

    pub fn cleanup(&mut self)  {
        
        todo!();
        /*
            const ScopedLock lock (cleanupLock);
                const ScopedLock sl (CURLSymbols::getLibcurlLock());

                if (curl != nullptr)
                {
                    symbols->curl_multi_remove_handle (multi, curl);

                    if (headerList != nullptr)
                    {
                        symbols->curl_slist_free_all (headerList);
                        headerList = nullptr;
                    }

                    symbols->curl_easy_cleanup (curl);
                    curl = nullptr;
                }

                if (multi != nullptr)
                {
                    symbols->curl_multi_cleanup (multi);
                    multi = nullptr;
                }
        */
    }

    pub fn cancel(&mut self)  {
        
        todo!();
        /*
            cleanup();
        */
    }

    pub fn set_options(&mut self) -> bool {
        
        todo!();
        /*
            auto address = url.toString (! addParametersToRequestBody);

                curl_version_info_data* data = symbols->curl_version_info (CURLVERSION_NOW);
                jassert (data != nullptr);

                if (! requestHeaders.endsWithChar ('\n'))
                    requestHeaders << "\r\n";

                if (hasBodyDataToSend)
                    WebInputStream::createHeadersAndPostData (url,
                                                              requestHeaders,
                                                              headersAndPostData,
                                                              addParametersToRequestBody);

                if (! requestHeaders.endsWithChar ('\n'))
                    requestHeaders << "\r\n";

                auto userAgent = String ("curl/") + data->version;

                if (symbols->curl_easy_setopt (curl, CURLOPT_URL, address.toRawUTF8()) == CURLE_OK
                    && symbols->curl_easy_setopt (curl, CURLOPT_WRITEDATA, this) == CURLE_OK
                    && symbols->curl_easy_setopt (curl, CURLOPT_WRITEFUNCTION, StaticCurlWrite) == CURLE_OK
                    && symbols->curl_easy_setopt (curl, CURLOPT_NOSIGNAL, 1) == CURLE_OK
                    && symbols->curl_easy_setopt (curl, CURLOPT_MAXREDIRS, static_cast<long> (maxRedirects)) == CURLE_OK
                    && symbols->curl_easy_setopt (curl, CURLOPT_USERAGENT, userAgent.toRawUTF8()) == CURLE_OK
                    && symbols->curl_easy_setopt (curl, CURLOPT_FOLLOWLOCATION, (maxRedirects > 0 ? 1 : 0)) == CURLE_OK)
                {
                    if (hasBodyDataToSend)
                    {
                        if (symbols->curl_easy_setopt (curl, CURLOPT_READDATA, this) != CURLE_OK
                            || symbols->curl_easy_setopt (curl, CURLOPT_READFUNCTION, StaticCurlRead) != CURLE_OK)
                            return false;

                        if (symbols->curl_easy_setopt (curl, CURLOPT_POST, 1) != CURLE_OK
                            || symbols->curl_easy_setopt (curl, CURLOPT_POSTFIELDSIZE_LARGE, static_cast<curl_off_t> (headersAndPostData.getSize())) != CURLE_OK)
                            return false;
                    }

                    // handle special http request commands
                    const auto hasSpecialRequestCmd = hasBodyDataToSend ? (httpRequest != "POST") : (httpRequest != "GET");

                    if (hasSpecialRequestCmd)
                        if (symbols->curl_easy_setopt (curl, CURLOPT_CUSTOMREQUEST, httpRequest.toRawUTF8()) != CURLE_OK)
                            return false;

                    if (symbols->curl_easy_setopt (curl, CURLOPT_HEADERDATA, this) != CURLE_OK
                        || symbols->curl_easy_setopt (curl, CURLOPT_HEADERFUNCTION, StaticCurlHeader) != CURLE_OK)
                        return false;

                    if (timeOutMs > 0)
                    {
                        auto timeOutSecs = ((long) timeOutMs + 999) / 1000;

                        if (symbols->curl_easy_setopt (curl, CURLOPT_CONNECTTIMEOUT, timeOutSecs) != CURLE_OK
                            || symbols->curl_easy_setopt (curl, CURLOPT_LOW_SPEED_LIMIT, 100) != CURLE_OK
                            || symbols->curl_easy_setopt (curl, CURLOPT_LOW_SPEED_TIME, timeOutSecs) != CURLE_OK)
                            return false;
                    }

                    return true;
                }

                return false;
        */
    }

    pub fn connect(&mut self, web_input_listener: *mut dyn WebInputStreamListener) -> bool {
        
        todo!();
        /*
            {
                    const ScopedLock lock (cleanupLock);

                    if (curl == nullptr)
                        return false;

                    if (! setOptions())
                    {
                        cleanup();
                        return false;
                    }

                    if (requestHeaders.isNotEmpty())
                    {
                        const StringArray headerLines = StringArray::fromLines (requestHeaders);

                        // fromLines will always return at least one line if the string is not empty
                        jassert (headerLines.size() > 0);
                        headerList = symbols->curl_slist_append (headerList, headerLines [0].toRawUTF8());

                        for (int i = 1; (i < headerLines.size() && headerList != nullptr); ++i)
                            headerList = symbols->curl_slist_append (headerList, headerLines [i].toRawUTF8());

                        if (headerList == nullptr)
                        {
                            cleanup();
                            return false;
                        }

                        if (symbols->curl_easy_setopt (curl, CURLOPT_HTTPHEADER, headerList) != CURLE_OK)
                        {
                            cleanup();
                            return false;
                        }
                    }
                }

                listener = webInputListener;

                if (hasBodyDataToSend)
                    postBuffer = &headersAndPostData;

                size_t lastPos = static_cast<size_t> (-1);

                // step until either: 1) there is an error 2) the transaction is complete
                // or 3) data is in the in buffer
                while ((! finished) && curlBuffer.isEmpty())
                {
                    {
                        const ScopedLock lock (cleanupLock);

                        if (curl == nullptr)
                            return false;
                    }

                    singleStep();

                    // call callbacks if this is a post request
                    if (hasBodyDataToSend && listener != nullptr && lastPos != postPosition)
                    {
                        lastPos = postPosition;

                        if (! listener->postDataSendProgress (owner, static_cast<int> (lastPos), static_cast<int> (headersAndPostData.getSize())))
                        {
                            // user has decided to abort the transaction
                            cleanup();
                            return false;
                        }
                    }
                }

                {
                    const ScopedLock lock (cleanupLock);

                    if (curl == nullptr)
                        return false;

                    long responseCode;
                    if (symbols->curl_easy_getinfo (curl, CURLINFO_RESPONSE_CODE, &responseCode) == CURLE_OK)
                        statusCode = static_cast<int> (responseCode);

                    // get content length size
                    double curlLength;
                    if (symbols->curl_easy_getinfo (curl, CURLINFO_CONTENT_LENGTH_DOWNLOAD, &curlLength) == CURLE_OK)
                        contentLength = static_cast<int64> (curlLength);
                }

                return true;
        */
    }

    pub fn finish(&mut self)  {
        
        todo!();
        /*
            const ScopedLock lock (cleanupLock);

                if (curl == nullptr)
                    return;

                for (;;)
                {
                    int cnt = 0;

                    if (CURLMsg* msg = symbols->curl_multi_info_read (multi, &cnt))
                    {
                        if (msg->msg == CURLMSG_DONE && msg->easy_handle == curl)
                        {
                            lastError = msg->data.result; // this is the error that stopped our process from continuing
                            break;
                        }
                    }
                    else
                    {
                        break;
                    }
                }

                finished = true;
        */
    }

    pub fn single_step(&mut self)  {
        
        todo!();
        /*
            if (lastError != CURLE_OK)
                    return;

                fd_set fdread, fdwrite, fdexcep;
                int maxfd = -1;
                long curl_timeo;

                {
                    const ScopedLock lock (cleanupLock);

                    if (multi == nullptr)
                        return;

                    if ((lastError = (int) symbols->curl_multi_timeout (multi, &curl_timeo)) != CURLM_OK)
                        return;
                }

                // why 980? see http://curl.haxx.se/libcurl/c/curl_multi_timeout.html
                if (curl_timeo < 0)
                    curl_timeo = 980;

                struct timeval tv;
                tv.tv_sec = curl_timeo / 1000;
                tv.tv_usec = (curl_timeo % 1000) * 1000;

                FD_ZERO (&fdread);
                FD_ZERO (&fdwrite);
                FD_ZERO (&fdexcep);

                {
                    const ScopedLock lock (cleanupLock);

                    if (multi == nullptr)
                        return;

                    if ((lastError = (int) symbols->curl_multi_fdset (multi, &fdread, &fdwrite, &fdexcep, &maxfd)) != CURLM_OK)
                        return;
                }

                if (maxfd != -1)
                {
                    if (select (maxfd + 1, &fdread, &fdwrite, &fdexcep, &tv) < 0)
                    {
                        lastError = -1;
                        return;
                    }
                }
                else
                {
                    // if curl does not return any sockets for to wait on, then the doc says to wait 100 ms
                    Thread::sleep (100);
                }

                int still_running = 0;
                int curlRet;

                {
                    const ScopedLock lock (cleanupLock);

                    while ((curlRet = (int) symbols->curl_multi_perform (multi, &still_running)) == CURLM_CALL_MULTI_PERFORM)
                    {}
                }

                if ((lastError = curlRet) != CURLM_OK)
                    return;

                if (still_running <= 0)
                    finish();
        */
    }

    pub fn read_or_skip(&mut self, 
        buffer:        *mut c_void,
        bytes_to_read: i32,
        skip:          bool) -> i32 {
        
        todo!();
        /*
            if (bytesToRead <= 0)
                    return 0;

                size_t pos = 0;
                size_t len = static_cast<size_t> (bytesToRead);

                while (len > 0)
                {
                    size_t bufferBytes = curlBuffer.getSize();
                    bool removeSection = true;

                    if (bufferBytes == 0)
                    {
                        // do not call curl again if we are finished
                        {
                            const ScopedLock lock (cleanupLock);

                            if (finished || curl == nullptr)
                                return static_cast<int> (pos);
                        }

                        skipBytes = skip ? len : 0;
                        singleStep();

                        // update the amount that was read/skipped from curl
                        bufferBytes = skip ? len - skipBytes : curlBuffer.getSize();
                        removeSection = ! skip;
                    }

                    // can we copy data from the internal buffer?
                    if (bufferBytes > 0)
                    {
                        size_t max = jmin (len, bufferBytes);

                        if (! skip)
                            memcpy (addBytesToPointer (buffer, pos), curlBuffer.getData(), max);

                        pos += max;
                        streamPos += static_cast<int64> (max);
                        len -= max;

                        if (removeSection)
                            curlBuffer.removeSection (0, max);
                    }
                }

                return static_cast<int> (pos);
        */
    }

    /**
      | CURL callbacks
      |
      */
    pub fn curl_write_callback(&mut self, 
        ptr:   *mut u8,
        size:  usize,
        nmemb: usize) -> usize {
        
        todo!();
        /*
            if (curl == nullptr || lastError != CURLE_OK)
                    return 0;

                const size_t len = size * nmemb;

                // skip bytes if necessary
                size_t max = jmin (skipBytes, len);
                skipBytes -= max;

                if (len > max)
                    curlBuffer.append (ptr + max, len - max);

                return len;
        */
    }

    pub fn curl_read_callback(&mut self, 
        ptr:   *mut u8,
        size:  usize,
        nmemb: usize) -> usize {
        
        todo!();
        /*
            if (curl == nullptr || postBuffer == nullptr || lastError != CURLE_OK)
                    return 0;

                const size_t len = size * nmemb;

                size_t max = jmin (postBuffer->getSize() - postPosition, len);
                memcpy (ptr, (char*)postBuffer->getData() + postPosition, max);
                postPosition += max;

                return max;
        */
    }

    pub fn curl_header_callback(&mut self, 
        ptr:   *mut u8,
        size:  usize,
        nmemb: usize) -> usize {
        
        todo!();
        /*
            if (curl == nullptr || lastError != CURLE_OK)
                    return 0;

                size_t len = size * nmemb;

                String header (ptr, len);

                if (! header.contains (":") && header.startsWithIgnoreCase ("HTTP/"))
                    responseHeaders.clear();
                else
                    responseHeaders += header;

                return len;
        */
    }

    /**
      | Static method wrappers
      |
      */
    pub fn static_curl_write(
        ptr:      *mut u8,
        size:     usize,
        nmemb:    usize,
        userdata: *mut c_void) -> usize {
        
        todo!();
        /*
            WebInputStream::Impl* wi = reinterpret_cast<WebInputStream::Impl*> (userdata);
                return wi->curlWriteCallback (ptr, size, nmemb);
        */
    }

    pub fn static_curl_read(
        ptr:      *mut u8,
        size:     usize,
        nmemb:    usize,
        userdata: *mut c_void) -> usize {
        
        todo!();
        /*
            WebInputStream::Impl* wi = reinterpret_cast<WebInputStream::Impl*> (userdata);
                return wi->curlReadCallback (ptr, size, nmemb);
        */
    }

    pub fn static_curl_header(
        ptr:      *mut u8,
        size:     usize,
        nmemb:    usize,
        userdata: *mut c_void) -> usize {
        
        todo!();
        /*
            WebInputStream::Impl* wi = reinterpret_cast<WebInputStream::Impl*> (userdata);
                return wi->curlHeaderCallback (ptr, size, nmemb);
        */
    }
}
