crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_Image.h]

pub enum ImagePixelFormat
{
    UnknownFormat,

    /**
      | each pixel is a 3-byte packed RGB colour
      | value. For byte order, see the PixelRGB
      | class.
      |
      */
    RGB,                

    /**
      | each pixel is a 4-byte ARGB premultiplied
      | colour value. For byte order, see the
      | PixelARGB class.
      |
      */
    ARGB,               

    /**
      | each pixel is a 1-byte alpha channel
      | value.
      |
      */
    SingleChannel       
}

/**
  | Retrieves a section of an image as raw
  | pixel data, so it can be read or written
  | to.
  | 
  | You should only use this class as a last
  | resort - messing about with the internals
  | of an image is only recommended for people
  | who really know what they're doing!
  | 
  | A BitmapData object should be used as
  | a temporary, stack-based object. Don't
  | keep one hanging around while the image
  | is being used elsewhere.
  | 
  | Depending on the way the image class
  | is implemented, this may create a temporary
  | buffer which is copied back to the image
  | when the object is deleted, or it may
  | just get a pointer directly into the
  | image's raw data.
  | 
  | You can use the stride and data values
  | in this class directly, but don't alter
  | them!
  | 
  | The actual format of the pixel data depends
  | on the image's format - see typename Image::getFormat(),
  | and the PixelRGB, PixelARGB and PixelAlpha
  | classes for more info.
  |
  */
#[no_copy]
pub struct ImageBitmapData {

    /**
      | The raw pixel data, packed according
      | to the image's pixel format.
      |
      */
    data:          *mut u8,

    /**
      | The format of the data.
      |
      */
    pixel_format:  ImagePixelFormat,

    /**
      | The number of bytes between each line.
      |
      */
    line_stride:   i32,

    /**
      | The number of bytes between each pixel.
      |
      */
    pixel_stride:  i32,

    width:         i32,
    height:        i32,
    data_releaser: Box<ImageBitmapDataReleaser>,
}

pub enum ImageBitmapDataReadWriteMode
{
    readOnly,
    writeOnly,
    readWrite
}

/**
  | Used internally by custom image types
  | to manage pixel data lifetime.
  |
  */
pub struct ImageBitmapDataReleaser { }

impl ImageBitmapData {

    /**
      | Returns a pointer to the start of a line
      | in the image.
      | 
      | The coordinate you provide here isn't
      | checked, so it's the caller's responsibility
      | to make sure it's not out-of-range.
      |
      */
    #[inline] pub fn get_line_pointer(&self, y: i32) -> *mut u8 {
        
        todo!();
        /*
            return data + (size_t) y * (size_t) lineStride;
        */
    }

    /**
      | Returns a pointer to a pixel in the image.
      | 
      | The coordinates you give here are not
      | checked, so it's the caller's responsibility
      | to make sure they're not out-of-range.
      |
      */
    #[inline] pub fn get_pixel_pointer(&self, x: i32, y: i32) -> *mut u8 {
        
        todo!();
        /*
            return data + (size_t) y * (size_t) lineStride + (size_t) x * (size_t) pixelStride;
        */
    }

    /**
      | Returns the size of the bitmap.
      |
      */
    pub fn get_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return Rectangle<int> (width, height);
        */
    }
    
    pub fn new_with_xywh_mode(
        im:   &mut Image,
        x:    i32,
        y:    i32,
        w:    i32,
        h:    i32,
        mode: ImageBitmapDataReadWriteMode) -> Self {
    
        todo!();
        /*
        : width(w),
        : height(h),

            // The BitmapData class must be given a valid image, and a valid rectangle within it!
        jassert (im.image != nullptr);
        jassert (x >= 0 && y >= 0 && w > 0 && h > 0 && x + w <= im.getWidth() && y + h <= im.getHeight());

        im.image->initialiseBitmapData (*this, x, y, mode);
        jassert (data != nullptr && pixelStride > 0 && lineStride != 0);
        */
    }
    
    pub fn new_with_xywh(
        im: &Image,
        x:  i32,
        y:  i32,
        w:  i32,
        h:  i32) -> Self {
    
        todo!();
        /*
        : width(w),
        : height(h),

            // The BitmapData class must be given a valid image, and a valid rectangle within it!
        jassert (im.image != nullptr);
        jassert (x >= 0 && y >= 0 && w > 0 && h > 0 && x + w <= im.getWidth() && y + h <= im.getHeight());

        im.image->initialiseBitmapData (*this, x, y, readOnly);
        jassert (data != nullptr && pixelStride > 0 && lineStride != 0);
        */
    }
    
    pub fn new_with_mode(
        im:   &Image,
        mode: ImageBitmapDataReadWriteMode) -> Self {
    
        todo!();
        /*


            : width (im.getWidth()),
          height (im.getHeight())
        // The BitmapData class must be given a valid image!
        jassert (im.image != nullptr);

        im.image->initialiseBitmapData (*this, 0, 0, mode);
        jassert (data != nullptr && pixelStride > 0 && lineStride != 0);
        */
    }
    
    /**
      | Returns the colour of a given pixel.
      | 
      | For performance reasons, this won't
      | do any bounds-checking on the coordinates,
      | so it's the caller's responsibility
      | to make sure they're within the image's
      | size.
      |
      */
    pub fn get_pixel_colour(&self, x: i32, y: i32) -> Colour {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (x, width) && isPositiveAndBelow (y, height));

        auto pixel = getPixelPointer (x, y);

        switch (pixelFormat)
        {
            case Image::ARGB:           return Colour ( ((const PixelARGB*)  pixel)->getUnpremultiplied());
            case Image::RGB:            return Colour (*((const PixelRGB*)   pixel));
            case Image::SingleChannel:  return Colour (*((const PixelAlpha*) pixel));
            case Image::UnknownFormat:
            default:                    jassertfalse; break;
        }

        return {};
        */
    }
    
    /**
      | Sets the colour of a given pixel.
      | 
      | For performance reasons, this won't
      | do any bounds-checking on the coordinates,
      | so it's the caller's responsibility
      | to make sure they're within the image's
      | size.
      |
      */
    pub fn set_pixel_colour(&self, 
        x:      i32,
        y:      i32,
        colour: Colour)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (x, width) && isPositiveAndBelow (y, height));

        auto pixel = getPixelPointer (x, y);
        auto col = colour.getPixelARGB();

        switch (pixelFormat)
        {
            case Image::ARGB:           ((PixelARGB*)  pixel)->set (col); break;
            case Image::RGB:            ((PixelRGB*)   pixel)->set (col); break;
            case Image::SingleChannel:  ((PixelAlpha*) pixel)->set (col); break;
            case Image::UnknownFormat:
            default:                    jassertfalse; break;
        }
        */
    }
}

