#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_14` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_14` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec>;
#[doc = "Field `PI_CS_MASK` reader - 1:0\\]
Defines which chip selects are active."]
pub type PiCsMaskR = crate::FieldReader;
#[doc = "Field `PI_CS_MASK` writer - 1:0\\]
Defines which chip selects are active."]
pub type PiCsMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RANK_NUM_PER_CKE` reader - 12:8\\]
Defines the number of chip selects share one cke"]
pub type PiRankNumPerCkeR = crate::FieldReader;
#[doc = "Field `PI_RANK_NUM_PER_CKE` writer - 12:8\\]
Defines the number of chip selects share one cke"]
pub type PiRankNumPerCkeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_SRX_LVL_TARGET_CS_EN` reader - 16:16\\]
Defines self refresh exit trigger target rank/ranks training or all ranks training. 1: The rank/ranks exit from self refresh will trigger the corresponding rank/ranks training. Note: If multiple ranks exit from self refresh, current design only support the multiple ranks srx command issues at the same time. 0: Any rank/ranks exit from self refresh will trigger all ranks training"]
pub type PiSrxLvlTargetCsEnR = crate::BitReader;
#[doc = "Field `PI_SRX_LVL_TARGET_CS_EN` writer - 16:16\\]
Defines self refresh exit trigger target rank/ranks training or all ranks training. 1: The rank/ranks exit from self refresh will trigger the corresponding rank/ranks training. Note: If multiple ranks exit from self refresh, current design only support the multiple ranks srx command issues at the same time. 0: Any rank/ranks exit from self refresh will trigger all ranks training"]
pub type PiSrxLvlTargetCsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TMRR` reader - 27:24\\]
DRAM tMRR value in memory clock cycles."]
pub type PiTmrrR = crate::FieldReader;
#[doc = "Field `PI_TMRR` writer - 27:24\\]
DRAM tMRR value in memory clock cycles."]
pub type PiTmrrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip selects are active."]
    #[inline(always)]
    pub fn pi_cs_mask(&self) -> PiCsMaskR {
        PiCsMaskR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Defines the number of chip selects share one cke"]
    #[inline(always)]
    pub fn pi_rank_num_per_cke(&self) -> PiRankNumPerCkeR {
        PiRankNumPerCkeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines self refresh exit trigger target rank/ranks training or all ranks training. 1: The rank/ranks exit from self refresh will trigger the corresponding rank/ranks training. Note: If multiple ranks exit from self refresh, current design only support the multiple ranks srx command issues at the same time. 0: Any rank/ranks exit from self refresh will trigger all ranks training"]
    #[inline(always)]
    pub fn pi_srx_lvl_target_cs_en(&self) -> PiSrxLvlTargetCsEnR {
        PiSrxLvlTargetCsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM tMRR value in memory clock cycles."]
    #[inline(always)]
    pub fn pi_tmrr(&self) -> PiTmrrR {
        PiTmrrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip selects are active."]
    #[inline(always)]
    #[must_use]
    pub fn pi_cs_mask(&mut self) -> PiCsMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec> {
        PiCsMaskW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Defines the number of chip selects share one cke"]
    #[inline(always)]
    #[must_use]
    pub fn pi_rank_num_per_cke(
        &mut self,
    ) -> PiRankNumPerCkeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec> {
        PiRankNumPerCkeW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines self refresh exit trigger target rank/ranks training or all ranks training. 1: The rank/ranks exit from self refresh will trigger the corresponding rank/ranks training. Note: If multiple ranks exit from self refresh, current design only support the multiple ranks srx command issues at the same time. 0: Any rank/ranks exit from self refresh will trigger all ranks training"]
    #[inline(always)]
    #[must_use]
    pub fn pi_srx_lvl_target_cs_en(
        &mut self,
    ) -> PiSrxLvlTargetCsEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec> {
        PiSrxLvlTargetCsEnW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM tMRR value in memory clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrr(&mut self) -> PiTmrrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec> {
        PiTmrrW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_14::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_14::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_14 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi14Spec {
    const RESET_VALUE: u32 = 0;
}
