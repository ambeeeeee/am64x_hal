#[doc = "Register `EPWM_REGS_PID` reader"]
pub type R = crate::R<EpwmRegsPidSpec>;
#[doc = "Register `EPWM_REGS_PID` writer"]
pub type W = crate::W<EpwmRegsPidSpec>;
#[doc = "Field `Y_MINOR` reader - 5:0\\]
Minor revision \\[Y\\]"]
pub type YMinorR = crate::FieldReader;
#[doc = "Field `Y_MINOR` writer - 5:0\\]
Minor revision \\[Y\\]"]
pub type YMinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
CUSTOM"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
CUSTOM"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `X_MAJOR` reader - 10:8\\]
Major revision \\[X\\]"]
pub type XMajorR = crate::FieldReader;
#[doc = "Field `X_MAJOR` writer - 10:8\\]
Major revision \\[X\\]"]
pub type XMajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R_RTL` reader - 15:11\\]
RTL version \\[R\\], maintained by IP design owner"]
pub type RRtlR = crate::FieldReader;
#[doc = "Field `R_RTL` writer - 15:11\\]
RTL version \\[R\\], maintained by IP design owner"]
pub type RRtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
FUNC"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
FUNC"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Used to distinguish between the old scheme and current"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Used to distinguish between the old scheme and current"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision \\[Y\\]"]
    #[inline(always)]
    pub fn y_minor(&self) -> YMinorR {
        YMinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
CUSTOM"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision \\[X\\]"]
    #[inline(always)]
    pub fn x_major(&self) -> XMajorR {
        XMajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version \\[R\\], maintained by IP design owner"]
    #[inline(always)]
    pub fn r_rtl(&self) -> RRtlR {
        RRtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Used to distinguish between the old scheme and current"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision \\[Y\\]"]
    #[inline(always)]
    #[must_use]
    pub fn y_minor(&mut self) -> YMinorW<EpwmRegsPidSpec> {
        YMinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
CUSTOM"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<EpwmRegsPidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision \\[X\\]"]
    #[inline(always)]
    #[must_use]
    pub fn x_major(&mut self) -> XMajorW<EpwmRegsPidSpec> {
        XMajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version \\[R\\], maintained by IP design owner"]
    #[inline(always)]
    #[must_use]
    pub fn r_rtl(&mut self) -> RRtlW<EpwmRegsPidSpec> {
        RRtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
FUNC"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<EpwmRegsPidSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Used to distinguish between the old scheme and current"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<EpwmRegsPidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "EHRPWM Peripheral ID Register. The IP revision register is used by software to track features, bugs, and compatibility.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsPidSpec;
impl crate::RegisterSpec for EpwmRegsPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epwm_regs_pid::R`](R) reader structure"]
impl crate::Readable for EpwmRegsPidSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_pid::W`](W) writer structure"]
impl crate::Writable for EpwmRegsPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_PID to value 0x5233_0903"]
impl crate::Resettable for EpwmRegsPidSpec {
    const RESET_VALUE: u32 = 0x5233_0903;
}
