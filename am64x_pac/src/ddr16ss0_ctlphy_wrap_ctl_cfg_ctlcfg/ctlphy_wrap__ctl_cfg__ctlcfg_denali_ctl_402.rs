#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_402` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_402` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec>;
#[doc = "Field `TDFI_WRCSLAT_F1` reader - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=1"]
pub type TdfiWrcslatF1R = crate::FieldReader;
#[doc = "Field `TDFI_WRCSLAT_F1` writer - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=1"]
pub type TdfiWrcslatF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_PHY_WRLAT_F1` reader - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=1"]
pub type TdfiPhyWrlatF1R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_WRLAT_F1` writer - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=1"]
pub type TdfiPhyWrlatF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_PHY_RDLAT_F2` reader - 23:16\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=2"]
pub type TdfiPhyRdlatF2R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_RDLAT_F2` writer - 23:16\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=2"]
pub type TdfiPhyRdlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=1"]
    #[inline(always)]
    pub fn tdfi_wrcslat_f1(&self) -> TdfiWrcslatF1R {
        TdfiWrcslatF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=1"]
    #[inline(always)]
    pub fn tdfi_phy_wrlat_f1(&self) -> TdfiPhyWrlatF1R {
        TdfiPhyWrlatF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=2"]
    #[inline(always)]
    pub fn tdfi_phy_rdlat_f2(&self) -> TdfiPhyRdlatF2R {
        TdfiPhyRdlatF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DFI tPHY_WRCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a write command and a dfi_wrdata_cs_n assertion. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrcslat_f1(&mut self) -> TdfiWrcslatF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec> {
        TdfiWrcslatF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DFI tPHY_WRLAT timing parameter. This is the number of DFI data phases between a write command and the first assertion of dfi_wrdata_en_pN. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_wrlat_f1(
        &mut self,
    ) -> TdfiPhyWrlatF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec> {
        TdfiPhyWrlatF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_rdlat_f2(
        &mut self,
    ) -> TdfiPhyRdlatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec> {
        TdfiPhyRdlatF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_402\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_402::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_402::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_402::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_402::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_402 to value 0x0006_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl402Spec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
