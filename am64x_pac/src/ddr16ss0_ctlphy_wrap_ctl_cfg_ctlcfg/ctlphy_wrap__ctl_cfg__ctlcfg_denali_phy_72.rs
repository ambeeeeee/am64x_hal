#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_72` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_72` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec>;
#[doc = "Field `PHY_PAD_RX_BIAS_EN_0` reader - 10:0\\]
Controls RX_BIAS_EN pin for each pad for slice 0."]
pub type PhyPadRxBiasEn0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_RX_BIAS_EN_0` writer - 10:0\\]
Controls RX_BIAS_EN pin for each pad for slice 0."]
pub type PhyPadRxBiasEn0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_STATIC_TOG_DISABLE_0` reader - 20:16\\]
Control to disable toggle during static activity for slice 0. bit0: Write path delay line disable; bit1: Read path delay line disable; bit2: Read data path disable; bit3: clk_phy disable; bit4: master delay line disable."]
pub type PhyStaticTogDisable0R = crate::FieldReader;
#[doc = "Field `PHY_STATIC_TOG_DISABLE_0` writer - 20:16\\]
Control to disable toggle during static activity for slice 0. bit0: Write path delay line disable; bit1: Read path delay line disable; bit2: Read data path disable; bit3: clk_phy disable; bit4: master delay line disable."]
pub type PhyStaticTogDisable0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_FDBK_PWR_CTRL_0` reader - 26:24\\]
Shutoff gate feedback IO to reduce power for slice 0."]
pub type PhyFdbkPwrCtrl0R = crate::FieldReader;
#[doc = "Field `PHY_FDBK_PWR_CTRL_0` writer - 26:24\\]
Shutoff gate feedback IO to reduce power for slice 0."]
pub type PhyFdbkPwrCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Controls RX_BIAS_EN pin for each pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_rx_bias_en_0(&self) -> PhyPadRxBiasEn0R {
        PhyPadRxBiasEn0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Control to disable toggle during static activity for slice 0. bit0: Write path delay line disable; bit1: Read path delay line disable; bit2: Read data path disable; bit3: clk_phy disable; bit4: master delay line disable."]
    #[inline(always)]
    pub fn phy_static_tog_disable_0(&self) -> PhyStaticTogDisable0R {
        PhyStaticTogDisable0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Shutoff gate feedback IO to reduce power for slice 0."]
    #[inline(always)]
    pub fn phy_fdbk_pwr_ctrl_0(&self) -> PhyFdbkPwrCtrl0R {
        PhyFdbkPwrCtrl0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Controls RX_BIAS_EN pin for each pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rx_bias_en_0(
        &mut self,
    ) -> PhyPadRxBiasEn0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec> {
        PhyPadRxBiasEn0W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Control to disable toggle during static activity for slice 0. bit0: Write path delay line disable; bit1: Read path delay line disable; bit2: Read data path disable; bit3: clk_phy disable; bit4: master delay line disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_static_tog_disable_0(
        &mut self,
    ) -> PhyStaticTogDisable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec> {
        PhyStaticTogDisable0W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Shutoff gate feedback IO to reduce power for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_fdbk_pwr_ctrl_0(
        &mut self,
    ) -> PhyFdbkPwrCtrl0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec> {
        PhyFdbkPwrCtrl0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_72::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_72::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_72::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_72::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_72 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy72Spec {
    const RESET_VALUE: u32 = 0;
}
