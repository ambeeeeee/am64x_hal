#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_194` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_194` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec>;
#[doc = "Field `PI_RDLAT_ADJ_F1` reader - 7:0\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 1."]
pub type PiRdlatAdjF1R = crate::FieldReader;
#[doc = "Field `PI_RDLAT_ADJ_F1` writer - 7:0\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 1."]
pub type PiRdlatAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_RDLAT_ADJ_F2` reader - 15:8\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 2."]
pub type PiRdlatAdjF2R = crate::FieldReader;
#[doc = "Field `PI_RDLAT_ADJ_F2` writer - 15:8\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 2."]
pub type PiRdlatAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_WRLAT_ADJ_F0` reader - 23:16\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 0."]
pub type PiWrlatAdjF0R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_ADJ_F0` writer - 23:16\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 0."]
pub type PiWrlatAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_WRLAT_ADJ_F1` reader - 31:24\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 1."]
pub type PiWrlatAdjF1R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_ADJ_F1` writer - 31:24\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 1."]
pub type PiWrlatAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 1."]
    #[inline(always)]
    pub fn pi_rdlat_adj_f1(&self) -> PiRdlatAdjF1R {
        PiRdlatAdjF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 2."]
    #[inline(always)]
    pub fn pi_rdlat_adj_f2(&self) -> PiRdlatAdjF2R {
        PiRdlatAdjF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 0."]
    #[inline(always)]
    pub fn pi_wrlat_adj_f0(&self) -> PiWrlatAdjF0R {
        PiWrlatAdjF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 1."]
    #[inline(always)]
    pub fn pi_wrlat_adj_f1(&self) -> PiWrlatAdjF1R {
        PiWrlatAdjF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlat_adj_f1(&mut self) -> PiRdlatAdjF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec> {
        PiRdlatAdjF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlat_adj_f2(&mut self) -> PiRdlatAdjF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec> {
        PiRdlatAdjF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_adj_f0(&mut self) -> PiWrlatAdjF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec> {
        PiWrlatAdjF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Adjusts the relative timing in memory clocks between DFI write commands and the dfi_wrdata_en signal for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_adj_f1(&mut self) -> PiWrlatAdjF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec> {
        PiWrlatAdjF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_194\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_194::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_194::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_194::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_194::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_194 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi194Spec {
    const RESET_VALUE: u32 = 0;
}
