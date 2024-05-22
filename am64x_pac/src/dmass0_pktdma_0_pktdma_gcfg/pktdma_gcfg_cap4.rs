#[doc = "Register `PKTDMA_GCFG_CAP4` reader"]
pub type R = crate::R<PktdmaGcfgCap4Spec>;
#[doc = "Register `PKTDMA_GCFG_CAP4` writer"]
pub type W = crate::W<PktdmaGcfgCap4Spec>;
#[doc = "Field `TFLOW_CNT` reader - 13:0\\]
Tx flow table entry count"]
pub type TflowCntR = crate::FieldReader<u16>;
#[doc = "Field `TFLOW_CNT` writer - 13:0\\]
Tx flow table entry count"]
pub type TflowCntW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Tx flow table entry count"]
    #[inline(always)]
    pub fn tflow_cnt(&self) -> TflowCntR {
        TflowCntR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Tx flow table entry count"]
    #[inline(always)]
    #[must_use]
    pub fn tflow_cnt(&mut self) -> TflowCntW<PktdmaGcfgCap4Spec> {
        TflowCntW::new(self, 0)
    }
}
#[doc = "The Capabilities Register 4 specifies how many resources this PKTDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_cap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_cap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaGcfgCap4Spec;
impl crate::RegisterSpec for PktdmaGcfgCap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_gcfg_cap4::R`](R) reader structure"]
impl crate::Readable for PktdmaGcfgCap4Spec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_gcfg_cap4::W`](W) writer structure"]
impl crate::Writable for PktdmaGcfgCap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_GCFG_CAP4 to value 0x0112"]
impl crate::Resettable for PktdmaGcfgCap4Spec {
    const RESET_VALUE: u32 = 0x0112;
}
