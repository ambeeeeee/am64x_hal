#[doc = "Register `CPTS_VBUSP_EVENT_1_REG` reader"]
pub type R = crate::R<CptsVbuspEvent1RegSpec>;
#[doc = "Register `CPTS_VBUSP_EVENT_1_REG` writer"]
pub type W = crate::W<CptsVbuspEvent1RegSpec>;
#[doc = "Field `SEQUENCE_ID` reader - 15:0\\]
Sequence ID"]
pub type SequenceIdR = crate::FieldReader<u16>;
#[doc = "Field `SEQUENCE_ID` writer - 15:0\\]
Sequence ID"]
pub type SequenceIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MESSAGE_TYPE` reader - 19:16\\]
Message type"]
pub type MessageTypeR = crate::FieldReader;
#[doc = "Field `MESSAGE_TYPE` writer - 19:16\\]
Message type"]
pub type MessageTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EVENT_TYPE` reader - 23:20\\]
Event type"]
pub type EventTypeR = crate::FieldReader;
#[doc = "Field `EVENT_TYPE` writer - 23:20\\]
Event type"]
pub type EventTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PORT_NUMBER` reader - 28:24\\]
Port number"]
pub type PortNumberR = crate::FieldReader;
#[doc = "Field `PORT_NUMBER` writer - 28:24\\]
Port number"]
pub type PortNumberW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PREMPT_QUEUE` reader - 29:29\\]
Prempt QUEUE"]
pub type PremptQueueR = crate::BitReader;
#[doc = "Field `PREMPT_QUEUE` writer - 29:29\\]
Prempt QUEUE"]
pub type PremptQueueW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Sequence ID"]
    #[inline(always)]
    pub fn sequence_id(&self) -> SequenceIdR {
        SequenceIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Message type"]
    #[inline(always)]
    pub fn message_type(&self) -> MessageTypeR {
        MessageTypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Event type"]
    #[inline(always)]
    pub fn event_type(&self) -> EventTypeR {
        EventTypeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port number"]
    #[inline(always)]
    pub fn port_number(&self) -> PortNumberR {
        PortNumberR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Prempt QUEUE"]
    #[inline(always)]
    pub fn prempt_queue(&self) -> PremptQueueR {
        PremptQueueR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Sequence ID"]
    #[inline(always)]
    #[must_use]
    pub fn sequence_id(&mut self) -> SequenceIdW<CptsVbuspEvent1RegSpec> {
        SequenceIdW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Message type"]
    #[inline(always)]
    #[must_use]
    pub fn message_type(&mut self) -> MessageTypeW<CptsVbuspEvent1RegSpec> {
        MessageTypeW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Event type"]
    #[inline(always)]
    #[must_use]
    pub fn event_type(&mut self) -> EventTypeW<CptsVbuspEvent1RegSpec> {
        EventTypeW::new(self, 20)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port number"]
    #[inline(always)]
    #[must_use]
    pub fn port_number(&mut self) -> PortNumberW<CptsVbuspEvent1RegSpec> {
        PortNumberW::new(self, 24)
    }
    #[doc = "Bit 29 - 29:29\\]
Prempt QUEUE"]
    #[inline(always)]
    #[must_use]
    pub fn prempt_queue(&mut self) -> PremptQueueW<CptsVbuspEvent1RegSpec> {
        PremptQueueW::new(self, 29)
    }
}
#[doc = "Event 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspEvent1RegSpec;
impl crate::RegisterSpec for CptsVbuspEvent1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_event_1_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspEvent1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_event_1_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspEvent1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_EVENT_1_REG to value 0"]
impl crate::Resettable for CptsVbuspEvent1RegSpec {
    const RESET_VALUE: u32 = 0;
}
