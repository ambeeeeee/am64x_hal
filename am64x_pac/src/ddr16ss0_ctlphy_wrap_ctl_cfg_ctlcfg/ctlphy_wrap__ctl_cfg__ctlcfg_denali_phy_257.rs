#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_257` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_257` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec>;
#[doc = "Field `PHY_IO_PAD_DELAY_TIMING_BYPASS_1` reader - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 1."]
pub type PhyIoPadDelayTimingBypass1R = crate::FieldReader;
#[doc = "Field `PHY_IO_PAD_DELAY_TIMING_BYPASS_1` writer - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 1."]
pub type PhyIoPadDelayTimingBypass1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_BYPASS_1` reader - 17:8\\]
Write DQS bypass mode slave delay setting for slice 1."]
pub type PhyClkWrdqsSlaveDelayBypass1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_BYPASS_1` writer - 17:8\\]
Write DQS bypass mode slave delay setting for slice 1."]
pub type PhyClkWrdqsSlaveDelayBypass1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_BYPASS_1` reader - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
pub type PhyWritePathLatAddBypass1R = crate::FieldReader;
#[doc = "Field `PHY_WRITE_PATH_LAT_ADD_BYPASS_1` writer - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
pub type PhyWritePathLatAddBypass1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 1."]
    #[inline(always)]
    pub fn phy_io_pad_delay_timing_bypass_1(&self) -> PhyIoPadDelayTimingBypass1R {
        PhyIoPadDelayTimingBypass1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Write DQS bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdqs_slave_delay_bypass_1(&self) -> PhyClkWrdqsSlaveDelayBypass1R {
        PhyClkWrdqsSlaveDelayBypass1R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
    #[inline(always)]
    pub fn phy_write_path_lat_add_bypass_1(&self) -> PhyWritePathLatAddBypass1R {
        PhyWritePathLatAddBypass1R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Feedback pad's OPAD and IPAD delay timing on bypass mode for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_io_pad_delay_timing_bypass_1(
        &mut self,
    ) -> PhyIoPadDelayTimingBypass1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec> {
        PhyIoPadDelayTimingBypass1W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Write DQS bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdqs_slave_delay_bypass_1(
        &mut self,
    ) -> PhyClkWrdqsSlaveDelayBypass1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec> {
        PhyClkWrdqsSlaveDelayBypass1W::new(self, 8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Number of cycles on bypass mode to delay the incoming dfi_wrdata_en/dfi_wrdata signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_path_lat_add_bypass_1(
        &mut self,
    ) -> PhyWritePathLatAddBypass1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec> {
        PhyWritePathLatAddBypass1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_257\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_257::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_257::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_257::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_257::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_257 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy257Spec {
    const RESET_VALUE: u32 = 0;
}