/**
  | Holds a fixed-size bitmap.
  | 
  | The image is stored in either 24-bit
  | RGB or 32-bit premultiplied-ARGB format.
  | 
  | To draw into an image, create a Graphics
  | object for it. e.g.
  | 
  | -----------
  | @code
  | 
  | // create a transparent 500x500 image..
  | Image myImage (Image::RGB, 500, 500, true);
  | 
  | Graphics g (myImage);
  | g.setColour (Colours::red);
  | g.fillEllipse (20, 20, 300, 200);  // draws a red ellipse in our image.
  | 
  | Other useful ways to create an image
  | are with the ImageCache class, or the
  | ImageFileFormat, which provides a
  | way to load common image files.
  | 
  | @see Graphics, ImageFileFormat, ImageCache,
  | ImageConvolutionKernel
  | 
  | @tags{Graphics}
  |
  */
#[derive(Default)]
#[leak_detector]
pub struct Image {
    image: ReferenceCountedObjectPtr<ImagePixelData>,
}

impl PartialEq<Image> for Image {
    
    /**
      | Returns true if the two images are referring
      | to the same internal, shared image.
      |
      */
    #[inline] fn eq(&self, other: &Image) -> bool {
        todo!();
        /*
            return image == other.image;
        */
    }
}

impl Eq for Image {}

impl Image {

    /**
      | Returns true if this image isn't null.
      | 
      | If you create an Image with the default
      | constructor, it has no size or content,
      | and is null until you reassign it to an
      | Image which contains some actual data.
      | 
      | The isNull() method is the opposite
      | of isValid(). @see isNull
      |
      */
    #[inline] pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return image != nullptr;
        */
    }

    /**
      | Returns true if this image is not valid.
      | 
      | If you create an Image with the default
      | constructor, it has no size or content,
      | and is null until you reassign it to an
      | Image which contains some actual data.
      | 
      | The isNull() method is the opposite
      | of isValid(). @see isValid
      |
      */
    #[inline] pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return image == nullptr;
        */
    }

    pub fn get_pixel_data(&self) -> *mut ImagePixelData {
        
        todo!();
        /*
            return image.get();
        */
    }
}

/**
  | This is a base class for holding image
  | data in implementation-specific ways.
  | 
  | You may never need to use this class directly
  | - it's used internally by the Image class
  | to store the actual image data. To access
  | pixel data directly, you should use
  | typename ImageBitmapData rather than this
  | class.
  | 
  | ImagePixelData objects are created
  | indirectly, by subclasses of ImageType.
  | @see Image, ImageType
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ImagePixelData {

    base:         ReferenceCountedObject,

    /**
      | The pixel format of the image data.
      |
      */
    pixel_format: ImagePixelFormat,

    width:        i32,
    height:       i32,

    /**
      | User-defined settings that are attached
      | to this image. @see typename Image::getProperties().
      |
      */
    user_data:    NamedValueSet,

    listeners:    ListenerList<Box<dyn ImagePixelDataListener>>,
}

pub type ImagePixelDataPtr = ReferenceCountedObjectPtr<ImagePixelData>;

/**
  | Used to receive callbacks for image
  | data changes
  |
  */
