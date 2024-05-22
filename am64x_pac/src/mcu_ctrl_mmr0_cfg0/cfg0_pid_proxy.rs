#[doc = "Register `CFG0_PID_PROXY` reader"]
pub type R = crate::R<Cfg0PidProxySpec>;
#[doc = "Register `CFG0_PID_PROXY` writer"]
pub type W = crate::W<Cfg0PidProxySpec>;
#[doc = "Field `PID_MINOR_PROXY` reader - "]
pub type PidMinorProxyR = crate::FieldReader;
#[doc = "Field `PID_MINOR_PROXY` writer - "]
pub type PidMinorProxyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PID_CUSTOM_PROXY` reader - "]
pub type PidCustomProxyR = crate::FieldReader;
#[doc = "Field `PID_CUSTOM_PROXY` writer - "]
pub type PidCustomProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PID_MAJOR_PROXY` reader - "]
pub type PidMajorProxyR = crate::FieldReader;
#[doc = "Field `PID_MAJOR_PROXY` writer - "]
pub type PidMajorProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PID_MISC_PROXY` reader - "]
pub type PidMiscProxyR = crate::FieldReader;
#[doc = "Field `PID_MISC_PROXY` writer - "]
pub type PidMiscProxyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PID_MSB16_PROXY` reader - "]
pub type PidMsb16ProxyR = crate::FieldReader<u16>;
#[doc = "Field `PID_MSB16_PROXY` writer - "]
pub type PidMsb16ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pid_minor_proxy(&self) -> PidMinorProxyR {
        PidMinorProxyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pid_custom_proxy(&self) -> PidCustomProxyR {
        PidCustomProxyR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pid_major_proxy(&self) -> PidMajorProxyR {
        PidMajorProxyR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pid_misc_proxy(&self) -> PidMiscProxyR {
        PidMiscProxyR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pid_msb16_proxy(&self) -> PidMsb16ProxyR {
        PidMsb16ProxyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pid_minor_proxy(&mut self) -> PidMinorProxyW<Cfg0PidProxySpec> {
        PidMinorProxyW::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn pid_custom_proxy(&mut self) -> PidCustomProxyW<Cfg0PidProxySpec> {
        PidCustomProxyW::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pid_major_proxy(&mut self) -> PidMajorProxyW<Cfg0PidProxySpec> {
        PidMajorProxyW::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pid_misc_proxy(&mut self) -> PidMiscProxyW<Cfg0PidProxySpec> {
        PidMiscProxyW::new(self, 11)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pid_msb16_proxy(&mut self) -> PidMsb16ProxyW<Cfg0PidProxySpec> {
        PidMsb16ProxyW::new(self, 16)
    }
}
#[doc = "CFG0_PID_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pid_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pid_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PidProxySpec;
impl crate::RegisterSpec for Cfg0PidProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pid_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PidProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pid_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PidProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PID_PROXY to value 0x0219"]
impl crate::Resettable for Cfg0PidProxySpec {
    const RESET_VALUE: u32 = 0x0219;
}
