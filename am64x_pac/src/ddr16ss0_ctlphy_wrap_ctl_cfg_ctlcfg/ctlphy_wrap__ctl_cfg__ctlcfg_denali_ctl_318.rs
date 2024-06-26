#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_318` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_318` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec>;
#[doc = "Field `CS_VAL_LOWER_0` reader - 15:0\\]
Lower bound address for chip select 0."]
pub type CsValLower0R = crate::FieldReader<u16>;
#[doc = "Field `CS_VAL_LOWER_0` writer - 15:0\\]
Lower bound address for chip select 0."]
pub type CsValLower0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CS_VAL_UPPER_0` reader - 31:16\\]
Upper bound address for chip select 0."]
pub type CsValUpper0R = crate::FieldReader<u16>;
#[doc = "Field `CS_VAL_UPPER_0` writer - 31:16\\]
Upper bound address for chip select 0."]
pub type CsValUpper0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Lower bound address for chip select 0."]
    #[inline(always)]
    pub fn cs_val_lower_0(&self) -> CsValLower0R {
        CsValLower0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Upper bound address for chip select 0."]
    #[inline(always)]
    pub fn cs_val_upper_0(&self) -> CsValUpper0R {
        CsValUpper0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Lower bound address for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn cs_val_lower_0(&mut self) -> CsValLower0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec> {
        CsValLower0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Upper bound address for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn cs_val_upper_0(&mut self) -> CsValUpper0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec> {
        CsValUpper0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_318\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_318::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_318::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_318::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_318::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_318 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl318Spec {
    const RESET_VALUE: u32 = 0;
}