pub trait ImagePixelDataListener
{
    fn image_data_changed(&mut self, _0: *mut ImagePixelData);
    fn image_data_being_deleted(&mut self, _0: *mut ImagePixelData);
}

/**
  | This base class is for handlers that
  | control a type of image manipulation
  | format, e.g. an in-memory bitmap, an
  | OpenGL image, CoreGraphics image,
  | etc.
  | 
  | @see SoftwareImageType, NativeImageType,
  | OpenGLImageType
  | 
  | @tags{Graphics}
  |
  */
pub trait ImageType {

    /**
      | Creates a new image of this type, and
      | the specified parameters.
      |
      */
    fn create(&self, 
            _0:                 ImagePixelFormat,
            width:              i32,
            height:             i32,
            should_clear_image: bool) -> ImagePixelDataPtr;

    /**
      | Must return a unique number to identify
      | this type.
      |
      */
    fn get_typeid(&self) -> i32;

    /**
      | Returns an image which is a copy of the
      | source image, but using this type of
      | storage mechanism.
      | 
      | For example, to make sure that an image
      | is stored in-memory, you could use:
      | 
      | -----------
      | @code
      | 
      | myImage = SoftwareImageType().convert (myImage);
      |
      */
    fn convert(&self, source: &Image) -> Image {
        
        todo!();
        /*
            if (source.isNull() || getTypeID() == source.getPixelData()->createType()->getTypeID())
            return source;

        const typename ImageBitmapData src (source, typename ImageBitmapData::readOnly);

        Image newImage (create (src.pixelFormat, src.width, src.height, false));
        typename ImageBitmapData dest (newImage, typename ImageBitmapData::writeOnly);

        if (src.pixelStride == dest.pixelStride && src.pixelFormat == dest.pixelFormat)
        {
            for (int y = 0; y < dest.height; ++y)
                memcpy (dest.getLinePointer (y), src.getLinePointer (y), (size_t) dest.lineStride);
        }
        else
        {
            for (int y = 0; y < dest.height; ++y)
                for (int x = 0; x < dest.width; ++x)
                    dest.setPixelColour (x, y, src.getPixelColour (x, y));
        }

        return newImage;
        */
    }
}

/**
  | An image storage type which holds the
  | pixels in-memory as a simple block of
  | values. @see ImageType, NativeImageType
  | 
  | @tags{Graphics}
  |
  */
pub struct SoftwareImageType { }

impl ImageType for SoftwareImageType {

    fn create(&self, 
        _0:          ImagePixelFormat,
        width:       i32,
        height:      i32,
        clear_image: bool) -> ImagePixelDataPtr {
        
        todo!();
        /*
        
        */
    }
    
    fn get_typeid(&self) -> i32 {
        
        todo!();
        /*
        
        */
    }
}

/**
  | An image storage type which holds the
  | pixels using whatever is the default
  | storage format on the current platform.
  | @see ImageType, SoftwareImageType
  | 
  | @tags{Graphics}
  |
  */
pub struct NativeImageType { }

impl ImageType for NativeImageType {

    fn create(&self, 
        _0:          ImagePixelFormat,
        width:       i32,
        height:      i32,
        clear_image: bool) -> ImagePixelDataPtr {
        
        todo!();
        /*
        
        */
    }
    
    fn get_typeid(&self) -> i32 {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/images/aloe_Image.cpp]
impl Drop for ImagePixelData {

    fn drop(&mut self) {
        todo!();
        /*
            listeners.call ([this] (Listener& l) { l.imageDataBeingDeleted (this); });
        */
    }
}

impl ImagePixelData {

    pub fn new_from_format_width_and_height(
        format: ImagePixelFormat,
        w:      i32,
        h:      i32) -> Self {
    
        todo!();
        /*
        : pixel_format(format),
        : width(w),
        : height(h),

            jassert (format == typename Image::RGB || format == typename Image::ARGB || format == typename Image::SingleChannel);
        jassert (w > 0 && h > 0); // It's illegal to create a zero-sized image!
        */
    }
    
    pub fn send_data_change_message(&mut self)  {
        
        todo!();
        /*
            listeners.call ([this] (Listener& l) { l.imageDataChanged (this); });
        */
    }
    
    pub fn get_shared_count(&self) -> i32 {
        
        todo!();
        /*
            return getReferenceCount();
        */
    }
}

#[leak_detector]
pub struct SoftwarePixelData {
    base:         ImagePixelData,
    image_data:   HeapBlock<u8>,
    pixel_stride: i32,
    line_stride:  i32,
}

impl SoftwarePixelData {

    pub fn new_from_format_width_height(
        format_to_use: ImagePixelFormat,
        w:             i32,
        h:             i32,
        clear_image:   bool) -> Self {
    
        todo!();
        /*


            : ImagePixelData (formatToUse, w, h),
              pixelStride (formatToUse == typename Image::RGB ? 3 : ((formatToUse == typename Image::ARGB) ? 4 : 1)),
              lineStride ((pixelStride * jmax (1, w) + 3) & ~3)

            imageData.allocate ((size_t) lineStride * (size_t) jmax (1, h), clearImage);
        */
    }
    
