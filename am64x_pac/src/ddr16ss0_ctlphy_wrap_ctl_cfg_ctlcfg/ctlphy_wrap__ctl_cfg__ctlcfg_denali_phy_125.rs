#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_125` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_125` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec>;
#[doc = "Field `PHY_DSLICE_PAD_BOOSTPN_SETTING_0` reader - 15:0\\]
Setting for boost P/N of pad for slice 0."]
pub type PhyDslicePadBoostpnSetting0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_DSLICE_PAD_BOOSTPN_SETTING_0` writer - 15:0\\]
Setting for boost P/N of pad for slice 0."]
pub type PhyDslicePadBoostpnSetting0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_DSLICE_PAD_RX_CTLE_SETTING_0` reader - 21:16\\]
Setting for RX ctle P/N of pad for slice 0."]
pub type PhyDslicePadRxCtleSetting0R = crate::FieldReader;
#[doc = "Field `PHY_DSLICE_PAD_RX_CTLE_SETTING_0` writer - 21:16\\]
Setting for RX ctle P/N of pad for slice 0."]
pub type PhyDslicePadRxCtleSetting0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Setting for boost P/N of pad for slice 0."]
    #[inline(always)]
    pub fn phy_dslice_pad_boostpn_setting_0(&self) -> PhyDslicePadBoostpnSetting0R {
        PhyDslicePadBoostpnSetting0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Setting for RX ctle P/N of pad for slice 0."]
    #[inline(always)]
    pub fn phy_dslice_pad_rx_ctle_setting_0(&self) -> PhyDslicePadRxCtleSetting0R {
        PhyDslicePadRxCtleSetting0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Setting for boost P/N of pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dslice_pad_boostpn_setting_0(
        &mut self,
    ) -> PhyDslicePadBoostpnSetting0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec> {
        PhyDslicePadBoostpnSetting0W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Setting for RX ctle P/N of pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dslice_pad_rx_ctle_setting_0(
        &mut self,
    ) -> PhyDslicePadRxCtleSetting0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec> {
        PhyDslicePadRxCtleSetting0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_125::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_125::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_125 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy125Spec {
    const RESET_VALUE: u32 = 0;
}
