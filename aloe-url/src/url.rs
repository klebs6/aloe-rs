crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_URL.h]

/**
  | Represents a Url and has a bunch of useful
  | functions to manipulate it.
  | 
  | This class can be used to launch URLs
  | in browsers, and also to create InputStreams
  | that can read from remote HTTP or FTP
  | sources.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct Url {

    #[cfg(target_os="ios")]
    bookmark:         Bookmark::Ptr,
    url:              String,
    post_data:        MemoryBlock,
    parameter_names:  Vec<String>,
    parameter_values: Vec<String>,
    files_to_upload:  ReferenceCountedArray<UrlUpload>,
}

impl Default for Url {
    
    /**
      | Creates an empty Url.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl Url {
    
    /**
      | Download the Url to a file.
      | 
      | This method attempts to download the
      | Url to a given file location.
      | 
      | Using this method to download files
      | on mobile is less flexible but more reliable
      | than using createInputStream or WebInputStreams
      | as it will attempt to download the file
      | using a native OS background network
      | task. Such tasks automatically deal
      | with network re-connections and continuing
      | your download while your app is suspended.
      |
      */
    #[cfg(ALOE_USE_CURL)]
    pub fn download_to_file(
        &mut self, 
        target_location: &File,
        extra_headers:   Option<String>,
        listener:        *mut dyn UrlDownloadTaskListener,
        should_use_post: Option<bool>

    ) -> Box<UrlDownloadTask> {

        let extra_headers: String = extra_headers.unwrap_or(String::new());
        let should_use_post: bool = should_use_post.unwrap_or(false);
        
        todo!();
        /*
            return Url::DownloadTask::createFallbackDownloader (*this, targetLocation, extraHeaders, listener, shouldUsePost);
        */
    }
    
    #[cfg(not(ALOE_USE_CURL))]
    pub fn download_to_file(
        &mut self, 
        target_location: &File,
        extra_headers:   String,
        listener:        *mut dyn UrlDownloadTaskListener,
        should_use_post: bool

    ) -> Box<UrlDownloadTask> {
        
        todo!();
        /*
            return Url::DownloadTask::createFallbackDownloader (*this, targetLocation, extraHeaders, listener, shouldUsePost);
        */
    }
}

impl Eq for Url {}

pub type UrlOpenStreamProgressCallback = fn(
    context:     *mut c_void,
    bytes_sent:  i32,
    total_bytes: i32
) -> bool;

pub enum UrlParameterHandling
{
    inAddress,
    inPostData
}

#[cfg(target_os="ios")]
pub struct UrlBookmark {
    base: ReferenceCountedObject,
    data: *mut c_void,
}

#[cfg(target_os="ios")]
impl Drop for UrlBookmark {

    fn drop(&mut self) {
        todo!();
        /* 
        [(NSData*) data release];
         */
    }
}

#[cfg(target_os="ios")]
pub type UrlBookmarkPtr = ReferenceCountedObjectPtr<Bookmark>;

#[cfg(target_os="ios")]
impl UrlBookmark {

    #[cfg(target_os="ios")]
    pub fn new_with_bookmark(bookmark_to_use: *mut c_void) -> Self {
    
        todo!();
        /*
        : data(bookmarkToUse),

        
        */
    }
}

///-------------------------------
#[no_copy]
#[leak_detector]
pub struct UrlUpload {
    base: ReferenceCountedObject,
    parameter_name: String,
    filename:       String,
    mime_type:      String,
    file:           File,
    data:           Box<MemoryBlock>,
}

impl UrlUpload {

    pub fn new(
        param: &String,
        name:  &String,
        mime:  &String,
        f:     &File,
        mb:    *mut MemoryBlock) -> Self {
    
        todo!();
        /*

            : parameterName (param), filename (name), mimeType (mime), file (f), data (mb)

        jassert (mimeType.isNotEmpty()); // You need to supply a mime type!
        */
    }
}

/** 
  | Class used to create a set of options to
  | pass to the createInputStream() method.
  |
  | You can chain together a series of calls
  | to this class's methods to create a set of
  | whatever options you want to specify, e.g.
  |
  |   @code
  |   if (auto inputStream = Url ("http://www.xyz.com/foobar")
  |         .createInputStream(
  |             Url::InputStreamOptions(Url::ParameterHandling::inAddress)
  |         .withConnectionTimeoutMs (1000)
  |         .withNumRedirectsToFollow (0)))
  |   {
  |       ...
  |   }
  |   @endcode
  */
pub struct UrlInputStreamOptions {
    parameter_handling:      UrlParameterHandling,
    progress_callback:       fn(_0: i32, _1: i32) -> bool, // default = nullptr
    extra_headers:           String,
    connection_time_out_ms:  i32, // default = 0
    response_headers:        *mut StringPairArray, // default = nullptr
    status_code:             *mut i32, // default = nullptr
    num_redirects_to_follow: i32, // default = 5
    http_request_cmd:        String,
}

impl UrlInputStreamOptions {

    pub fn get_parameter_handling(&self) -> UrlParameterHandling {
        
        todo!();
        /*
            return parameterHandling;
        */
    }
    
    pub fn get_progress_callback(&self) -> fn(_0: i32, _1: i32) -> bool {
        
        todo!();
        /*
            return progressCallback;
        */
    }
    
    pub fn get_extra_headers(&self) -> String {
        
        todo!();
        /*
            return extraHeaders;
        */
    }
    
    pub fn get_connection_timeout_ms(&self) -> i32 {
        
        todo!();
        /*
            return connectionTimeOutMs;
        */
    }
    
    pub fn get_response_headers(&self) -> *mut StringPairArray {
        
        todo!();
        /*
            return responseHeaders;
        */
    }
    
    pub fn get_status_code(&self) -> *mut i32 {
        
        todo!();
        /*
            return statusCode;
        */
    }
    
    pub fn get_num_redirects_to_follow(&self) -> i32 {
        
        todo!();
        /*
            return numRedirectsToFollow;
        */
    }
    
    pub fn get_http_request_cmd(&self) -> String {
        
        todo!();
        /*
            return httpRequestCmd;
        */
    }
    
    /**
      | Constructor.
      | 
      | If parameterHandling is ParameterHandling::inPostData,
      | any Url parameters that have been set
      | will be transferred via the request
      | body data. Otherwise the parameters
      | will be added to the Url address.
      |
      */
    pub fn new_with_parameter_handling(handling: UrlParameterHandling) -> Self {
    
        todo!();
        /*
        : parameter_handling(handling),

        
        */
    }
    
