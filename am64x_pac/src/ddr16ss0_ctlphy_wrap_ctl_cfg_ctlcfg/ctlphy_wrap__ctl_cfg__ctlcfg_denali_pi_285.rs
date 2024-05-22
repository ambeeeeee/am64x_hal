#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_285` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_285` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec>;
#[doc = "Field `PI_SLICE_PER_DEV_0` reader - 1:0\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
pub type PiSlicePerDev0R = crate::FieldReader;
#[doc = "Field `PI_SLICE_PER_DEV_0` writer - 1:0\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
pub type PiSlicePerDev0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_SLICE_PER_DEV_1` reader - 9:8\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
pub type PiSlicePerDev1R = crate::FieldReader;
#[doc = "Field `PI_SLICE_PER_DEV_1` writer - 9:8\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
pub type PiSlicePerDev1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_MR6_VREF_0_0` reader - 21:16\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref0_0R = crate::FieldReader;
#[doc = "Field `PI_MR6_VREF_0_0` writer - 21:16\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref0_0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_MR6_VREF_0_1` reader - 29:24\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref0_1R = crate::FieldReader;
#[doc = "Field `PI_MR6_VREF_0_1` writer - 29:24\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
pub type PiMr6Vref0_1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
    #[inline(always)]
    pub fn pi_slice_per_dev_0(&self) -> PiSlicePerDev0R {
        PiSlicePerDev0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
    #[inline(always)]
    pub fn pi_slice_per_dev_1(&self) -> PiSlicePerDev1R {
        PiSlicePerDev1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    pub fn pi_mr6_vref_0_0(&self) -> PiMr6Vref0_0R {
        PiMr6Vref0_0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    pub fn pi_mr6_vref_0_1(&self) -> PiMr6Vref0_1R {
        PiMr6Vref0_1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
    #[inline(always)]
    #[must_use]
    pub fn pi_slice_per_dev_0(
        &mut self,
    ) -> PiSlicePerDev0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec> {
        PiSlicePerDev0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Indicates the number of data slices per memory device. The device width divided by 8."]
    #[inline(always)]
    #[must_use]
    pub fn pi_slice_per_dev_1(
        &mut self,
    ) -> PiSlicePerDev1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec> {
        PiSlicePerDev1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr6_vref_0_0(&mut self) -> PiMr6Vref0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec> {
        PiMr6Vref0_0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
The parameter stores the vref value of every devices of the same CS. It is updated after WDQLVL PDA mode completed. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr6_vref_0_1(&mut self) -> PiMr6Vref0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec> {
        PiMr6Vref0_1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_285\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_285::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_285::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_285::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_285::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_285 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi285Spec {
    const RESET_VALUE: u32 = 0;
}
