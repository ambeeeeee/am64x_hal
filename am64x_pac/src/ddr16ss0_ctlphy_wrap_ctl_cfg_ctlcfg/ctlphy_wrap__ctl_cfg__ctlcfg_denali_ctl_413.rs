#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_413` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_413` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec>;
#[doc = "Field `TDFI_CTRLUPD_MIN` reader - 15:0\\]
Defines the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
pub type TdfiCtrlupdMinR = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CTRLUPD_MIN` writer - 15:0\\]
Defines the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
pub type TdfiCtrlupdMinW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TDFI_DRAM_CLK_DISABLE` reader - 19:16\\]
Defines the DFI tDRAM_CLK_DISABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
pub type TdfiDramClkDisableR = crate::FieldReader;
#[doc = "Field `TDFI_DRAM_CLK_DISABLE` writer - 19:16\\]
Defines the DFI tDRAM_CLK_DISABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
pub type TdfiDramClkDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_DRAM_CLK_ENABLE` reader - 27:24\\]
Defines the DFI tDRAM_CLK_ENABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
pub type TdfiDramClkEnableR = crate::FieldReader;
#[doc = "Field `TDFI_DRAM_CLK_ENABLE` writer - 27:24\\]
Defines the DFI tDRAM_CLK_ENABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
pub type TdfiDramClkEnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
    #[inline(always)]
    pub fn tdfi_ctrlupd_min(&self) -> TdfiCtrlupdMinR {
        TdfiCtrlupdMinR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tDRAM_CLK_DISABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
    #[inline(always)]
    pub fn tdfi_dram_clk_disable(&self) -> TdfiDramClkDisableR {
        TdfiDramClkDisableR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tDRAM_CLK_ENABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
    #[inline(always)]
    pub fn tdfi_dram_clk_enable(&self) -> TdfiDramClkEnableR {
        TdfiDramClkEnableR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_min(
        &mut self,
    ) -> TdfiCtrlupdMinW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec> {
        TdfiCtrlupdMinW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tDRAM_CLK_DISABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clock_disable assertion and the memory clock disable."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_dram_clk_disable(
        &mut self,
    ) -> TdfiDramClkDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec> {
        TdfiDramClkDisableW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tDRAM_CLK_ENABLE timing parameter \\[in DFI clocks\\], the delay between a dfi_dram_clk_disable de-assertion and the memory clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_dram_clk_enable(
        &mut self,
    ) -> TdfiDramClkEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec> {
        TdfiDramClkEnableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_413\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_413::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_413::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_413::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_413::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_413 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl413Spec {
    const RESET_VALUE: u32 = 0;
}
