#[doc = "Register `EPWM_REGS_AQSFRC` reader"]
pub type R = crate::R<EpwmRegsAqsfrcSpec>;
#[doc = "Register `EPWM_REGS_AQSFRC` writer"]
pub type W = crate::W<EpwmRegsAqsfrcSpec>;
#[doc = "Field `ACTSFA` reader - 1:0\\]
Action When One-Time Software Force A Is Invoked"]
pub type ActsfaR = crate::FieldReader;
#[doc = "Field `ACTSFA` writer - 1:0\\]
Action When One-Time Software Force A Is Invoked"]
pub type ActsfaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OTSFA` reader - 2:2\\]
One-Time Software Forced Event on Output A"]
pub type OtsfaR = crate::BitReader;
#[doc = "Field `OTSFA` writer - 2:2\\]
One-Time Software Forced Event on Output A"]
pub type OtsfaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTSFB` reader - 4:3\\]
Action when One-Time Software Force B Is invoked"]
pub type ActsfbR = crate::FieldReader;
#[doc = "Field `ACTSFB` writer - 4:3\\]
Action when One-Time Software Force B Is invoked"]
pub type ActsfbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OTSFB` reader - 5:5\\]
One-Time Software Forced Event on Output B"]
pub type OtsfbR = crate::BitReader;
#[doc = "Field `OTSFB` writer - 5:5\\]
One-Time Software Forced Event on Output B"]
pub type OtsfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLDCSF` reader - 7:6\\]
AQCSFRC Active Register Reload From Shadow Options"]
pub type RldcsfR = crate::FieldReader;
#[doc = "Field `RLDCSF` writer - 7:6\\]
AQCSFRC Active Register Reload From Shadow Options"]
pub type RldcsfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Action When One-Time Software Force A Is Invoked"]
    #[inline(always)]
    pub fn actsfa(&self) -> ActsfaR {
        ActsfaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
One-Time Software Forced Event on Output A"]
    #[inline(always)]
    pub fn otsfa(&self) -> OtsfaR {
        OtsfaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Action when One-Time Software Force B Is invoked"]
    #[inline(always)]
    pub fn actsfb(&self) -> ActsfbR {
        ActsfbR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
One-Time Software Forced Event on Output B"]
    #[inline(always)]
    pub fn otsfb(&self) -> OtsfbR {
        OtsfbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
AQCSFRC Active Register Reload From Shadow Options"]
    #[inline(always)]
    pub fn rldcsf(&self) -> RldcsfR {
        RldcsfR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Action When One-Time Software Force A Is Invoked"]
    #[inline(always)]
    #[must_use]
    pub fn actsfa(&mut self) -> ActsfaW<EpwmRegsAqsfrcSpec> {
        ActsfaW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
One-Time Software Forced Event on Output A"]
    #[inline(always)]
    #[must_use]
    pub fn otsfa(&mut self) -> OtsfaW<EpwmRegsAqsfrcSpec> {
        OtsfaW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Action when One-Time Software Force B Is invoked"]
    #[inline(always)]
    #[must_use]
    pub fn actsfb(&mut self) -> ActsfbW<EpwmRegsAqsfrcSpec> {
        ActsfbW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
One-Time Software Forced Event on Output B"]
    #[inline(always)]
    #[must_use]
    pub fn otsfb(&mut self) -> OtsfbW<EpwmRegsAqsfrcSpec> {
        OtsfbW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
AQCSFRC Active Register Reload From Shadow Options"]
    #[inline(always)]
    #[must_use]
    pub fn rldcsf(&mut self) -> RldcsfW<EpwmRegsAqsfrcSpec> {
        RldcsfW::new(self, 6)
    }
}
#[doc = "Action Qualifier Software Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_aqsfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_aqsfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsAqsfrcSpec;
impl crate::RegisterSpec for EpwmRegsAqsfrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_aqsfrc::R`](R) reader structure"]
impl crate::Readable for EpwmRegsAqsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_aqsfrc::W`](W) writer structure"]
impl crate::Writable for EpwmRegsAqsfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_AQSFRC to value 0"]
impl crate::Resettable for EpwmRegsAqsfrcSpec {
    const RESET_VALUE: u16 = 0;
}
