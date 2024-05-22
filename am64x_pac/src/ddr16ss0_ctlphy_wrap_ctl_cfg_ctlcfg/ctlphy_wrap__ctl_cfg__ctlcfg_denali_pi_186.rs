#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_186` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_186` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec>;
#[doc = "Field `PI_ODTLON_F1` reader - 3:0\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 1."]
pub type PiOdtlonF1R = crate::FieldReader;
#[doc = "Field `PI_ODTLON_F1` writer - 3:0\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 1."]
pub type PiOdtlonF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTON_MIN_F1` reader - 11:8\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 1."]
pub type PiTodtonMinF1R = crate::FieldReader;
#[doc = "Field `PI_TODTON_MIN_F1` writer - 11:8\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 1."]
pub type PiTodtonMinF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_ODTLON_F2` reader - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 2."]
pub type PiOdtlonF2R = crate::FieldReader;
#[doc = "Field `PI_ODTLON_F2` writer - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 2."]
pub type PiOdtlonF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTON_MIN_F2` reader - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 2."]
pub type PiTodtonMinF2R = crate::FieldReader;
#[doc = "Field `PI_TODTON_MIN_F2` writer - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 2."]
pub type PiTodtonMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 1."]
    #[inline(always)]
    pub fn pi_odtlon_f1(&self) -> PiOdtlonF1R {
        PiOdtlonF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 1."]
    #[inline(always)]
    pub fn pi_todton_min_f1(&self) -> PiTodtonMinF1R {
        PiTodtonMinF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 2."]
    #[inline(always)]
    pub fn pi_odtlon_f2(&self) -> PiOdtlonF2R {
        PiOdtlonF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 2."]
    #[inline(always)]
    pub fn pi_todton_min_f2(&self) -> PiTodtonMinF2R {
        PiTodtonMinF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odtlon_f1(&mut self) -> PiOdtlonF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec> {
        PiOdtlonF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todton_min_f1(&mut self) -> PiTodtonMinF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec> {
        PiTodtonMinF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odtlon_f2(&mut self) -> PiOdtlonF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec> {
        PiOdtlonF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todton_min_f2(&mut self) -> PiTodtonMinF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec> {
        PiTodtonMinF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_186\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_186::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_186::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_186::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_186::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_186 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi186Spec {
    const RESET_VALUE: u32 = 0;
}
