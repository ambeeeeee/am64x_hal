#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_341` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_341` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec>;
#[doc = "Field `INT_STATUS_MISC` reader - 15:0\\]
Status of interrupts in the controller related to Miscellaneous features. READ-ONLY"]
pub type IntStatusMiscR = crate::FieldReader<u16>;
#[doc = "Field `INT_STATUS_MISC` writer - 15:0\\]
Status of interrupts in the controller related to Miscellaneous features. READ-ONLY"]
pub type IntStatusMiscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INT_STATUS_BIST` reader - 23:16\\]
Status of interrupts in the controller related to BIST. READ-ONLY"]
pub type IntStatusBistR = crate::FieldReader;
#[doc = "Field `INT_STATUS_BIST` writer - 23:16\\]
Status of interrupts in the controller related to BIST. READ-ONLY"]
pub type IntStatusBistW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of interrupts in the controller related to Miscellaneous features. READ-ONLY"]
    #[inline(always)]
    pub fn int_status_misc(&self) -> IntStatusMiscR {
        IntStatusMiscR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Status of interrupts in the controller related to BIST. READ-ONLY"]
    #[inline(always)]
    pub fn int_status_bist(&self) -> IntStatusBistR {
        IntStatusBistR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of interrupts in the controller related to Miscellaneous features. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_misc(&mut self) -> IntStatusMiscW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec> {
        IntStatusMiscW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Status of interrupts in the controller related to BIST. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_bist(&mut self) -> IntStatusBistW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec> {
        IntStatusBistW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_341\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_341::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_341::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_341::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_341::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_341 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl341Spec {
    const RESET_VALUE: u32 = 0;
}
