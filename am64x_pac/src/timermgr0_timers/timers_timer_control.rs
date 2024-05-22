#[doc = "Register `TIMERS_TIMER_CONTROL` reader"]
pub type R = crate::R<TimersTimerControlSpec>;
#[doc = "Register `TIMERS_TIMER_CONTROL` writer"]
pub type W = crate::W<TimersTimerControlSpec>;
#[doc = "Field `TIMER_ENABLED` reader - 0:0\\]
Write 1 to enable, 0 to disable the timer."]
pub type TimerEnabledR = crate::BitReader;
#[doc = "Field `TIMER_ENABLED` writer - 0:0\\]
Write 1 to enable, 0 to disable the timer."]
pub type TimerEnabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_TIMER` reader - 1:1\\]
This may be used to touch/set a timer. When a 1 is written, the corresponding timer will be refreshed with the current value in its TIMER_SETUP_VALUES register. Will always read 0"]
pub type SetTimerR = crate::BitReader;
#[doc = "Field `SET_TIMER` writer - 1:1\\]
This may be used to touch/set a timer. When a 1 is written, the corresponding timer will be refreshed with the current value in its TIMER_SETUP_VALUES register. Will always read 0"]
pub type SetTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_EXPIRED` reader - 2:2\\]
The status of the current timer. 1 = expired"]
pub type TimerExpiredR = crate::BitReader;
#[doc = "Field `TIMER_EXPIRED` writer - 2:2\\]
The status of the current timer. 1 = expired"]
pub type TimerExpiredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_RESET` reader - 8:8\\]
Automatically reset the timer when it expires. Provides the option of a periodic timer, rather than one that needs to be cleared after each expiration. Added for hardware usage of the timers, so the expirations can occur regularly without software reprogramming them."]
pub type AutoResetR = crate::BitReader;
#[doc = "Field `AUTO_RESET` writer - 8:8\\]
Automatically reset the timer when it expires. Provides the option of a periodic timer, rather than one that needs to be cleared after each expiration. Added for hardware usage of the timers, so the expirations can occur regularly without software reprogramming them."]
pub type AutoResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to enable, 0 to disable the timer."]
    #[inline(always)]
    pub fn timer_enabled(&self) -> TimerEnabledR {
        TimerEnabledR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This may be used to touch/set a timer. When a 1 is written, the corresponding timer will be refreshed with the current value in its TIMER_SETUP_VALUES register. Will always read 0"]
    #[inline(always)]
    pub fn set_timer(&self) -> SetTimerR {
        SetTimerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
The status of the current timer. 1 = expired"]
    #[inline(always)]
    pub fn timer_expired(&self) -> TimerExpiredR {
        TimerExpiredR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Automatically reset the timer when it expires. Provides the option of a periodic timer, rather than one that needs to be cleared after each expiration. Added for hardware usage of the timers, so the expirations can occur regularly without software reprogramming them."]
    #[inline(always)]
    pub fn auto_reset(&self) -> AutoResetR {
        AutoResetR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to enable, 0 to disable the timer."]
    #[inline(always)]
    #[must_use]
    pub fn timer_enabled(&mut self) -> TimerEnabledW<TimersTimerControlSpec> {
        TimerEnabledW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This may be used to touch/set a timer. When a 1 is written, the corresponding timer will be refreshed with the current value in its TIMER_SETUP_VALUES register. Will always read 0"]
    #[inline(always)]
    #[must_use]
    pub fn set_timer(&mut self) -> SetTimerW<TimersTimerControlSpec> {
        SetTimerW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
The status of the current timer. 1 = expired"]
    #[inline(always)]
    #[must_use]
    pub fn timer_expired(&mut self) -> TimerExpiredW<TimersTimerControlSpec> {
        TimerExpiredW::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Automatically reset the timer when it expires. Provides the option of a periodic timer, rather than one that needs to be cleared after each expiration. Added for hardware usage of the timers, so the expirations can occur regularly without software reprogramming them."]
    #[inline(always)]
    #[must_use]
    pub fn auto_reset(&mut self) -> AutoResetW<TimersTimerControlSpec> {
        AutoResetW::new(self, 8)
    }
}
#[doc = "Modifies the behavior of timer N with control signals below\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timers_timer_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timers_timer_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimersTimerControlSpec;
impl crate::RegisterSpec for TimersTimerControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timers_timer_control::R`](R) reader structure"]
impl crate::Readable for TimersTimerControlSpec {}
#[doc = "`write(|w| ..)` method takes [`timers_timer_control::W`](W) writer structure"]
impl crate::Writable for TimersTimerControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERS_TIMER_CONTROL to value 0"]
impl crate::Resettable for TimersTimerControlSpec {
    const RESET_VALUE: u32 = 0;
}
