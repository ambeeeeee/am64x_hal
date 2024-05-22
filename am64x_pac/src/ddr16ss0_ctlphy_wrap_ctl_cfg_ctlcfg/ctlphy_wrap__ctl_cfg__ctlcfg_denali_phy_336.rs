#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_336` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_336` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec>;
#[doc = "Field `PHY_PAD_RX_DCD_1_1` reader - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd1_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_1_1` writer - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd1_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_2_1` reader - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd2_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_2_1` writer - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd2_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_3_1` reader - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd3_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_3_1` writer - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd3_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_4_1` reader - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd4_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_4_1` writer - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd4_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_1_1(&self) -> PhyPadRxDcd1_1R {
        PhyPadRxDcd1_1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_2_1(&self) -> PhyPadRxDcd2_1R {
        PhyPadRxDcd2_1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_3_1(&self) -> PhyPadRxDcd3_1R {
        PhyPadRxDcd3_1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_4_1(&self) -> PhyPadRxDcd4_1R {
        PhyPadRxDcd4_1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_1_1(
        &mut self,
    ) -> PhyPadRxDcd1_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec> {
        PhyPadRxDcd1_1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_2_1(
        &mut self,
    ) -> PhyPadRxDcd2_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec> {
        PhyPadRxDcd2_1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_3_1(
        &mut self,
    ) -> PhyPadRxDcd3_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec> {
        PhyPadRxDcd3_1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_4_1(
        &mut self,
    ) -> PhyPadRxDcd4_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec> {
        PhyPadRxDcd4_1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_336\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_336::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_336::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_336::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_336::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_336 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy336Spec {
    const RESET_VALUE: u32 = 0;
}
