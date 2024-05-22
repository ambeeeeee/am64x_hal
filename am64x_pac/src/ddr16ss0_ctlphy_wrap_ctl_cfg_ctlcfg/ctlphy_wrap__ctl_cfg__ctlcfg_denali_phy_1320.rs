#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1320` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1320` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec>;
#[doc = "Field `PHY_LS_IDLE_EN` reader - 0:0\\]
Indicates the Reduced Idle Power State is enabled in low power mode."]
pub type PhyLsIdleEnR = crate::BitReader;
#[doc = "Field `PHY_LS_IDLE_EN` writer - 0:0\\]
Indicates the Reduced Idle Power State is enabled in low power mode."]
pub type PhyLsIdleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_CTRLUPD_CNTR_CFG` reader - 17:8\\]
Specifies the number of cycles the PHY takes from light sleep req deassert to ack deassert in low power mode."]
pub type PhyLpCtrlupdCntrCfgR = crate::FieldReader<u16>;
#[doc = "Field `PHY_LP_CTRLUPD_CNTR_CFG` writer - 17:8\\]
Specifies the number of cycles the PHY takes from light sleep req deassert to ack deassert in low power mode."]
pub type PhyLpCtrlupdCntrCfgW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the Reduced Idle Power State is enabled in low power mode."]
    #[inline(always)]
    pub fn phy_ls_idle_en(&self) -> PhyLsIdleEnR {
        PhyLsIdleEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Specifies the number of cycles the PHY takes from light sleep req deassert to ack deassert in low power mode."]
    #[inline(always)]
    pub fn phy_lp_ctrlupd_cntr_cfg(&self) -> PhyLpCtrlupdCntrCfgR {
        PhyLpCtrlupdCntrCfgR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the Reduced Idle Power State is enabled in low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ls_idle_en(&mut self) -> PhyLsIdleEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec> {
        PhyLsIdleEnW::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Specifies the number of cycles the PHY takes from light sleep req deassert to ack deassert in low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_ctrlupd_cntr_cfg(
        &mut self,
    ) -> PhyLpCtrlupdCntrCfgW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec> {
        PhyLpCtrlupdCntrCfgW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1320\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1320::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1320::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1320::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1320::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1320 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1320Spec {
    const RESET_VALUE: u32 = 0;
}
