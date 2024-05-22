#[doc = "Register `PKTDMA_GCFG_CAP0` reader"]
pub type R = crate::R<PktdmaGcfgCap0Spec>;
#[doc = "Register `PKTDMA_GCFG_CAP0` writer"]
pub type W = crate::W<PktdmaGcfgCap0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The Capabilities Register 0 specifies which standard features this PKTDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_cap0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_cap0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaGcfgCap0Spec;
impl crate::RegisterSpec for PktdmaGcfgCap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_gcfg_cap0::R`](R) reader structure"]
impl crate::Readable for PktdmaGcfgCap0Spec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_gcfg_cap0::W`](W) writer structure"]
impl crate::Writable for PktdmaGcfgCap0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_GCFG_CAP0 to value 0"]
impl crate::Resettable for PktdmaGcfgCap0Spec {
    const RESET_VALUE: u32 = 0;
}
