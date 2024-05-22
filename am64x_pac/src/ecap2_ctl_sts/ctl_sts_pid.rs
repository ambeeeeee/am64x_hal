#[doc = "Register `CTL_STS_PID` reader"]
pub type R = crate::R<CtlStsPidSpec>;
#[doc = "Register `CTL_STS_PID` writer"]
pub type W = crate::W<CtlStsPidSpec>;
#[doc = "Field `MINOR` reader - "]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - "]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - "]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - "]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - "]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - "]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - "]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - "]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNCTION` reader - "]
pub type FunctionR = crate::FieldReader<u16>;
#[doc = "Field `FUNCTION` writer - "]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SCHEME` reader - "]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - "]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<CtlStsPidSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<CtlStsPidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<CtlStsPidSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<CtlStsPidSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<CtlStsPidSpec> {
        FunctionW::new(self, 16)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<CtlStsPidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "CTL_STS_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsPidSpec;
impl crate::RegisterSpec for CtlStsPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_pid::R`](R) reader structure"]
impl crate::Readable for CtlStsPidSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_pid::W`](W) writer structure"]
impl crate::Writable for CtlStsPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_PID to value 0x5234_4100"]
impl crate::Resettable for CtlStsPidSpec {
    const RESET_VALUE: u32 = 0x5234_4100;
}
