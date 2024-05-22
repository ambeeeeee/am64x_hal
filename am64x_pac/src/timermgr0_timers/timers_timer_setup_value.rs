#[doc = "Register `TIMERS_TIMER_SETUP_VALUE` reader"]
pub type R = crate::R<TimersTimerSetupValueSpec>;
#[doc = "Register `TIMERS_TIMER_SETUP_VALUE` writer"]
pub type W = crate::W<TimersTimerSetupValueSpec>;
#[doc = "Field `COUNT` reader - 31:0\\]
The number of ticks of the timer_clock before this timer would expire when reprogrammed"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 31:0\\]
The number of ticks of the timer_clock before this timer would expire when reprogrammed"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The number of ticks of the timer_clock before this timer would expire when reprogrammed"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The number of ticks of the timer_clock before this timer would expire when reprogrammed"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<TimersTimerSetupValueSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "This reprograms timer N with the written value. This number will be the number of ticks of the timer_clock before the timer expires, if timer N and the timer manager itself are both enabled via TM_CTRL and TIMER_CONTROL_N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timers_timer_setup_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timers_timer_setup_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimersTimerSetupValueSpec;
impl crate::RegisterSpec for TimersTimerSetupValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timers_timer_setup_value::R`](R) reader structure"]
impl crate::Readable for TimersTimerSetupValueSpec {}
#[doc = "`write(|w| ..)` method takes [`timers_timer_setup_value::W`](W) writer structure"]
impl crate::Writable for TimersTimerSetupValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERS_TIMER_SETUP_VALUE to value 0"]
impl crate::Resettable for TimersTimerSetupValueSpec {
    const RESET_VALUE: u32 = 0;
}
