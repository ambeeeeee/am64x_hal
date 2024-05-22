#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1054` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1054` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec>;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_2` reader - 5:0\\]
Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in CA training."]
pub type PhyAdrCalvlTrainMask2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_TRAIN_MASK_2` writer - 5:0\\]
Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in CA training."]
pub type PhyAdrCalvlTrainMask2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_CSLVL_TRAIN_MASK_2` reader - 13:8\\]
Mask bit for CS training participation for address slice 2. Set to 1 to indicate that the bit is participating in CS training."]
pub type PhyAdrCslvlTrainMask2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CSLVL_TRAIN_MASK_2` writer - 13:8\\]
Mask bit for CS training participation for address slice 2. Set to 1 to indicate that the bit is participating in CS training."]
pub type PhyAdrCslvlTrainMask2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_STATIC_TOG_DISABLE_2` reader - 19:16\\]
Toggle control during static activity for address slice 2. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
pub type PhyAdrStaticTogDisable2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_STATIC_TOG_DISABLE_2` writer - 19:16\\]
Toggle control during static activity for address slice 2. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
pub type PhyAdrStaticTogDisable2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_SW_TXIO_CTRL_2` reader - 29:24\\]
Controls address pad output enable for address slice 2. Set to 1 to disable output enable."]
pub type PhyAdrSwTxioCtrl2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SW_TXIO_CTRL_2` writer - 29:24\\]
Controls address pad output enable for address slice 2. Set to 1 to disable output enable."]
pub type PhyAdrSwTxioCtrl2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in CA training."]
    #[inline(always)]
    pub fn phy_adr_calvl_train_mask_2(&self) -> PhyAdrCalvlTrainMask2R {
        PhyAdrCalvlTrainMask2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Mask bit for CS training participation for address slice 2. Set to 1 to indicate that the bit is participating in CS training."]
    #[inline(always)]
    pub fn phy_adr_cslvl_train_mask_2(&self) -> PhyAdrCslvlTrainMask2R {
        PhyAdrCslvlTrainMask2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Toggle control during static activity for address slice 2. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
    #[inline(always)]
    pub fn phy_adr_static_tog_disable_2(&self) -> PhyAdrStaticTogDisable2R {
        PhyAdrStaticTogDisable2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Controls address pad output enable for address slice 2. Set to 1 to disable output enable."]
    #[inline(always)]
    pub fn phy_adr_sw_txio_ctrl_2(&self) -> PhyAdrSwTxioCtrl2R {
        PhyAdrSwTxioCtrl2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Mask bit for CA training participation for address slice 2. Set to 1 to indicate that the bit is participating in CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_train_mask_2(
        &mut self,
    ) -> PhyAdrCalvlTrainMask2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec> {
        PhyAdrCalvlTrainMask2W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Mask bit for CS training participation for address slice 2. Set to 1 to indicate that the bit is participating in CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_cslvl_train_mask_2(
        &mut self,
    ) -> PhyAdrCslvlTrainMask2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec> {
        PhyAdrCslvlTrainMask2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Toggle control during static activity for address slice 2. Set bit to dsiable toggling, bit0: Write path delay line, bit1: Read path delay line, bit2: Read data path, bit3: clk_phy, bit4: master delay line."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_static_tog_disable_2(
        &mut self,
    ) -> PhyAdrStaticTogDisable2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec> {
        PhyAdrStaticTogDisable2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Controls address pad output enable for address slice 2. Set to 1 to disable output enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_txio_ctrl_2(
        &mut self,
    ) -> PhyAdrSwTxioCtrl2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec> {
        PhyAdrSwTxioCtrl2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1054\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1054::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1054::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1054::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1054::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1054 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1054Spec {
    const RESET_VALUE: u32 = 0;
}
