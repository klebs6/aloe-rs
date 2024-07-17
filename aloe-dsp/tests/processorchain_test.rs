crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/processors/aloe_ProcessorChain_test.cpp]

pub struct ProcessorChainTest {
    base: UnitTest,
}

impl Default for ProcessorChainTest {
    
    fn default() -> Self {
        todo!();
        /*
            : UnitTest ("ProcessorChain", UnitTestCategories::dsp
        */
    }
}

impl ProcessorChainTest {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("After calling setBypass, processor is bypassed");
            {
                ProcessorChain<MockProcessor<1>, MockProcessor<2>> chain;

                setBypassed<0> (chain, true);
                expect (isBypassed<0> (chain));
                setBypassed<0> (chain, false);
                expect (! isBypassed<0> (chain));

                setBypassed<1> (chain, true);
                expect (isBypassed<1> (chain));
                setBypassed<1> (chain, false);
                expect (! isBypassed<1> (chain));
            }

            beginTest ("After calling prepare, all processors are prepared");
            {
                ProcessorChain<MockProcessor<1>, MockProcessor<2>> chain;

                expect (! get<0> (chain).isPrepared);
                expect (! get<1> (chain).isPrepared);

                chain.prepare (ProcessSpec{});

                expect (get<0> (chain).isPrepared);
                expect (get<1> (chain).isPrepared);
            }

            beginTest ("After calling reset, all processors are reset");
            {
                ProcessorChain<MockProcessor<1>, MockProcessor<2>> chain;

                expect (! get<0> (chain).isReset);
                expect (! get<1> (chain).isReset);

                chain.reset();

                expect (get<0> (chain).isReset);
                expect (get<1> (chain).isReset);
            }

            beginTest ("After calling process, all processors contribute to processing");
            {
                ProcessorChain<MockProcessor<1>, MockProcessor<2>> chain;

                AudioBuffer<float> buffer (1, 1);
                AudioBlock<float> block (buffer);
                ProcessContextReplacing<float> context (block);

                block.clear();
                chain.process (context);
                expectEquals (buffer.getSample (0, 0), 3.0f);
                expect (get<0> (chain).bufferWasClear);
                expect (! get<1> (chain).bufferWasClear);

                setBypassed<0> (chain, true);
                block.clear();
                chain.process (context);
                expectEquals (buffer.getSample (0, 0), 2.0f);
                expect (get<0> (chain).bufferWasClear);
                expect (get<1> (chain).bufferWasClear);

                setBypassed<1> (chain, true);
                block.clear();
                chain.process (context);
                expectEquals (buffer.getSample (0, 0), 0.0f);
                expect (get<0> (chain).bufferWasClear);
                expect (get<1> (chain).bufferWasClear);

                setBypassed<0> (chain, false);
                block.clear();
                chain.process (context);
                expectEquals (buffer.getSample (0, 0), 1.0f);
                expect (get<0> (chain).bufferWasClear);
                expect (! get<1> (chain).bufferWasClear);
            }
        */
    }
}

lazy_static!{
    /*
    static ProcessorChainTest processorChainUnitTest;
    */
}