    pub fn initialise_bitmap_data(
        &mut self, 
        bitmap: &mut ImageBitmapData,
        x:      i32,
        y:      i32,
        mode:   ImageBitmapDataReadWriteMode)  {
        
        todo!();
        /*
            bitmap.data = imageData + (size_t) x * (size_t) pixelStride + (size_t) y * (size_t) lineStride;
            bitmap.pixelFormat = pixelFormat;
            bitmap.lineStride = lineStride;
            bitmap.pixelStride = pixelStride;

            if (mode != typename ImageBitmapData::readOnly)
                sendDataChangeMessage();
        */
    }
    
    pub fn clone(&mut self) -> ImagePixelDataPtr {
        
        todo!();
        /*
            auto s = new SoftwarePixelData (pixelFormat, width, height, false);
            memcpy (s->imageData, imageData, (size_t) lineStride * (size_t) height);
            return *s;
        */
    }
    
    pub fn create_type(&self) -> Box<dyn ImageType> {
        
        todo!();
        /*
            return std::make_unique<SoftwareImageType>();
        */
    }
}

impl SoftwareImageType {

    pub fn create(&self, 
        format:      ImagePixelFormat,
        width:       i32,
        height:      i32,
        clear_image: bool) -> ImagePixelDataPtr {
        
        todo!();
        /*
            return *new SoftwarePixelData (format, width, height, clearImage);
        */
    }
    
    pub fn get_typeid(&self) -> i32 {
        
        todo!();
        /*
            return 2;
        */
    }
}

impl NativeImageType {

    pub fn get_typeid(&self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }

    #[cfg(any(any(target_os="windows",target_os="linux"),target_os="bsd"))]
    pub fn create(&self, 
        format:      ImagePixelFormat,
        width:       i32,
        height:      i32,
        clear_image: bool) -> ImagePixelDataPtr {
        
        todo!();
        /*
            return new SoftwarePixelData (format, width, height, clearImage);
        */
    }
}

///------------------------
#[no_copy]
#[leak_detector]
pub struct SubsectionPixelData {
    base:         ImagePixelData,
    source_image: ImagePixelDataPtr,
    area:         Rectangle<i32>,
}

impl SubsectionPixelData {

    pub fn new_from_source_and_rectangle(
        source: ImagePixelDataPtr,
        r:      Rectangle<i32>) -> Self {
    
        todo!();
        /*


            : ImagePixelData (source->pixelFormat, r.getWidth(), r.getHeight()),
              sourceImage (std::move (source)), area (r)
        */
    }
    
    pub fn initialise_bitmap_data(&mut self, 
        bitmap: &mut ImageBitmapData,
        x:      i32,
        y:      i32,
        mode:   ImageBitmapDataReadWriteMode)  {
        
        todo!();
        /*
            sourceImage->initialiseBitmapData (bitmap, x + area.getX(), y + area.getY(), mode);

            if (mode != typename ImageBitmapData::readOnly)
                sendDataChangeMessage();
        */
    }
    
    pub fn clone(&mut self) -> ImagePixelDataPtr {
        
        todo!();
        /*
            jassert (getReferenceCount() > 0); // (This method can't be used on an unowned pointer, as it will end up self-deleting)
            auto type = createType();

            Image newImage (type->create (pixelFormat, area.getWidth(), area.getHeight(), pixelFormat != typename Image::RGB));

            {
                Graphics g (newImage);
                g.drawImageAt (Image (*this), 0, 0);
            }

            return *newImage.getPixelData();
        */
    }
    
    pub fn create_type(&self) -> Box<dyn ImageType> {
        
        todo!();
        /*
            return sourceImage->createType();
        */
    }

    /**
      | as we always hold a reference to image,
      | don't double count
      |
      */
    pub fn get_shared_count(&self) -> i32 {
        
        todo!();
        /*
            return getReferenceCount() + sourceImage->getSharedCount() - 1;
        */
    }
}

impl Image {
    
    /**
      | Returns an image which refers to a subsection
      | of this image.
      | 
      | This will not make a copy of the original
      | - the new image will keep a reference
      | to it, so that if the original image is
      | changed, the contents of the subsection
      | will also change. Likewise if you draw
      | into the subimage, you'll also be drawing
      | onto that area of the original image.
      | Note that if you use operator= to make
      | the original Image object refer to something
      | else, the subsection image won't pick
      | up this change, it'll remain pointing
      | at the original.
      | 
      | The area passed-in will be clipped to
      | the bounds of this image, so this may
      | return a smaller image than the area
      | you asked for, or even a null image if
      | the area was out-of-bounds.
      |
      */
    pub fn get_clipped_image(&self, area: &Rectangle<i32>) -> Image {
        
        todo!();
        /*
            if (area.contains (getBounds()))
            return *this;

        auto validArea = area.getIntersection (getBounds());

        if (validArea.isEmpty())
            return {};

        return Image (*new SubsectionPixelData (image, validArea));
        */
    }
    
