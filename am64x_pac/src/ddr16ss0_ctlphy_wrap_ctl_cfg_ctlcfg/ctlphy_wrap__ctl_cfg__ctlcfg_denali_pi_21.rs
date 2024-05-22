#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_21` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_21` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec>;
#[doc = "Field `PI_SWLVL_RD_SLICE_1` reader - 0:0\\]
SW leveling read command in WDQ training. WRITE-ONLY"]
pub type PiSwlvlRdSlice1R = crate::BitReader;
#[doc = "Field `PI_SWLVL_RD_SLICE_1` writer - 0:0\\]
SW leveling read command in WDQ training. WRITE-ONLY"]
pub type PiSwlvlRdSlice1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_VREF_UPDATE_SLICE_1` reader - 8:8\\]
SW leveling vref update command in WDQ training. WRITE-ONLY"]
pub type PiSwlvlVrefUpdateSlice1R = crate::BitReader;
#[doc = "Field `PI_SWLVL_VREF_UPDATE_SLICE_1` writer - 8:8\\]
SW leveling vref update command in WDQ training. WRITE-ONLY"]
pub type PiSwlvlVrefUpdateSlice1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SW_WDQLVL_RESP_1` reader - 17:16\\]
Leveling response for data slice 1. READ-ONLY"]
pub type PiSwWdqlvlResp1R = crate::FieldReader;
#[doc = "Field `PI_SW_WDQLVL_RESP_1` writer - 17:16\\]
Leveling response for data slice 1. READ-ONLY"]
pub type PiSwWdqlvlResp1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_SWLVL_SM2_START` reader - 24:24\\]
SW leveling start command for stage 2. WRITE-ONLY"]
pub type PiSwlvlSm2StartR = crate::BitReader;
#[doc = "Field `PI_SWLVL_SM2_START` writer - 24:24\\]
SW leveling start command for stage 2. WRITE-ONLY"]
pub type PiSwlvlSm2StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SW leveling read command in WDQ training. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_rd_slice_1(&self) -> PiSwlvlRdSlice1R {
        PiSwlvlRdSlice1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SW leveling vref update command in WDQ training. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_vref_update_slice_1(&self) -> PiSwlvlVrefUpdateSlice1R {
        PiSwlvlVrefUpdateSlice1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Leveling response for data slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn pi_sw_wdqlvl_resp_1(&self) -> PiSwWdqlvlResp1R {
        PiSwWdqlvlResp1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
SW leveling start command for stage 2. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_sm2_start(&self) -> PiSwlvlSm2StartR {
        PiSwlvlSm2StartR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SW leveling read command in WDQ training. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_rd_slice_1(
        &mut self,
    ) -> PiSwlvlRdSlice1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec> {
        PiSwlvlRdSlice1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SW leveling vref update command in WDQ training. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_vref_update_slice_1(
        &mut self,
    ) -> PiSwlvlVrefUpdateSlice1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec> {
        PiSwlvlVrefUpdateSlice1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Leveling response for data slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_wdqlvl_resp_1(
        &mut self,
    ) -> PiSwWdqlvlResp1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec> {
        PiSwWdqlvlResp1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
SW leveling start command for stage 2. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_sm2_start(
        &mut self,
    ) -> PiSwlvlSm2StartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec> {
        PiSwlvlSm2StartW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_21::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_21::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_21 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi21Spec {
    const RESET_VALUE: u32 = 0;
}
