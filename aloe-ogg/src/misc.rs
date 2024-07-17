/*!
  function: miscellaneous prototypes
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/misc.h]

lazy_static!{
    /*
    extern void *_vorbis_block_alloc(vorbis_block *vb,long bytes);
    extern void _vorbis_block_ripcord(vorbis_block *vb);
    extern int ov_ilog(ogg_uint32_t v);
    */
}

#[cfg(ANALYSIS)]
lazy_static!{
    /*
    extern int analysis_noisy;
    extern void _analysis_output(char *base,int i,float *v,int n,int bark,int dB,
                                 ogg_int64_t off);
    extern void _analysis_output_always(char *base,int i,float *v,int n,int bark,int dB,
                                 ogg_int64_t off);
    */
}

#[cfg(DEBUG_MALLOC)]
pub const VDBG_GRAPHFILE: &'static str = "malloc.m";

#[cfg(DEBUG_MALLOC)]
lazy_static!{
    /*
    extern void *_VDBG_malloc(void *ptr,long bytes,char *file,long line);
    extern void _VDBG_free(void *ptr,char *file,long line);
    */
}

#[cfg(DEBUG_MALLOC)]
#[cfg(not(MISC_C))]
macro_rules! _ogg_malloc {
    ($x:ident) => {
        /*
                _VDBG_malloc(NULL,(x),__FILE__,__LINE__)
        */
    }
}

#[cfg(DEBUG_MALLOC)]
#[cfg(not(MISC_C))]
macro_rules! _ogg_calloc {
    ($x:ident, $y:ident) => {
        /*
                _VDBG_malloc(NULL,(x)*(y),__FILE__,__LINE__)
        */
    }
}

#[cfg(DEBUG_MALLOC)]
#[cfg(not(MISC_C))]
macro_rules! _ogg_realloc {
    ($x:ident, $y:ident) => {
        /*
                _VDBG_malloc((x),(y),__FILE__,__LINE__)
        */
    }
}