    pub fn new_from_instance(instance: ReferenceCountedObjectPtr<ImagePixelData>) -> Self {
    
        todo!();
        /*
            : image (std::move (instance))
        */
    }
    
    /**
      | Creates an image with a specified size
      | and format.
      | 
      | The image's internal type will be of
      | the NativeImageType class - to specify
      | a different type, use the other constructor,
      | which takes an ImageType to use.
      | 
      | -----------
      | @param format
      | 
      | the preferred pixel format. Note that
      | this is only a *hint* which is passed
      | to the ImageType class - different ImageTypes
      | may not support all formats, so may substitute
      | e.g. ARGB for RGB.
      | ----------
      | @param imageWidth
      | 
      | the desired width of the image, in pixels
      | - this value must be greater than zero
      | (otherwise a width of 1 will be used)
      | ----------
      | @param imageHeight
      | 
      | the desired width of the image, in pixels
      | - this value must be greater than zero
      | (otherwise a height of 1 will be used)
      | ----------
      | @param clearImage
      | 
      | if true, the image will initially be
      | cleared to black (if it's RGB) or transparent
      | black (if it's ARGB). If false, the image
      | may contain junk initially, so you need
      | to make sure you overwrite it thoroughly.
      |
      */
    pub fn new_from_format_width_height(
        format:      ImagePixelFormat,
        width:       i32,
        height:      i32,
        clear_image: bool) -> Self {
    
        todo!();
        /*
            : image (NativeImageType().create (format, width, height, clearImage))
        */
    }
    
    /**
      | Creates an image with a specified size
      | and format.
      | 
      | -----------
      | @param format
      | 
      | the preferred pixel format. Note that
      | this is only a *hint* which is passed
      | to the ImageType class - different ImageTypes
      | may not support all formats, so may substitute
      | e.g. ARGB for RGB.
      | ----------
      | @param imageWidth
      | 
      | the desired width of the image, in pixels
      | - this value must be greater than zero
      | (otherwise a width of 1 will be used)
      | ----------
      | @param imageHeight
      | 
      | the desired width of the image, in pixels
      | - this value must be greater than zero
      | (otherwise a height of 1 will be used)
      | ----------
      | @param clearImage
      | 
      | if true, the image will initially be
      | cleared to black (if it's RGB) or transparent
      | black (if it's ARGB). If false, the image
      | may contain junk initially, so you need
      | to make sure you overwrite it thoroughly.
      | ----------
      | @param type
      | 
      | the type of image - this lets you specify
      | the internal format that will be used
      | to allocate and manage the image data.
      |
      */
    pub fn new_from_format_width_height_and_type(
        format:      ImagePixelFormat,
        width:       i32,
        height:      i32,
        clear_image: bool,
        ty:          &dyn ImageType) -> Self {
    
        todo!();
        /*
        : image(type.create (format, width, height, clearImage)),
        */
    }
    
    /**
      | Creates a shared reference to another
      | image.
      | 
      | This won't create a duplicate of the
      | image - when Image objects are copied,
      | they simply point to the same shared
      | image data. To make sure that an Image
      | object has its own unique, unshared
      | internal data, call duplicateIfShared().
      |
      */
    pub fn new_from_other_ref(other: &Image) -> Self {
    
        todo!();
        /*
        : image(other.image),
        */
    }
    
    /**
      | Makes this image refer to the same underlying
      | image as another object.
      | 
      | This won't create a duplicate of the
      | image - when Image objects are copied,
      | they simply point to the same shared
      | image data. To make sure that an Image
      | object has its own unique, unshared
      | internal data, call duplicateIfShared().
      |
      */
    pub fn assign_from_ref(&mut self, other: &Image) -> &mut Image {
        
        todo!();
        /*
            image = other.image;
        return *this;
        */
    }
    
    /**
      | Move constructor
      |
      */
    pub fn new_from_other(other: Image) -> Self {
    
        todo!();
        /*
        : image(std::move (other.image)),

        
        */
    }
    
    /**
      | Move assignment operator
      |
      */
    pub fn assign_from(&mut self, other: Image) -> &mut Image {
        
        todo!();
        /*
            image = std::move (other.image);
        return *this;
        */
    }
    
    /**
      | Returns the number of Image objects
      | which are currently referring to the
      | same internal shared image data.
      | 
      | @see duplicateIfShared
      |
      */
    pub fn get_reference_count(&self) -> i32 {
        
        todo!();
        /*
            return image == nullptr ? 0 : image->getSharedCount();
        */
    }
    
    /**
      | Returns the image's width (in pixels).
      |
      */
    pub fn get_width(&self) -> i32 {
        
        todo!();
        /*
            return image == nullptr ? 0 : image->width;
        */
    }
    
