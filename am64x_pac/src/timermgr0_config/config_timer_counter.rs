#[doc = "Register `CONFIG_TIMER_COUNTER` reader"]
pub type R = crate::R<ConfigTimerCounterSpec>;
#[doc = "Register `CONFIG_TIMER_COUNTER` writer"]
pub type W = crate::W<ConfigTimerCounterSpec>;
#[doc = "Field `COUNT` reader - 31:0\\]
The current timer_counter value, in the timebase being used by all timers in this module"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 31:0\\]
The current timer_counter value, in the timebase being used by all timers in this module"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The current timer_counter value, in the timebase being used by all timers in this module"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The current timer_counter value, in the timebase being used by all timers in this module"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<ConfigTimerCounterSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "This register contains the current counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timer_counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timer_counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigTimerCounterSpec;
impl crate::RegisterSpec for ConfigTimerCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_timer_counter::R`](R) reader structure"]
impl crate::Readable for ConfigTimerCounterSpec {}
#[doc = "`write(|w| ..)` method takes [`config_timer_counter::W`](W) writer structure"]
impl crate::Writable for ConfigTimerCounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_TIMER_COUNTER to value 0"]
impl crate::Resettable for ConfigTimerCounterSpec {
    const RESET_VALUE: u32 = 0;
}
