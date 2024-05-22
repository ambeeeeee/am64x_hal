#[doc = "Register `CONFIG_TIMEOUT_STATUS0` reader"]
pub type R = crate::R<ConfigTimeoutStatus0Spec>;
#[doc = "Register `CONFIG_TIMEOUT_STATUS0` writer"]
pub type W = crate::W<ConfigTimeoutStatus0Spec>;
#[doc = "Field `NUM_EXPIRED_TIMERS` reader - 11:0\\]
The total number of expired timers"]
pub type NumExpiredTimersR = crate::FieldReader<u16>;
#[doc = "Field `NUM_EXPIRED_TIMERS` writer - 11:0\\]
The total number of expired timers"]
pub type NumExpiredTimersW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EXPIRED_TIMER0` reader - 22:12\\]
The ID of the first timer to expire"]
pub type ExpiredTimer0R = crate::FieldReader<u16>;
#[doc = "Field `EXPIRED_TIMER0` writer - 22:12\\]
The ID of the first timer to expire"]
pub type ExpiredTimer0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `VALID0` reader - 23:23\\]
1 indicates that expired_timer0 is a valid expired timer. If num_expired_timers > 0, this should always be a 1"]
pub type Valid0R = crate::BitReader;
#[doc = "Field `VALID0` writer - 23:23\\]
1 indicates that expired_timer0 is a valid expired timer. If num_expired_timers > 0, this should always be a 1"]
pub type Valid0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
The total number of expired timers"]
    #[inline(always)]
    pub fn num_expired_timers(&self) -> NumExpiredTimersR {
        NumExpiredTimersR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:22 - 22:12\\]
The ID of the first timer to expire"]
    #[inline(always)]
    pub fn expired_timer0(&self) -> ExpiredTimer0R {
        ExpiredTimer0R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 23 - 23:23\\]
1 indicates that expired_timer0 is a valid expired timer. If num_expired_timers > 0, this should always be a 1"]
    #[inline(always)]
    pub fn valid0(&self) -> Valid0R {
        Valid0R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
The total number of expired timers"]
    #[inline(always)]
    #[must_use]
    pub fn num_expired_timers(&mut self) -> NumExpiredTimersW<ConfigTimeoutStatus0Spec> {
        NumExpiredTimersW::new(self, 0)
    }
    #[doc = "Bits 12:22 - 22:12\\]
The ID of the first timer to expire"]
    #[inline(always)]
    #[must_use]
    pub fn expired_timer0(&mut self) -> ExpiredTimer0W<ConfigTimeoutStatus0Spec> {
        ExpiredTimer0W::new(self, 12)
    }
    #[doc = "Bit 23 - 23:23\\]
1 indicates that expired_timer0 is a valid expired timer. If num_expired_timers > 0, this should always be a 1"]
    #[inline(always)]
    #[must_use]
    pub fn valid0(&mut self) -> Valid0W<ConfigTimeoutStatus0Spec> {
        Valid0W::new(self, 23)
    }
}
#[doc = "This register should be read whenever the timer interrupt fires. It indicates the total number of timers that have expired and the ID of the first timer to expire. If NUM_EXPIRED_TIMERS is 1, this is the only MMR that needs to be read. Depending on the value of NUM_EXPIRED_TIMERS, either TIMEOUT_STATUS_1 or TIMEOUT_STATUS_BANK may be read by the software to avoid needing to read all 32 TIMEOUT_STATUS_N registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timeout_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timeout_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigTimeoutStatus0Spec;
impl crate::RegisterSpec for ConfigTimeoutStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_timeout_status0::R`](R) reader structure"]
impl crate::Readable for ConfigTimeoutStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`config_timeout_status0::W`](W) writer structure"]
impl crate::Writable for ConfigTimeoutStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_TIMEOUT_STATUS0 to value 0"]
impl crate::Resettable for ConfigTimeoutStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
