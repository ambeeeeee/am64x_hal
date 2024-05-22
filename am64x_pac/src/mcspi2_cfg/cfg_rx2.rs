#[doc = "Register `CFG_RX2` reader"]
pub type R = crate::R<CfgRx2Spec>;
#[doc = "Register `CFG_RX2` writer"]
pub type W = crate::W<CfgRx2Spec>;
#[doc = "Field `RDATA` reader - 31:0\\]
Channel 2 Received Data"]
pub type RdataR = crate::FieldReader<u32>;
#[doc = "Field `RDATA` writer - 31:0\\]
Channel 2 Received Data"]
pub type RdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 Received Data"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 Received Data"]
    #[inline(always)]
    #[must_use]
    pub fn rdata(&mut self) -> RdataW<CfgRx2Spec> {
        RdataW::new(self, 0)
    }
}
#[doc = "This register contains a single SPI word received through the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRx2Spec;
impl crate::RegisterSpec for CfgRx2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_rx2::R`](R) reader structure"]
impl crate::Readable for CfgRx2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx2::W`](W) writer structure"]
impl crate::Writable for CfgRx2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_RX2 to value 0"]
impl crate::Resettable for CfgRx2Spec {
    const RESET_VALUE: u32 = 0;
}
