#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_120` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_120` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec>;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_0` reader - 3:0\\]
Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyRddqsLatencyAdjust0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_LATENCY_ADJUST_0` writer - 3:0\\]
Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyRddqsLatencyAdjust0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_0` reader - 10:8\\]
Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
pub type PhyWritePathLatAdd0R = crate::FieldReader;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_0` writer - 10:8\\]
Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
pub type PhyWritePathLatAdd0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_0` reader - 25:16\\]
Write level delay threshold above which will be considered in previous cycle for slice 0."]
pub type PhyWrlvlDelayEarlyThreshold0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRLVL_DELAY_EARLY_THRESHOLD_0` writer - 25:16\\]
Write level delay threshold above which will be considered in previous cycle for slice 0."]
pub type PhyWrlvlDelayEarlyThreshold0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_latency_adjust_0(&self) -> PhyRddqsLatencyAdjust0R {
        PhyRddqsLatencyAdjust0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
    #[inline(always)]
    pub fn phy_write_path_lat_add_0(&self) -> PhyWritePathLatAdd0R {
        PhyWritePathLatAdd0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Write level delay threshold above which will be considered in previous cycle for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_delay_early_threshold_0(&self) -> PhyWrlvlDelayEarlyThreshold0R {
        PhyWrlvlDelayEarlyThreshold0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_latency_adjust_0(
        &mut self,
    ) -> PhyRddqsLatencyAdjust0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec> {
        PhyRddqsLatencyAdjust0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Number of cycles to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_path_lat_add_0(
        &mut self,
    ) -> PhyWritePathLatAdd0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec> {
        PhyWritePathLatAdd0W::new(self, 8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Write level delay threshold above which will be considered in previous cycle for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_delay_early_threshold_0(
        &mut self,
    ) -> PhyWrlvlDelayEarlyThreshold0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec> {
        PhyWrlvlDelayEarlyThreshold0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_120::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_120::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_120::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_120::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_120 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy120Spec {
    const RESET_VALUE: u32 = 0;
}
