#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_331` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_331` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec>;
#[doc = "Field `PHY_DQS_TSEL_SELECT_1` reader - 15:0\\]
Operation type tsel select values for DQS signals for slice 1."]
pub type PhyDqsTselSelect1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_DQS_TSEL_SELECT_1` writer - 15:0\\]
Operation type tsel select values for DQS signals for slice 1."]
pub type PhyDqsTselSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_TWO_CYC_PREAMBLE_1` reader - 17:16\\]
2 cycle preamble support for slice 1. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
pub type PhyTwoCycPreamble1R = crate::FieldReader;
#[doc = "Field `PHY_TWO_CYC_PREAMBLE_1` writer - 17:16\\]
2 cycle preamble support for slice 1. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
pub type PhyTwoCycPreamble1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_VREF_INITIAL_START_POINT_1` reader - 30:24\\]
Data slice initial VREF training start value for slice 1."]
pub type PhyVrefInitialStartPoint1R = crate::FieldReader;
#[doc = "Field `PHY_VREF_INITIAL_START_POINT_1` writer - 30:24\\]
Data slice initial VREF training start value for slice 1."]
pub type PhyVrefInitialStartPoint1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Operation type tsel select values for DQS signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_tsel_select_1(&self) -> PhyDqsTselSelect1R {
        PhyDqsTselSelect1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
2 cycle preamble support for slice 1. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_two_cyc_preamble_1(&self) -> PhyTwoCycPreamble1R {
        PhyTwoCycPreamble1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice initial VREF training start value for slice 1."]
    #[inline(always)]
    pub fn phy_vref_initial_start_point_1(&self) -> PhyVrefInitialStartPoint1R {
        PhyVrefInitialStartPoint1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Operation type tsel select values for DQS signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_select_1(
        &mut self,
    ) -> PhyDqsTselSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec> {
        PhyDqsTselSelect1W::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
2 cycle preamble support for slice 1. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_two_cyc_preamble_1(
        &mut self,
    ) -> PhyTwoCycPreamble1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec> {
        PhyTwoCycPreamble1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice initial VREF training start value for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_initial_start_point_1(
        &mut self,
    ) -> PhyVrefInitialStartPoint1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec> {
        PhyVrefInitialStartPoint1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_331\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_331::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_331::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_331::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_331::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_331 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy331Spec {
    const RESET_VALUE: u32 = 0;
}