    /**
      | A callback function to keep track of
      | the operation's progress.
      | 
      | This can be useful for lengthy POST operations,
      | so that you can provide user feedback.
      |
      */
    pub fn with_progress_callback(&self, cb: fn(_0: i32, _1: i32) -> bool) -> UrlInputStreamOptions {
        
        todo!();
        /*
            return with (*this, &InputStreamOptions::progressCallback, std::move (cb));
        */
    }
    
    /**
      | A string that will be appended onto the
      | headers that are used for the request.
      | 
      | It must be a valid set of HTML header directives,
      | separated by newlines.
      |
      */
    pub fn with_extra_headers(&self, headers: &String) -> UrlInputStreamOptions {
        
        todo!();
        /*
            return with (*this, &InputStreamOptions::extraHeaders, headers);
        */
    }
    
    /**
      | Specifies a timeout for the request
      | in milliseconds.
      | 
      | If 0, this will use whatever default
      | setting the OS chooses. If a negative
      | number, it will be infinite.
      |
      */
    pub fn with_connection_timeout_ms(&self, timeout: i32) -> UrlInputStreamOptions {
        
        todo!();
        /*
            return with (*this, &InputStreamOptions::connectionTimeOutMs, timeout);
        */
    }
    
    /**
      | If this is non-null, all the (key, value)
      | pairs received as headers in the response
      | will be stored in this array.
      |
      */
    pub fn with_response_headers(&self, headers: *mut StringPairArray) -> UrlInputStreamOptions {
        
        todo!();
        /*
            return with (*this, &InputStreamOptions::responseHeaders, headers);
        */
    }
    
    /**
      | If this is non-null, it will get set to
      | the http status code, if one is known,
      | or 0 if a code isn't available.
      |
      */
    pub fn with_status_code(&self, status: *mut i32) -> UrlInputStreamOptions {
        
        todo!();
        /*
            return with (*this, &InputStreamOptions::statusCode, status);
        */
    }
    
    /**
      | Specifies the number of redirects that
      | will be followed before returning a
      | response.
      | 
      | N.B. This will be ignored on Android
      | which follows up to 5 redirects.
      |
      */
    pub fn with_num_redirects_to_follow(&self, num_redirects: i32) -> UrlInputStreamOptions {
        
        todo!();
        /*
            return with (*this, &InputStreamOptions::numRedirectsToFollow, numRedirects);
        */
    }
    
    /**
      | Specifies which HTTP request command
      | to use.
      | 
      | If this is not set, then the command will
      | be POST if parameterHandling is set
      | to ParameterHandling::inPostData
      | or if any POST data has been specified
      | via withPOSTData(), withFileToUpload(),
      | or withDataToUpload(). Otherwise
      | it will be GET.
      |
      */
    pub fn with_http_request_cmd(&self, cmd: &String) -> UrlInputStreamOptions {
        
        todo!();
        /*
            return with (*this, &InputStreamOptions::httpRequestCmd, cmd);
        */
    }
}

/**
  | Used to receive callbacks for download
  | progress.
  |
  */
pub trait UrlDownloadTaskListener {

    /**
      | Called when the download has finished.
      | Be aware that this callback may come
      | on an arbitrary thread.
      |
      */
    fn finished(
        &mut self, 
        task:    *mut UrlDownloadTask,
        success: bool
    );

    /**
      | Called periodically by the OS to indicate
      | download progress.
      | 
      | Beware that this callback may come on
      | an arbitrary thread.
      |
      */
    fn progress(
        &mut self, 
        task:             *mut UrlDownloadTask,
        bytes_downloaded: i64,
        total_length:     i64)  {
        
        todo!();
        /*
        
        */
    }
}

/**
  | Represents a download task.
  | 
  | Returned by downloadToFile() to allow
  | querying and controlling the download
  | task.
  |
  */
#[no_copy]
#[leak_detector]
pub struct UrlDownloadTask {
    content_length:  i64, // default = -1
    downloaded:      i64, // default = 0
    finished:        bool, // default = false
    error:           bool, // default = false
    http_code:       i32, // default = -1
    target_location: File,
}

impl Drop for UrlDownloadTask {

    /**
      | Releases the resources of the download
      | task, unregisters the listener and
      | cancels the download if necessary.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

impl UrlDownloadTask {

    /**
      | Returns the total length of the download
      | task.
      | 
      | This may return -1 if the length was not
      | returned by the server.
      |
      */
    pub fn get_total_length(&self) -> i64 {
        
        todo!();
        /*
            return contentLength;
        */
    }

    /**
      | Returns the number of bytes that have
      | been downloaded so far.
      |
      */
    pub fn get_length_downloaded(&self) -> i64 {
        
        todo!();
        /*
            return downloaded;
        */
    }

    /**
      | Returns true if the download finished
      | or there was an error.
      |
      */
    pub fn is_finished(&self) -> bool {
        
        todo!();
        /*
            return finished;
        */
    }

    /**
      | Returns the status code of the server's
      | response.
      | 
      | This will only be valid after the download
      | has finished.
      | 
      | @see isFinished
      |
      */
    pub fn status_code(&self) -> i32 {
        
        todo!();
        /*
            return httpCode;
        */
    }

    /**
      | Returns true if there was an error.
      |
      */
    #[inline] pub fn had_error(&self) -> bool {
        
        todo!();
        /*
            return error;
        */
    }

    /**
      | Returns the target file location that
      | was provided in Url::downloadToFile.
      |
      */
    pub fn get_target_location(&self) -> File {
        
        todo!();
        /*
            return targetLocation;
        */
    }
    
    /** internal **/
    #[cfg(target_os="ios")]
    pub fn aloe_ios_url_session_notify(_0: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn create_fallback_downloader(&mut self, 
        url_to_use:           &Url,
        target_file_to_use:   &File,
        extra_headers_to_use: &String,
        listener_to_use:      *mut dyn UrlDownloadTaskListener,
        use_post_request:     bool) -> Box<UrlDownloadTask> {
        
        todo!();
        /*
            const size_t bufferSize = 0x8000;
        targetFileToUse.deleteFile();

        if (auto outputStream = targetFileToUse.createOutputStream (bufferSize))
        {
            auto stream = std::make_unique<WebInputStream> (urlToUse, usePostRequest);
            stream->withExtraHeaders (extraHeadersToUse);

            if (stream->connect (nullptr))
                return std::make_unique<FallbackDownloadTask> (std::move (outputStream),
                                                               bufferSize,
                                                               std::move (stream),
                                                               listenerToUse);
        }

        return nullptr;
        */
    }
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_URL.cpp]
impl PartialEq<Url> for Url {
    
    /**
      | Compares two URLs.
      | 
      | All aspects of the URLs must be identical
      | for them to match, including any parameters,
      | upload files, etc.
      |
      */
    #[inline] fn eq(&self, other: &Url) -> bool {
        todo!();
        /*
            return url == other.url
            && postData == other.postData
            && parameterNames == other.parameterNames
            && parameterValues == other.parameterValues
            && filesToUpload == other.filesToUpload;
        */
    }
}

