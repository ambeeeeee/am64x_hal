#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_171` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_171` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F2` reader - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=2"]
pub type HwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F2` writer - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=2"]
pub type HwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F0` reader - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=0"]
pub type LpcPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F0` writer - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=0"]
pub type LpcPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=2"]
    #[inline(always)]
    pub fn hw_promote_threshold_f2(&self) -> HwPromoteThresholdF2R {
        HwPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=0"]
    #[inline(always)]
    pub fn lpc_promote_threshold_f0(&self) -> LpcPromoteThresholdF0R {
        LpcPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn hw_promote_threshold_f2(
        &mut self,
    ) -> HwPromoteThresholdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec> {
        HwPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn lpc_promote_threshold_f0(
        &mut self,
    ) -> LpcPromoteThresholdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec> {
        LpcPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_171\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_171::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_171::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_171::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_171::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_171 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl171Spec {
    const RESET_VALUE: u32 = 0;
}
