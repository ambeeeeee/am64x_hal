#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_74` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_74` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec>;
#[doc = "Field `PHY_DQ_TSEL_ENABLE_0` reader - 2:0\\]
Operation type tsel enables for DQ/DM signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqTselEnable0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_ENABLE_0` writer - 2:0\\]
Operation type tsel enables for DQ/DM signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqTselEnable0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_DQ_TSEL_SELECT_0` reader - 23:8\\]
Operation type tsel select values for DQ/DM signals for slice 0."]
pub type PhyDqTselSelect0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_DQ_TSEL_SELECT_0` writer - 23:8\\]
Operation type tsel select values for DQ/DM signals for slice 0."]
pub type PhyDqTselSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_DQS_TSEL_ENABLE_0` reader - 26:24\\]
Operation type tsel enables for DQS signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqsTselEnable0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_ENABLE_0` writer - 26:24\\]
Operation type tsel enables for DQS signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqsTselEnable0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Operation type tsel enables for DQ/DM signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_dq_tsel_enable_0(&self) -> PhyDqTselEnable0R {
        PhyDqTselEnable0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Operation type tsel select values for DQ/DM signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_tsel_select_0(&self) -> PhyDqTselSelect0R {
        PhyDqTselSelect0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Operation type tsel enables for DQS signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_dqs_tsel_enable_0(&self) -> PhyDqsTselEnable0R {
        PhyDqsTselEnable0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Operation type tsel enables for DQ/DM signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_enable_0(
        &mut self,
    ) -> PhyDqTselEnable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec> {
        PhyDqTselEnable0W::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Operation type tsel select values for DQ/DM signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_select_0(
        &mut self,
    ) -> PhyDqTselSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec> {
        PhyDqTselSelect0W::new(self, 8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Operation type tsel enables for DQS signals for slice 0. Bit \\[0\\]
enables tsel_en during read cycles. Bit \\[1\\]
enables tsel_en during write cycles. Bit \\[2\\]
enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_enable_0(
        &mut self,
    ) -> PhyDqsTselEnable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec> {
        PhyDqsTselEnable0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_74::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_74::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_74::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_74::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_74 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy74Spec {
    const RESET_VALUE: u32 = 0;
}
