#[doc = "Register `CFG0_PID` reader"]
pub type R = crate::R<Cfg0PidSpec>;
#[doc = "Register `CFG0_PID` writer"]
pub type W = crate::W<Cfg0PidSpec>;
#[doc = "Field `PID_MINOR` reader - "]
pub type PidMinorR = crate::FieldReader;
#[doc = "Field `PID_MINOR` writer - "]
pub type PidMinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PID_CUSTOM` reader - "]
pub type PidCustomR = crate::FieldReader;
#[doc = "Field `PID_CUSTOM` writer - "]
pub type PidCustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PID_MAJOR` reader - "]
pub type PidMajorR = crate::FieldReader;
#[doc = "Field `PID_MAJOR` writer - "]
pub type PidMajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PID_MISC` reader - "]
pub type PidMiscR = crate::FieldReader;
#[doc = "Field `PID_MISC` writer - "]
pub type PidMiscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PID_MSB16` reader - "]
pub type PidMsb16R = crate::FieldReader<u16>;
#[doc = "Field `PID_MSB16` writer - "]
pub type PidMsb16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pid_minor(&self) -> PidMinorR {
        PidMinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pid_custom(&self) -> PidCustomR {
        PidCustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pid_major(&self) -> PidMajorR {
        PidMajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pid_misc(&self) -> PidMiscR {
        PidMiscR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pid_msb16(&self) -> PidMsb16R {
        PidMsb16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pid_minor(&mut self) -> PidMinorW<Cfg0PidSpec> {
        PidMinorW::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn pid_custom(&mut self) -> PidCustomW<Cfg0PidSpec> {
        PidCustomW::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pid_major(&mut self) -> PidMajorW<Cfg0PidSpec> {
        PidMajorW::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pid_misc(&mut self) -> PidMiscW<Cfg0PidSpec> {
        PidMiscW::new(self, 11)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pid_msb16(&mut self) -> PidMsb16W<Cfg0PidSpec> {
        PidMsb16W::new(self, 16)
    }
}
#[doc = "CFG0_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PidSpec;
impl crate::RegisterSpec for Cfg0PidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pid::R`](R) reader structure"]
impl crate::Readable for Cfg0PidSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pid::W`](W) writer structure"]
impl crate::Writable for Cfg0PidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PID to value 0x0219"]
impl crate::Resettable for Cfg0PidSpec {
    const RESET_VALUE: u32 = 0x0219;
}
