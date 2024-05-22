#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_144` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_144` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec>;
#[doc = "Field `PI_ZQ_REQ_PENDING` reader - 16:16\\]
Indicates that a ZQ command is currently in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
pub type PiZqReqPendingR = crate::BitReader;
#[doc = "Field `PI_ZQ_REQ_PENDING` writer - 16:16\\]
Indicates that a ZQ command is currently in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
pub type PiZqReqPendingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - 16:16\\]
Indicates that a ZQ command is currently in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
    #[inline(always)]
    pub fn pi_zq_req_pending(&self) -> PiZqReqPendingR {
        PiZqReqPendingR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - 16:16\\]
Indicates that a ZQ command is currently in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_zq_req_pending(
        &mut self,
    ) -> PiZqReqPendingW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec> {
        PiZqReqPendingW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_144::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_144::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_144::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_144::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_144 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi144Spec {
    const RESET_VALUE: u32 = 0;
}
