#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_12` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_12` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec>;
#[doc = "Field `PHY_PRBS_PATTERN_MASK_0` reader - 8:0\\]
PRBS7 mask signal for slice 0."]
pub type PhyPrbsPatternMask0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PRBS_PATTERN_MASK_0` writer - 8:0\\]
PRBS7 mask signal for slice 0."]
pub type PhyPrbsPatternMask0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_ENABLE_0` reader - 16:16\\]
Read Leveling Multi-pattern enable for slice 0."]
pub type PhyRdlvlMultiPattEnable0R = crate::BitReader;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_ENABLE_0` writer - 16:16\\]
Read Leveling Multi-pattern enable for slice 0."]
pub type PhyRdlvlMultiPattEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_RST_DISABLE_0` reader - 24:24\\]
Read Leveling read level windows disable reset for slice 0."]
pub type PhyRdlvlMultiPattRstDisable0R = crate::BitReader;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_RST_DISABLE_0` writer - 24:24\\]
Read Leveling read level windows disable reset for slice 0."]
pub type PhyRdlvlMultiPattRstDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
PRBS7 mask signal for slice 0."]
    #[inline(always)]
    pub fn phy_prbs_pattern_mask_0(&self) -> PhyPrbsPatternMask0R {
        PhyPrbsPatternMask0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Read Leveling Multi-pattern enable for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_multi_patt_enable_0(&self) -> PhyRdlvlMultiPattEnable0R {
        PhyRdlvlMultiPattEnable0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Read Leveling read level windows disable reset for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_multi_patt_rst_disable_0(&self) -> PhyRdlvlMultiPattRstDisable0R {
        PhyRdlvlMultiPattRstDisable0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
PRBS7 mask signal for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_prbs_pattern_mask_0(
        &mut self,
    ) -> PhyPrbsPatternMask0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec> {
        PhyPrbsPatternMask0W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Read Leveling Multi-pattern enable for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_multi_patt_enable_0(
        &mut self,
    ) -> PhyRdlvlMultiPattEnable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec> {
        PhyRdlvlMultiPattEnable0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Read Leveling read level windows disable reset for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_multi_patt_rst_disable_0(
        &mut self,
    ) -> PhyRdlvlMultiPattRstDisable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec> {
        PhyRdlvlMultiPattRstDisable0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_12::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_12::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_12 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy12Spec {
    const RESET_VALUE: u32 = 0;
}
