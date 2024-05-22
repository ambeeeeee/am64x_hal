#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_296` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_296` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F1` reader - 15:0\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=1"]
pub type ZqCalstartNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F1` writer - 15:0\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=1"]
pub type ZqCalstartNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F1` reader - 31:16\\]
ZQ START number of long counts until the high priority request is asserted. FC=1"]
pub type ZqCalstartHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F1` writer - 31:16\\]
ZQ START number of long counts until the high priority request is asserted. FC=1"]
pub type ZqCalstartHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=1"]
    #[inline(always)]
    pub fn zq_calstart_norm_threshold_f1(&self) -> ZqCalstartNormThresholdF1R {
        ZqCalstartNormThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ START number of long counts until the high priority request is asserted. FC=1"]
    #[inline(always)]
    pub fn zq_calstart_high_threshold_f1(&self) -> ZqCalstartHighThresholdF1R {
        ZqCalstartHighThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_norm_threshold_f1(
        &mut self,
    ) -> ZqCalstartNormThresholdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec> {
        ZqCalstartNormThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ START number of long counts until the high priority request is asserted. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_high_threshold_f1(
        &mut self,
    ) -> ZqCalstartHighThresholdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec> {
        ZqCalstartHighThresholdF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_296\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_296::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_296::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_296::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_296::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_296 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl296Spec {
    const RESET_VALUE: u32 = 0;
}
