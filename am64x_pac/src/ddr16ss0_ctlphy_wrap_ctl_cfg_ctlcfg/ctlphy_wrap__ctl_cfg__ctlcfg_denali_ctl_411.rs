#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_411` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_411` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec>;
#[doc = "Field `TDFI_WRCSLAT_F2` reader - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=2"]
pub type TdfiWrcslatF2R = crate::FieldReader;
#[doc = "Field `TDFI_WRCSLAT_F2` writer - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=2"]
pub type TdfiWrcslatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_PHY_WRLAT_F2` reader - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=2"]
pub type TdfiPhyWrlatF2R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_WRLAT_F2` writer - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=2"]
pub type TdfiPhyWrlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLL_RST_DELAY` reader - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstDelayR = crate::FieldReader<u16>;
#[doc = "Field `DLL_RST_DELAY` writer - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
pub type DllRstDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=2"]
    #[inline(always)]
    pub fn tdfi_wrcslat_f2(&self) -> TdfiWrcslatF2R {
        TdfiWrcslatF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=2"]
    #[inline(always)]
    pub fn tdfi_phy_wrlat_f2(&self) -> TdfiPhyWrlatF2R {
        TdfiPhyWrlatF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    pub fn dll_rst_delay(&self) -> DllRstDelayR {
        DllRstDelayR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrcslat_f2(&mut self) -> TdfiWrcslatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec> {
        TdfiWrcslatF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_wrlat_f2(
        &mut self,
    ) -> TdfiPhyWrlatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec> {
        TdfiPhyWrlatF2W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held. If this signal is not being used by the PHY, this parameter may be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn dll_rst_delay(&mut self) -> DllRstDelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec> {
        DllRstDelayW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_411\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_411::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_411::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_411::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_411::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_411 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl411Spec {
    const RESET_VALUE: u32 = 0;
}
