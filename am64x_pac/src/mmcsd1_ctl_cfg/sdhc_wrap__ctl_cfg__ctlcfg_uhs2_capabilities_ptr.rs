#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_capabilities_ptr` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_capabilities_ptr` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec>;
#[doc = "Field `UHS2_CAPABILITIES_PTR` reader - 15:0\\]
Pointer for UHS-II Capabilities Register"]
pub type Uhs2CapabilitiesPtrR = crate::FieldReader<u16>;
#[doc = "Field `UHS2_CAPABILITIES_PTR` writer - 15:0\\]
Pointer for UHS-II Capabilities Register"]
pub type Uhs2CapabilitiesPtrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Pointer for UHS-II Capabilities Register"]
    #[inline(always)]
    pub fn uhs2_capabilities_ptr(&self) -> Uhs2CapabilitiesPtrR {
        Uhs2CapabilitiesPtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Pointer for UHS-II Capabilities Register"]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_capabilities_ptr(
        &mut self,
    ) -> Uhs2CapabilitiesPtrW<SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec> {
        Uhs2CapabilitiesPtrW::new(self, 0)
    }
}
#[doc = "This register is pointer for UHS-II Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_capabilities_ptr to value 0x0272"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec {
    const RESET_VALUE: u16 = 0x0272;
}
