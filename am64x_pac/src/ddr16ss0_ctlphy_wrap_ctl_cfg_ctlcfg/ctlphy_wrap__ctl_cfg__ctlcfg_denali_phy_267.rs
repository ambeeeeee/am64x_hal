#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_267` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_267` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec>;
#[doc = "Field `PHY_DQ_IDLE_1` reader - 8:0\\]
When set to 1, the inavlid DQ will be driven to high, when set to 0, the invalid DQ will be driven to low for slice 1."]
pub type PhyDqIdle1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_DQ_IDLE_1` writer - 8:0\\]
When set to 1, the inavlid DQ will be driven to high, when set to 0, the invalid DQ will be driven to low for slice 1."]
pub type PhyDqIdle1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PHY_PDA_MODE_EN_1` reader - 16:16\\]
When set to 1, the invalid DQs will be driven by the dfi_wrdata to make sure the tpda_s and tpda_h's timing is meet for slice 1."]
pub type PhyPdaModeEn1R = crate::BitReader;
#[doc = "Field `PHY_PDA_MODE_EN_1` writer - 16:16\\]
When set to 1, the invalid DQs will be driven by the dfi_wrdata to make sure the tpda_s and tpda_h's timing is meet for slice 1."]
pub type PhyPdaModeEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PRBS_PATTERN_START_1` reader - 30:24\\]
PRBS7 start pattern for slice 1."]
pub type PhyPrbsPatternStart1R = crate::FieldReader;
#[doc = "Field `PHY_PRBS_PATTERN_START_1` writer - 30:24\\]
PRBS7 start pattern for slice 1."]
pub type PhyPrbsPatternStart1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
When set to 1, the inavlid DQ will be driven to high, when set to 0, the invalid DQ will be driven to low for slice 1."]
    #[inline(always)]
    pub fn phy_dq_idle_1(&self) -> PhyDqIdle1R {
        PhyDqIdle1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
When set to 1, the invalid DQs will be driven by the dfi_wrdata to make sure the tpda_s and tpda_h's timing is meet for slice 1."]
    #[inline(always)]
    pub fn phy_pda_mode_en_1(&self) -> PhyPdaModeEn1R {
        PhyPdaModeEn1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:30 - 30:24\\]
PRBS7 start pattern for slice 1."]
    #[inline(always)]
    pub fn phy_prbs_pattern_start_1(&self) -> PhyPrbsPatternStart1R {
        PhyPrbsPatternStart1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
When set to 1, the inavlid DQ will be driven to high, when set to 0, the invalid DQ will be driven to low for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_idle_1(&mut self) -> PhyDqIdle1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec> {
        PhyDqIdle1W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
When set to 1, the invalid DQs will be driven by the dfi_wrdata to make sure the tpda_s and tpda_h's timing is meet for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pda_mode_en_1(
        &mut self,
    ) -> PhyPdaModeEn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec> {
        PhyPdaModeEn1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
PRBS7 start pattern for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_prbs_pattern_start_1(
        &mut self,
    ) -> PhyPrbsPatternStart1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec> {
        PhyPrbsPatternStart1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_267\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_267::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_267::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_267::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_267::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_267 to value 0x0100_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy267Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
