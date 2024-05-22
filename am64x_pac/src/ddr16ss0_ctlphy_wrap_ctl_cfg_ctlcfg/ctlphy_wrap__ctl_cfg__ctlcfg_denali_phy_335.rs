#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_335` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_335` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec>;
#[doc = "Field `PHY_WDQLVL_PER_START_OFFSET_1` reader - 5:0\\]
Peridic training start point offset for slice 1."]
pub type PhyWdqlvlPerStartOffset1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_PER_START_OFFSET_1` writer - 5:0\\]
Peridic training start point offset for slice 1."]
pub type PhyWdqlvlPerStartOffset1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_FAST_LVL_EN_1` reader - 11:8\\]
Enable for fast multi-pattern window search for slice 1."]
pub type PhyFastLvlEn1R = crate::FieldReader;
#[doc = "Field `PHY_FAST_LVL_EN_1` writer - 11:8\\]
Enable for fast multi-pattern window search for slice 1."]
pub type PhyFastLvlEn1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_PAD_TX_DCD_1` reader - 20:16\\]
Controls TX_DCD pin for each pad for slice 1."]
pub type PhyPadTxDcd1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_TX_DCD_1` writer - 20:16\\]
Controls TX_DCD pin for each pad for slice 1."]
pub type PhyPadTxDcd1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_0_1` reader - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd0_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_0_1` writer - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd0_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Peridic training start point offset for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_per_start_offset_1(&self) -> PhyWdqlvlPerStartOffset1R {
        PhyWdqlvlPerStartOffset1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enable for fast multi-pattern window search for slice 1."]
    #[inline(always)]
    pub fn phy_fast_lvl_en_1(&self) -> PhyFastLvlEn1R {
        PhyFastLvlEn1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls TX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_tx_dcd_1(&self) -> PhyPadTxDcd1R {
        PhyPadTxDcd1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_0_1(&self) -> PhyPadRxDcd0_1R {
        PhyPadRxDcd0_1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Peridic training start point offset for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_per_start_offset_1(
        &mut self,
    ) -> PhyWdqlvlPerStartOffset1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec> {
        PhyWdqlvlPerStartOffset1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enable for fast multi-pattern window search for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fast_lvl_en_1(
        &mut self,
    ) -> PhyFastLvlEn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec> {
        PhyFastLvlEn1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls TX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_tx_dcd_1(&mut self) -> PhyPadTxDcd1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec> {
        PhyPadTxDcd1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_0_1(
        &mut self,
    ) -> PhyPadRxDcd0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec> {
        PhyPadRxDcd0_1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_335\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_335::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_335::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_335::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_335::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_335 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy335Spec {
    const RESET_VALUE: u32 = 0;
}
