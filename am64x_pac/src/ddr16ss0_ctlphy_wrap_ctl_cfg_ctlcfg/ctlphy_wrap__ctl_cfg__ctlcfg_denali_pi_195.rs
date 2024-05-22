#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_195` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_195` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec>;
#[doc = "Field `PI_WRLAT_ADJ_F2` reader - 7:0\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 2."]
pub type PiWrlatAdjF2R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_ADJ_F2` writer - 7:0\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 2."]
pub type PiWrlatAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_PHY_WRDATA_F0` reader - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 0, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_PHY_WRDATA_F0` writer - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 0, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataF0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_TDFI_PHY_WRDATA_F1` reader - 18:16\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 1, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_PHY_WRDATA_F1` writer - 18:16\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 1, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataF1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_TDFI_PHY_WRDATA_F2` reader - 26:24\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 2, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_PHY_WRDATA_F2` writer - 26:24\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 2, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type PiTdfiPhyWrdataF2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 2."]
    #[inline(always)]
    pub fn pi_wrlat_adj_f2(&self) -> PiWrlatAdjF2R {
        PiWrlatAdjF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 0, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    pub fn pi_tdfi_phy_wrdata_f0(&self) -> PiTdfiPhyWrdataF0R {
        PiTdfiPhyWrdataF0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 1, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    pub fn pi_tdfi_phy_wrdata_f1(&self) -> PiTdfiPhyWrdataF1R {
        PiTdfiPhyWrdataF1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 2, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    pub fn pi_tdfi_phy_wrdata_f2(&self) -> PiTdfiPhyWrdataF2R {
        PiTdfiPhyWrdataF2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_adj_f2(&mut self) -> PiWrlatAdjF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec> {
        PiWrlatAdjF2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 0, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phy_wrdata_f0(
        &mut self,
    ) -> PiTdfiPhyWrdataF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec> {
        PiTdfiPhyWrdataF0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 1, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phy_wrdata_f1(
        &mut self,
    ) -> PiTdfiPhyWrdataF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec> {
        PiTdfiPhyWrdataF1W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\]
for frequency set 2, the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phy_wrdata_f2(
        &mut self,
    ) -> PiTdfiPhyWrdataF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec> {
        PiTdfiPhyWrdataF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_195\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_195::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_195::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_195::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_195::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_195 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi195Spec {
    const RESET_VALUE: u32 = 0;
}
