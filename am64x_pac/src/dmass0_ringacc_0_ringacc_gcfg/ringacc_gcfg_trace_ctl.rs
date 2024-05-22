#[doc = "Register `RINGACC_GCFG_trace_ctl` reader"]
pub type R = crate::R<RingaccGcfgTraceCtlSpec>;
#[doc = "Register `RINGACC_GCFG_trace_ctl` writer"]
pub type W = crate::W<RingaccGcfgTraceCtlSpec>;
#[doc = "Field `QUEUE` reader - 15:0\\]
Queue number when tracing a single queue."]
pub type QueueR = crate::FieldReader<u16>;
#[doc = "Field `QUEUE` writer - 15:0\\]
Queue number when tracing a single queue."]
pub type QueueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MSG` reader - 29:29\\]
Trace message data 0 = include only the operation 1 = include message data."]
pub type MsgR = crate::BitReader;
#[doc = "Field `MSG` writer - 29:29\\]
Trace message data 0 = include only the operation 1 = include message data."]
pub type MsgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALL_QUEUES` reader - 30:30\\]
Trace everything 0 = only the selected queue 1 = every queue."]
pub type AllQueuesR = crate::BitReader;
#[doc = "Field `ALL_QUEUES` writer - 30:30\\]
Trace everything 0 = only the selected queue 1 = every queue."]
pub type AllQueuesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - 31:31\\]
Trace enable 0 = disable 1 = enable."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 31:31\\]
Trace enable 0 = disable 1 = enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Queue number when tracing a single queue."]
    #[inline(always)]
    pub fn queue(&self) -> QueueR {
        QueueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - 29:29\\]
Trace message data 0 = include only the operation 1 = include message data."]
    #[inline(always)]
    pub fn msg(&self) -> MsgR {
        MsgR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Trace everything 0 = only the selected queue 1 = every queue."]
    #[inline(always)]
    pub fn all_queues(&self) -> AllQueuesR {
        AllQueuesR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Trace enable 0 = disable 1 = enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Queue number when tracing a single queue."]
    #[inline(always)]
    #[must_use]
    pub fn queue(&mut self) -> QueueW<RingaccGcfgTraceCtlSpec> {
        QueueW::new(self, 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Trace message data 0 = include only the operation 1 = include message data."]
    #[inline(always)]
    #[must_use]
    pub fn msg(&mut self) -> MsgW<RingaccGcfgTraceCtlSpec> {
        MsgW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Trace everything 0 = only the selected queue 1 = every queue."]
    #[inline(always)]
    #[must_use]
    pub fn all_queues(&mut self) -> AllQueuesW<RingaccGcfgTraceCtlSpec> {
        AllQueuesW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Trace enable 0 = disable 1 = enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<RingaccGcfgTraceCtlSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Trace Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_trace_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_trace_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccGcfgTraceCtlSpec;
impl crate::RegisterSpec for RingaccGcfgTraceCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_gcfg_trace_ctl::R`](R) reader structure"]
impl crate::Readable for RingaccGcfgTraceCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_gcfg_trace_ctl::W`](W) writer structure"]
impl crate::Writable for RingaccGcfgTraceCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_GCFG_trace_ctl to value 0"]
impl crate::Resettable for RingaccGcfgTraceCtlSpec {
    const RESET_VALUE: u32 = 0;
}
