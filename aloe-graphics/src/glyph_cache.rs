crate::ix!();

/**
  | Holds a cache of recently-used glyph
  | objects of some type.
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct GlyphCache<CachedGlyphType,RenderTargetType> {
    base:           DeletedAtShutdown,
    glyphs:         ReferenceCountedArray<CachedGlyphType>,
    access_counter: Atomic<i32>,
    hits:           Atomic<i32>,
    misses:         Atomic<i32>,
    lock:           CriticalSection,
    phantom:        PhantomData<RenderTargetType>,
}

impl<CachedGlyphType,RenderTargetType> Default for GlyphCache<CachedGlyphType,RenderTargetType> {
    
    fn default() -> Self {
        todo!();
        /*


            reset();
        */
    }
}

impl<CachedGlyphType,RenderTargetType> Drop for GlyphCache<CachedGlyphType,RenderTargetType> {

    fn drop(&mut self) {
        todo!();
        /*
            getSingletonPointer() = nullptr;
        */
    }
}

impl<CachedGlyphType,RenderTargetType> GlyphCache<CachedGlyphType,RenderTargetType> {
    
    pub fn get_instance<'a>() -> &'a mut GlyphCache<CachedGlyphType,RenderTargetType> {
        
        todo!();
        /*
            auto& g = getSingletonPointer();

            if (g == nullptr)
                g = new GlyphCache();

            return *g;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
            glyphs.clear();
            addNewGlyphSlots (120);
            hits = 0;
            misses = 0;
        */
    }
    
    pub fn draw_glyph(&mut self, 
        target:       &mut RenderTargetType,
        font:         &Font,
        glyph_number: i32,
        pos:          Point<f32>)  {
        
        todo!();
        /*
            if (auto glyph = findOrCreateGlyph (font, glyphNumber))
            {
                glyph->lastAccessCount = ++accessCounter;
                glyph->draw (target, pos);
            }
        */
    }
    
    pub fn find_or_create_glyph(&mut self, 
        font:         &Font,
        glyph_number: i32) -> ReferenceCountedObjectPtr<CachedGlyphType> {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            if (auto g = findExistingGlyph (font, glyphNumber))
            {
                ++hits;
                return g;
            }

            ++misses;
            auto g = getGlyphForReuse();
            jassert (g != nullptr);
            g->generate (font, glyphNumber);
            return g;
        */
    }
    
    pub fn find_existing_glyph(&self, 
        font:         &Font,
        glyph_number: i32) -> ReferenceCountedObjectPtr<CachedGlyphType> {
        
        todo!();
        /*
            for (auto g : glyphs)
                if (g->glyph == glyphNumber && g->font == font)
                    return *g;

            return {};
        */
    }
    
    pub fn get_glyph_for_reuse(&mut self) -> ReferenceCountedObjectPtr<CachedGlyphType> {
        
        todo!();
        /*
            if (hits.get() + misses.get() > glyphs.size() * 16)
            {
                if (misses.get() * 2 > hits.get())
                    addNewGlyphSlots (32);

                hits = 0;
                misses = 0;
            }

            if (auto g = findLeastRecentlyUsedGlyph())
                return *g;

            addNewGlyphSlots (32);
            return glyphs.getLast();
        */
    }
    
    pub fn add_new_glyph_slots(&mut self, num: i32)  {
        
        todo!();
        /*
            glyphs.ensureStorageAllocated (glyphs.size() + num);

            while (--num >= 0)
                glyphs.add (new CachedGlyphType());
        */
    }
    
    pub fn find_least_recently_used_glyph(&self) -> *mut CachedGlyphType {
        
        todo!();
        /*
            CachedGlyphType* oldest = nullptr;
            auto oldestCounter = std::numeric_limits<int>::max();

            for (auto* g : glyphs)
            {
                if (g->lastAccessCount <= oldestCounter
                     && g->getReferenceCount() == 1)
                {
                    oldestCounter = g->lastAccessCount;
                    oldest = g;
                }
            }

            return oldest;
        */
    }
    
    pub fn get_singleton_pointer() 
        -> &'static mut *mut GlyphCache<CachedGlyphType,RenderTargetType> 
    {
        todo!();
        /*
            static GlyphCache* g = nullptr;
            return g;
        */
    }
}
