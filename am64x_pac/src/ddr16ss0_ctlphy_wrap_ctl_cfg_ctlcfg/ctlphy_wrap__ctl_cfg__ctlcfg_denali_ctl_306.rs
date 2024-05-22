#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_306` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_306` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec>;
#[doc = "Field `ZQCL_F0` reader - 11:0\\]
Number of cycles needed for a ZQCL command. FC=0"]
pub type ZqclF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQCL_F0` writer - 11:0\\]
Number of cycles needed for a ZQCL command. FC=0"]
pub type ZqclF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ZQCS_F0` reader - 27:16\\]
Number of cycles needed for a ZQCS command. FC=0"]
pub type ZqcsF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQCS_F0` writer - 27:16\\]
Number of cycles needed for a ZQCS command. FC=0"]
pub type ZqcsF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of cycles needed for a ZQCL command. FC=0"]
    #[inline(always)]
    pub fn zqcl_f0(&self) -> ZqclF0R {
        ZqclF0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQCS command. FC=0"]
    #[inline(always)]
    pub fn zqcs_f0(&self) -> ZqcsF0R {
        ZqcsF0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of cycles needed for a ZQCL command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zqcl_f0(&mut self) -> ZqclF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec> {
        ZqclF0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQCS command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zqcs_f0(&mut self) -> ZqcsF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec> {
        ZqcsF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_306\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_306::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_306::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_306::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_306::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_306 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl306Spec {
    const RESET_VALUE: u32 = 0;
}
