#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_270` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_270` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec>;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_1` reader - 3:0\\]
Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
pub type PhyGateErrorDelaySelect1R = crate::FieldReader;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_1` writer - 3:0\\]
Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
pub type PhyGateErrorDelaySelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SC_PHY_SNAP_OBS_REGS_1` reader - 8:8\\]
Initiates a snapshot of the internal observation registers for slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhySnapObsRegs1R = crate::BitReader;
#[doc = "Field `SC_PHY_SNAP_OBS_REGS_1` writer - 8:8\\]
Initiates a snapshot of the internal observation registers for slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhySnapObsRegs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_1` reader - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate1 signal for on-the-fly read DQS training for slice 1."]
pub type PhyGateSmpl1SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL1_SLAVE_DELAY_1` writer - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate1 signal for on-the-fly read DQS training for slice 1."]
pub type PhyGateSmpl1SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
    #[inline(always)]
    pub fn phy_gate_error_delay_select_1(&self) -> PhyGateErrorDelaySelect1R {
        PhyGateErrorDelaySelect1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiates a snapshot of the internal observation registers for slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_snap_obs_regs_1(&self) -> ScPhySnapObsRegs1R {
        ScPhySnapObsRegs1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate1 signal for on-the-fly read DQS training for slice 1."]
    #[inline(always)]
    pub fn phy_gate_smpl1_slave_delay_1(&self) -> PhyGateSmpl1SlaveDelay1R {
        PhyGateSmpl1SlaveDelay1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_error_delay_select_1(
        &mut self,
    ) -> PhyGateErrorDelaySelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec> {
        PhyGateErrorDelaySelect1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiates a snapshot of the internal observation registers for slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_snap_obs_regs_1(
        &mut self,
    ) -> ScPhySnapObsRegs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec> {
        ScPhySnapObsRegs1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Number of cycles to delay the read DQS gate signal to generate gate1 signal for on-the-fly read DQS training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_smpl1_slave_delay_1(
        &mut self,
    ) -> PhyGateSmpl1SlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec> {
        PhyGateSmpl1SlaveDelay1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_270\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_270::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_270::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_270::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_270::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_270 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy270Spec {
    const RESET_VALUE: u32 = 0;
}