impl Url {

    /**
      | Returns an array of the names of all the
      | Url's parameters.
      | 
      | e.g. for the url "www.fish.com?type=haddock&amount=some+fish",
      | this array would contain two items:
      | "type" and "amount".
      | 
      | You can call getParameterValues()
      | to get the corresponding value of each
      | parameter. Note that the list can contain
      | multiple parameters with the same name.
      | 
      | @see getParameterValues, withParameter
      |
      */
    pub fn get_parameter_names(&self) -> &Vec<String> {
        
        todo!();
        /*
            return parameterNames;
        */
    }

    /**
      | Returns an array of the values of all
      | the Url's parameters.
      | 
      | e.g. for the url "www.fish.com?type=haddock&amount=some+fish",
      | this array would contain two items:
      | "haddock" and "some fish".
      | 
      | The values returned will have been cleaned
      | up to remove any escape characters.
      | 
      | You can call getParameterNames() to
      | get the corresponding name of each parameter.
      | Note that the list can contain multiple
      | parameters with the same name.
      | 
      | @see getParameterNames, withParameter
      |
      */
    pub fn get_parameter_values(&self) -> &Vec<String> {
        
        todo!();
        /*
            return parameterValues;
        */
    }

    /**
      | Returns the data that was set using withPOSTData().
      |
      */
    pub fn get_post_data(&self) -> String {
        
        todo!();
        /*
            return postData.toString();
        */
    }

    /**
      | Returns the data that was set using withPOSTData()
      | as a MemoryBlock.
      |
      */
    pub fn get_post_data_as_memory_block(&self) -> &MemoryBlock {
        
        todo!();
        /*
            return postData;
        */
    }

    /**
      | Creates a Url from a string.
      | 
      | This will parse any embedded parameters
      | after a '?' character and store them
      | in the list (see getParameterNames
      | etc). If you don't want this to happen,
      | you can use createWithoutParsing().
      |
      */
    pub fn new(u: &String) -> Self {
    
        todo!();
        /*
        : url(u),

            init();
        */
    }

    /**
      | Creates Url referring to a local file
      | on your disk using the file:// scheme.
      |
      */
    pub fn new_from_local_file(local_file: File) -> Self {
    
        todo!();
        /*

            if (localFile == File())
            return;

       #if ALOE_WINDOWS
        bool isUncPath = localFile.getFullPathName().startsWith ("\\\\");
       #endif

        while (! localFile.isRoot())
        {
            url = "/" + addEscapeChars (localFile.getFileName(), false) + url;
            localFile = localFile.getParentDirectory();
        }

        url = addEscapeChars (localFile.getFileName(), false) + url;

       #if ALOE_WINDOWS
        if (isUncPath)
        {
            url = url.fromFirstOccurrenceOf ("/", false, false);
        }
        else
       #endif
        {
            if (! url.startsWithChar (L'/'))
                url = "/" + url;
        }

        url = "file://" + url;

        jassert (isWellFormed());
        */
    }

    pub fn init(&mut self)  {
        
        todo!();
        /*
            auto i = url.indexOfChar ('?');

        if (i >= 0)
        {
            do
            {
                auto nextAmp   = url.indexOfChar (i + 1, '&');
                auto equalsPos = url.indexOfChar (i + 1, '=');

                if (nextAmp < 0)
                {
                    addParameter (removeEscapeChars (equalsPos < 0 ? url.substring (i + 1) : url.substring (i + 1, equalsPos)),
                                  equalsPos < 0 ? String() : removeEscapeChars (url.substring (equalsPos + 1)));
                }
                else if (nextAmp > 0 && equalsPos < nextAmp)
                {
                    addParameter (removeEscapeChars (equalsPos < 0 ? url.substring (i + 1, nextAmp) : url.substring (i + 1, equalsPos)),
                                  equalsPos < 0 ? String() : removeEscapeChars (url.substring (equalsPos + 1, nextAmp)));
                }

                i = nextAmp;
            }
            while (i >= 0);

            url = url.upToFirstOccurrenceOf ("?", false, false);
        }
        */
    }

    pub fn new_from_u(
        u:  &String,
        _1: i32) -> Self {
    
        todo!();
        /*
        : url(u),

        
        */
    }

    /**
      | Returns a Url without attempting to
      | remove any embedded parameters from
      | the string.
      | 
      | This may be necessary if you need to create
      | a request that involves both POST parameters
      | and parameters which are embedded in
      | the Url address itself.
      |
      */
    pub fn create_without_parsing(&mut self, u: &String) -> Url {
        
        todo!();
        /*
            return Url (u, 0);
        */
    }

    pub fn add_parameter(&mut self, 
        name:  &String,
        value: &String)  {
        
        todo!();
        /*
            parameterNames.add (name);
        parameterValues.add (value);
        */
    }

    /**
      | Returns a string version of the Url.
      | 
      | -----------
      | @param includeGetParameters
      | 
      | if this is true and any parameters have
      | been set with the withParameter() method,
      | then the string will have these appended
      | on the end and Url-encoded.
      | 
      | @see getQueryString
      |
      */
    pub fn to_string(&self, include_get_parameters: bool) -> String {
        
        todo!();
        /*
            if (includeGetParameters)
            return url + getQueryString();

        return url;
        */
    }

