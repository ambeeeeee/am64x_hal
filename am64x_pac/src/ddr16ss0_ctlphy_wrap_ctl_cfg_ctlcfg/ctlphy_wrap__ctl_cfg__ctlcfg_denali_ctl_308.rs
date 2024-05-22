#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_308` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_308` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec>;
#[doc = "Field `ZQINIT_F1` reader - 11:0\\]
Number of cycles needed for a ZQINIT command. FC=1"]
pub type ZqinitF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQINIT_F1` writer - 11:0\\]
Number of cycles needed for a ZQINIT command. FC=1"]
pub type ZqinitF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ZQCL_F1` reader - 27:16\\]
Number of cycles needed for a ZQCL command. FC=1"]
pub type ZqclF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQCL_F1` writer - 27:16\\]
Number of cycles needed for a ZQCL command. FC=1"]
pub type ZqclF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of cycles needed for a ZQINIT command. FC=1"]
    #[inline(always)]
    pub fn zqinit_f1(&self) -> ZqinitF1R {
        ZqinitF1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQCL command. FC=1"]
    #[inline(always)]
    pub fn zqcl_f1(&self) -> ZqclF1R {
        ZqclF1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of cycles needed for a ZQINIT command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn zqinit_f1(&mut self) -> ZqinitF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec> {
        ZqinitF1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQCL command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn zqcl_f1(&mut self) -> ZqclF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec> {
        ZqclF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_308\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_308::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_308::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_308::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_308::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_308 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl308Spec {
    const RESET_VALUE: u32 = 0;
}
