crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2ChainAndPolygonContact.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2ChainAndPolygonContact.cpp]

pub struct b2ChainAndPolygonContact {
    base: b2Contact,
}

impl b2ChainAndPolygonContact {

    pub fn create(&mut self, 
        fixturea:  *mut b2Fixture,
        indexa:    i32,
        fixtureb:  *mut b2Fixture,
        indexb:    i32,
        allocator: *mut b2BlockAllocator) -> *mut b2Contact {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2ChainAndPolygonContact));
        return new (mem) b2ChainAndPolygonContact(fixtureA, indexA, fixtureB, indexB);
        */
    }
    
    pub fn destroy(&mut self, 
        contact:   *mut b2Contact,
        allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            ((b2ChainAndPolygonContact*)contact)->~b2ChainAndPolygonContact();
        allocator->Free(contact, sizeof(b2ChainAndPolygonContact));
        */
    }
    
    pub fn new(
        fixturea: *mut b2Fixture,
        indexa:   i32,
        fixtureb: *mut b2Fixture,
        indexb:   i32) -> Self {
    
        todo!();
        /*


            : b2Contact(fixtureA, indexA, fixtureB, indexB)

        b2Assert(m_fixtureA->GetType() == b2Shape::e_chain);
        b2Assert(m_fixtureB->GetType() == b2Shape::e_polygon);
        */
    }
    
    pub fn evaluate(&mut self, 
        manifold: *mut b2Manifold,
        xfa:      &b2Transform,
        xfb:      &b2Transform)  {
        
        todo!();
        /*
            b2ChainShape* chain = (b2ChainShape*)m_fixtureA->GetShape();
        b2EdgeShape edge;
        chain->GetChildEdge(&edge, m_indexA);
        b2CollideEdgeAndPolygon(    manifold, &edge, xfA,
                                    (b2PolygonShape*)m_fixtureB->GetShape(), xfB);
        */
    }
}
