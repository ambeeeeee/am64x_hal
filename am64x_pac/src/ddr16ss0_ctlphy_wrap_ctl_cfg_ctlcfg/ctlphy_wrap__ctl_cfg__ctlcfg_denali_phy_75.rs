#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_75` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_75` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec>;
#[doc = "Field `PHY_DQS_TSEL_SELECT_0` reader - 15:0\\]
Operation type tsel select values for DQS signals for slice 0."]
pub type PhyDqsTselSelect0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_DQS_TSEL_SELECT_0` writer - 15:0\\]
Operation type tsel select values for DQS signals for slice 0."]
pub type PhyDqsTselSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_TWO_CYC_PREAMBLE_0` reader - 17:16\\]
2 cycle preamble support for slice 0. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
pub type PhyTwoCycPreamble0R = crate::FieldReader;
#[doc = "Field `PHY_TWO_CYC_PREAMBLE_0` writer - 17:16\\]
2 cycle preamble support for slice 0. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
pub type PhyTwoCycPreamble0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_VREF_INITIAL_START_POINT_0` reader - 30:24\\]
Data slice initial VREF training start value for slice 0."]
pub type PhyVrefInitialStartPoint0R = crate::FieldReader;
#[doc = "Field `PHY_VREF_INITIAL_START_POINT_0` writer - 30:24\\]
Data slice initial VREF training start value for slice 0."]
pub type PhyVrefInitialStartPoint0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Operation type tsel select values for DQS signals for slice 0."]
    #[inline(always)]
    pub fn phy_dqs_tsel_select_0(&self) -> PhyDqsTselSelect0R {
        PhyDqsTselSelect0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
2 cycle preamble support for slice 0. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_two_cyc_preamble_0(&self) -> PhyTwoCycPreamble0R {
        PhyTwoCycPreamble0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice initial VREF training start value for slice 0."]
    #[inline(always)]
    pub fn phy_vref_initial_start_point_0(&self) -> PhyVrefInitialStartPoint0R {
        PhyVrefInitialStartPoint0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Operation type tsel select values for DQS signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_select_0(
        &mut self,
    ) -> PhyDqsTselSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec> {
        PhyDqsTselSelect0W::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
2 cycle preamble support for slice 0. Bit \\[0\\]
controls the 2 cycle read preamble. Bit \\[1\\]
controls the 2 cycle write preamble. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_two_cyc_preamble_0(
        &mut self,
    ) -> PhyTwoCycPreamble0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec> {
        PhyTwoCycPreamble0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice initial VREF training start value for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_initial_start_point_0(
        &mut self,
    ) -> PhyVrefInitialStartPoint0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec> {
        PhyVrefInitialStartPoint0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_75::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_75::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_75::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_75::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_75 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy75Spec {
    const RESET_VALUE: u32 = 0;
}
