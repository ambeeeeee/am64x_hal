#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1364` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1364` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec>;
#[doc = "Field `PHY_DDL_AC_MASK` reader - 5:0\\]
PHY Address/Control DDL BIST mask."]
pub type PhyDdlAcMaskR = crate::FieldReader;
#[doc = "Field `PHY_DDL_AC_MASK` writer - 5:0\\]
PHY Address/Control DDL BIST mask."]
pub type PhyDdlAcMaskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_INIT_UPDATE_CONFIG` reader - 10:8\\]
PHY init update function configuration."]
pub type PhyInitUpdateConfigR = crate::FieldReader;
#[doc = "Field `PHY_INIT_UPDATE_CONFIG` writer - 10:8\\]
PHY init update function configuration."]
pub type PhyInitUpdateConfigW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_DDL_TRACK_UPD_THRESHOLD_AC` reader - 23:16\\]
Specify threshold value for PHY init update tracking for AC slice."]
pub type PhyDdlTrackUpdThresholdAcR = crate::FieldReader;
#[doc = "Field `PHY_DDL_TRACK_UPD_THRESHOLD_AC` writer - 23:16\\]
Specify threshold value for PHY init update tracking for AC slice."]
pub type PhyDdlTrackUpdThresholdAcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
PHY Address/Control DDL BIST mask."]
    #[inline(always)]
    pub fn phy_ddl_ac_mask(&self) -> PhyDdlAcMaskR {
        PhyDdlAcMaskR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
PHY init update function configuration."]
    #[inline(always)]
    pub fn phy_init_update_config(&self) -> PhyInitUpdateConfigR {
        PhyInitUpdateConfigR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Specify threshold value for PHY init update tracking for AC slice."]
    #[inline(always)]
    pub fn phy_ddl_track_upd_threshold_ac(&self) -> PhyDdlTrackUpdThresholdAcR {
        PhyDdlTrackUpdThresholdAcR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
PHY Address/Control DDL BIST mask."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_ac_mask(&mut self) -> PhyDdlAcMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec> {
        PhyDdlAcMaskW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
PHY init update function configuration."]
    #[inline(always)]
    #[must_use]
    pub fn phy_init_update_config(
        &mut self,
    ) -> PhyInitUpdateConfigW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec> {
        PhyInitUpdateConfigW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Specify threshold value for PHY init update tracking for AC slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_track_upd_threshold_ac(
        &mut self,
    ) -> PhyDdlTrackUpdThresholdAcW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec> {
        PhyDdlTrackUpdThresholdAcW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1364\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1364::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1364::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1364::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1364::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1364 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1364Spec {
    const RESET_VALUE: u32 = 0;
}
