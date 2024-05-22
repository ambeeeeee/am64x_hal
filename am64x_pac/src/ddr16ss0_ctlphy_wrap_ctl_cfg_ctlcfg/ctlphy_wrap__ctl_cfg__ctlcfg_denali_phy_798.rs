#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_798` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_798` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec>;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_1` reader - 5:0\\]
Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in CA training."]
pub type PhyAdrCalvlTrainMask1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_1` writer - 5:0\\]
Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in CA training."]
pub type PhyAdrCalvlTrainMask1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_CSLVL_TRAIN_MASK_1` reader - 13:8\\]
Mask bit for CS training participation for address slice 1. Set to 1 to indicate that the bit is participating in CS training."]
pub type PhyAdrCslvlTrainMask1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CSLVL_TRAIN_MASK_1` writer - 13:8\\]
Mask bit for CS training participation for address slice 1. Set to 1 to indicate that the bit is participating in CS training."]
pub type PhyAdrCslvlTrainMask1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_STATIC_TOG_DISABLE_1` reader - 19:16\\]
Toggle control during static activity for address slice 1. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
pub type PhyAdrStaticTogDisable1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_STATIC_TOG_DISABLE_1` writer - 19:16\\]
Toggle control during static activity for address slice 1. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
pub type PhyAdrStaticTogDisable1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_SW_TXIO_CTRL_1` reader - 29:24\\]
Controls address pad output enable for address slice 1. Set to 1 to disable output enable."]
pub type PhyAdrSwTxioCtrl1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SW_TXIO_CTRL_1` writer - 29:24\\]
Controls address pad output enable for address slice 1. Set to 1 to disable output enable."]
pub type PhyAdrSwTxioCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in CA training."]
    #[inline(always)]
    pub fn phy_adr_calvl_train_mask_1(&self) -> PhyAdrCalvlTrainMask1R {
        PhyAdrCalvlTrainMask1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Mask bit for CS training participation for address slice 1. Set to 1 to indicate that the bit is participating in CS training."]
    #[inline(always)]
    pub fn phy_adr_cslvl_train_mask_1(&self) -> PhyAdrCslvlTrainMask1R {
        PhyAdrCslvlTrainMask1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Toggle control during static activity for address slice 1. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
    #[inline(always)]
    pub fn phy_adr_static_tog_disable_1(&self) -> PhyAdrStaticTogDisable1R {
        PhyAdrStaticTogDisable1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Controls address pad output enable for address slice 1. Set to 1 to disable output enable."]
    #[inline(always)]
    pub fn phy_adr_sw_txio_ctrl_1(&self) -> PhyAdrSwTxioCtrl1R {
        PhyAdrSwTxioCtrl1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Mask bit for CA training participation for address slice 1. Set to 1 to indicate that the bit is participating in CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_train_mask_1(
        &mut self,
    ) -> PhyAdrCalvlTrainMask1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec> {
        PhyAdrCalvlTrainMask1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Mask bit for CS training participation for address slice 1. Set to 1 to indicate that the bit is participating in CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_cslvl_train_mask_1(
        &mut self,
    ) -> PhyAdrCslvlTrainMask1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec> {
        PhyAdrCslvlTrainMask1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Toggle control during static activity for address slice 1. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_static_tog_disable_1(
        &mut self,
    ) -> PhyAdrStaticTogDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec> {
        PhyAdrStaticTogDisable1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Controls address pad output enable for address slice 1. Set to 1 to disable output enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_txio_ctrl_1(
        &mut self,
    ) -> PhyAdrSwTxioCtrl1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec> {
        PhyAdrSwTxioCtrl1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_798\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_798::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_798::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_798::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_798::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_798 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy798Spec {
    const RESET_VALUE: u32 = 0;
}
