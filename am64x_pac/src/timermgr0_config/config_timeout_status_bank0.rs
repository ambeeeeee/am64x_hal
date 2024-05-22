#[doc = "Register `CONFIG_TIMEOUT_STATUS_BANK0` reader"]
pub type R = crate::R<ConfigTimeoutStatusBank0Spec>;
#[doc = "Register `CONFIG_TIMEOUT_STATUS_BANK0` writer"]
pub type W = crate::W<ConfigTimeoutStatusBank0Spec>;
#[doc = "Field `BANK_STATUS` reader - 31:0\\]
A 1 in bit N indicates that the corresponding bank has expired timers"]
pub type BankStatusR = crate::FieldReader<u32>;
#[doc = "Field `BANK_STATUS` writer - 31:0\\]
A 1 in bit N indicates that the corresponding bank has expired timers"]
pub type BankStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A 1 in bit N indicates that the corresponding bank has expired timers"]
    #[inline(always)]
    pub fn bank_status(&self) -> BankStatusR {
        BankStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A 1 in bit N indicates that the corresponding bank has expired timers"]
    #[inline(always)]
    #[must_use]
    pub fn bank_status(&mut self) -> BankStatusW<ConfigTimeoutStatusBank0Spec> {
        BankStatusW::new(self, 0)
    }
}
#[doc = "This register contains the status of each timer bank for banks 31:0. When servicing the timer interrupt, if the num_expired_timers bit is greater than 3, this register may be read to see which banks contain expired timers. The TIMER_STATUS_N registers corresponding to those banks may then be read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timeout_status_bank0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timeout_status_bank0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigTimeoutStatusBank0Spec;
impl crate::RegisterSpec for ConfigTimeoutStatusBank0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_timeout_status_bank0::R`](R) reader structure"]
impl crate::Readable for ConfigTimeoutStatusBank0Spec {}
#[doc = "`write(|w| ..)` method takes [`config_timeout_status_bank0::W`](W) writer structure"]
impl crate::Writable for ConfigTimeoutStatusBank0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_TIMEOUT_STATUS_BANK0 to value 0"]
impl crate::Resettable for ConfigTimeoutStatusBank0Spec {
    const RESET_VALUE: u32 = 0;
}
