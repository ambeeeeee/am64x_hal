#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1374` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1374` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec>;
#[doc = "Field `PHY_PLL_CTRL` reader - 12:0\\]
PHY clock PLL controls."]
pub type PhyPllCtrlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PLL_CTRL` writer - 12:0\\]
PHY clock PLL controls."]
pub type PhyPllCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `PHY_LOW_FREQ_SEL` reader - 16:16\\]
Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
pub type PhyLowFreqSelR = crate::BitReader;
#[doc = "Field `PHY_LOW_FREQ_SEL` writer - 16:16\\]
Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
pub type PhyLowFreqSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
PHY clock PLL controls."]
    #[inline(always)]
    pub fn phy_pll_ctrl(&self) -> PhyPllCtrlR {
        PhyPllCtrlR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
    #[inline(always)]
    pub fn phy_low_freq_sel(&self) -> PhyLowFreqSelR {
        PhyLowFreqSelR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
PHY clock PLL controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_ctrl(&mut self) -> PhyPllCtrlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec> {
        PhyPllCtrlW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables the PHY to enter/exit the PLL domain from the negative clock edge. Set to 1 at low frequencies to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_low_freq_sel(
        &mut self,
    ) -> PhyLowFreqSelW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec> {
        PhyLowFreqSelW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1374\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1374::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1374::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1374::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1374::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1374 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1374Spec {
    const RESET_VALUE: u32 = 0;
}
