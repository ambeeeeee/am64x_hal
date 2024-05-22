#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_295` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_295` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec>;
#[doc = "Field `ZQ_CS_TIMEOUT_F0` reader - 15:0\\]
ZQ CS number of long counts until the timeout is asserted. FC=0"]
pub type ZqCsTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_TIMEOUT_F0` writer - 15:0\\]
ZQ CS number of long counts until the timeout is asserted. FC=0"]
pub type ZqCsTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_PROMOTE_THRESHOLD_F0` reader - 31:16\\]
ZQ SW promotion number of long counts until the high priority request is asserted. FC=0"]
pub type ZqPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_PROMOTE_THRESHOLD_F0` writer - 31:16\\]
ZQ SW promotion number of long counts until the high priority request is asserted. FC=0"]
pub type ZqPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ CS number of long counts until the timeout is asserted. FC=0"]
    #[inline(always)]
    pub fn zq_cs_timeout_f0(&self) -> ZqCsTimeoutF0R {
        ZqCsTimeoutF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ SW promotion number of long counts until the high priority request is asserted. FC=0"]
    #[inline(always)]
    pub fn zq_promote_threshold_f0(&self) -> ZqPromoteThresholdF0R {
        ZqPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ CS number of long counts until the timeout is asserted. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_timeout_f0(&mut self) -> ZqCsTimeoutF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec> {
        ZqCsTimeoutF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ SW promotion number of long counts until the high priority request is asserted. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zq_promote_threshold_f0(
        &mut self,
    ) -> ZqPromoteThresholdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec> {
        ZqPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_295\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_295::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_295::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_295::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_295::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_295 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl295Spec {
    const RESET_VALUE: u32 = 0;
}
