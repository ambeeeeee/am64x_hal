#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_412` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_412` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec>;
#[doc = "Field `DLL_RST_ADJ_DLY` reader - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstAdjDlyR = crate::FieldReader;
#[doc = "Field `DLL_RST_ADJ_DLY` writer - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstAdjDlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UPDATE_ERROR_STATUS` reader - 14:8\\]
Identifies the source of any DFI MC-initiated or PHY-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
pub type UpdateErrorStatusR = crate::FieldReader;
#[doc = "Field `UPDATE_ERROR_STATUS` writer - 14:8\\]
Identifies the source of any DFI MC-initiated or PHY-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
pub type UpdateErrorStatusW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DRAM_CLK_DISABLE` reader - 17:16\\]
Set value for the dfi_dram_clk_disable signal. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
pub type DramClkDisableR = crate::FieldReader;
#[doc = "Field `DRAM_CLK_DISABLE` writer - 17:16\\]
Set value for the dfi_dram_clk_disable signal. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
pub type DramClkDisableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    pub fn dll_rst_adj_dly(&self) -> DllRstAdjDlyR {
        DllRstAdjDlyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Identifies the source of any DFI MC-initiated or PHY-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
    #[inline(always)]
    pub fn update_error_status(&self) -> UpdateErrorStatusR {
        UpdateErrorStatusR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Set value for the dfi_dram_clk_disable signal. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    pub fn dram_clk_disable(&self) -> DramClkDisableR {
        DramClkDisableR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum cycles after setting master delay in DLL until the DLL reset signal dll_rst_n may be asserted. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn dll_rst_adj_dly(&mut self) -> DllRstAdjDlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec> {
        DllRstAdjDlyW::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Identifies the source of any DFI MC-initiated or PHY-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn update_error_status(
        &mut self,
    ) -> UpdateErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec> {
        UpdateErrorStatusW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Set value for the dfi_dram_clk_disable signal. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn dram_clk_disable(
        &mut self,
    ) -> DramClkDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec> {
        DramClkDisableW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_412\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_412::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_412::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_412::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_412::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_412 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl412Spec {
    const RESET_VALUE: u32 = 0;
}
