#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_268` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_268` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec>;
#[doc = "Field `PHY_PRBS_PATTERN_MASK_1` reader - 8:0\\]
PRBS7 mask signal for slice 1."]
pub type PhyPrbsPatternMask1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PRBS_PATTERN_MASK_1` writer - 8:0\\]
PRBS7 mask signal for slice 1."]
pub type PhyPrbsPatternMask1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_ENABLE_1` reader - 16:16\\]
Read Leveling Multi-pattern enable for slice 1."]
pub type PhyRdlvlMultiPattEnable1R = crate::BitReader;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_ENABLE_1` writer - 16:16\\]
Read Leveling Multi-pattern enable for slice 1."]
pub type PhyRdlvlMultiPattEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_RST_DISABLE_1` reader - 24:24\\]
Read Leveling read level windows disable reset for slice 1."]
pub type PhyRdlvlMultiPattRstDisable1R = crate::BitReader;
#[doc = "Field `PHY_RDLVL_MULTI_PATT_RST_DISABLE_1` writer - 24:24\\]
Read Leveling read level windows disable reset for slice 1."]
pub type PhyRdlvlMultiPattRstDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
PRBS7 mask signal for slice 1."]
    #[inline(always)]
    pub fn phy_prbs_pattern_mask_1(&self) -> PhyPrbsPatternMask1R {
        PhyPrbsPatternMask1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Read Leveling Multi-pattern enable for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_multi_patt_enable_1(&self) -> PhyRdlvlMultiPattEnable1R {
        PhyRdlvlMultiPattEnable1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Read Leveling read level windows disable reset for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_multi_patt_rst_disable_1(&self) -> PhyRdlvlMultiPattRstDisable1R {
        PhyRdlvlMultiPattRstDisable1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
PRBS7 mask signal for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_prbs_pattern_mask_1(
        &mut self,
    ) -> PhyPrbsPatternMask1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec> {
        PhyPrbsPatternMask1W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Read Leveling Multi-pattern enable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_multi_patt_enable_1(
        &mut self,
    ) -> PhyRdlvlMultiPattEnable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec> {
        PhyRdlvlMultiPattEnable1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Read Leveling read level windows disable reset for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_multi_patt_rst_disable_1(
        &mut self,
    ) -> PhyRdlvlMultiPattRstDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec> {
        PhyRdlvlMultiPattRstDisable1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_268\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_268::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_268::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_268::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_268::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_268 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy268Spec {
    const RESET_VALUE: u32 = 0;
}
