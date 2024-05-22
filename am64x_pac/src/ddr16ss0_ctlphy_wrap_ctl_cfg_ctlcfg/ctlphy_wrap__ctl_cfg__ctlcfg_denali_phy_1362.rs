#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1362` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1362` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec>;
#[doc = "Field `PHY_DDL_AC_ENABLE` reader - 31:0\\]
PHY Address/Control DDL BIST mode enable."]
pub type PhyDdlAcEnableR = crate::FieldReader<u32>;
#[doc = "Field `PHY_DDL_AC_ENABLE` writer - 31:0\\]
PHY Address/Control DDL BIST mode enable."]
pub type PhyDdlAcEnableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
PHY Address/Control DDL BIST mode enable."]
    #[inline(always)]
    pub fn phy_ddl_ac_enable(&self) -> PhyDdlAcEnableR {
        PhyDdlAcEnableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
PHY Address/Control DDL BIST mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_ac_enable(
        &mut self,
    ) -> PhyDdlAcEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec> {
        PhyDdlAcEnableW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1362\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1362::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1362::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1362::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1362::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1362 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1362Spec {
    const RESET_VALUE: u32 = 0;
}
