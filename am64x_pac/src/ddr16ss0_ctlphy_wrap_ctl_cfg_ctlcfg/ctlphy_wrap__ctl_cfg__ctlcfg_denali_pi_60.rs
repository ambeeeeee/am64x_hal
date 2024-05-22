#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_60` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_60` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec>;
#[doc = "Field `PI_TCACKEL` reader - 4:0\\]
DRAM tCACKEL value in memory cycles."]
pub type PiTcackelR = crate::FieldReader;
#[doc = "Field `PI_TCACKEL` writer - 4:0\\]
DRAM tCACKEL value in memory cycles."]
pub type PiTcackelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TCAMRD` reader - 13:8\\]
DRAM tCAMRD value in memory cycles."]
pub type PiTcamrdR = crate::FieldReader;
#[doc = "Field `PI_TCAMRD` writer - 13:8\\]
DRAM tCAMRD value in memory cycles."]
pub type PiTcamrdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TCACKEH` reader - 20:16\\]
DRAM tCACKEH value in memory cycles."]
pub type PiTcackehR = crate::FieldReader;
#[doc = "Field `PI_TCACKEH` writer - 20:16\\]
DRAM tCACKEH value in memory cycles."]
pub type PiTcackehW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TCAEXT` reader - 28:24\\]
DRAM tCAEXT value in memory cycles."]
pub type PiTcaextR = crate::FieldReader;
#[doc = "Field `PI_TCAEXT` writer - 28:24\\]
DRAM tCAEXT value in memory cycles."]
pub type PiTcaextW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM tCACKEL value in memory cycles."]
    #[inline(always)]
    pub fn pi_tcackel(&self) -> PiTcackelR {
        PiTcackelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM tCAMRD value in memory cycles."]
    #[inline(always)]
    pub fn pi_tcamrd(&self) -> PiTcamrdR {
        PiTcamrdR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM tCACKEH value in memory cycles."]
    #[inline(always)]
    pub fn pi_tcackeh(&self) -> PiTcackehR {
        PiTcackehR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM tCAEXT value in memory cycles."]
    #[inline(always)]
    pub fn pi_tcaext(&self) -> PiTcaextR {
        PiTcaextR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM tCACKEL value in memory cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcackel(&mut self) -> PiTcackelW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec> {
        PiTcackelW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM tCAMRD value in memory cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcamrd(&mut self) -> PiTcamrdW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec> {
        PiTcamrdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM tCACKEH value in memory cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcackeh(&mut self) -> PiTcackehW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec> {
        PiTcackehW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM tCAEXT value in memory cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcaext(&mut self) -> PiTcaextW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec> {
        PiTcaextW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_60::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_60::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_60 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi60Spec {
    const RESET_VALUE: u32 = 0;
}
