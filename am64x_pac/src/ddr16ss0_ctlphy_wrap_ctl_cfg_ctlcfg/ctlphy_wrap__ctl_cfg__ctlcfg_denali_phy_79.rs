#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_79` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_79` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec>;
#[doc = "Field `PHY_WDQLVL_PER_START_OFFSET_0` reader - 5:0\\]
Peridic training start point offset for slice 0."]
pub type PhyWdqlvlPerStartOffset0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_PER_START_OFFSET_0` writer - 5:0\\]
Peridic training start point offset for slice 0."]
pub type PhyWdqlvlPerStartOffset0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_FAST_LVL_EN_0` reader - 11:8\\]
Enable for fast multi-pattern window search for slice 0."]
pub type PhyFastLvlEn0R = crate::FieldReader;
#[doc = "Field `PHY_FAST_LVL_EN_0` writer - 11:8\\]
Enable for fast multi-pattern window search for slice 0."]
pub type PhyFastLvlEn0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_PAD_TX_DCD_0` reader - 20:16\\]
Controls TX_DCD pin for each pad for slice 0."]
pub type PhyPadTxDcd0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_TX_DCD_0` writer - 20:16\\]
Controls TX_DCD pin for each pad for slice 0."]
pub type PhyPadTxDcd0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_0_0` reader - 28:24\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd0_0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_0_0` writer - 28:24\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd0_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Peridic training start point offset for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_per_start_offset_0(&self) -> PhyWdqlvlPerStartOffset0R {
        PhyWdqlvlPerStartOffset0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enable for fast multi-pattern window search for slice 0."]
    #[inline(always)]
    pub fn phy_fast_lvl_en_0(&self) -> PhyFastLvlEn0R {
        PhyFastLvlEn0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls TX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_tx_dcd_0(&self) -> PhyPadTxDcd0R {
        PhyPadTxDcd0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_0_0(&self) -> PhyPadRxDcd0_0R {
        PhyPadRxDcd0_0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Peridic training start point offset for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_per_start_offset_0(
        &mut self,
    ) -> PhyWdqlvlPerStartOffset0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec> {
        PhyWdqlvlPerStartOffset0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enable for fast multi-pattern window search for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fast_lvl_en_0(&mut self) -> PhyFastLvlEn0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec> {
        PhyFastLvlEn0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls TX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_tx_dcd_0(&mut self) -> PhyPadTxDcd0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec> {
        PhyPadTxDcd0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_0_0(
        &mut self,
    ) -> PhyPadRxDcd0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec> {
        PhyPadRxDcd0_0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_79\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_79::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_79::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_79::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_79::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_79 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy79Spec {
    const RESET_VALUE: u32 = 0;
}
