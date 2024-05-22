#[doc = "Register `BCDMA_RCHAN_RTHRD_ID` reader"]
pub type R = crate::R<BcdmaRchanRthrdIdSpec>;
#[doc = "Register `BCDMA_RCHAN_RTHRD_ID` writer"]
pub type W = crate::W<BcdmaRchanRthrdIdSpec>;
#[doc = "Field `THREAD_ID` reader - 15:0\\]
Thread ID: This field contains the (up-to) 16-bit value which will be output on the strm_o_thread_id output during all transactions for this channel."]
pub type ThreadIdR = crate::FieldReader<u16>;
#[doc = "Field `THREAD_ID` writer - 15:0\\]
Thread ID: This field contains the (up-to) 16-bit value which will be output on the strm_o_thread_id output during all transactions for this channel."]
pub type ThreadIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Thread ID: This field contains the (up-to) 16-bit value which will be output on the strm_o_thread_id output during all transactions for this channel."]
    #[inline(always)]
    pub fn thread_id(&self) -> ThreadIdR {
        ThreadIdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Thread ID: This field contains the (up-to) 16-bit value which will be output on the strm_o_thread_id output during all transactions for this channel."]
    #[inline(always)]
    #[must_use]
    pub fn thread_id(&mut self) -> ThreadIdW<BcdmaRchanRthrdIdSpec> {
        ThreadIdW::new(self, 0)
    }
}
#[doc = "The thread ID mapping register is used to pair the Rx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchan_rthrd_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchan_rthrd_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRchanRthrdIdSpec;
impl crate::RegisterSpec for BcdmaRchanRthrdIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_rchan_rthrd_id::R`](R) reader structure"]
impl crate::Readable for BcdmaRchanRthrdIdSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_rchan_rthrd_id::W`](W) writer structure"]
impl crate::Writable for BcdmaRchanRthrdIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RCHAN_RTHRD_ID to value 0"]
impl crate::Resettable for BcdmaRchanRthrdIdSpec {
    const RESET_VALUE: u32 = 0;
}