#[cfg(DEBUG_MALLOC)]
#[cfg(not(MISC_C))]
macro_rules! _ogg_free {
    ($x:ident) => {
        /*
                _VDBG_free((x),__FILE__,__LINE__)
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/libvorbis-1.3.7/lib/misc.c]

pub const HEAD_ALIGN: usize = 32;

lazy_static!{
    /*
    static pthread_mutex_t memlock=PTHREAD_MUTEX_INITIALIZER;
    static void **pointers=NULL;
    static long *insertlist=NULL; /* We can't embed this in the pointer list;
                              a pointer can have any value... */

    static char **files=NULL;
    static long *file_bytes=NULL;
    static int  filecount=0;

    static int ptop=0;
    static int palloced=0;
    static int pinsert=0;
    */
}

pub struct Head {
    file:  *mut u8,
    line:  i64,
    ptr:   i64,
    bytes: i64,
}

lazy_static!{
    /*
    long global_bytes=0;
    long start_time=-1;
    */
}

pub fn insert(
        ptr:   *mut c_void,
        bytes: i64,
        file:  *mut u8,
        line:  i64)  {
    
    todo!();
    /*
        ((head *)ptr)->file=file;
      ((head *)ptr)->line=line;
      ((head *)ptr)->ptr=pinsert;
      ((head *)ptr)->bytes=bytes-HEAD_ALIGN;

      pthread_mutex_lock(&memlock);
      if(pinsert>=palloced){
        palloced+=64;
        if(pointers){
          pointers=(void **)realloc(pointers,sizeof(void **)*palloced);
          insertlist=(long *)realloc(insertlist,sizeof(long *)*palloced);
        }else{
          pointers=(void **)malloc(sizeof(void **)*palloced);
          insertlist=(long *)malloc(sizeof(long *)*palloced);
        }
      }

      pointers[pinsert]=ptr;

      if(pinsert==ptop)
        pinsert=++ptop;
      else
        pinsert=insertlist[pinsert];

    #ifdef _VDBG_GRAPHFILE
      {
        FILE *out;
        struct timeval tv;
        static struct timezone tz;
        int i;
        char buffer[80];
        gettimeofday(&tv,&tz);

        for(i=0;i<filecount;i++)
          if(!strcmp(file,files[i]))break;

        if(i==filecount){
          filecount++;
          if(!files){
            files=malloc(filecount*sizeof(*files));
            file_bytes=malloc(filecount*sizeof(*file_bytes));
          }else{
            files=realloc(files,filecount*sizeof(*files));
            file_bytes=realloc(file_bytes,filecount*sizeof(*file_bytes));
          }
          files[i]=strdup(file);
          file_bytes[i]=0;
        }

        file_bytes[i]+=bytes-HEAD_ALIGN;

        if(start_time==-1)start_time=(tv.tv_sec*1000)+(tv.tv_usec/1000);

        snprintf(buffer,80,"%s%s",file,_VDBG_GRAPHFILE);
        out=fopen(buffer,"a");
        fprintf(out,"%ld, %ld\n",-start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                file_bytes[i]-(bytes-HEAD_ALIGN));
        fprintf(out,"%ld, %ld # FILE %s LINE %ld\n",
                -start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                file_bytes[i],file,line);
        fclose(out);

        out=fopen(_VDBG_GRAPHFILE,"a");
        fprintf(out,"%ld, %ld\n",-start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                global_bytes);
        fprintf(out,"%ld, %ld\n",-start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                global_bytes+(bytes-HEAD_ALIGN));
        fclose(out);
      }
    #endif

      global_bytes+=(bytes-HEAD_ALIGN);

      pthread_mutex_unlock(&memlock);
      return(ptr+HEAD_ALIGN);
    */
}

pub fn ripremove(ptr: *mut c_void)  {
    
    todo!();
    /*
        int insert;
      pthread_mutex_lock(&memlock);

    #ifdef _VDBG_GRAPHFILE
      {
        FILE *out=fopen(_VDBG_GRAPHFILE,"a");
        struct timeval tv;
        static struct timezone tz;
        char buffer[80];
        char *file =((head *)ptr)->file;
        long bytes =((head *)ptr)->bytes;
        int i;

        gettimeofday(&tv,&tz);
        fprintf(out,"%ld, %ld\n",-start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                global_bytes);
        fprintf(out,"%ld, %ld\n",-start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                global_bytes-((head *)ptr)->bytes);
        fclose(out);

        for(i=0;i<filecount;i++)
          if(!strcmp(file,files[i]))break;

        snprintf(buffer,80,"%s%s",file,_VDBG_GRAPHFILE);
        out=fopen(buffer,"a");
        fprintf(out,"%ld, %ld\n",-start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                file_bytes[i]);
        fprintf(out,"%ld, %ld\n",-start_time+(tv.tv_sec*1000)+(tv.tv_usec/1000),
                file_bytes[i]-bytes);
        fclose(out);

        file_bytes[i]-=bytes;

      }
    #endif

      global_bytes-=((head *)ptr)->bytes;

      insert=((head *)ptr)->ptr;
      insertlist[insert]=pinsert;
      pinsert=insert;

      if(pointers[insert]==NULL){
        fprintf(stderr,"DEBUGGING MALLOC ERROR: freeing previously freed memory\n");
        fprintf(stderr,"\t%s %ld\n",((head *)ptr)->file,((head *)ptr)->line);
      }

      if(global_bytes<0){
        fprintf(stderr,"DEBUGGING MALLOC ERROR: freeing unmalloced memory\n");
      }

      pointers[insert]=NULL;
      pthread_mutex_unlock(&memlock);
    */
}

pub fn vdbg_dump()  {
    
    todo!();
    /*
        int i;
      pthread_mutex_lock(&memlock);
      for(i=0;i<ptop;i++){
        head *ptr=pointers[i];
        if(ptr)
          fprintf(stderr,"unfreed bytes from %s:%ld\n",
                  ptr->file,ptr->line);
      }

      pthread_mutex_unlock(&memlock);
    */
}

pub fn vdbg_malloc(
        ptr:   *mut c_void,
        bytes: i64,
        file:  *mut u8,
        line:  i64)  {
    
    todo!();
    /*
        if(bytes<=0)
        fprintf(stderr,"bad malloc request (%ld bytes) from %s:%ld\n",bytes,file,line);

      bytes+=HEAD_ALIGN;
      if(ptr){
        ptr-=HEAD_ALIGN;
        _ripremove(ptr);
        ptr=realloc(ptr,bytes);
      }else{
        ptr=malloc(bytes);
        memset(ptr,0,bytes);
      }
      return _insert(ptr,bytes,file,line);
    */
}

pub fn vdbg_free(
        ptr:  *mut c_void,
        file: *mut u8,
        line: i64)  {
    
    todo!();
    /*
        if(ptr){
        ptr-=HEAD_ALIGN;
        _ripremove(ptr);
        free(ptr);
      }
    */
}
