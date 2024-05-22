#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_170` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_170` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F0` reader - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=0"]
pub type HwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F0` writer - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=0"]
pub type HwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F1` reader - 31:16\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=1"]
pub type HwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F1` writer - 31:16\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=1"]
pub type HwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=0"]
    #[inline(always)]
    pub fn hw_promote_threshold_f0(&self) -> HwPromoteThresholdF0R {
        HwPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=1"]
    #[inline(always)]
    pub fn hw_promote_threshold_f1(&self) -> HwPromoteThresholdF1R {
        HwPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn hw_promote_threshold_f0(
        &mut self,
    ) -> HwPromoteThresholdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec> {
        HwPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
HW interface promotion number of long counts until the high priority request is asserted. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn hw_promote_threshold_f1(
        &mut self,
    ) -> HwPromoteThresholdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec> {
        HwPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_170\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_170::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_170::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_170::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_170::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_170 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl170Spec {
    const RESET_VALUE: u32 = 0;
}