    /**
      | Returns true if the Url is an empty string.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return url.isEmpty();
        */
    }

    /**
      | True if it seems to be valid.
      |
      */
    pub fn is_well_formed(&self) -> bool {
        
        todo!();
        /*
            //xxx TODO
        return url.isNotEmpty();
        */
    }

    /**
      | Returns just the domain part of the Url.
      | 
      | e.g. for "http://www.xyz.com/foobar",
      | this will return "www.xyz.com".
      |
      */
    pub fn get_domain(&self) -> String {
        
        todo!();
        /*
            return getDomainInternal (false);
        */
    }

    /**
      | Returns the path part of the Url.
      | 
      | e.g. for "http://www.xyz.com/foo/bar?x=1",
      | this will return "foo/bar".
      | 
      | -----------
      | @param includeGetParameters
      | 
      | if this is true and any parameters have
      | been set with the withParameter() method,
      | then the string will have these appended
      | on the end and Url-encoded.
      | 
      | @see getQueryString
      |
      */
    pub fn get_sub_path(
        &self, 
        include_get_parameters: Option<bool>

    ) -> String {

        let include_get_parameters: bool = include_get_parameters.unwrap_or(false);
        
        todo!();
        /*
            auto startOfPath = URLHelpers::findStartOfPath (url);
        auto subPath = startOfPath <= 0 ? String()
                                        : url.substring (startOfPath);

        if (includeGetParameters)
            subPath += getQueryString();

        return subPath;
        */
    }

    /**
      | If any parameters are set, returns these
      | Url-encoded, including the "?" prefix.
      |
      */
    pub fn get_query_string(&self) -> String {
        
        todo!();
        /*
            if (parameterNames.size() > 0)
            return "?" + URLHelpers::getMangledParameters (*this);

        return {};
        */
    }

    /**
      | Returns the scheme of the Url.
      | 
      | e.g. for "http://www.xyz.com/foobar",
      | this will return "http" (it won't include
      | the colon).
      |
      */
    pub fn get_scheme(&self) -> String {
        
        todo!();
        /*
            return url.substring (0, URLHelpers::findEndOfScheme (url) - 1);
        */
    }
    
    /**
      | Returns true if this Url refers to a local
      | file.
      |
      */
    #[cfg(not(target_os="android"))]
    pub fn is_local_file(&self) -> bool {
        
        todo!();
        /*
            return getScheme() == "file";
        */
    }

    /**
      | Returns the file path of the local file
      | to which this Url refers to.
      | 
      | If the Url does not represent a local
      | file Url (i.e. the Url's scheme is not
      | 'file') then this method will assert.
      | 
      | This method also supports converting
      | Android's content:// URLs to local
      | file paths.
      | 
      | @see isLocalFile
      |
      */
    #[cfg(not(target_os="android"))]
    pub fn get_local_file(&self) -> File {
        
        todo!();
        /*
            return fileFromFileSchemeURL (*this);
        */
    }

    /**
      | Returns the file name.
      | 
      | For all but Android's content:// scheme,
      | it will simply return the last segment
      | of the Url, e.g. for "http://www.xyz.com/foo/bar.txt",
      | this will return "bar.txt".
      | 
      | For Android's content:// scheme, it
      | will attempt to resolve the filename
      | located under the Url.
      |
      */
    #[cfg(not(target_os="android"))]
    pub fn get_file_name(&self) -> String {
        
        todo!();
        /*
            return toString (false).fromLastOccurrenceOf ("/", false, true);
        */
    }
    
    pub fn to_handling(&mut self, use_post_data: bool) -> UrlParameterHandling {
        
        todo!();
        /*
            return usePostData ? ParameterHandling::inPostData : ParameterHandling::inAddress;
        */
    }

    pub fn file_from_file_schemeurl(&mut self, fileurl: &Url) -> File {
        
        todo!();
        /*
            if (! fileURL.isLocalFile())
        {
            jassertfalse;
            return {};
        }

        auto path = removeEscapeChars (fileURL.getDomainInternal (true)).replace ("+", "%2B");

       #if ALOE_WINDOWS
        bool isUncPath = (! fileURL.url.startsWith ("file:///"));
       #else
        path = File::getSeparatorString() + path;
       #endif

        auto urlElements = Vec<String>::fromTokens (fileURL.getSubPath(), "/", "");

        for (auto urlElement : urlElements)
            path += File::getSeparatorString() + removeEscapeChars (urlElement.replace ("+", "%2B"));

       #if ALOE_WINDOWS
        if (isUncPath)
            path = "\\\\" + path;
       #endif

        return path;
        */
    }

    /**
      | Attempts to read a port number from the
      | Url.
      | 
      | -----------
      | @return
      | 
      | the port number, or 0 if none is explicitly
      | specified.
      |
      */
    pub fn get_port(&self) -> i32 {
        
        todo!();
        /*
            auto colonPos = url.indexOfChar (URLHelpers::findStartOfNetLocation (url), ':');

        return colonPos > 0 ? url.substring (colonPos + 1).getIntValue() : 0;
        */
    }

    /**
      | Returns a new version of this Url with
      | a different domain and path.
      | 
      | e.g. if the Url is "http://www.xyz.com/foo?x=1"
      | and you call this with "abc.com/zzz",
      | it'll return "http://abc.com/zzz?x=1".
      | 
      | @see withNewSubPath
      |
      */
    pub fn with_new_domain_and_path(&self, newurl: &String) -> Url {
        
        todo!();
        /*
            Url u (*this);
        u.url = newURL;
        return u;
        */
    }

    /**
      | Returns a new version of this Url with
      | a different sub-path.
      | 
      | e.g. if the Url is "http://www.xyz.com/foo?x=1"
      | and you call this with "bar", it'll return
      | "http://www.xyz.com/bar?x=1".
      | 
      | @see withNewDomainAndPath
      |
      */
    pub fn with_new_sub_path(&self, new_path: &String) -> Url {
        
        todo!();
        /*
            Url u (*this);

        auto startOfPath = URLHelpers::findStartOfPath (url);

        if (startOfPath > 0)
            u.url = url.substring (0, startOfPath);

        URLHelpers::concatenatePaths (u.url, newPath);
        return u;
        */
    }

    /**
      | Attempts to return a Url which is the
      | parent folder containing this Url.
      | 
      | If there isn't a parent, this method
      | will just return a copy of this Url.
      |
      */
    pub fn get_parenturl(&self) -> Url {
        
        todo!();
        /*
            Url u (*this);
        u.url = URLHelpers::removeLastPathSection (u.url);
        return u;
        */
    }

    /**
      | Returns a new Url that refers to a sub-path
      | relative to this one.
      | 
      | e.g. if the Url is "http://www.xyz.com/foo"
      | and you call this with "bar", it'll return
      | "http://www.xyz.com/foo/bar".
      | 
      | Note that there's no way for this method
      | to know whether the original Url is a
      | file or directory, so it's up to you to
      | make sure it's a directory. It also won't
      | attempt to be smart about the content
      | of the childPath string, so if this string
      | is an absolute Url, it'll still just
      | get bolted onto the end of the path.
      | 
      | @see File::getChildFile
      |
      */
    pub fn get_childurl(&self, sub_path: &String) -> Url {
        
        todo!();
        /*
            Url u (*this);
        URLHelpers::concatenatePaths (u.url, subPath);
        return u;
        */
    }

    pub fn has_body_data_to_send(&self) -> bool {
        
        todo!();
        /*
            return filesToUpload.size() > 0 || ! postData.isEmpty();
        */
    }

    pub fn create_headers_and_post_data(&self, 
        headers:                &mut String,
        post_data_to_write:     &mut MemoryBlock,
        add_parameters_to_body: bool)  {
        
        todo!();
        /*
            MemoryOutputStream data (postDataToWrite, false);

        if (filesToUpload.size() > 0)
        {
            // (this doesn't currently support mixing custom post-data with uploads..)
            jassert (postData.isEmpty());

            auto boundary = String::toHexString (Random::getSystemRandom().nextInt64());

            headers << "Content-Type: multipart/form-data; boundary=" << boundary << "\r\n";

            data << "--" << boundary;

            for (int i = 0; i < parameterNames.size(); ++i)
            {
                data << "\r\nContent-Disposition: form-data; name=\"" << parameterNames[i]
                     << "\"\r\n\r\n" << parameterValues[i]
                     << "\r\n--" << boundary;
            }

            for (auto* f : filesToUpload)
            {
                data << "\r\nContent-Disposition: form-data; name=\"" << f->parameterName
                     << "\"; filename=\"" << f->filename << "\"\r\n";

                if (f->mimeType.isNotEmpty())
                    data << "Content-Type: " << f->mimeType << "\r\n";

                data << "Content-Transfer-Encoding: binary\r\n\r\n";

                if (f->data != nullptr)
                    data << *f->data;
                else
                    data << f->file;

                data << "\r\n--" << boundary;
            }

            data << "--\r\n";
        }
        else
        {
            if (addParametersToBody)
                data << URLHelpers::getMangledParameters (*this);

            data << postData;

            // if the user-supplied headers didn't contain a content-type, add one now..
            if (! headers.containsIgnoreCase ("Content-Type"))
                headers << "Content-Type: application/x-www-form-urlencoded\r\n";

            headers << "Content-length: " << (int) data.getDataSize() << "\r\n";
        }
        */
    }

    /**
      | Takes a guess as to whether a string might
      | be a valid website address. This isn't
      | foolproof!
      |
      */
    pub fn is_probablya_websiteurl(&mut self, possibleurl: &String) -> bool {
        
        todo!();
        /*
            for (auto* protocol : { "http:", "https:", "ftp:" })
            if (possibleURL.startsWithIgnoreCase (protocol))
                return true;

        if (possibleURL.containsChar ('@') || possibleURL.containsChar (' '))
            return false;

        auto topLevelDomain = possibleURL.upToFirstOccurrenceOf ("/", false, false)
                                         .fromLastOccurrenceOf (".", false, false);

        return topLevelDomain.isNotEmpty() && topLevelDomain.length() <= 3;
        */
    }

    /**
      | Takes a guess as to whether a string might
      | be a valid email address. This isn't
      | foolproof!
      |
      */
    pub fn is_probably_an_email_address(&mut self, possible_email_address: &String) -> bool {
        
        todo!();
        /*
            auto atSign = possibleEmailAddress.indexOfChar ('@');

        return atSign > 0
            && possibleEmailAddress.lastIndexOfChar ('.') > (atSign + 1)
            && ! possibleEmailAddress.endsWithChar ('.');
        */
    }

    pub fn get_domain_internal(&self, ignore_port: bool) -> String {
        
        todo!();
        /*
            auto start = URLHelpers::findStartOfNetLocation (url);
        auto end1 = url.indexOfChar (start, '/');
        auto end2 = ignorePort ? -1 : url.indexOfChar (start, ':');

        auto end = (end1 < 0 && end2 < 0) ? std::numeric_limits<int>::max()
                                          : ((end1 < 0 || end2 < 0) ? jmax (end1, end2)
                                                                    : jmin (end1, end2));
        return url.substring (start, end);
        */
    }

    pub fn get_mangled_parameters(url: &Url) -> String {
        
        todo!();
        /*
            jassert (url.getParameterNames().size() == url.getParameterValues().size());
                String p;

                for (int i = 0; i < url.getParameterNames().size(); ++i)
                {
                    if (i > 0)
                        p << '&';

                    auto val = url.getParameterValues()[i];

                    p << Url::addEscapeChars (url.getParameterNames()[i], true);

                    if (val.isNotEmpty())
                        p << '=' << Url::addEscapeChars (val, true);
                }

                return p;
        */
    }

    pub fn find_end_of_scheme(url: &String) -> i32 {
        
        todo!();
        /*
            int i = 0;

                while (CharacterFunctions::isLetterOrDigit (url[i])
                       || url[i] == '+' || url[i] == '-' || url[i] == '.')
                    ++i;

                return url.substring (i).startsWith ("://") ? i + 1 : 0;
        */
    }

    pub fn find_start_of_net_location(url: &String) -> i32 {
        
        todo!();
        /*
            int start = findEndOfScheme (url);

                while (url[start] == '/')
                    ++start;

                return start;
        */
    }

    pub fn find_start_of_path(url: &String) -> i32 {
        
        todo!();
        /*
            return url.indexOfChar (findStartOfNetLocation (url), '/') + 1;
        */
    }

    pub fn concatenate_paths(
            path:   &mut String,
            suffix: &String)  {
        
        todo!();
        /*
            if (! path.endsWithChar ('/'))
                    path << '/';

                if (suffix.startsWithChar ('/'))
                    path += suffix.substring (1);
                else
                    path += suffix;
        */
    }

    pub fn remove_last_path_section(url: &String) -> String {
        
        todo!();
        /*
            auto startOfPath = findStartOfPath (url);
                auto lastSlash = url.lastIndexOfChar ('/');

                if (lastSlash > startOfPath && lastSlash == url.length() - 1)
                    return removeLastPathSection (url.dropLastCharacters (1));

                if (lastSlash < 0)
                    return url;

                return url.substring (0, std::max (startOfPath, lastSlash));
        */
    }
}

