//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jconfig.h]



#define HAVE_PROTOTYPES
#define HAVE_UNSIGNED_CHAR
#define HAVE_UNSIGNED_SHORT
/* #define c_void char */
/* #define const */
#define HAVE_STDDEF_H

#ifndef HAVE_STDLIB_H
 #define HAVE_STDLIB_H
#endif

/* Define "boolean" as unsigned char, not int, per Windows custom */
#ifndef __RPCNDR_H__        /* don't conflict if rpcndr.h already read */
typedef unsigned char boolean;
#endif

#define HAVE_BOOLEAN        /* prevent jmorecfg.h from redefining it */


#ifdef JPEG_CJPEG_DJPEG
#define BMP_SUPPORTED       /* BMP image file format */
#define GIF_SUPPORTED       /* GIF image file format */
#define PPM_SUPPORTED       /* PBMPLUS PPM/PGM image file format */
#define TARGA_SUPPORTED     /* Targa image file format */

#define TWO_FILE_COMMANDLINE    /* optional */
#define USE_SETMODE     /* Microsoft has setmode() */
#endif /* JPEG_CJPEG_DJPEG */
