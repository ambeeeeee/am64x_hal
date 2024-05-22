#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_44` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_44` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec>;
#[doc = "Field `PHY_CALVL_VREF_DRIVING_SLICE_0` reader - 0:0\\]
Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
pub type PhyCalvlVrefDrivingSlice0R = crate::BitReader;
#[doc = "Field `PHY_CALVL_VREF_DRIVING_SLICE_0` writer - 0:0\\]
Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
pub type PhyCalvlVrefDrivingSlice0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_MANUAL_CLEAR_0` reader - 13:8\\]
Manual reset/clear of internal logic for slice 0. Bit \\[0\\]
initiates manual setup of the read DQS gate. Bit \\[1\\]
is reset of read entry FIFO pointers. Bit \\[2\\]
is reset of master delay min/max lock values. Bit \\[3\\]
is manual reset of master delay unlock counter. Bit \\[4\\]
is reset of leveling error bit in the leveling status registers. Bit \\[5\\]
is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset. WRITE-ONLY"]
pub type ScPhyManualClear0R = crate::FieldReader;
#[doc = "Field `SC_PHY_MANUAL_CLEAR_0` writer - 13:8\\]
Manual reset/clear of internal logic for slice 0. Bit \\[0\\]
initiates manual setup of the read DQS gate. Bit \\[1\\]
is reset of read entry FIFO pointers. Bit \\[2\\]
is reset of master delay min/max lock values. Bit \\[3\\]
is manual reset of master delay unlock counter. Bit \\[4\\]
is reset of leveling error bit in the leveling status registers. Bit \\[5\\]
is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset. WRITE-ONLY"]
pub type ScPhyManualClear0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_FIFO_PTR_OBS_0` reader - 23:16\\]
Observation register containing read entry FIFO pointers for slice 0. READ-ONLY"]
pub type PhyFifoPtrObs0R = crate::FieldReader;
#[doc = "Field `PHY_FIFO_PTR_OBS_0` writer - 23:16\\]
Observation register containing read entry FIFO pointers for slice 0. READ-ONLY"]
pub type PhyFifoPtrObs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
    #[inline(always)]
    pub fn phy_calvl_vref_driving_slice_0(&self) -> PhyCalvlVrefDrivingSlice0R {
        PhyCalvlVrefDrivingSlice0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Manual reset/clear of internal logic for slice 0. Bit \\[0\\]
initiates manual setup of the read DQS gate. Bit \\[1\\]
is reset of read entry FIFO pointers. Bit \\[2\\]
is reset of master delay min/max lock values. Bit \\[3\\]
is manual reset of master delay unlock counter. Bit \\[4\\]
is reset of leveling error bit in the leveling status registers. Bit \\[5\\]
is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_manual_clear_0(&self) -> ScPhyManualClear0R {
        ScPhyManualClear0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Observation register containing read entry FIFO pointers for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_0(&self) -> PhyFifoPtrObs0R {
        PhyFifoPtrObs0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates if slice 0 is used to drive the VREF value to the device during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_calvl_vref_driving_slice_0(
        &mut self,
    ) -> PhyCalvlVrefDrivingSlice0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec> {
        PhyCalvlVrefDrivingSlice0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Manual reset/clear of internal logic for slice 0. Bit \\[0\\]
initiates manual setup of the read DQS gate. Bit \\[1\\]
is reset of read entry FIFO pointers. Bit \\[2\\]
is reset of master delay min/max lock values. Bit \\[3\\]
is manual reset of master delay unlock counter. Bit \\[4\\]
is reset of leveling error bit in the leveling status registers. Bit \\[5\\]
is clearing of the gate tracking observation register. Set each bit to 1 to initiate/reset. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_manual_clear_0(
        &mut self,
    ) -> ScPhyManualClear0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec> {
        ScPhyManualClear0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Observation register containing read entry FIFO pointers for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_fifo_ptr_obs_0(
        &mut self,
    ) -> PhyFifoPtrObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec> {
        PhyFifoPtrObs0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_44::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_44::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_44 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy44Spec {
    const RESET_VALUE: u32 = 0;
}