#[cfg(target_os="ios")]
pub fn set_url_bookmark(
        u:        &mut Url,
        bookmark: *mut c_void)  {
    
    todo!();
    /*
        u.bookmark = new Url::Bookmark (bookmark);
    */
}

#[cfg(target_os="ios")]
pub fn get_url_bookmark(u: &mut Url)  {
    
    todo!();
    /*
        if (u.bookmark.get() == nullptr)
            return nullptr;

        return u.bookmark.get()->data;
    */
}

///----------------------------------
#[cfg(target_os="ios")]
pub struct iOSFileStreamWrapperFlush<Stream> {

}

#[cfg(target_os="ios")]
impl<Stream> iOSFileStreamWrapperFlush<Stream> {

    pub fn flush(_0: *mut Stream)  {
        
        todo!();
        /*
        
        */
    }
}

#[cfg(target_os="ios")]
impl iOSFileStreamWrapperFlush<FileOutputStream> {

    pub fn flush(o: *mut OutputStream)  {
        
        todo!();
        /*
            o->flush();
        */
    }
}

///----------------------------------
#[cfg(target_os="ios")]
pub struct iOSFileStreamWrapper<Stream> {
    base:                      Stream,
    url:                       Url,
    security_access_succeeded: bool, // default = false
}

#[cfg(target_os="ios")]
impl Drop for iOSFileStreamWrapper {
    fn drop(&mut self) {
        todo!();
        /* 
            iOSFileStreamWrapperFlush<Stream>::flush (this);

            if (NSData* bookmark = (NSData*) getURLBookmark (url))
            {
                BOOL isBookmarkStale = false;
                NSError* error = nil;

                auto nsURL = [NSURL URLByResolvingBookmarkData: bookmark
                                                       options: 0
                                                 relativeToURL: nil
                                           bookmarkDataIsStale: &isBookmarkStale
                                                         error: &error];

                if (error == nil)
                {
                    if (isBookmarkStale)
                        updateStaleBookmark (nsURL, url);

                    [nsURL stopAccessingSecurityScopedResource];
                }
                else
                {
                    auto desc = [error localizedDescription];
                    ignoreUnused (desc);
                    jassertfalse;
                }
            }
         */
    }
}

