crate::ix!();

/**
  | OpenGL versions, used by setOpenGLVersionRequired().
  |
  */
pub enum OpenGLContextOpenGLVersion
{
    defaultGLVersion = 0,
    openGL3_2
}

pub fn find_null_terminator<Char,R>(ptr: *const Char) -> *mut R {

    todo!();
        /*
            while (*ptr != 0)
            ++ptr;

        return ptr;
        */
}

pub fn get_open_gl_version() -> OpenGLVersion {
    
    todo!();
        /*
            const auto* versionBegin = glGetString (GL_VERSION);

        if (versionBegin == nullptr)
            return {};

        const auto* versionEnd = findNullTerminator (versionBegin);
        const std::string versionString (versionBegin, versionEnd);
        const auto spaceSeparated = StringArray::fromTokens (versionString.c_str(), false);

        if (spaceSeparated.isEmpty())
            return {};

        const auto pointSeparated = StringArray::fromTokens (spaceSeparated[0], ".", "");

        const auto major = pointSeparated[0].getIntValue();
        const auto minor = pointSeparated[1].getIntValue();

        return { major, minor };
        */
}

#[derive(Default)]
pub struct OpenGLVersion {
    major: i32, // default = 0
    minor: i32, // default = 0
}

impl PartialEq<OpenGLVersion> for OpenGLVersion {
    
    #[inline] fn eq(&self, other: &OpenGLVersion) -> bool {
        todo!();
        /*
            return toTuple() == other.toTuple();
        */
    }
}

impl Eq for OpenGLVersion {}

impl Ord for OpenGLVersion {
    
    #[inline] fn cmp(&self, other: &OpenGLVersion) -> std::cmp::Ordering {
        todo!();
        /*
            return toTuple() < other.toTuple();
        */
    }
}

impl PartialOrd<OpenGLVersion> for OpenGLVersion {
    #[inline] fn partial_cmp(&self, other: &OpenGLVersion) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl OpenGLVersion {

    pub fn new_major(major_in: i32) -> Self {
    
        todo!();
        /*
        : version(majorIn, 0),
        */
    }
    
    pub fn new(
        major_in: i32,
        minor_in: i32) -> Self {
    
        todo!();
        /*
        : major(majorIn),
        : minor(minorIn),
        */
    }
    
    pub fn to_tuple(&self) -> (i32,i32) {
        
        todo!();
        /*
            return std::make_tuple (major, minor);
        */
    }
}
