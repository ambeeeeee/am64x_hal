#[doc = "Register `CFG_TCRR` reader"]
pub type R = crate::R<CfgTcrrSpec>;
#[doc = "Register `CFG_TCRR` writer"]
pub type W = crate::W<CfgTcrrSpec>;
#[doc = "Field `TIMER_COUNTER` reader - 31:0\\]
Timer Counter"]
pub type TimerCounterR = crate::FieldReader<u32>;
#[doc = "Field `TIMER_COUNTER` writer - 31:0\\]
Timer Counter"]
pub type TimerCounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Timer Counter"]
    #[inline(always)]
    pub fn timer_counter(&self) -> TimerCounterR {
        TimerCounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Timer Counter"]
    #[inline(always)]
    #[must_use]
    pub fn timer_counter(&mut self) -> TimerCounterW<CfgTcrrSpec> {
        TimerCounterW::new(self, 0)
    }
}
#[doc = "This register holds the value of the internal counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTcrrSpec;
impl crate::RegisterSpec for CfgTcrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tcrr::R`](R) reader structure"]
impl crate::Readable for CfgTcrrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tcrr::W`](W) writer structure"]
impl crate::Writable for CfgTcrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TCRR to value 0"]
impl crate::Resettable for CfgTcrrSpec {
    const RESET_VALUE: u32 = 0;
}
