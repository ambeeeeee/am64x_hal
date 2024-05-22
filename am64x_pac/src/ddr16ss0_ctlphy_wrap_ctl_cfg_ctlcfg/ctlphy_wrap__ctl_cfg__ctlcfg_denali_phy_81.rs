#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_81` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_81` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec>;
#[doc = "Field `PHY_PAD_RX_DCD_5_0` reader - 4:0\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd5_0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_5_0` writer - 4:0\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd5_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_6_0` reader - 12:8\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd6_0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_6_0` writer - 12:8\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd6_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_RX_DCD_7_0` reader - 20:16\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd7_0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_RX_DCD_7_0` writer - 20:16\\]
Controls RX_DCD pin for each pad for slice 0."]
pub type PhyPadRxDcd7_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_DM_RX_DCD_0` reader - 28:24\\]
Controls RX_DCD pin for dm pad for slice 0."]
pub type PhyPadDmRxDcd0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_DM_RX_DCD_0` writer - 28:24\\]
Controls RX_DCD pin for dm pad for slice 0."]
pub type PhyPadDmRxDcd0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_5_0(&self) -> PhyPadRxDcd5_0R {
        PhyPadRxDcd5_0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_6_0(&self) -> PhyPadRxDcd6_0R {
        PhyPadRxDcd6_0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_rx_dcd_7_0(&self) -> PhyPadRxDcd7_0R {
        PhyPadRxDcd7_0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for dm pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_dm_rx_dcd_0(&self) -> PhyPadDmRxDcd0R {
        PhyPadDmRxDcd0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_5_0(
        &mut self,
    ) -> PhyPadRxDcd5_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec> {
        PhyPadRxDcd5_0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_6_0(
        &mut self,
    ) -> PhyPadRxDcd6_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec> {
        PhyPadRxDcd6_0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Controls RX_DCD pin for each pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_dcd_7_0(
        &mut self,
    ) -> PhyPadRxDcd7_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec> {
        PhyPadRxDcd7_0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Controls RX_DCD pin for dm pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dm_rx_dcd_0(
        &mut self,
    ) -> PhyPadDmRxDcd0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec> {
        PhyPadDmRxDcd0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_81::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_81::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_81::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_81::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_81 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy81Spec {
    const RESET_VALUE: u32 = 0;
}
