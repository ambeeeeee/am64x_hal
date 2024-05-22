#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_95` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_95` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec>;
#[doc = "Field `PHY_MASTER_DELAY_HALF_MEASURE_0` reader - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 0."]
pub type PhyMasterDelayHalfMeasure0R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_HALF_MEASURE_0` writer - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 0."]
pub type PhyMasterDelayHalfMeasure0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RPTR_UPDATE_0` reader - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
pub type PhyRptrUpdate0R = crate::FieldReader;
#[doc = "Field `PHY_RPTR_UPDATE_0` writer - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
pub type PhyRptrUpdate0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRLVL_DLY_STEP_0` reader - 23:16\\]
DQS slave delay step size during write leveling for slice 0."]
pub type PhyWrlvlDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_STEP_0` writer - 23:16\\]
DQS slave delay step size during write leveling for slice 0."]
pub type PhyWrlvlDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WRLVL_DLY_FINE_STEP_0` reader - 27:24\\]
DQS slave delay fine step size during write leveling for slice 0."]
pub type PhyWrlvlDlyFineStep0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_FINE_STEP_0` writer - 27:24\\]
DQS slave delay fine step size during write leveling for slice 0."]
pub type PhyWrlvlDlyFineStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 0."]
    #[inline(always)]
    pub fn phy_master_delay_half_measure_0(&self) -> PhyMasterDelayHalfMeasure0R {
        PhyMasterDelayHalfMeasure0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
    #[inline(always)]
    pub fn phy_rptr_update_0(&self) -> PhyRptrUpdate0R {
        PhyRptrUpdate0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DQS slave delay step size during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_step_0(&self) -> PhyWrlvlDlyStep0R {
        PhyWrlvlDlyStep0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DQS slave delay fine step size during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_fine_step_0(&self) -> PhyWrlvlDlyFineStep0R {
        PhyWrlvlDlyFineStep0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_half_measure_0(
        &mut self,
    ) -> PhyMasterDelayHalfMeasure0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec> {
        PhyMasterDelayHalfMeasure0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rptr_update_0(
        &mut self,
    ) -> PhyRptrUpdate0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec> {
        PhyRptrUpdate0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DQS slave delay step size during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_step_0(
        &mut self,
    ) -> PhyWrlvlDlyStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec> {
        PhyWrlvlDlyStep0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DQS slave delay fine step size during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_fine_step_0(
        &mut self,
    ) -> PhyWrlvlDlyFineStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec> {
        PhyWrlvlDlyFineStep0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_95::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_95::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_95::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_95::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_95 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy95Spec {
    const RESET_VALUE: u32 = 0;
}
