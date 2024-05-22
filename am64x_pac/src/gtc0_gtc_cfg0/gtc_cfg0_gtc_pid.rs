#[doc = "Register `GTC_CFG0_GTC_PID` reader"]
pub type R = crate::R<GtcCfg0GtcPidSpec>;
#[doc = "Register `GTC_CFG0_GTC_PID` writer"]
pub type W = crate::W<GtcCfg0GtcPidSpec>;
#[doc = "Field `GTC_PID_Y_MINOR` reader - 5:0\\]
Minor revision number - actual value determined by RTL"]
pub type GtcPidYMinorR = crate::FieldReader;
#[doc = "Field `GTC_PID_Y_MINOR` writer - 5:0\\]
Minor revision number - actual value determined by RTL"]
pub type GtcPidYMinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GTC_PID_CUSTOM` reader - 7:6\\]
Custom revision number - actual value determined by RTL"]
pub type GtcPidCustomR = crate::FieldReader;
#[doc = "Field `GTC_PID_CUSTOM` writer - 7:6\\]
Custom revision number - actual value determined by RTL"]
pub type GtcPidCustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTC_PID_X_MAJOR` reader - 10:8\\]
Major revision number - actual value determined by RTL"]
pub type GtcPidXMajorR = crate::FieldReader;
#[doc = "Field `GTC_PID_X_MAJOR` writer - 10:8\\]
Major revision number - actual value determined by RTL"]
pub type GtcPidXMajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTC_PID_R_RTL` reader - 15:11\\]
RTL revision number - actual value determined by RTL"]
pub type GtcPidRRtlR = crate::FieldReader;
#[doc = "Field `GTC_PID_R_RTL` writer - 15:11\\]
RTL revision number - actual value determined by RTL"]
pub type GtcPidRRtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GTC_PID_FUNC` reader - 27:16\\]
Module functional identifier - GTC module"]
pub type GtcPidFuncR = crate::FieldReader<u16>;
#[doc = "Field `GTC_PID_FUNC` writer - 27:16\\]
Module functional identifier - GTC module"]
pub type GtcPidFuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `GTC_PID_BU` reader - 29:28\\]
Business unit - Processors"]
pub type GtcPidBuR = crate::FieldReader;
#[doc = "Field `GTC_PID_BU` writer - 29:28\\]
Business unit - Processors"]
pub type GtcPidBuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTC_PID_SCHEME` reader - 31:30\\]
PID follows new scheme"]
pub type GtcPidSchemeR = crate::FieldReader;
#[doc = "Field `GTC_PID_SCHEME` writer - 31:30\\]
PID follows new scheme"]
pub type GtcPidSchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn gtc_pid_y_minor(&self) -> GtcPidYMinorR {
        GtcPidYMinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn gtc_pid_custom(&self) -> GtcPidCustomR {
        GtcPidCustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn gtc_pid_x_major(&self) -> GtcPidXMajorR {
        GtcPidXMajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn gtc_pid_r_rtl(&self) -> GtcPidRRtlR {
        GtcPidRRtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module functional identifier - GTC module"]
    #[inline(always)]
    pub fn gtc_pid_func(&self) -> GtcPidFuncR {
        GtcPidFuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business unit - Processors"]
    #[inline(always)]
    pub fn gtc_pid_bu(&self) -> GtcPidBuR {
        GtcPidBuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID follows new scheme"]
    #[inline(always)]
    pub fn gtc_pid_scheme(&self) -> GtcPidSchemeR {
        GtcPidSchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_pid_y_minor(&mut self) -> GtcPidYMinorW<GtcCfg0GtcPidSpec> {
        GtcPidYMinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_pid_custom(&mut self) -> GtcPidCustomW<GtcCfg0GtcPidSpec> {
        GtcPidCustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_pid_x_major(&mut self) -> GtcPidXMajorW<GtcCfg0GtcPidSpec> {
        GtcPidXMajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_pid_r_rtl(&mut self) -> GtcPidRRtlW<GtcCfg0GtcPidSpec> {
        GtcPidRRtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module functional identifier - GTC module"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_pid_func(&mut self) -> GtcPidFuncW<GtcCfg0GtcPidSpec> {
        GtcPidFuncW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business unit - Processors"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_pid_bu(&mut self) -> GtcPidBuW<GtcCfg0GtcPidSpec> {
        GtcPidBuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID follows new scheme"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_pid_scheme(&mut self) -> GtcPidSchemeW<GtcCfg0GtcPidSpec> {
        GtcPidSchemeW::new(self, 30)
    }
}
#[doc = "GTC_CFG0_GTC_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg0_gtc_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg0_gtc_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg0GtcPidSpec;
impl crate::RegisterSpec for GtcCfg0GtcPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg0_gtc_pid::R`](R) reader structure"]
impl crate::Readable for GtcCfg0GtcPidSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg0_gtc_pid::W`](W) writer structure"]
impl crate::Writable for GtcCfg0GtcPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG0_GTC_PID to value 0x7562_0000"]
impl crate::Resettable for GtcCfg0GtcPidSpec {
    const RESET_VALUE: u32 = 0x7562_0000;
}
