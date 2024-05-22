#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec>;
#[doc = "Field `PHY_IO_PAD_DELAY_TIMING_BYPASS_0` reader - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 0."]
pub type PhyIoPadDelayTimingBypass0R = crate::FieldReader;
#[doc = "Field `PHY_IO_PAD_DELAY_TIMING_BYPASS_0` writer - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 0."]
pub type PhyIoPadDelayTimingBypass0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_BYPASS_0` reader - 17:8\\]
Write DQS bypass mode slave delay setting for slice 0."]
pub type PhyClkWrdqsSlaveDelayBypass0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_BYPASS_0` writer - 17:8\\]
Write DQS bypass mode slave delay setting for slice 0."]
pub type PhyClkWrdqsSlaveDelayBypass0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_BYPASS_0` reader - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
pub type PhyWritePathLatAddBypass0R = crate::FieldReader;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_BYPASS_0` writer - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
pub type PhyWritePathLatAddBypass0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 0."]
    #[inline(always)]
    pub fn phy_io_pad_delay_timing_bypass_0(&self) -> PhyIoPadDelayTimingBypass0R {
        PhyIoPadDelayTimingBypass0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Write DQS bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdqs_slave_delay_bypass_0(&self) -> PhyClkWrdqsSlaveDelayBypass0R {
        PhyClkWrdqsSlaveDelayBypass0R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
    #[inline(always)]
    pub fn phy_write_path_lat_add_bypass_0(&self) -> PhyWritePathLatAddBypass0R {
        PhyWritePathLatAddBypass0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_io_pad_delay_timing_bypass_0(
        &mut self,
    ) -> PhyIoPadDelayTimingBypass0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec> {
        PhyIoPadDelayTimingBypass0W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Write DQS bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdqs_slave_delay_bypass_0(
        &mut self,
    ) -> PhyClkWrdqsSlaveDelayBypass0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec> {
        PhyClkWrdqsSlaveDelayBypass0W::new(self, 8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_path_lat_add_bypass_0(
        &mut self,
    ) -> PhyWritePathLatAddBypass0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec> {
        PhyWritePathLatAddBypass0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1Spec {
    const RESET_VALUE: u32 = 0;
}
