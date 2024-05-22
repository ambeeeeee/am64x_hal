#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_185` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_185` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec>;
#[doc = "Field `PI_TODTL_2CMD_F2` reader - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 2."]
pub type PiTodtl2cmdF2R = crate::FieldReader;
#[doc = "Field `PI_TODTL_2CMD_F2` writer - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 2."]
pub type PiTodtl2cmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_ODT_EN_F2` reader - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 2."]
pub type PiOdtEnF2R = crate::BitReader;
#[doc = "Field `PI_ODT_EN_F2` writer - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 2."]
pub type PiOdtEnF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_ODTLON_F0` reader - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 0."]
pub type PiOdtlonF0R = crate::FieldReader;
#[doc = "Field `PI_ODTLON_F0` writer - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 0."]
pub type PiOdtlonF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTON_MIN_F0` reader - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 0."]
pub type PiTodtonMinF0R = crate::FieldReader;
#[doc = "Field `PI_TODTON_MIN_F0` writer - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 0."]
pub type PiTodtonMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 2."]
    #[inline(always)]
    pub fn pi_todtl_2cmd_f2(&self) -> PiTodtl2cmdF2R {
        PiTodtl2cmdF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 2."]
    #[inline(always)]
    pub fn pi_odt_en_f2(&self) -> PiOdtEnF2R {
        PiOdtEnF2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 0."]
    #[inline(always)]
    pub fn pi_odtlon_f0(&self) -> PiOdtlonF0R {
        PiOdtlonF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 0."]
    #[inline(always)]
    pub fn pi_todton_min_f0(&self) -> PiTodtonMinF0R {
        PiTodtonMinF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todtl_2cmd_f2(&mut self) -> PiTodtl2cmdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec> {
        PiTodtl2cmdF2W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable support of DRAM ODT. When enabled, PI will assert and de-assert ODT output to DRAM as needed for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_en_f2(&mut self) -> PiOdtEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec> {
        PiOdtEnF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the latency from a CAS-2 command to the tODTon reference for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odtlon_f0(&mut self) -> PiOdtlonF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec> {
        PiOdtlonF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todton_min_f0(&mut self) -> PiTodtonMinF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec> {
        PiTodtonMinF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_185\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_185::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_185::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_185::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_185::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_185 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi185Spec {
    const RESET_VALUE: u32 = 0;
}
