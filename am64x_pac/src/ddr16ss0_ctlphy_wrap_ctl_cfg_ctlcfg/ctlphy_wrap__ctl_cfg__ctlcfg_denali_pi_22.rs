#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_22` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_22` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec>;
#[doc = "Field `PI_SWLVL_SM2_WR` reader - 0:0\\]
SW leveling write command for stage 2. WRITE-ONLY"]
pub type PiSwlvlSm2WrR = crate::BitReader;
#[doc = "Field `PI_SWLVL_SM2_WR` writer - 0:0\\]
SW leveling write command for stage 2. WRITE-ONLY"]
pub type PiSwlvlSm2WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_SM2_RD` reader - 8:8\\]
SW leveling read command for stage 2. WRITE-ONLY"]
pub type PiSwlvlSm2RdR = crate::BitReader;
#[doc = "Field `PI_SWLVL_SM2_RD` writer - 8:8\\]
SW leveling read command for stage 2. WRITE-ONLY"]
pub type PiSwlvlSm2RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SEQUENTIAL_LVL_REQ` reader - 16:16\\]
User request to initiate all possible leveling sequences. Set to 1 to trigger. WRITE-ONLY"]
pub type PiSequentialLvlReqR = crate::BitReader;
#[doc = "Field `PI_SEQUENTIAL_LVL_REQ` writer - 16:16\\]
User request to initiate all possible leveling sequences. Set to 1 to trigger. WRITE-ONLY"]
pub type PiSequentialLvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DFS_PERIOD_EN` reader - 24:24\\]
Enable the DFS triggered periodic leveling."]
pub type PiDfsPeriodEnR = crate::BitReader;
#[doc = "Field `PI_DFS_PERIOD_EN` writer - 24:24\\]
Enable the DFS triggered periodic leveling."]
pub type PiDfsPeriodEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SW leveling write command for stage 2. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_sm2_wr(&self) -> PiSwlvlSm2WrR {
        PiSwlvlSm2WrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SW leveling read command for stage 2. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_sm2_rd(&self) -> PiSwlvlSm2RdR {
        PiSwlvlSm2RdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
User request to initiate all possible leveling sequences. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_sequential_lvl_req(&self) -> PiSequentialLvlReqR {
        PiSequentialLvlReqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable the DFS triggered periodic leveling."]
    #[inline(always)]
    pub fn pi_dfs_period_en(&self) -> PiDfsPeriodEnR {
        PiDfsPeriodEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SW leveling write command for stage 2. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_sm2_wr(&mut self) -> PiSwlvlSm2WrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec> {
        PiSwlvlSm2WrW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SW leveling read command for stage 2. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_sm2_rd(&mut self) -> PiSwlvlSm2RdW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec> {
        PiSwlvlSm2RdW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
User request to initiate all possible leveling sequences. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sequential_lvl_req(
        &mut self,
    ) -> PiSequentialLvlReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec> {
        PiSequentialLvlReqW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable the DFS triggered periodic leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfs_period_en(&mut self) -> PiDfsPeriodEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec> {
        PiDfsPeriodEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_22::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_22::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_22 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi22Spec {
    const RESET_VALUE: u32 = 0;
}
