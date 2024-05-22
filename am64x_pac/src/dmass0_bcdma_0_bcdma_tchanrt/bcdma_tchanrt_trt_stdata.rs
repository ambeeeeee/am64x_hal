#[doc = "Register `BCDMA_TCHANRT_TRT_STDATA` reader"]
pub type R = crate::R<BcdmaTchanrtTrtStdataSpec>;
#[doc = "Register `BCDMA_TCHANRT_TRT_STDATA` writer"]
pub type W = crate::W<BcdmaTchanrtTrtStdataSpec>;
#[doc = "Field `STATE_INFO` reader - 31:0\\]
See Tx state mapping table"]
pub type StateInfoR = crate::FieldReader<u32>;
#[doc = "Field `STATE_INFO` writer - 31:0\\]
See Tx state mapping table"]
pub type StateInfoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
See Tx state mapping table"]
    #[inline(always)]
    pub fn state_info(&self) -> StateInfoR {
        StateInfoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
See Tx state mapping table"]
    #[inline(always)]
    #[must_use]
    pub fn state_info(&mut self) -> StateInfoW<BcdmaTchanrtTrtStdataSpec> {
        StateInfoW::new(self, 0)
    }
}
#[doc = "The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_stdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_stdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaTchanrtTrtStdataSpec;
impl crate::RegisterSpec for BcdmaTchanrtTrtStdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_tchanrt_trt_stdata::R`](R) reader structure"]
impl crate::Readable for BcdmaTchanrtTrtStdataSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_tchanrt_trt_stdata::W`](W) writer structure"]
impl crate::Writable for BcdmaTchanrtTrtStdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_TCHANRT_TRT_STDATA to value 0"]
impl crate::Resettable for BcdmaTchanrtTrtStdataSpec {
    const RESET_VALUE: u32 = 0;
}