    /**
      | Returns the image's height (in pixels).
      |
      */
    pub fn get_height(&self) -> i32 {
        
        todo!();
        /*
            return image == nullptr ? 0 : image->height;
        */
    }
    
    /**
      | Returns a rectangle with the same size
      | as this image.
      | 
      | The rectangle's origin is always (0,
      | 0).
      |
      */
    pub fn get_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return image == nullptr ? Rectangle<int>() : Rectangle<int> (image->width, image->height);
        */
    }
    
    /**
      | Returns the image's pixel format.
      |
      */
    pub fn get_format(&self) -> ImagePixelFormat {
        
        todo!();
        /*
            return image == nullptr ? UnknownFormat : image->pixelFormat;
        */
    }
    
    /**
      | True if the image's format is ARGB.
      |
      */
    pub fn isargb(&self) -> bool {
        
        todo!();
        /*
            return getFormat() == ARGB;
        */
    }
    
    /**
      | True if the image's format is RGB.
      |
      */
    pub fn isrgb(&self) -> bool {
        
        todo!();
        /*
            return getFormat() == RGB;
        */
    }
    
    /**
      | True if the image's format is a single-channel
      | alpha map.
      |
      */
    pub fn is_single_channel(&self) -> bool {
        
        todo!();
        /*
            return getFormat() == SingleChannel;
        */
    }
    
    /**
      | True if the image contains an alpha-channel.
      |
      */
    pub fn has_alpha_channel(&self) -> bool {
        
        todo!();
        /*
            return getFormat() != RGB;
        */
    }
    
    /**
      | Makes sure that no other Image objects
      | share the same underlying data as this
      | one.
      | 
      | If no other Image objects refer to the
      | same shared data as this one, this method
      | has no effect. But if there are other
      | references to the data, this will create
      | a new copy of the data internally.
      | 
      | Call this if you want to draw onto the
      | image, but want to make sure that this
      | doesn't affect any other code that may
      | be sharing the same data.
      | 
      | @see getReferenceCount
      |
      */
    pub fn duplicate_if_shared(&mut self)  {
        
        todo!();
        /*
            if (getReferenceCount() > 1)
            image = image->clone();
        */
    }
    
    /**
      | Creates a copy of this image.
      | 
      | -----------
      | @note
      | 
      | it's usually more efficient to use duplicateIfShared(),
      | because it may not be necessary to copy
      | an image if nothing else is using it.
      | @see getReferenceCount
      |
      */
    pub fn create_copy(&self) -> Image {
        
        todo!();
        /*
            if (image != nullptr)
            return Image (image->clone());

        return {};
        */
    }
    
    /**
      | Returns a version of this image with
      | a different image format.
      | 
      | A new image is returned which has been
      | converted to the specified format.
      | 
      | -----------
      | @note
      | 
      | if the new format is no different to the
      | current one, this will just return a
      | reference to the original image, and
      | won't actually create a copy.
      |
      */
    pub fn converted_to_format(&self, new_format: ImagePixelFormat) -> Image {
        
        todo!();
        /*
            if (image == nullptr || newFormat == image->pixelFormat)
            return *this;

        auto w = image->width, h = image->height;

        auto type = image->createType();
        Image newImage (type->create (newFormat, w, h, false));

        if (newFormat == SingleChannel)
        {
            if (! hasAlphaChannel())
            {
                newImage.clear (getBounds(), Colours::black);
            }
            else
            {
                const BitmapData destData (newImage, 0, 0, w, h, BitmapData::writeOnly);
                const BitmapData srcData (*this, 0, 0, w, h);

                for (int y = 0; y < h; ++y)
                {
                    auto src = reinterpret_cast<const PixelARGB*> (srcData.getLinePointer (y));
                    auto dst = destData.getLinePointer (y);

                    for (int x = 0; x < w; ++x)
                        dst[x] = src[x].getAlpha();
                }
            }
        }
        else if (image->pixelFormat == SingleChannel && newFormat == Image::ARGB)
        {
            const BitmapData destData (newImage, 0, 0, w, h, BitmapData::writeOnly);
            const BitmapData srcData (*this, 0, 0, w, h);

            for (int y = 0; y < h; ++y)
            {
                auto src = reinterpret_cast<const PixelAlpha*> (srcData.getLinePointer (y));
                auto dst = reinterpret_cast<PixelARGB*> (destData.getLinePointer (y));

                for (int x = 0; x < w; ++x)
                    dst[x].set (src[x]);
            }
        }
        else
        {
            if (hasAlphaChannel())
                newImage.clear (getBounds());

            Graphics g (newImage);
            g.drawImageAt (*this, 0, 0);
        }

        return newImage;
        */
    }
    
    /**
      | Returns a NamedValueSet that is attached
      | to the image and which can be used for
      | associating custom values with it.
      | 
      | If this is a null image, this will return
      | a null pointer.
      |
      */
    pub fn get_properties(&self) -> *mut NamedValueSet {
        
        todo!();
        /*
            return image == nullptr ? nullptr : &(image->userData);
        */
    }
    
