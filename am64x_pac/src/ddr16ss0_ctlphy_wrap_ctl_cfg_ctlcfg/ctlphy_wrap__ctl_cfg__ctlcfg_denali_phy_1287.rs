#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1287` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1287` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec>;
#[doc = "Field `SC_PHY_MANUAL_UPDATE` reader - 0:0\\]
Manual update of all slave delay line settings. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyManualUpdateR = crate::BitReader;
#[doc = "Field `SC_PHY_MANUAL_UPDATE` writer - 0:0\\]
Manual update of all slave delay line settings. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyManualUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MANUAL_UPDATE_PHYUPD_ENABLE` reader - 8:8\\]
Manual update selection of all slave delay line settings. Set 1 to assert phyupd_req and wait phyupd_ack to update delay line, set 0 to update delay line directly."]
pub type PhyManualUpdatePhyupdEnableR = crate::BitReader;
#[doc = "Field `PHY_MANUAL_UPDATE_PHYUPD_ENABLE` writer - 8:8\\]
Manual update selection of all slave delay line settings. Set 1 to assert phyupd_req and wait phyupd_ack to update delay line, set 0 to update delay line directly."]
pub type PhyManualUpdatePhyupdEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CSLVL_START` reader - 26:16\\]
Defines the CS training DDL start value."]
pub type PhyCslvlStartR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_START` writer - 26:16\\]
Defines the CS training DDL start value."]
pub type PhyCslvlStartW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Manual update of all slave delay line settings. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_manual_update(&self) -> ScPhyManualUpdateR {
        ScPhyManualUpdateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Manual update selection of all slave delay line settings. Set 1 to assert phyupd_req and wait phyupd_ack to update delay line, set 0 to update delay line directly."]
    #[inline(always)]
    pub fn phy_manual_update_phyupd_enable(&self) -> PhyManualUpdatePhyupdEnableR {
        PhyManualUpdatePhyupdEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Defines the CS training DDL start value."]
    #[inline(always)]
    pub fn phy_cslvl_start(&self) -> PhyCslvlStartR {
        PhyCslvlStartR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Manual update of all slave delay line settings. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_manual_update(
        &mut self,
    ) -> ScPhyManualUpdateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec> {
        ScPhyManualUpdateW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Manual update selection of all slave delay line settings. Set 1 to assert phyupd_req and wait phyupd_ack to update delay line, set 0 to update delay line directly."]
    #[inline(always)]
    #[must_use]
    pub fn phy_manual_update_phyupd_enable(
        &mut self,
    ) -> PhyManualUpdatePhyupdEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec> {
        PhyManualUpdatePhyupdEnableW::new(self, 8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Defines the CS training DDL start value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_start(&mut self) -> PhyCslvlStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec> {
        PhyCslvlStartW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1287\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1287::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1287::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1287::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1287::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1287 to value 0x0100"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1287Spec {
    const RESET_VALUE: u32 = 0x0100;
}
