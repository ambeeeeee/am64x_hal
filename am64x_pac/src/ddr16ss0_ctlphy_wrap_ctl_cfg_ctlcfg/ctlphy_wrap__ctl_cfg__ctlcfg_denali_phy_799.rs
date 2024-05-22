#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_799` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_799` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec>;
#[doc = "Field `PHY_ADR_SW_TXPWR_CTRL_1` reader - 5:0\\]
Disable address output enables in deep sleep mode for address slice 1."]
pub type PhyAdrSwTxpwrCtrl1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SW_TXPWR_CTRL_1` writer - 5:0\\]
Disable address output enables in deep sleep mode for address slice 1."]
pub type PhyAdrSwTxpwrCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Disable address output enables in deep sleep mode for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_sw_txpwr_ctrl_1(&self) -> PhyAdrSwTxpwrCtrl1R {
        PhyAdrSwTxpwrCtrl1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Disable address output enables in deep sleep mode for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_txpwr_ctrl_1(
        &mut self,
    ) -> PhyAdrSwTxpwrCtrl1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec> {
        PhyAdrSwTxpwrCtrl1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_799\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_799::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_799::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_799::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_799::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_799 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy799Spec {
    const RESET_VALUE: u32 = 0;
}
