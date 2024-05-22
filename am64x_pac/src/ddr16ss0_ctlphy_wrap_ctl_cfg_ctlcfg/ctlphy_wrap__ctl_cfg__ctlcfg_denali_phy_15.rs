#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_15` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_15` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec>;
#[doc = "Field `PHY_LPDDR_0` reader - 0:0\\]
Adds a cycle of delay for the slice 0 to match the address slice. Set to 1 to add a cycle"]
pub type PhyLpddr0R = crate::BitReader;
#[doc = "Field `PHY_LPDDR_0` writer - 0:0\\]
Adds a cycle of delay for the slice 0 to match the address slice. Set to 1 to add a cycle"]
pub type PhyLpddr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MEM_CLASS_0` reader - 10:8\\]
Indicates the type of DRAM for slice 0. 0 for DDR3, 1 for DDR4, 2 for DDR5, 4 for LPDDR2, 5 for LPDDR3. 6 for LPDDR4"]
pub type PhyMemClass0R = crate::FieldReader;
#[doc = "Field `PHY_MEM_CLASS_0` writer - 10:8\\]
Indicates the type of DRAM for slice 0. 0 for DDR3, 1 for DDR4, 2 for DDR5, 4 for LPDDR2, 5 for LPDDR3. 6 for LPDDR4"]
pub type PhyMemClass0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_GATE_SMPL2_SLAVE_DELAY_0` reader - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate2 signal for on-the-fly read DQS training for slice 0."]
pub type PhyGateSmpl2SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL2_SLAVE_DELAY_0` writer - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate2 signal for on-the-fly read DQS training for slice 0."]
pub type PhyGateSmpl2SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Adds a cycle of delay for the slice 0 to match the address slice. Set to 1 to add a cycle"]
    #[inline(always)]
    pub fn phy_lpddr_0(&self) -> PhyLpddr0R {
        PhyLpddr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Indicates the type of DRAM for slice 0. 0 for DDR3, 1 for DDR4, 2 for DDR5, 4 for LPDDR2, 5 for LPDDR3. 6 for LPDDR4"]
    #[inline(always)]
    pub fn phy_mem_class_0(&self) -> PhyMemClass0R {
        PhyMemClass0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate2 signal for on-the-fly read DQS training for slice 0."]
    #[inline(always)]
    pub fn phy_gate_smpl2_slave_delay_0(&self) -> PhyGateSmpl2SlaveDelay0R {
        PhyGateSmpl2SlaveDelay0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Adds a cycle of delay for the slice 0 to match the address slice. Set to 1 to add a cycle"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr_0(&mut self) -> PhyLpddr0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec> {
        PhyLpddr0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Indicates the type of DRAM for slice 0. 0 for DDR3, 1 for DDR4, 2 for DDR5, 4 for LPDDR2, 5 for LPDDR3. 6 for LPDDR4"]
    #[inline(always)]
    #[must_use]
    pub fn phy_mem_class_0(&mut self) -> PhyMemClass0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec> {
        PhyMemClass0W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate2 signal for on-the-fly read DQS training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_smpl2_slave_delay_0(
        &mut self,
    ) -> PhyGateSmpl2SlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec> {
        PhyGateSmpl2SlaveDelay0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_15::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_15::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_15 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy15Spec {
    const RESET_VALUE: u32 = 0;
}
