#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_170` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_170` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec>;
#[doc = "Field `PI_ZQINIT_F1` reader - 11:0\\]
Number of cycles needed for a ZQINIT command for frequency set 1."]
pub type PiZqinitF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_ZQINIT_F1` writer - 11:0\\]
Number of cycles needed for a ZQINIT command for frequency set 1."]
pub type PiZqinitF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PI_ZQINIT_F2` reader - 27:16\\]
Number of cycles needed for a ZQINIT command for frequency set 2."]
pub type PiZqinitF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_ZQINIT_F2` writer - 27:16\\]
Number of cycles needed for a ZQINIT command for frequency set 2."]
pub type PiZqinitF2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of cycles needed for a ZQINIT command for frequency set 1."]
    #[inline(always)]
    pub fn pi_zqinit_f1(&self) -> PiZqinitF1R {
        PiZqinitF1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQINIT command for frequency set 2."]
    #[inline(always)]
    pub fn pi_zqinit_f2(&self) -> PiZqinitF2R {
        PiZqinitF2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of cycles needed for a ZQINIT command for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_zqinit_f1(&mut self) -> PiZqinitF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec> {
        PiZqinitF1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQINIT command for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_zqinit_f2(&mut self) -> PiZqinitF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec> {
        PiZqinitF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_170\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_170::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_170::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_170::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_170::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_170 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi170Spec {
    const RESET_VALUE: u32 = 0;
}