#[cfg(target_os="ios")]
impl<Stream> iOSFileStreamWrapper<Stream> {

    #[cfg(target_os="ios")]
    pub fn new(url_to_use: &mut Url) -> Self {
    
        todo!();
        /*

            : Stream (getLocalFileAccess (urlToUse)),
              url (urlToUse)
        */
    }
    
    #[cfg(target_os="ios")]
    pub fn get_local_file_access(&mut self, url_to_use: &mut Url) -> File {
        
        todo!();
        /*
            if (NSData* bookmark = (NSData*) getURLBookmark (urlToUse))
            {
                BOOL isBookmarkStale = false;
                NSError* error = nil;

                auto nsURL = [NSURL URLByResolvingBookmarkData: bookmark
                                                       options: 0
                                                 relativeToURL: nil
                                           bookmarkDataIsStale: &isBookmarkStale
                                                          error: &error];

                if (error == nil)
                {
                    securityAccessSucceeded = [nsURL startAccessingSecurityScopedResource];

                    if (isBookmarkStale)
                        updateStaleBookmark (nsURL, urlToUse);

                    return urlToUse.getLocalFile();
                }

                auto desc = [error localizedDescription];
                ignoreUnused (desc);
                jassertfalse;
            }

            return urlToUse.getLocalFile();
        */
    }
    
    #[cfg(target_os="ios")]
    pub fn update_stale_bookmark(&mut self, 
        nsurl:    *mut NSURL,
        aloe_url: &mut Url)  {
        
        todo!();
        /*
            NSError* error = nil;

            NSData* bookmark = [nsURL bookmarkDataWithOptions: NSURLBookmarkCreationSuitableForBookmarkFile
                               includingResourceValuesForKeys: nil
                                                relativeToURL: nil
                                                        error: &error];

            if (error == nil)
                setURLBookmark (aloeUrl, (void*) bookmark);
            else
                jassertfalse;
        */
    }
}

pub fn with<Member, Item>(
        options: UrlInputStreamOptions,
        member:  Member,
        item:    Item) -> UrlInputStreamOptions {

    todo!();
    /*
        options.*member = std::forward<Item> (item);
        return options;
    */
}

/**
  | Attempts to open a stream that can read
  | from this Url.
  | 
  | -----------
  | @note
  | 
  | this method will block until the first
  | byte of data has been received or an error
  | has occurred.
  | ----------
  | @note
  | 
  | on some platforms (Android, for example)
  | it's not permitted to do any network
  | action from the message thread, so you
  | must only call it from a background thread.
  | 
  | Unless the Url represents a local file,
  | this method returns an instance of a
  | WebInputStream. You can use dynamic_cast
  | to cast the return value to a WebInputStream
  | which allows you more fine-grained
  | control of the transfer process.
  | 
  | If the Url represents a local file, then
  | this method simply returns a FileInputStream.
  | 
  | -----------
  | @param options
  | 
  | a set of options that will be used when
  | opening the stream.
  | 
  | -----------
  | @return
  | 
  | a valid input stream, or nullptr if there
  | was an error trying to open it.
  |
  */
pub fn create_input_stream<R: Read>(options: &UrlInputStreamOptions) -> Box<R> {
    
    todo!();
    /*
        if (isLocalFile())
        {
           #if ALOE_IOS
            // We may need to refresh the embedded bookmark.
            return std::make_unique<iOSFileStreamWrapper<FileInputStream>> (const_cast<Url&> (*this));
           #else
            return getLocalFile().createInputStream();
           #endif
        }

        auto webInputStream = [&]
        {
            const auto usePost = options.getParameterHandling() == ParameterHandling::inPostData;
            auto stream = std::make_unique<WebInputStream> (*this, usePost);

            auto extraHeaders = options.getExtraHeaders();

            if (extraHeaders.isNotEmpty())
                stream->withExtraHeaders (extraHeaders);

            auto timeout = options.getConnectionTimeoutMs();

            if (timeout != 0)
                stream->withConnectionTimeout (timeout);

            auto requestCmd = options.getHttpRequestCmd();

            if (requestCmd.isNotEmpty())
                stream->withCustomRequestCommand (requestCmd);

            stream->withNumRedirectsToFollow (options.getNumRedirectsToFollow());

            return stream;
        }();

        struct ProgressCallbackCaller  : public WebInputStream::Listener
        {
            ProgressCallbackCaller (std::function<bool (int, int)> progressCallbackToUse)
                : callback (std::move (progressCallbackToUse))
            {
            }

            bool postDataSendProgress (WebInputStream&, int bytesSent, int totalBytes) override
            {
                return callback (bytesSent, totalBytes);
            }

            std::function<bool (int, int)> callback;
        };

        auto callbackCaller = [&options]() -> std::unique_ptr<ProgressCallbackCaller>
        {
            if (auto progressCallback = options.getProgressCallback())
                return std::make_unique<ProgressCallbackCaller> (progressCallback);

            return {};
        }();

        auto success = webInputStream->connect (callbackCaller.get());

        if (auto* status = options.getStatusCode())
            *status = webInputStream->getStatusCode();

        if (auto* responseHeaders = options.getResponseHeaders())
            *responseHeaders = webInputStream->getResponseHeaders();

        if (! success || webInputStream->isError())
            return nullptr;

        // std::move() needed here for older compilers
        ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wredundant-move")
        return std::move (webInputStream);
        ALOE_END_IGNORE_WARNINGS_GCC_LIKE
    */
}

#[cfg(target_os="android")]
pub fn aloe_create_content_uri_output_stream(_0: &Url) -> *mut OutputStream {
    
    todo!();
    /*
    
    */
}

impl Url {
    
