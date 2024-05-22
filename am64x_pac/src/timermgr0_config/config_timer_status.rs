#[doc = "Register `CONFIG_TIMER_STATUS` reader"]
pub type R = crate::R<ConfigTimerStatusSpec>;
#[doc = "Register `CONFIG_TIMER_STATUS` writer"]
pub type W = crate::W<ConfigTimerStatusSpec>;
#[doc = "Field `STATUS` reader - 31:0\\]
Each bit is the timeout status for an individual timer"]
pub type StatusR = crate::FieldReader<u32>;
#[doc = "Field `STATUS` writer - 31:0\\]
Each bit is the timeout status for an individual timer"]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Each bit is the timeout status for an individual timer"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Each bit is the timeout status for an individual timer"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<ConfigTimerStatusSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "Each bit is the timeout status for an individual timer. 0 = timer has not timed out or is disabled, 1 = timer has timed out\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timer_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timer_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigTimerStatusSpec;
impl crate::RegisterSpec for ConfigTimerStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_timer_status::R`](R) reader structure"]
impl crate::Readable for ConfigTimerStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`config_timer_status::W`](W) writer structure"]
impl crate::Writable for ConfigTimerStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_TIMER_STATUS to value 0"]
impl crate::Resettable for ConfigTimerStatusSpec {
    const RESET_VALUE: u32 = 0;
}
