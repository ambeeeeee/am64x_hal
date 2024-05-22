#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_320` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_320` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec>;
#[doc = "Field `CS_VAL_LOWER_1` reader - 15:0\\]
Lower bound address for chip select 1."]
pub type CsValLower1R = crate::FieldReader<u16>;
#[doc = "Field `CS_VAL_LOWER_1` writer - 15:0\\]
Lower bound address for chip select 1."]
pub type CsValLower1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CS_VAL_UPPER_1` reader - 31:16\\]
Upper bound address for chip select 1."]
pub type CsValUpper1R = crate::FieldReader<u16>;
#[doc = "Field `CS_VAL_UPPER_1` writer - 31:16\\]
Upper bound address for chip select 1."]
pub type CsValUpper1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Lower bound address for chip select 1."]
    #[inline(always)]
    pub fn cs_val_lower_1(&self) -> CsValLower1R {
        CsValLower1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Upper bound address for chip select 1."]
    #[inline(always)]
    pub fn cs_val_upper_1(&self) -> CsValUpper1R {
        CsValUpper1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Lower bound address for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn cs_val_lower_1(&mut self) -> CsValLower1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec> {
        CsValLower1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Upper bound address for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn cs_val_upper_1(&mut self) -> CsValUpper1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec> {
        CsValUpper1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_320\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_320::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_320::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_320::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_320::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_320 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl320Spec {
    const RESET_VALUE: u32 = 0;
}