    /**
      | Attempts to open an output stream to
      | a Url for writing
      | 
      | This method can only be used for certain
      | scheme types such as local files and
      | content:// URIs on Android.
      |
      */
    pub fn create_output_stream<W: Write>(&self) -> Box<W> {
        
        todo!();
        /*
            if (isLocalFile())
        {
           #if ALOE_IOS
            // We may need to refresh the embedded bookmark.
            return std::make_unique<iOSFileStreamWrapper<FileOutputStream>> (const_cast<Url&> (*this));
           #else
            return std::make_unique<FileOutputStream> (getLocalFile());
           #endif
        }

       #if ALOE_ANDROID
        return std::unique_ptr<OutputStream> (aloe_CreateContentURIOutputStream (*this));
       #else
        return nullptr;
       #endif
        */
    }
    
    /**
      | Tries to download the entire contents
      | of this Url into a binary data block.
      | 
      | If it succeeds, this will return true
      | and append the data it read onto the end
      | of the memory block.
      | 
      | Note that on some platforms (Android,
      | for example) it's not permitted to do
      | any network action from the message
      | thread, so you must only call it from
      | a background thread.
      | 
      | -----------
      | @param destData
      | 
      | the memory block to append the new data
      | to.
      | ----------
      | @param usePostCommand
      | 
      | whether to use a POST command to get the
      | data (uses a GET command if this is false).
      | 
      | @see readEntireTextStream, readEntireXmlStream
      |
      */
    pub fn read_entire_binary_stream(
        &self, 
        dest_data:        &mut MemoryBlock,
        use_post_command: Option<bool>

    ) -> bool {

        let use_post_command: bool = use_post_command.unwrap_or(false);
        
        todo!();
        /*
            const std::unique_ptr<InputStream> in (isLocalFile() ? getLocalFile().createInputStream()
                                                             : createInputStream (InputStreamOptions (toHandling (usePostCommand))));

        if (in != nullptr)
        {
            in->readIntoMemoryBlock (destData);
            return true;
        }

        return false;
        */
    }
    
    /**
      | Tries to download the entire contents
      | of this Url as a string.
      | 
      | If it fails, this will return an empty
      | string, otherwise it will return the
      | contents of the downloaded file. If
      | you need to distinguish between a read
      | operation that fails and one that returns
      | an empty string, you'll need to use a
      | different method, such as readEntireBinaryStream().
      | 
      | Note that on some platforms (Android,
      | for example) it's not permitted to do
      | any network action from the message
      | thread, so you must only call it from
      | a background thread.
      | 
      | -----------
      | @param usePostCommand
      | 
      | whether to use a POST command to get the
      | data (uses a GET command if this is false).
      | 
      | @see readEntireBinaryStream, readEntireXmlStream
      |
      */
    pub fn read_entire_text_stream(
        &self, 
        use_post_command: Option<bool>

    ) -> String {

        let use_post_command: bool = use_post_command.unwrap_or(false);
        
        todo!();
        /*
            const std::unique_ptr<InputStream> in (isLocalFile() ? getLocalFile().createInputStream()
                                                             : createInputStream (InputStreamOptions (toHandling (usePostCommand))));

        if (in != nullptr)
            return in->readEntireStreamAsString();

        return {};
        */
    }
    
    /**
      | Tries to download the entire contents
      | of this Url and parse it as XML.
      | 
      | If it fails, or if the text that it reads
      | can't be parsed as XML, this will return
      | nullptr.
      | 
      | Note that on some platforms (Android,
      | for example) it's not permitted to do
      | any network action from the message
      | thread, so you must only call it from
      | a background thread.
      | 
      | -----------
      | @param usePostCommand
      | 
      | whether to use a POST command to get the
      | data (uses a GET command if this is false).
      | 
      | @see readEntireBinaryStream, readEntireTextStream
      |
      */
    pub fn read_entire_xml_stream(&self, use_post_command: Option<bool>) -> Box<XmlElement> {

        let use_post_command: bool = use_post_command.unwrap_or(false);
        
        todo!();
        /*
            return parseXML (readEntireTextStream (usePostCommand));
        */
    }
    
    /**
      | Returns a copy of this Url, with a GET
      | or POST parameter added to the end.
      | 
      | Any control characters in the value
      | will be Url-encoded.
      | 
      | e.g. calling "withParameter ("amount",
      | "some fish") for the url "www.fish.com"
      | would produce a new url whose `toString
      | (true)` method would return "www.fish.com?amount=some+fish".
      | 
      | @see getParameterNames, getParameterValues
      |
      */
    pub fn with_parameter(&self, 
        parameter_name:  &String,
        parameter_value: &String) -> Url {
        
        todo!();
        /*
            auto u = *this;
        u.addParameter (parameterName, parameterValue);
        return u;
        */
    }
    
    /**
      | Returns a copy of this Url, with a set
      | of GET or POST parameters added.
      | 
      | This is a convenience method, equivalent
      | to calling withParameter for each value.
      | 
      | @see withParameter
      |
      */
    pub fn with_parameters(&self, parameters_to_add: &StringPairArray) -> Url {
        
        todo!();
        /*
            auto u = *this;

        for (int i = 0; i < parametersToAdd.size(); ++i)
            u.addParameter (parametersToAdd.getAllKeys()[i],
                            parametersToAdd.getAllValues()[i]);

        return u;
        */
    }
    
    /**
      | Returns a copy of this Url, with a block
      | of data to send as the POST data.
      | 
      | If the Url already contains some POST
      | data, this will replace it, rather than
      | being appended to it.
      | 
      | If no HTTP command is set when calling
      | createInputStream() to read from this
      | Url and some data has been set, it will
      | do a POST request.
      |
      */
    pub fn with_post_data(&self, new_post_data: &String) -> Url {
        
        todo!();
        /*
            return withPOSTData (MemoryBlock (newPostData.toRawUTF8(), newPostData.getNumBytesAsUTF8()));
        */
    }
    
    /**
      | Returns a copy of this Url, with a block
      | of data to send as the POST data.
      | 
      | If the Url already contains some POST
      | data, this will replace it, rather than
      | being appended to it.
      | 
      | If no HTTP command is set when calling
      | createInputStream() to read from this
      | Url and some data has been set, it will
      | do a POST request.
      |
      */
    pub fn with_post_data_from_memory_block(&self, new_post_data: &MemoryBlock) -> Url {
        
        todo!();
        /*
            auto u = *this;
        u.postData = newPostData;
        return u;
        */
    }
    
    pub fn with_upload(&self, f: *mut UrlUpload) -> Url {
        
        todo!();
        /*
            auto u = *this;

        for (int i = u.filesToUpload.size(); --i >= 0;)
            if (u.filesToUpload.getObjectPointerUnchecked (i)->parameterName == f->parameterName)
                u.filesToUpload.remove (i);

        u.filesToUpload.add (f);
        return u;
        */
    }
    
