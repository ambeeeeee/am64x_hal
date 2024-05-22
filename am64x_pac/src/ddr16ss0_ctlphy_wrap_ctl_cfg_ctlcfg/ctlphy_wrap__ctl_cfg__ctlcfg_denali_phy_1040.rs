#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1040` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1040` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec>;
#[doc = "Field `PHY_ADR_CALVL_DEBUG_MODE_2` reader - 0:0\\]
Enables CA training debug mode for address slice 2. Set to 1 to enable."]
pub type PhyAdrCalvlDebugMode2R = crate::BitReader;
#[doc = "Field `PHY_ADR_CALVL_DEBUG_MODE_2` writer - 0:0\\]
Enables CA training debug mode for address slice 2. Set to 1 to enable."]
pub type PhyAdrCalvlDebugMode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_CALVL_DEBUG_CONT_2` reader - 8:8\\]
Allows the CA training state machine to advance \\[when in debug mode\\]
for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrCalvlDebugCont2R = crate::BitReader;
#[doc = "Field `SC_PHY_ADR_CALVL_DEBUG_CONT_2` writer - 8:8\\]
Allows the CA training state machine to advance \\[when in debug mode\\]
for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrCalvlDebugCont2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_ADR_CALVL_ERROR_CLR_2` reader - 16:16\\]
Clears the CA training state machine error status for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrCalvlErrorClr2R = crate::BitReader;
#[doc = "Field `SC_PHY_ADR_CALVL_ERROR_CLR_2` writer - 16:16\\]
Clears the CA training state machine error status for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyAdrCalvlErrorClr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_CALVL_OBS_SELECT_2` reader - 26:24\\]
CA bit lane to observe result from OBS0 during CA training for address slice 2."]
pub type PhyAdrCalvlObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_OBS_SELECT_2` writer - 26:24\\]
CA bit lane to observe result from OBS0 during CA training for address slice 2."]
pub type PhyAdrCalvlObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables CA training debug mode for address slice 2. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_adr_calvl_debug_mode_2(&self) -> PhyAdrCalvlDebugMode2R {
        PhyAdrCalvlDebugMode2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows the CA training state machine to advance \\[when in debug mode\\]
for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_adr_calvl_debug_cont_2(&self) -> ScPhyAdrCalvlDebugCont2R {
        ScPhyAdrCalvlDebugCont2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Clears the CA training state machine error status for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_adr_calvl_error_clr_2(&self) -> ScPhyAdrCalvlErrorClr2R {
        ScPhyAdrCalvlErrorClr2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
CA bit lane to observe result from OBS0 during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_obs_select_2(&self) -> PhyAdrCalvlObsSelect2R {
        PhyAdrCalvlObsSelect2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables CA training debug mode for address slice 2. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_debug_mode_2(
        &mut self,
    ) -> PhyAdrCalvlDebugMode2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec> {
        PhyAdrCalvlDebugMode2W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows the CA training state machine to advance \\[when in debug mode\\]
for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_calvl_debug_cont_2(
        &mut self,
    ) -> ScPhyAdrCalvlDebugCont2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec> {
        ScPhyAdrCalvlDebugCont2W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Clears the CA training state machine error status for address slice 2. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_calvl_error_clr_2(
        &mut self,
    ) -> ScPhyAdrCalvlErrorClr2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec> {
        ScPhyAdrCalvlErrorClr2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
CA bit lane to observe result from OBS0 during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_obs_select_2(
        &mut self,
    ) -> PhyAdrCalvlObsSelect2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec> {
        PhyAdrCalvlObsSelect2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1040\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1040::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1040::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1040::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1040::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1040 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1040Spec {
    const RESET_VALUE: u32 = 0;
}
