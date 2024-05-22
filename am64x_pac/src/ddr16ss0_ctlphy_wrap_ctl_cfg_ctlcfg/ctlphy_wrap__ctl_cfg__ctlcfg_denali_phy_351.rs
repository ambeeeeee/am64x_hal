#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_351` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_351` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec>;
#[doc = "Field `PHY_MASTER_DELAY_HALF_MEASURE_1` reader - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 1."]
pub type PhyMasterDelayHalfMeasure1R = crate::FieldReader;
#[doc = "Field `PHY_MASTER_DELAY_HALF_MEASURE_1` writer - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 1."]
pub type PhyMasterDelayHalfMeasure1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RPTR_UPDATE_1` reader - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
pub type PhyRptrUpdate1R = crate::FieldReader;
#[doc = "Field `PHY_RPTR_UPDATE_1` writer - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
pub type PhyRptrUpdate1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRLVL_DLY_STEP_1` reader - 23:16\\]
DQS slave delay step size during write leveling for slice 1."]
pub type PhyWrlvlDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_STEP_1` writer - 23:16\\]
DQS slave delay step size during write leveling for slice 1."]
pub type PhyWrlvlDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WRLVL_DLY_FINE_STEP_1` reader - 27:24\\]
DQS slave delay fine step size during write leveling for slice 1."]
pub type PhyWrlvlDlyFineStep1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_DLY_FINE_STEP_1` writer - 27:24\\]
DQS slave delay fine step size during write leveling for slice 1."]
pub type PhyWrlvlDlyFineStep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 1."]
    #[inline(always)]
    pub fn phy_master_delay_half_measure_1(&self) -> PhyMasterDelayHalfMeasure1R {
        PhyMasterDelayHalfMeasure1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
    #[inline(always)]
    pub fn phy_rptr_update_1(&self) -> PhyRptrUpdate1R {
        PhyRptrUpdate1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DQS slave delay step size during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_step_1(&self) -> PhyWrlvlDlyStep1R {
        PhyWrlvlDlyStep1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DQS slave delay fine step size during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_dly_fine_step_1(&self) -> PhyWrlvlDlyFineStep1R {
        PhyWrlvlDlyFineStep1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle in the data slice master for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_half_measure_1(
        &mut self,
    ) -> PhyMasterDelayHalfMeasure1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec> {
        PhyMasterDelayHalfMeasure1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rptr_update_1(
        &mut self,
    ) -> PhyRptrUpdate1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec> {
        PhyRptrUpdate1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DQS slave delay step size during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_step_1(
        &mut self,
    ) -> PhyWrlvlDlyStep1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec> {
        PhyWrlvlDlyStep1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DQS slave delay fine step size during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_dly_fine_step_1(
        &mut self,
    ) -> PhyWrlvlDlyFineStep1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec> {
        PhyWrlvlDlyFineStep1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_351\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_351::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_351::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_351::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_351::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_351 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy351Spec {
    const RESET_VALUE: u32 = 0;
}
