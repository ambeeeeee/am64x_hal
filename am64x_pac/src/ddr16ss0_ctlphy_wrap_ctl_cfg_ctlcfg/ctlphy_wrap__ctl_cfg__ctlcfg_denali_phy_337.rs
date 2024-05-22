#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_337` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_337` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec>;
#[doc = "Field `PHY_PAD_RX_DCD_5_1` reader - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd5_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_5_1` writer - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd5_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_6_1` reader - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd6_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_6_1` writer - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd6_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_7_1` reader - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd7_1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_7_1` writer - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
pub type PhyPadRxDcd7_1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_DM_RX_DCD_1` reader - 28:24\\]
Controls RX_DCD pin for dm pad for slice 1."]
pub type PhyPadDmRxDcd1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_DM_RX_DCD_1` writer - 28:24\\]
Controls RX_DCD pin for dm pad for slice 1."]
pub type PhyPadDmRxDcd1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_5_1(&self) -> PhyPadRxDcd5_1R {
        PhyPadRxDcd5_1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_6_1(&self) -> PhyPadRxDcd6_1R {
        PhyPadRxDcd6_1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_7_1(&self) -> PhyPadRxDcd7_1R {
        PhyPadRxDcd7_1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for dm pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_dm_rx_dcd_1(&self) -> PhyPadDmRxDcd1R {
        PhyPadDmRxDcd1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_5_1(
        &mut self,
    ) -> PhyPadRxDcd5_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec> {
        PhyPadRxDcd5_1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_6_1(
        &mut self,
    ) -> PhyPadRxDcd6_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec> {
        PhyPadRxDcd6_1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls RX_DCD pin for each pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_7_1(
        &mut self,
    ) -> PhyPadRxDcd7_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec> {
        PhyPadRxDcd7_1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for dm pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dm_rx_dcd_1(
        &mut self,
    ) -> PhyPadDmRxDcd1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec> {
        PhyPadDmRxDcd1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_337\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_337::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_337::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_337::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_337::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_337 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy337Spec {
    const RESET_VALUE: u32 = 0;
}
