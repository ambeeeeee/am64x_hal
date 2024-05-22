#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_19` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_19` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec>;
#[doc = "Field `MRR_ERROR_STATUS` reader - 0:0\\]
Indicates that an MRR was issued while in self-refresh. Value of 1 indicates a violation. READ-ONLY"]
pub type MrrErrorStatusR = crate::BitReader;
#[doc = "Field `MRR_ERROR_STATUS` writer - 0:0\\]
Indicates that an MRR was issued while in self-refresh. Value of 1 indicates a violation. READ-ONLY"]
pub type MrrErrorStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_FREQ_RATIO_F0` reader - 9:8\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=0"]
pub type DfiFreqRatioF0R = crate::FieldReader;
#[doc = "Field `DFI_FREQ_RATIO_F0` writer - 9:8\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=0"]
pub type DfiFreqRatioF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DFI_FREQ_RATIO_F1` reader - 17:16\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=1"]
pub type DfiFreqRatioF1R = crate::FieldReader;
#[doc = "Field `DFI_FREQ_RATIO_F1` writer - 17:16\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=1"]
pub type DfiFreqRatioF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DFI_FREQ_RATIO_F2` reader - 25:24\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=2"]
pub type DfiFreqRatioF2R = crate::FieldReader;
#[doc = "Field `DFI_FREQ_RATIO_F2` writer - 25:24\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=2"]
pub type DfiFreqRatioF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that an MRR was issued while in self-refresh. Value of 1 indicates a violation. READ-ONLY"]
    #[inline(always)]
    pub fn mrr_error_status(&self) -> MrrErrorStatusR {
        MrrErrorStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=0"]
    #[inline(always)]
    pub fn dfi_freq_ratio_f0(&self) -> DfiFreqRatioF0R {
        DfiFreqRatioF0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=1"]
    #[inline(always)]
    pub fn dfi_freq_ratio_f1(&self) -> DfiFreqRatioF1R {
        DfiFreqRatioF1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=2"]
    #[inline(always)]
    pub fn dfi_freq_ratio_f2(&self) -> DfiFreqRatioF2R {
        DfiFreqRatioF2R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates that an MRR was issued while in self-refresh. Value of 1 indicates a violation. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn mrr_error_status(&mut self) -> MrrErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec> {
        MrrErrorStatusW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_freq_ratio_f0(
        &mut self,
    ) -> DfiFreqRatioF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec> {
        DfiFreqRatioF0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_freq_ratio_f1(
        &mut self,
    ) -> DfiFreqRatioF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec> {
        DfiFreqRatioF1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines how dfi_freq_ratio is driven on the DFI bus. For LPDDR5, specifies the DFI clock to WCK ratio. For all other memory classes, specifies the controller clock to DFI PHY clock ratio. Program to zero for a 1:1 ratio, one for a 1:2 ratio, and two for a 1:4 ratio. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_freq_ratio_f2(
        &mut self,
    ) -> DfiFreqRatioF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec> {
        DfiFreqRatioF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_19::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_19::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_19 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl19Spec {
    const RESET_VALUE: u32 = 0;
}
