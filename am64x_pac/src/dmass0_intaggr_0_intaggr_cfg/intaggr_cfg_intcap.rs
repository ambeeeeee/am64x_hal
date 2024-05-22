#[doc = "Register `INTAGGR_CFG_INTCAP` reader"]
pub type R = crate::R<IntaggrCfgIntcapSpec>;
#[doc = "Register `INTAGGR_CFG_INTCAP` writer"]
pub type W = crate::W<IntaggrCfgIntcapSpec>;
#[doc = "Field `SEVT_CNT` reader - 15:0\\]
Number of 'event to virt int' mapping registers"]
pub type SevtCntR = crate::FieldReader<u16>;
#[doc = "Field `SEVT_CNT` writer - 15:0\\]
Number of 'event to virt int' mapping registers"]
pub type SevtCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VINTR_CNT` reader - 31:16\\]
Virtual interrupt register/pin count"]
pub type VintrCntR = crate::FieldReader<u16>;
#[doc = "Field `VINTR_CNT` writer - 31:16\\]
Virtual interrupt register/pin count"]
pub type VintrCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of 'event to virt int' mapping registers"]
    #[inline(always)]
    pub fn sevt_cnt(&self) -> SevtCntR {
        SevtCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Virtual interrupt register/pin count"]
    #[inline(always)]
    pub fn vintr_cnt(&self) -> VintrCntR {
        VintrCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Number of 'event to virt int' mapping registers"]
    #[inline(always)]
    #[must_use]
    pub fn sevt_cnt(&mut self) -> SevtCntW<IntaggrCfgIntcapSpec> {
        SevtCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Virtual interrupt register/pin count"]
    #[inline(always)]
    #[must_use]
    pub fn vintr_cnt(&mut self) -> VintrCntW<IntaggrCfgIntcapSpec> {
        VintrCntW::new(self, 16)
    }
}
#[doc = "The IntCap Register contains information on virtual interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_cfg_intcap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_cfg_intcap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrCfgIntcapSpec;
impl crate::RegisterSpec for IntaggrCfgIntcapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_cfg_intcap::R`](R) reader structure"]
impl crate::Readable for IntaggrCfgIntcapSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_cfg_intcap::W`](W) writer structure"]
impl crate::Writable for IntaggrCfgIntcapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_CFG_INTCAP to value 0x0184_0000"]
impl crate::Resettable for IntaggrCfgIntcapSpec {
    const RESET_VALUE: u64 = 0x0184_0000;
}
