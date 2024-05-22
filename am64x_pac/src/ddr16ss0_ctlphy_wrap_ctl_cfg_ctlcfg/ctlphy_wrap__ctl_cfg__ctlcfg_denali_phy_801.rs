#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_801` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_801` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec>;
#[doc = "Field `PHY_ADR0_SW_WRADDR_SHIFT_1` reader - 4:0\\]
Manual override of CA bit 0 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr0SwWraddrShift1R = crate::FieldReader;
#[doc = "Field `PHY_ADR0_SW_WRADDR_SHIFT_1` writer - 4:0\\]
Manual override of CA bit 0 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr0SwWraddrShift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_1` reader - 18:8\\]
CA bit 0 slave delay setting for address slice 1."]
pub type PhyAdr0ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_1` writer - 18:8\\]
CA bit 0 slave delay setting for address slice 1."]
pub type PhyAdr0ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR1_SW_WRADDR_SHIFT_1` reader - 28:24\\]
Manual override of CA bit 1 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr1SwWraddrShift1R = crate::FieldReader;
#[doc = "Field `PHY_ADR1_SW_WRADDR_SHIFT_1` writer - 28:24\\]
Manual override of CA bit 1 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr1SwWraddrShift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Manual override of CA bit 0 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr0_sw_wraddr_shift_1(&self) -> PhyAdr0SwWraddrShift1R {
        PhyAdr0SwWraddrShift1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:18 - 18:8\\]
CA bit 0 slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr0_clk_wr_slave_delay_1(&self) -> PhyAdr0ClkWrSlaveDelay1R {
        PhyAdr0ClkWrSlaveDelay1R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Manual override of CA bit 1 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr1_sw_wraddr_shift_1(&self) -> PhyAdr1SwWraddrShift1R {
        PhyAdr1SwWraddrShift1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Manual override of CA bit 0 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr0_sw_wraddr_shift_1(
        &mut self,
    ) -> PhyAdr0SwWraddrShift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec> {
        PhyAdr0SwWraddrShift1W::new(self, 0)
    }
    #[doc = "Bits 8:18 - 18:8\\]
CA bit 0 slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr0_clk_wr_slave_delay_1(
        &mut self,
    ) -> PhyAdr0ClkWrSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec> {
        PhyAdr0ClkWrSlaveDelay1W::new(self, 8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Manual override of CA bit 1 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr1_sw_wraddr_shift_1(
        &mut self,
    ) -> PhyAdr1SwWraddrShift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec> {
        PhyAdr1SwWraddrShift1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_801\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_801::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_801::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_801::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_801::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_801 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy801Spec {
    const RESET_VALUE: u32 = 0;
}
