#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_796` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_796` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec>;
#[doc = "Field `PHY_ADR_ADDR_SEL_1` reader - 29:0\\]
Selects which DFI address pins connect to which CA pins for LPDDR3/4 for address slice 1."]
pub type PhyAdrAddrSel1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_ADDR_SEL_1` writer - 29:0\\]
Selects which DFI address pins connect to which CA pins for LPDDR3/4 for address slice 1."]
pub type PhyAdrAddrSel1W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 29:0\\]
Selects which DFI address pins connect to which CA pins for LPDDR3/4 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_addr_sel_1(&self) -> PhyAdrAddrSel1R {
        PhyAdrAddrSel1R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 29:0\\]
Selects which DFI address pins connect to which CA pins for LPDDR3/4 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_addr_sel_1(
        &mut self,
    ) -> PhyAdrAddrSel1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec> {
        PhyAdrAddrSel1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_796\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_796::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_796::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_796::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_796::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_796 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy796Spec {
    const RESET_VALUE: u32 = 0;
}
