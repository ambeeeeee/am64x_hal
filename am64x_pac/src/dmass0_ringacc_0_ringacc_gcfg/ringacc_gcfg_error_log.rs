#[doc = "Register `RINGACC_GCFG_error_log` reader"]
pub type R = crate::R<RingaccGcfgErrorLogSpec>;
#[doc = "Register `RINGACC_GCFG_error_log` writer"]
pub type W = crate::W<RingaccGcfgErrorLogSpec>;
#[doc = "Field `QUEUE` reader - 15:0\\]
Queue that received the bus error."]
pub type QueueR = crate::FieldReader<u16>;
#[doc = "Field `QUEUE` writer - 15:0\\]
Queue that received the bus error."]
pub type QueueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PUSH` reader - 31:31\\]
Bus error was caused by a push. 0 = pop. 1 = push."]
pub type PushR = crate::BitReader;
#[doc = "Field `PUSH` writer - 31:31\\]
Bus error was caused by a push. 0 = pop. 1 = push."]
pub type PushW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Queue that received the bus error."]
    #[inline(always)]
    pub fn queue(&self) -> QueueR {
        QueueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Bus error was caused by a push. 0 = pop. 1 = push."]
    #[inline(always)]
    pub fn push(&self) -> PushR {
        PushR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Queue that received the bus error."]
    #[inline(always)]
    #[must_use]
    pub fn queue(&mut self) -> QueueW<RingaccGcfgErrorLogSpec> {
        QueueW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Bus error was caused by a push. 0 = pop. 1 = push."]
    #[inline(always)]
    #[must_use]
    pub fn push(&mut self) -> PushW<RingaccGcfgErrorLogSpec> {
        PushW::new(self, 31)
    }
}
#[doc = "Error Log Register. A read of this register will clear the pending error log event and allow a new error to be captured. It does not clear the contents of this register which are only valid while the error event is pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_error_log::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_error_log::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccGcfgErrorLogSpec;
impl crate::RegisterSpec for RingaccGcfgErrorLogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_gcfg_error_log::R`](R) reader structure"]
impl crate::Readable for RingaccGcfgErrorLogSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_gcfg_error_log::W`](W) writer structure"]
impl crate::Writable for RingaccGcfgErrorLogSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_GCFG_error_log to value 0"]
impl crate::Resettable for RingaccGcfgErrorLogSpec {
    const RESET_VALUE: u32 = 0;
}
