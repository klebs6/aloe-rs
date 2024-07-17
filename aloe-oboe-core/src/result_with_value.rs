crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/ResultWithValue.h]

/**
  | A ResultWithValue can store both the
  | result of an operation (either OK or
  | an error) and a value.
  | 
  | It has been designed for cases where
  | the caller needs to know whether an operation
  | succeeded and, if it did, a value which
  | was obtained during the operation.
  | 
  | For example, when reading from a stream
  | the caller needs to know the result of
  | the read operation and, if it was successful,
  | how many frames were read. Note that
  | ResultWithValue can be evaluated as
  | a boolean so it's simple to check whether
  | the result is OK.
  |
  | <code>
  | ResultWithValue<int32_t> resultOfRead = myStream.read(&buffer, numFrames, timeoutNanoseconds);
  |
  | if (resultOfRead) {
  |     LOGD("Frames read: %d", resultOfRead.value());
  | } else {
  |     LOGD("Error reading from stream: %s", resultOfRead.error());
  | }
  | </code>
  */
pub struct OboeResultWithValue<T> {
    value: T,
    error: OboeResult,
}

impl<T> Into<bool> for OboeResultWithValue<T> {
    
    /**
      | @return
      | 
      | true if OK
      |
      */
    #[inline] fn into(self) -> bool {
        todo!();
        /*
            return mError == OboeResult::OK;
        */
    }
}

impl<T> Not for OboeResultWithValue<T> {
    type Output = bool;

    /**
     | Quick way to check for an error.
     |
     | The caller could write something like this:
     |
     | <code>
     |     if (!result) { printf("Got error %s\n", convertToText(result.error())); }
     | </code>
     |
     | @return true if an error occurred
     */
    #[inline] fn not(self) -> Self::Output {
        todo!();
        /*
            return mError != OboeResult::OK;
        */
    }
}

impl<T> Into<OboeResult> for OboeResultWithValue<T> {

    /**
     | Implicitly convert to a OboeResult. This
     | enables easy comparison with OboeResult
     | values. Example:
     |
     | <code>
     |     ResultWithValue result = openStream();
     |     if (result == OboeResult::ErrorNoMemory) { 
     |     // tell user they're out of memory 
     |     }
     | </code>
     */
    #[inline] fn into(self) -> OboeResult {
        todo!();
        /*
            return mError;
        */
    }
}

impl<T> OboeResultWithValue<T> {

    /**
      | Construct a ResultWithValue containing
      | an error result.
      | 
      | -----------
      | @param error
      | 
      | The error
      |
      */
    pub fn new_with_error(error: OboeResult) -> Self {
    
        todo!();
        /*


            : mValue{}
                , mError(error)
        */
    }

    /**
      | Construct a ResultWithValue containing
      | an OK result and a value.
      | 
      | -----------
      | @param value
      | 
      | the value to store
      |
      */
    pub fn new_ok_with_value(value: T) -> Self {
    
        todo!();
        /*
        : value(value),
        : error(OboeResult::OK),

        
        */
    }

    /**
      | Get the result.
      | 
      | -----------
      | @return
      | 
      | the result
      |
      */
    pub fn error(&self) -> OboeResult {
        
        todo!();
        /*
            return mError;
        */
    }

    /**
      | Get the value @return
      |
      */
    pub fn value(&self) -> T {
        
        todo!();
        /*
            return mValue;
        */
    }

    /**
      | Create a ResultWithValue from a number.
      | 
      | If the number is positive the ResultWithValue
      | will have a result of OboeResult::OK and
      | the value will contain the number. If
      | the number is negative the result will
      | be obtained from the negative number
      | (numeric error codes can be found in
      | AAudio.h) and the value will be null.
      |
      */
    pub fn create_based_on_sign(numeric_result: T) -> OboeResultWithValue<T> {
        
        todo!();
        /*
            // Ensure that the type is either an integer or float
            static_assert(std::is_arithmetic<T>::value,
                          "createBasedOnSign can only be called for numeric types (int or float)");

            if (numericResult >= 0){
                return ResultWithValue<T>(numericResult);
            } else {
                return ResultWithValue<T>(static_cast<OboeResult>(numericResult));
            }
        */
    }
}

/**
  | If the result is `OK` then return the
  | value, otherwise return a human-readable
  | error message.
  |
  */
impl<T> fmt::Display for OboeResultWithValue<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        /*
            if (!result) {
            strm << convertToText(result.error());
        } else {
            strm << result.value();
        }
       return strm;
        */
    }
}
