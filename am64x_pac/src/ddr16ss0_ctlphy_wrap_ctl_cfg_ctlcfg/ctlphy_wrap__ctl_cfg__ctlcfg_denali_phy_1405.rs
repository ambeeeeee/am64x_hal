#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1405` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1405` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec>;
#[doc = "Field `PHY_CAL_CLK_SELECT_0` reader - 2:0\\]
Pad calibration pad clock frequency select setting for block 0."]
pub type PhyCalClkSelect0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_CLK_SELECT_0` writer - 2:0\\]
Pad calibration pad clock frequency select setting for block 0."]
pub type PhyCalClkSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_CAL_VREF_SWITCH_TIMER_0` reader - 23:8\\]
The settling time for a switch in VREF during IO pad calibration."]
pub type PhyCalVrefSwitchTimer0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CAL_VREF_SWITCH_TIMER_0` writer - 23:8\\]
The settling time for a switch in VREF during IO pad calibration."]
pub type PhyCalVrefSwitchTimer0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_CAL_SETTLING_PRD_0` reader - 30:24\\]
Number of clock cycles to extend dfi_phyupd_req after the ack is received for settling of final values"]
pub type PhyCalSettlingPrd0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_SETTLING_PRD_0` writer - 30:24\\]
Number of clock cycles to extend dfi_phyupd_req after the ack is received for settling of final values"]
pub type PhyCalSettlingPrd0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Pad calibration pad clock frequency select setting for block 0."]
    #[inline(always)]
    pub fn phy_cal_clk_select_0(&self) -> PhyCalClkSelect0R {
        PhyCalClkSelect0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
The settling time for a switch in VREF during IO pad calibration."]
    #[inline(always)]
    pub fn phy_cal_vref_switch_timer_0(&self) -> PhyCalVrefSwitchTimer0R {
        PhyCalVrefSwitchTimer0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Number of clock cycles to extend dfi_phyupd_req after the ack is received for settling of final values"]
    #[inline(always)]
    pub fn phy_cal_settling_prd_0(&self) -> PhyCalSettlingPrd0R {
        PhyCalSettlingPrd0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Pad calibration pad clock frequency select setting for block 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_clk_select_0(
        &mut self,
    ) -> PhyCalClkSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec> {
        PhyCalClkSelect0W::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
The settling time for a switch in VREF during IO pad calibration."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_vref_switch_timer_0(
        &mut self,
    ) -> PhyCalVrefSwitchTimer0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec> {
        PhyCalVrefSwitchTimer0W::new(self, 8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Number of clock cycles to extend dfi_phyupd_req after the ack is received for settling of final values"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_settling_prd_0(
        &mut self,
    ) -> PhyCalSettlingPrd0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec> {
        PhyCalSettlingPrd0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1405\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1405::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1405::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1405::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1405::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1405 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1405Spec {
    const RESET_VALUE: u32 = 0;
}
