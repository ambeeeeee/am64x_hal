#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_805` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_805` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec>;
#[doc = "Field `PHY_ADR4_CLK_WR_SLAVE_DELAY_1` reader - 10:0\\]
CA bit 4 slave delay setting for address slice 1."]
pub type PhyAdr4ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR4_CLK_WR_SLAVE_DELAY_1` writer - 10:0\\]
CA bit 4 slave delay setting for address slice 1."]
pub type PhyAdr4ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR5_SW_WRADDR_SHIFT_1` reader - 20:16\\]
Manual override of CA bit 5 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr5SwWraddrShift1R = crate::FieldReader;
#[doc = "Field `PHY_ADR5_SW_WRADDR_SHIFT_1` writer - 20:16\\]
Manual override of CA bit 5 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr5SwWraddrShift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
CA bit 4 slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr4_clk_wr_slave_delay_1(&self) -> PhyAdr4ClkWrSlaveDelay1R {
        PhyAdr4ClkWrSlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Manual override of CA bit 5 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr5_sw_wraddr_shift_1(&self) -> PhyAdr5SwWraddrShift1R {
        PhyAdr5SwWraddrShift1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
CA bit 4 slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr4_clk_wr_slave_delay_1(
        &mut self,
    ) -> PhyAdr4ClkWrSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec> {
        PhyAdr4ClkWrSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Manual override of CA bit 5 of automatic half_cycle_shift/cycle_shift for address slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
is the cycle_shift value. For bits \\[4:3\\], clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr5_sw_wraddr_shift_1(
        &mut self,
    ) -> PhyAdr5SwWraddrShift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec> {
        PhyAdr5SwWraddrShift1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_805\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_805::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_805::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_805::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_805::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_805 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy805Spec {
    const RESET_VALUE: u32 = 0;
}