    /**
      | Clears a section of the image with a given
      | colour.
      | 
      | This won't do any alpha-blending - it
      | just sets all pixels in the image to the
      | given colour (which may be non-opaque
      | if the image has an alpha channel).
      |
      */
    pub fn clear(
        &mut self, 
        area:               &Rectangle<i32>,
        colour_to_clear_to: Option<Colour>

    )  {

        let colour_to_clear_to: Colour = colour_to_clear_to.unwrap_or(colours::transparent_black);
        
        todo!();
        /*
            if (image != nullptr)
        {
            auto g = image->createLowLevelContext();
            g->setFill (colourToClearTo);
            g->fillRect (area, true);
        }
        */
    }
    
    /**
      | Returns the colour of one of the pixels
      | in the image.
      | 
      | If the coordinates given are beyond
      | the image's boundaries, this will return
      | Colours::transparentBlack.
      | 
      | @see setPixelAt, typename ImageBitmapData::getPixelColour
      |
      */
    pub fn get_pixel_at(&self, x: i32, y: i32) -> Colour {
        
        todo!();
        /*
            if (isPositiveAndBelow (x, getWidth()) && isPositiveAndBelow (y, getHeight()))
        {
            const BitmapData srcData (*this, x, y, 1, 1);
            return srcData.getPixelColour (0, 0);
        }

        return {};
        */
    }
    
    /**
      | Sets the colour of one of the image's
      | pixels.
      | 
      | If the coordinates are beyond the image's
      | boundaries, then nothing will happen.
      | 
      | -----------
      | @note
      | 
      | this won't do any alpha-blending, it'll
      | just replace the existing pixel with
      | the given one. The colour's opacity
      | will be ignored if this image doesn't
      | have an alpha-channel.
      | 
      | @see getPixelAt, typename ImageBitmapData::setPixelColour
      |
      */
    pub fn set_pixel_at(&mut self, 
        x:      i32,
        y:      i32,
        colour: Colour)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (x, getWidth()) && isPositiveAndBelow (y, getHeight()))
        {
            const BitmapData destData (*this, x, y, 1, 1, BitmapData::writeOnly);
            destData.setPixelColour (0, 0, colour);
        }
        */
    }
    
    /**
      | Changes the opacity of a pixel.
      | 
      | This only has an effect if the image has
      | an alpha channel and if the given coordinates
      | are inside the image's boundary.
      | 
      | The multiplier must be in the range 0
      | to 1.0, and the current alpha at the given
      | coordinates will be multiplied by this
      | value.
      | 
      | @see setPixelAt
      |
      */
    pub fn multiply_alpha_at(&mut self, 
        x:          i32,
        y:          i32,
        multiplier: f32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (x, getWidth()) && isPositiveAndBelow (y, getHeight())
             && hasAlphaChannel())
        {
            const BitmapData destData (*this, x, y, 1, 1, BitmapData::readWrite);

            if (isARGB())
                reinterpret_cast<PixelARGB*> (destData.data)->multiplyAlpha (multiplier);
            else
                *(destData.data) = (uint8) (*(destData.data) * multiplier);
        }
        */
    }
}

///------------------------
pub struct PixelIterator<PixelType> { 
    phantom: PhantomData<PixelType>,
}

impl<PixelType> PixelIterator<PixelType> {

    pub fn iterate<PixelOperation>(
        data:     &ImageBitmapData,
        pixel_op: &PixelOperation)  {
    
        todo!();
        /*
            for (int y = 0; y < data.height; ++y)
            {
                auto p = data.getLinePointer (y);

                for (int x = 0; x < data.width; ++x)
                {
                    pixelOp (*reinterpret_cast<PixelType*> (p));
                    p += data.pixelStride;
                }
            }
        */
    }
}

///------------------------
pub fn perform_pixel_op<PixelOperation>(
        data:     &ImageBitmapData,
        pixel_op: &PixelOperation)  {

    todo!();
        /*
            switch (data.pixelFormat)
        {
            case Image::ARGB:           PixelIterator<PixelARGB> ::iterate (data, pixelOp); break;
            case Image::RGB:            PixelIterator<PixelRGB>  ::iterate (data, pixelOp); break;
            case Image::SingleChannel:  PixelIterator<PixelAlpha>::iterate (data, pixelOp); break;
            case Image::UnknownFormat:
            default:                    jassertfalse; break;
        }
        */
}

///---------------------
pub struct AlphaMultiplyOp {
    alpha: f32,
}

impl AlphaMultiplyOp {

    pub fn invoke<PixelType>(&self, pixel: &mut PixelType)  {
    
        todo!();
        /*
            pixel.multiplyAlpha (alpha);
        */
    }
}

///------------------------
impl Image {

