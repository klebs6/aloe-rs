crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct AtomicTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod atomic_tests {
    use super::*;

    #[derive(Default)]
    pub struct AtomicTester<Type> {

    }

    impl AtomicTester<Type> {

        pub fn test_integer(test: &mut UnitTest)  {
            
            todo!();
            /*
                Atomic<Type> a, b;
                    Type c;

                    a.set ((Type) 10);
                    c = (Type) 10;

                    test.expect (a.value == c);
                    test.expect (a.get() == c);

                    a += 15;
                    c += 15;
                    test.expect (a.get() == c);
                    a.memoryBarrier();

                    a -= 5;
                    c -= 5;
                    test.expect (a.get() == c);

                    test.expect (++a == ++c);
                    ++a;
                    ++c;
                    test.expect (--a == --c);
                    test.expect (a.get() == c);
                    a.memoryBarrier();

                    testFloat (test);
            */
        }
        
        pub fn test_float(test: &mut UnitTest)  {
            
            todo!();
            /*
                Atomic<Type> a, b;
                    a = (Type) 101;
                    a.memoryBarrier();

                    /*  These are some simple test cases to check the atomics - let me know
                        if any of these assertions fail on your system!
                    */
                    test.expect (a.get() == (Type) 101);
                    test.expect (! a.compareAndSetBool ((Type) 300, (Type) 200));
                    test.expect (a.get() == (Type) 101);
                    test.expect (a.compareAndSetBool ((Type) 200, a.get()));
                    test.expect (a.get() == (Type) 200);

                    test.expect (a.exchange ((Type) 300) == (Type) 200);
                    test.expect (a.get() == (Type) 300);

                    b = a;
                    test.expect (b.get() == a.get());
            */
        }
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for AtomicTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Atomics", UnitTestCategories::threads
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl AtomicTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Misc");

            char a1[7];
            expect (numElementsInArray(a1) == 7);
            int a2[3];
            expect (numElementsInArray(a2) == 3);

            expect (ByteOrder::swap ((uint16) 0x1122) == 0x2211);
            expect (ByteOrder::swap ((uint32) 0x11223344) == 0x44332211);
            expect (ByteOrder::swap ((uint64) 0x1122334455667788ULL) == (uint64) 0x8877665544332211LL);

            beginTest ("Atomic int");
            AtomicTester <int>::testInteger (*this);
            beginTest ("Atomic unsigned int");
            AtomicTester <unsigned int>::testInteger (*this);
            beginTest ("Atomic int32");
            AtomicTester <int32>::testInteger (*this);
            beginTest ("Atomic uint32");
            AtomicTester <uint32>::testInteger (*this);
            beginTest ("Atomic long");
            AtomicTester <long>::testInteger (*this);
            beginTest ("Atomic int*");
            AtomicTester <int*>::testInteger (*this);
            beginTest ("Atomic float");
            AtomicTester <float>::testFloat (*this);
          #if ! ALOE_64BIT_ATOMICS_UNAVAILABLE  // 64-bit intrinsics aren't available on some old platforms
            beginTest ("Atomic int64");
            AtomicTester <int64>::testInteger (*this);
            beginTest ("Atomic uint64");
            AtomicTester <uint64>::testInteger (*this);
            beginTest ("Atomic double");
            AtomicTester <double>::testFloat (*this);
          #endif
            beginTest ("Atomic pointer increment/decrement");
            Atomic<int*> a (a2); int* b (a2);
            expect (++a == ++b);

            {
                beginTest ("Atomic void*");
                Atomic<void*> atomic;
                void* c;

                atomic.set ((void*) 10);
                c = (void*) 10;

                expect (atomic.value == c);
                expect (atomic.get() == c);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static AtomicTests atomicUnitTests;
    */
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct ThreadLocalValueUnitTest {
    base:                UnitTest,
    base2:               Thread,
    main_thread_result:  Atomic<i32>,
    aux_thread_result:   Atomic<i32>,
    shared_thread_local: Atomic<*mut ThreadLocalValue<i32>>,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for ThreadLocalValueUnitTest {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("ThreadLocalValue", UnitTestCategories::threads),
              Thread ("ThreadLocalValue Thread"
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl ThreadLocalValueUnitTest {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("values are thread local");

            {
                ThreadLocalValue<int> threadLocal;

                sharedThreadLocal = &threadLocal;

                sharedThreadLocal.get()->get() = 1;

                startThread();
                signalThreadShouldExit();
                waitForThreadToExit (-1);

                mainThreadResult = sharedThreadLocal.get()->get();

                expectEquals (mainThreadResult.get(), 1);
                expectEquals (auxThreadResult.get(), 2);
            }

            beginTest ("values are per-instance");

            {
                ThreadLocalValue<int> a, b;

                a.get() = 1;
                b.get() = 2;

                expectEquals (a.get(), 1);
                expectEquals (b.get(), 2);
            }
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            sharedThreadLocal.get()->get() = 2;
            auxThreadResult = sharedThreadLocal.get()->get();
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    ThreadLocalValueUnitTest threadLocalValueUnitTest;
    */
}


