#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_172` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_172` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F1` reader - 15:0\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=1"]
pub type LpcPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F1` writer - 15:0\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=1"]
pub type LpcPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F2` reader - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=2"]
pub type LpcPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `LPC_PROMOTE_THRESHOLD_F2` writer - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=2"]
pub type LpcPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=1"]
    #[inline(always)]
    pub fn lpc_promote_threshold_f1(&self) -> LpcPromoteThresholdF1R {
        LpcPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=2"]
    #[inline(always)]
    pub fn lpc_promote_threshold_f2(&self) -> LpcPromoteThresholdF2R {
        LpcPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lpc_promote_threshold_f1(
        &mut self,
    ) -> LpcPromoteThresholdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec> {
        LpcPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
LPC promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and auto low power commands. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpc_promote_threshold_f2(
        &mut self,
    ) -> LpcPromoteThresholdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec> {
        LpcPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_172\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_172::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_172::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_172::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_172::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_172 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl172Spec {
    const RESET_VALUE: u32 = 0;
}