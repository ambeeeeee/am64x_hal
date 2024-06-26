#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_41` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_41` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec>;
#[doc = "Field `PI_RDLVL_PAT_6` reader - 31:0\\]
Non-default pattern 6 used for read data eye training of DDR4 or LPDDR4, and read dbi training of DDR4."]
pub type PiRdlvlPat6R = crate::FieldReader<u32>;
#[doc = "Field `PI_RDLVL_PAT_6` writer - 31:0\\]
Non-default pattern 6 used for read data eye training of DDR4 or LPDDR4, and read dbi training of DDR4."]
pub type PiRdlvlPat6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Non-default pattern 6 used for read data eye training of DDR4 or LPDDR4, and read dbi training of DDR4."]
    #[inline(always)]
    pub fn pi_rdlvl_pat_6(&self) -> PiRdlvlPat6R {
        PiRdlvlPat6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Non-default pattern 6 used for read data eye training of DDR4 or LPDDR4, and read dbi training of DDR4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pat_6(&mut self) -> PiRdlvlPat6W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec> {
        PiRdlvlPat6W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_41::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_41::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_41 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi41Spec {
    const RESET_VALUE: u32 = 0;
}
