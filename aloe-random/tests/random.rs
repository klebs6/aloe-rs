#[cfg(ALOE_UNIT_TESTS)]
pub struct RandomTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for RandomTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Random", UnitTestCategories::maths
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl RandomTests {
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Random");

            Random r = getRandom();

            for (int i = 2000; --i >= 0;)
            {
                expect (r.nextDouble() >= 0.0 && r.nextDouble() < 1.0);
                expect (r.nextFloat() >= 0.0f && r.nextFloat() < 1.0f);
                expect (r.nextInt (5) >= 0 && r.nextInt (5) < 5);
                expect (r.nextInt (1) == 0);

                int n = r.nextInt (50) + 1;
                expect (r.nextInt (n) >= 0 && r.nextInt (n) < n);

                n = r.nextInt (0x7ffffffe) + 1;
                expect (r.nextInt (n) >= 0 && r.nextInt (n) < n);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static RandomTests randomTests;
    */
}