    /**
      | Changes the overall opacity of the image.
      | 
      | This will multiply the alpha value of
      | each pixel in the image by the given amount
      | (limiting the resulting alpha values
      | between 0 and 255). This allows you to
      | make an image more or less transparent.
      | 
      | If the image doesn't have an alpha channel,
      | this won't have any effect.
      |
      */
    pub fn multiply_all_alphas(&mut self, amount_to_multiply_by: f32)  {
        
        todo!();
        /*
            jassert (hasAlphaChannel());

        const BitmapData destData (*this, 0, 0, getWidth(), getHeight(), BitmapData::readWrite);
        performPixelOp (destData, AlphaMultiplyOp { amountToMultiplyBy });
        */
    }
}

///------------------------
pub struct DesaturateOp { }

impl DesaturateOp {

    pub fn invoke<PixelType>(&self, pixel: &mut PixelType)  {
    
        todo!();
        /*
            pixel.desaturate();
        */
    }
}

///------------------------
impl Image {
    
    /**
      | Changes all the colours to be shades
      | of grey, based on their current luminosity.
      |
      */
    pub fn desaturate(&mut self)  {
        
        todo!();
        /*
            if (isARGB() || isRGB())
        {
            const BitmapData destData (*this, 0, 0, getWidth(), getHeight(), BitmapData::readWrite);
            performPixelOp (destData, DesaturateOp());
        }
        */
    }
    
    /**
      | Creates a RectangleList containing
      | rectangles for all non-transparent
      | pixels of the image.
      | 
      | -----------
      | @param result
      | 
      | the list that will have the area added
      | to it
      | ----------
      | @param alphaThreshold
      | 
      | for a semi-transparent image, any pixels
      | whose alpha is above this level will
      | be considered opaque
      |
      */
    pub fn create_solid_area_mask(&self, 
        result:          &mut RectangleList<i32>,
        alpha_threshold: f32)  {
        
        todo!();
        /*
            if (hasAlphaChannel())
        {
            auto threshold = (uint8) jlimit (0, 255, roundToInt (alphaThreshold * 255.0f));
            SparseSet<int> pixelsOnRow;

            const BitmapData srcData (*this, 0, 0, getWidth(), getHeight());

            for (int y = 0; y < srcData.height; ++y)
            {
                pixelsOnRow.clear();
                auto lineData = srcData.getLinePointer (y);

                if (isARGB())
                {
                    for (int x = 0; x < srcData.width; ++x)
                    {
                        if (reinterpret_cast<const PixelARGB*> (lineData)->getAlpha() >= threshold)
                            pixelsOnRow.addRange (Range<int> (x, x + 1));

                        lineData += srcData.pixelStride;
                    }
                }
                else
                {
                    for (int x = 0; x < srcData.width; ++x)
                    {
                        if (*lineData >= threshold)
                            pixelsOnRow.addRange (Range<int> (x, x + 1));

                        lineData += srcData.pixelStride;
                    }
                }

                for (int i = 0; i < pixelsOnRow.getNumRanges(); ++i)
                {
                    auto range = pixelsOnRow.getRange (i);
                    result.add (Rectangle<int> (range.getStart(), y, range.getLength(), 1));
                }

                result.consolidate();
            }
        }
        else
        {
            result.add (0, 0, getWidth(), getHeight());
        }
        */
    }
    
    /**
      | Copies a section of the image to somewhere
      | else within itself.
      |
      */
    pub fn move_image_section(&mut self, 
        dx: i32,
        dy: i32,
        sx: i32,
        sy: i32,
        w:  i32,
        h:  i32)  {
        
        todo!();
        /*
            if (dx < 0)
        {
            w += dx;
            sx -= dx;
            dx = 0;
        }

        if (dy < 0)
        {
            h += dy;
            sy -= dy;
            dy = 0;
        }

        if (sx < 0)
        {
            w += sx;
            dx -= sx;
            sx = 0;
        }

        if (sy < 0)
        {
            h += sy;
            dy -= sy;
            sy = 0;
        }

        const int minX = jmin (dx, sx);
        const int minY = jmin (dy, sy);

        w = jmin (w, getWidth()  - jmax (sx, dx));
        h = jmin (h, getHeight() - jmax (sy, dy));

        if (w > 0 && h > 0)
        {
            auto maxX = jmax (dx, sx) + w;
            auto maxY = jmax (dy, sy) + h;

            const BitmapData destData (*this, minX, minY, maxX - minX, maxY - minY, BitmapData::readWrite);

            auto dst = destData.getPixelPointer (dx - minX, dy - minY);
            auto src = destData.getPixelPointer (sx - minX, sy - minY);

            auto lineSize = (size_t) destData.pixelStride * (size_t) w;

            if (dy > sy)
            {
                while (--h >= 0)
                {
                    const int offset = h * destData.lineStride;
                    memmove (dst + offset, src + offset, lineSize);
                }
            }
            else if (dst != src)
            {
                while (--h >= 0)
                {
                    memmove (dst, src, lineSize);
                    dst += destData.lineStride;
                    src += destData.lineStride;
                }
            }
        }
        */
    }
}
