#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_124` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_124` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec>;
#[doc = "Field `PHYMSTR_DFI4_PROMOTE_THRESHOLD_F0` reader - 15:0\\]
Defines the DFI\\[4.0 and 4.0v2\\]
PHY master request promotion number of regular \\[not long\\]
counts until the high priority request is asserted. FC=0"]
pub type PhymstrDfi4PromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `PHYMSTR_DFI4_PROMOTE_THRESHOLD_F0` writer - 15:0\\]
Defines the DFI\\[4.0 and 4.0v2\\]
PHY master request promotion number of regular \\[not long\\]
counts until the high priority request is asserted. FC=0"]
pub type PhymstrDfi4PromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the DFI\\[4.0 and 4.0v2\\]
PHY master request promotion number of regular \\[not long\\]
counts until the high priority request is asserted. FC=0"]
    #[inline(always)]
    pub fn phymstr_dfi4_promote_threshold_f0(&self) -> PhymstrDfi4PromoteThresholdF0R {
        PhymstrDfi4PromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the DFI\\[4.0 and 4.0v2\\]
PHY master request promotion number of regular \\[not long\\]
counts until the high priority request is asserted. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_dfi4_promote_threshold_f0(
        &mut self,
    ) -> PhymstrDfi4PromoteThresholdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec> {
        PhymstrDfi4PromoteThresholdF0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_124::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_124::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_124::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_124::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_124 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl124Spec {
    const RESET_VALUE: u32 = 0;
}
