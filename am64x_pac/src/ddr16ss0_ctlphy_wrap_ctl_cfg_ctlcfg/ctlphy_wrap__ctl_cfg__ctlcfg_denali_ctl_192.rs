#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_192` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_192` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec>;
#[doc = "Field `TVRCG_ENABLE_F0` reader - 9:0\\]
JEDEC TVRCG_ENABLE time. FC=0"]
pub type TvrcgEnableF0R = crate::FieldReader<u16>;
#[doc = "Field `TVRCG_ENABLE_F0` writer - 9:0\\]
JEDEC TVRCG_ENABLE time. FC=0"]
pub type TvrcgEnableF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TVRCG_DISABLE_F0` reader - 25:16\\]
JEDEC TVRCG_DISABLE time. FC=0"]
pub type TvrcgDisableF0R = crate::FieldReader<u16>;
#[doc = "Field `TVRCG_DISABLE_F0` writer - 25:16\\]
JEDEC TVRCG_DISABLE time. FC=0"]
pub type TvrcgDisableF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TVRCG_ENABLE time. FC=0"]
    #[inline(always)]
    pub fn tvrcg_enable_f0(&self) -> TvrcgEnableF0R {
        TvrcgEnableF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
JEDEC TVRCG_DISABLE time. FC=0"]
    #[inline(always)]
    pub fn tvrcg_disable_f0(&self) -> TvrcgDisableF0R {
        TvrcgDisableF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TVRCG_ENABLE time. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tvrcg_enable_f0(&mut self) -> TvrcgEnableF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec> {
        TvrcgEnableF0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
JEDEC TVRCG_DISABLE time. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tvrcg_disable_f0(
        &mut self,
    ) -> TvrcgDisableF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec> {
        TvrcgDisableF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_192\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_192::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_192::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_192::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_192::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_192 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl192Spec {
    const RESET_VALUE: u32 = 0;
}