    /**
      | Returns a copy of this Url, with a file-upload
      | type parameter added to it.
      | 
      | When performing a POST where one of your
      | parameters is a binary file, this lets
      | you specify the file.
      | 
      | Note that the filename is stored, but
      | the file itself won't actually be read
      | until this Url is later used to create
      | a network input stream. If you want to
      | upload data from memory, use withDataToUpload().
      | 
      | @see withDataToUpload
      |
      */
    pub fn with_file_to_upload(&self, 
        parameter_name: &String,
        file_to_upload: &File,
        mime_type:      &String) -> Url {
        
        todo!();
        /*
            return withUpload (new Upload (parameterName, fileToUpload.getFileName(),
                                       mimeType, fileToUpload, nullptr));
        */
    }
    
    /**
      | Returns a copy of this Url, with a file-upload
      | type parameter added to it.
      | 
      | When performing a POST where one of your
      | parameters is a binary file, this lets
      | you specify the file content.
      | 
      | Note that the filename parameter should
      | not be a full path, it's just the last
      | part of the filename.
      | 
      | @see withFileToUpload
      |
      */
    pub fn with_data_to_upload(&self, 
        parameter_name:         &String,
        filename:               &String,
        file_content_to_upload: &MemoryBlock,
        mime_type:              &String) -> Url {
        
        todo!();
        /*
            return withUpload (new Upload (parameterName, filename, mimeType, File(),
                                       new MemoryBlock (fileContentToUpload)));
        */
    }
    
    /**
      | Replaces any escape character sequences
      | in a string with their original character
      | codes.
      | 
      | E.g. any instances of "%20" will be replaced
      | by a space.
      | 
      | This is the opposite of addEscapeChars().
      | 
      | @see addEscapeChars
      |
      */
    pub fn remove_escape_chars(&mut self, s: &String) -> String {
        
        todo!();
        /*
            auto result = s.replaceCharacter ('+', ' ');

        if (! result.containsChar ('%'))
            return result;

        // We need to operate on the string as raw UTF8 chars, and then recombine them into unicode
        // after all the replacements have been made, so that multi-byte chars are handled.
        Vec<char> utf8 (result.toRawUTF8(), (int) result.getNumBytesAsUTF8());

        for (int i = 0; i < utf8.size(); ++i)
        {
            if (utf8.getUnchecked(i) == '%')
            {
                auto hexDigit1 = CharacterFunctions::getHexDigitValue ((aloe_wchar) (uint8) utf8 [i + 1]);
                auto hexDigit2 = CharacterFunctions::getHexDigitValue ((aloe_wchar) (uint8) utf8 [i + 2]);

                if (hexDigit1 >= 0 && hexDigit2 >= 0)
                {
                    utf8.set (i, (char) ((hexDigit1 << 4) + hexDigit2));
                    utf8.removeRange (i + 1, 2);
                }
            }
        }

        return String::fromUTF8 (utf8.getRawDataPointer(), utf8.size());
        */
    }
    
    /**
      | Adds escape sequences to a string to
      | encode any characters that aren't legal
      | in a Url.
      | 
      | E.g. any spaces will be replaced with
      | "%20".
      | 
      | This is the opposite of removeEscapeChars().
      | 
      | -----------
      | @param stringToAddEscapeCharsTo
      | 
      | the string to escape.
      | ----------
      | @param isParameter
      | 
      | if true then the string is going to be
      | used as a parameter, so it also encodes
      | '$' and ',' (which would otherwise be
      | legal in a Url.
      | ----------
      | @param roundBracketsAreLegal
      | 
      | technically round brackets are ok in
      | URLs, however, some servers (like AWS)
      | also want round brackets to be escaped.
      | 
      | @see removeEscapeChars
      |
      */
    pub fn add_escape_chars(
        &mut self, 
        s:                        &String,
        is_parameter:             bool,
        round_brackets_are_legal: Option<bool>

    ) -> String {

        let round_brackets_are_legal: bool = round_brackets_are_legal.unwrap_or(true);
        
        todo!();
        /*
            String legalChars (isParameter ? "_-.~"
                                       : ",$_-.*!'");

        if (roundBracketsAreLegal)
            legalChars += "()";

        Vec<char> utf8 (s.toRawUTF8(), (int) s.getNumBytesAsUTF8());

        for (int i = 0; i < utf8.size(); ++i)
        {
            auto c = utf8.getUnchecked(i);

            if (! (CharacterFunctions::isLetterOrDigit (c)
                     || legalChars.containsChar ((aloe_wchar) c)))
            {
                utf8.set (i, '%');
                utf8.insert (++i, "0123456789ABCDEF" [((uint8) c) >> 4]);
                utf8.insert (++i, "0123456789ABCDEF" [c & 15]);
            }
        }

        return String::fromUTF8 (utf8.getRawDataPointer(), utf8.size());
        */
    }
    
    /**
      | Tries to launch the system's default
      | browser to open the Url.
      | 
      | -----------
      | @return
      | 
      | true if this seems to have worked.
      |
      */
    pub fn launch_in_default_browser(&self) -> bool {
        
        todo!();
        /*
            auto u = toString (true);

        if (u.containsChar ('@') && ! u.containsChar (':'))
            u = "mailto:" + u;

        return Process::openDocument (u, {});
        */
    }
    
    /**
      | This method has been deprecated.
      | 
      | New code should use the method which
      | takes an InputStreamOptions argument
      | instead.
      | 
      | @see InputStreamOptions
      |
      */
    pub fn create_input_stream<R: Read>(
        &self, 
        use_post_command:        bool,
        cb:                      *mut UrlOpenStreamProgressCallback,
        context:                 *mut c_void,
        headers:                 String,
        time_out_ms:             Option<i32>,
        response_headers:        *mut StringPairArray,
        status_code:             *mut i32,
        num_redirects_to_follow: Option<i32>,
        http_request_cmd:        String) -> Box<R> {

        let time_out_ms:             i32 = time_out_ms.unwrap_or(0);
        let num_redirects_to_follow: i32 = num_redirects_to_follow.unwrap_or(5);
        
        todo!();
        /*
            std::function<bool (int, int)> callback;

        if (cb != nullptr)
            callback = [context, cb] (int sent, int total) { return cb (context, sent, total); };

        return createInputStream (InputStreamOptions (toHandling (usePostCommand))
                                    .withProgressCallback (std::move (callback))
                                    .withExtraHeaders (headers)
                                    .withConnectionTimeoutMs (timeOutMs)
                                    .withResponseHeaders (responseHeaders)
                                    .withStatusCode (statusCode)
                                    .withNumRedirectsToFollow(numRedirectsToFollow)
                                    .withHttpRequestCmd (httpRequestCmd));
        */
    }
}
