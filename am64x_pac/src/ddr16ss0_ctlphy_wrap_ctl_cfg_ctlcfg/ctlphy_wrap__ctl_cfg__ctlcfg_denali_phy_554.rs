#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_554` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_554` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec>;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_0` reader - 3:0\\]
Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
pub type PhyAdrCalvlCaptureCnt0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_CAPTURE_CNT_0` writer - 3:0\\]
Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
pub type PhyAdrCalvlCaptureCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_MEAS_DLY_STEP_ENABLE_0` reader - 8:8\\]
Enables delay parameter setting using phy_adr_meas_dly_step_value for address slice 0."]
pub type PhyAdrMeasDlyStepEnable0R = crate::BitReader;
#[doc = "Field `PHY_ADR_MEAS_DLY_STEP_ENABLE_0` writer - 8:8\\]
Enables delay parameter setting using phy_adr_meas_dly_step_value for address slice 0."]
pub type PhyAdrMeasDlyStepEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_capture_cnt_0(&self) -> PhyAdrCalvlCaptureCnt0R {
        PhyAdrCalvlCaptureCnt0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables delay parameter setting using phy_adr_meas_dly_step_value for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_meas_dly_step_enable_0(&self) -> PhyAdrMeasDlyStepEnable0R {
        PhyAdrMeasDlyStepEnable0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of samples to take at each ADDR slave delay setting during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_capture_cnt_0(
        &mut self,
    ) -> PhyAdrCalvlCaptureCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec> {
        PhyAdrCalvlCaptureCnt0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables delay parameter setting using phy_adr_meas_dly_step_value for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_meas_dly_step_enable_0(
        &mut self,
    ) -> PhyAdrMeasDlyStepEnable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec> {
        PhyAdrMeasDlyStepEnable0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_554\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_554::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_554::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_554::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_554::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_554 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy554Spec {
    const RESET_VALUE: u32 = 0;
}
