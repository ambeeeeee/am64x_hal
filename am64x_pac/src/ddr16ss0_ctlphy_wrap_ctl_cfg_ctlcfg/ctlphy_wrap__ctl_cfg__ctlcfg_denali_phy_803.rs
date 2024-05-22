#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_803` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_803` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec>;
#[doc = "Field `PHY_ADR2_CLK_WR_SLAVE_DELAY_1` reader - 10:0\\]
CA bit 2 slave delay setting for address slice 1."]
pub type PhyAdr2ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR2_CLK_WR_SLAVE_DELAY_1` writer - 10:0\\]
CA bit 2 slave delay setting for address slice 1."]
pub type PhyAdr2ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR3_SW_WRADDR_SHIFT_1` reader - 20:16\\]
Manual override of CA bit 3 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr3SwWraddrShift1R = crate::FieldReader;
#[doc = "Field `PHY_ADR3_SW_WRADDR_SHIFT_1` writer - 20:16\\]
Manual override of CA bit 3 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr3SwWraddrShift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
CA bit 2 slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr2_clk_wr_slave_delay_1(&self) -> PhyAdr2ClkWrSlaveDelay1R {
        PhyAdr2ClkWrSlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Manual override of CA bit 3 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr3_sw_wraddr_shift_1(&self) -> PhyAdr3SwWraddrShift1R {
        PhyAdr3SwWraddrShift1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
CA bit 2 slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr2_clk_wr_slave_delay_1(
        &mut self,
    ) -> PhyAdr2ClkWrSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec> {
        PhyAdr2ClkWrSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Manual override of CA bit 3 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr3_sw_wraddr_shift_1(
        &mut self,
    ) -> PhyAdr3SwWraddrShift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec> {
        PhyAdr3SwWraddrShift1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_803\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_803::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_803::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_803::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_803::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_803 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy803Spec {
    const RESET_VALUE: u32 = 0;
}
