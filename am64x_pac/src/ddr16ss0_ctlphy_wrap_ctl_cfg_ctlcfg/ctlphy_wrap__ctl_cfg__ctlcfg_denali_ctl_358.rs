#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_358` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_358` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec>;
#[doc = "Field `INT_MASK_DFI` reader - 7:0\\]
Mask for the controller_int signal from the INT_MASK_DFI parameter"]
pub type IntMaskDfiR = crate::FieldReader;
#[doc = "Field `INT_MASK_DFI` writer - 7:0\\]
Mask for the controller_int signal from the INT_MASK_DFI parameter"]
pub type IntMaskDfiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_MASK_FREQ` reader - 23:16\\]
Mask for the controller_int signal from the INT_MASK_FREQ parameter"]
pub type IntMaskFreqR = crate::FieldReader;
#[doc = "Field `INT_MASK_FREQ` writer - 23:16\\]
Mask for the controller_int signal from the INT_MASK_FREQ parameter"]
pub type IntMaskFreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_MASK_INIT` reader - 31:24\\]
Mask for the controller_int signal from the INT_MASK_INIT parameter"]
pub type IntMaskInitR = crate::FieldReader;
#[doc = "Field `INT_MASK_INIT` writer - 31:24\\]
Mask for the controller_int signal from the INT_MASK_INIT parameter"]
pub type IntMaskInitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Mask for the controller_int signal from the INT_MASK_DFI parameter"]
    #[inline(always)]
    pub fn int_mask_dfi(&self) -> IntMaskDfiR {
        IntMaskDfiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Mask for the controller_int signal from the INT_MASK_FREQ parameter"]
    #[inline(always)]
    pub fn int_mask_freq(&self) -> IntMaskFreqR {
        IntMaskFreqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Mask for the controller_int signal from the INT_MASK_INIT parameter"]
    #[inline(always)]
    pub fn int_mask_init(&self) -> IntMaskInitR {
        IntMaskInitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Mask for the controller_int signal from the INT_MASK_DFI parameter"]
    #[inline(always)]
    #[must_use]
    pub fn int_mask_dfi(&mut self) -> IntMaskDfiW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec> {
        IntMaskDfiW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Mask for the controller_int signal from the INT_MASK_FREQ parameter"]
    #[inline(always)]
    #[must_use]
    pub fn int_mask_freq(&mut self) -> IntMaskFreqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec> {
        IntMaskFreqW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Mask for the controller_int signal from the INT_MASK_INIT parameter"]
    #[inline(always)]
    #[must_use]
    pub fn int_mask_init(&mut self) -> IntMaskInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec> {
        IntMaskInitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_358\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_358::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_358::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_358::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_358::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_358 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl358Spec {
    const RESET_VALUE: u32 = 0;
}
