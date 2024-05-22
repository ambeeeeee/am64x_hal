#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_357` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_357` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec>;
#[doc = "Field `INT_MASK_MISC` reader - 15:0\\]
Mask for the controller_int signal from the INT_MASK_MISC parameter"]
pub type IntMaskMiscR = crate::FieldReader<u16>;
#[doc = "Field `INT_MASK_MISC` writer - 15:0\\]
Mask for the controller_int signal from the INT_MASK_MISC parameter"]
pub type IntMaskMiscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INT_MASK_BIST` reader - 23:16\\]
Mask for the controller_int signal from the INT_MASK_BIST parameter"]
pub type IntMaskBistR = crate::FieldReader;
#[doc = "Field `INT_MASK_BIST` writer - 23:16\\]
Mask for the controller_int signal from the INT_MASK_BIST parameter"]
pub type IntMaskBistW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Mask for the controller_int signal from the INT_MASK_MISC parameter"]
    #[inline(always)]
    pub fn int_mask_misc(&self) -> IntMaskMiscR {
        IntMaskMiscR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Mask for the controller_int signal from the INT_MASK_BIST parameter"]
    #[inline(always)]
    pub fn int_mask_bist(&self) -> IntMaskBistR {
        IntMaskBistR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Mask for the controller_int signal from the INT_MASK_MISC parameter"]
    #[inline(always)]
    #[must_use]
    pub fn int_mask_misc(&mut self) -> IntMaskMiscW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec> {
        IntMaskMiscW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Mask for the controller_int signal from the INT_MASK_BIST parameter"]
    #[inline(always)]
    #[must_use]
    pub fn int_mask_bist(&mut self) -> IntMaskBistW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec> {
        IntMaskBistW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_357\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_357::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_357::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_357::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_357::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_357 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl357Spec {
    const RESET_VALUE: u32 = 0;
}
