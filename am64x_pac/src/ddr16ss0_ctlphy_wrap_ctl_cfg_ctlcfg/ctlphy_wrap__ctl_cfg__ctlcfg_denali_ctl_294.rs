#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_294` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_294` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F0` reader - 15:0\\]
ZQ START number of long counts until the timeout is asserted. FC=0"]
pub type ZqCalstartTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F0` writer - 15:0\\]
ZQ START number of long counts until the timeout is asserted. FC=0"]
pub type ZqCalstartTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F0` reader - 31:16\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=0"]
pub type ZqCallatchTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F0` writer - 31:16\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=0"]
pub type ZqCallatchTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ START number of long counts until the timeout is asserted. FC=0"]
    #[inline(always)]
    pub fn zq_calstart_timeout_f0(&self) -> ZqCalstartTimeoutF0R {
        ZqCalstartTimeoutF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=0"]
    #[inline(always)]
    pub fn zq_callatch_timeout_f0(&self) -> ZqCallatchTimeoutF0R {
        ZqCallatchTimeoutF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ START number of long counts until the timeout is asserted. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_timeout_f0(
        &mut self,
    ) -> ZqCalstartTimeoutF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec> {
        ZqCalstartTimeoutF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_timeout_f0(
        &mut self,
    ) -> ZqCallatchTimeoutF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec> {
        ZqCallatchTimeoutF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_294\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_294::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_294::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_294::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_294::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_294 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl294Spec {
    const RESET_VALUE: u32 = 0;
}
