#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_343` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_343` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec>;
#[doc = "Field `INT_STATUS_MODE` reader - 7:0\\]
Status of interrupts in the controller related to Memory Mode Settings. READ-ONLY"]
pub type IntStatusModeR = crate::FieldReader;
#[doc = "Field `INT_STATUS_MODE` writer - 7:0\\]
Status of interrupts in the controller related to Memory Mode Settings. READ-ONLY"]
pub type IntStatusModeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_STATUS_PARITY` reader - 15:8\\]
Status of interrupts in the controller related to Parity. READ-ONLY"]
pub type IntStatusParityR = crate::FieldReader;
#[doc = "Field `INT_STATUS_PARITY` writer - 15:8\\]
Status of interrupts in the controller related to Parity. READ-ONLY"]
pub type IntStatusParityW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status of interrupts in the controller related to Memory Mode Settings. READ-ONLY"]
    #[inline(always)]
    pub fn int_status_mode(&self) -> IntStatusModeR {
        IntStatusModeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Status of interrupts in the controller related to Parity. READ-ONLY"]
    #[inline(always)]
    pub fn int_status_parity(&self) -> IntStatusParityR {
        IntStatusParityR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status of interrupts in the controller related to Memory Mode Settings. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_mode(&mut self) -> IntStatusModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec> {
        IntStatusModeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Status of interrupts in the controller related to Parity. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_parity(
        &mut self,
    ) -> IntStatusParityW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec> {
        IntStatusParityW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_343\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_343::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_343::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_343::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_343::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_343 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl343Spec {
    const RESET_VALUE: u32 = 0;
}
