#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_178` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_178` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec>;
#[doc = "Field `INIT_FREQ` reader - 1:0\\]
Specifies what frequency register copy will be in use by the memory after initialization completes."]
pub type InitFreqR = crate::FieldReader;
#[doc = "Field `INIT_FREQ` writer - 1:0\\]
Specifies what frequency register copy will be in use by the memory after initialization completes."]
pub type InitFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DFIBUS_BOOT_FREQ` reader - 9:8\\]
Defines the DFI bus boot frequency register copy"]
pub type DfibusBootFreqR = crate::FieldReader;
#[doc = "Field `DFIBUS_BOOT_FREQ` writer - 9:8\\]
Defines the DFI bus boot frequency register copy"]
pub type DfibusBootFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies what frequency register copy will be in use by the memory after initialization completes."]
    #[inline(always)]
    pub fn init_freq(&self) -> InitFreqR {
        InitFreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the DFI bus boot frequency register copy"]
    #[inline(always)]
    pub fn dfibus_boot_freq(&self) -> DfibusBootFreqR {
        DfibusBootFreqR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies what frequency register copy will be in use by the memory after initialization completes."]
    #[inline(always)]
    #[must_use]
    pub fn init_freq(&mut self) -> InitFreqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec> {
        InitFreqW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the DFI bus boot frequency register copy"]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_boot_freq(
        &mut self,
    ) -> DfibusBootFreqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec> {
        DfibusBootFreqW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_178\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_178::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_178::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_178::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_178::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_178 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl178Spec {
    const RESET_VALUE: u32 = 0;
}
