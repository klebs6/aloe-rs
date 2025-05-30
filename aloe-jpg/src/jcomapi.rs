/*!
  | jcomapi.c
  | 
  | This file contains application interface
  | routines that are used for both compression
  | and decompression.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/image_formats/jpglib/jcomapi.c]

/**
  | Abort processing of a JPEG compression
  | or decompression operation, but don't
  | destroy the object itself.
  | 
  | For this, we merely clean up all the nonpermanent
  | memory pools.
  | 
  | -----------
  | @note
  | 
  | temp files (virtual arrays) are not
  | allowed to belong to the permanent pool,
  | so we will be able to close all temp files
  | here.
  | 
  | Closing a data source or destination,
  | if necessary, is the application's
  | responsibility.
  |
  */
pub fn jpeg_abort(cinfo: JCommonPtr)  {
    
    todo!();
        /*
            int pool;

      /* Do nothing if called on a not-initialized or destroyed JPEG object. */
      if (cinfo->mem == NULL)
        return;

      /* Releasing pools in reverse order might help avoid fragmentation
       * with some (brain-damaged) malloc libraries.
       */
      for (pool = JPOOL_NUMPOOLS-1; pool > JPOOL_PERMANENT; pool--) {
        (*cinfo->mem->free_pool) (cinfo, pool);
      }

      /* Reset overall state for possible reuse of object */
      if (cinfo->is_decompressor) {
        cinfo->global_state = DSTATE_START;
        /* Try to keep application from accessing now-deleted marker list.
         * A bit kludgy to do it here, but this is the most central place.
         */
        ((JDecompressPtr) cinfo)->marker_list = NULL;
      } else {
        cinfo->global_state = CSTATE_START;
      }
        */
}

/**
  | Destruction of a JPEG object.
  | 
  | Everything gets deallocated except
  | the master jpeg_compress_struct itself
  | and the error manager struct. Both of
  | these are supplied by the application
  | and must be freed, if necessary, by the
  | application. (Often they are on the
  | stack and so don't need to be freed anyway.)
  | 
  | Closing a data source or destination,
  | if necessary, is the application's
  | responsibility.
  |
  */
pub fn jpeg_destroy(cinfo: JCommonPtr)  {
    
    todo!();
        /*
            /* We need only tell the memory manager to release everything. */
      /* NB: mem pointer is NULL if memory mgr failed to initialize. */
      if (cinfo->mem != NULL)
        (*cinfo->mem->self_destruct) (cinfo);
      cinfo->mem = NULL;        /* be safe if jpeg_destroy is called twice */
      cinfo->global_state = 0;  /* mark it destroyed */
        */
}

/**
  | Convenience routines for allocating
  | quantization and Huffman tables. (Would
  | jutils.c be a more reasonable place
  | to put these?)
  |
  */
pub fn jpeg_alloc_quant_table(cinfo: JCommonPtr) -> *mut JQuantTbl {
    
    todo!();
        /*
            JQUANT_TBL *tbl;

      tbl = (JQUANT_TBL *)
        (*cinfo->mem->alloc_small) (cinfo, JPOOL_PERMANENT, SIZEOF(JQUANT_TBL));
      tbl->sent_table = FALSE;  /* make sure this is false in any new table */
      return tbl;
        */
}

pub fn jpeg_alloc_huff_table(cinfo: JCommonPtr) -> *mut JHuffTbl {
    
    todo!();
        /*
            JHUFF_TBL *tbl;

      tbl = (JHUFF_TBL *)
        (*cinfo->mem->alloc_small) (cinfo, JPOOL_PERMANENT, SIZEOF(JHUFF_TBL));
      tbl->sent_table = FALSE;  /* make sure this is false in any new table */
      return tbl;
        */
}
