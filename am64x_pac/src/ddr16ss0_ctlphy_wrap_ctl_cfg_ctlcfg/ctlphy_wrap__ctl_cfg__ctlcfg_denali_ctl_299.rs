#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_299` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_299` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F1` reader - 15:0\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=1"]
pub type ZqCallatchTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F1` writer - 15:0\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=1"]
pub type ZqCallatchTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CS_TIMEOUT_F1` reader - 31:16\\]
ZQ CS number of long counts until the timeout is asserted. FC=1"]
pub type ZqCsTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_TIMEOUT_F1` writer - 31:16\\]
ZQ CS number of long counts until the timeout is asserted. FC=1"]
pub type ZqCsTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=1"]
    #[inline(always)]
    pub fn zq_callatch_timeout_f1(&self) -> ZqCallatchTimeoutF1R {
        ZqCallatchTimeoutF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ CS number of long counts until the timeout is asserted. FC=1"]
    #[inline(always)]
    pub fn zq_cs_timeout_f1(&self) -> ZqCsTimeoutF1R {
        ZqCsTimeoutF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ZQ LATCH number of long counts until the timeout is asserted. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_timeout_f1(
        &mut self,
    ) -> ZqCallatchTimeoutF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec> {
        ZqCallatchTimeoutF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ CS number of long counts until the timeout is asserted. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_timeout_f1(&mut self) -> ZqCsTimeoutF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec> {
        ZqCsTimeoutF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_299\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_299::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_299::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_299::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_299::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_299 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl299Spec {
    const RESET_VALUE: u32 = 0;
}
