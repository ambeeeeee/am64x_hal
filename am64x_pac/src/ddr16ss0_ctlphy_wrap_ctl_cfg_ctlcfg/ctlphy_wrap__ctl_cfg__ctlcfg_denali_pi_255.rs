#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_255` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_255` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec>;
#[doc = "Field `PI_TEXCKE_F2` reader - 5:0\\]
DRAM CKE low after SREF command timing for frequency set 2."]
pub type PiTexckeF2R = crate::FieldReader;
#[doc = "Field `PI_TEXCKE_F2` writer - 5:0\\]
DRAM CKE low after SREF command timing for frequency set 2."]
pub type PiTexckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TDLL_F0` reader - 23:8\\]
DRAM tDLL value for frequency set 0 in cycles."]
pub type PiTdllF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDLL_F0` writer - 23:8\\]
DRAM tDLL value for frequency set 0 in cycles."]
pub type PiTdllF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
DRAM CKE low after SREF command timing for frequency set 2."]
    #[inline(always)]
    pub fn pi_texcke_f2(&self) -> PiTexckeF2R {
        PiTexckeF2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
DRAM tDLL value for frequency set 0 in cycles."]
    #[inline(always)]
    pub fn pi_tdll_f0(&self) -> PiTdllF0R {
        PiTdllF0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
DRAM CKE low after SREF command timing for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_texcke_f2(&mut self) -> PiTexckeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec> {
        PiTexckeF2W::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
DRAM tDLL value for frequency set 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdll_f0(&mut self) -> PiTdllF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec> {
        PiTdllF0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_255\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_255::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_255::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_255::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_255::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_255 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi255Spec {
    const RESET_VALUE: u32 = 0;
}
