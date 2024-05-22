#[doc = "Register `RINGACC_GCFG_overflow` reader"]
pub type R = crate::R<RingaccGcfgOverflowSpec>;
#[doc = "Register `RINGACC_GCFG_overflow` writer"]
pub type W = crate::W<RingaccGcfgOverflowSpec>;
#[doc = "Field `QUEUE` reader - 15:0\\]
Queue to send overflow messages. A value of 0xffff will disable the overflow function."]
pub type QueueR = crate::FieldReader<u16>;
#[doc = "Field `QUEUE` writer - 15:0\\]
Queue to send overflow messages. A value of 0xffff will disable the overflow function."]
pub type QueueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Queue to send overflow messages. A value of 0xffff will disable the overflow function."]
    #[inline(always)]
    pub fn queue(&self) -> QueueR {
        QueueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Queue to send overflow messages. A value of 0xffff will disable the overflow function."]
    #[inline(always)]
    #[must_use]
    pub fn queue(&mut self) -> QueueW<RingaccGcfgOverflowSpec> {
        QueueW::new(self, 0)
    }
}
#[doc = "Overflow Queue Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_overflow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_overflow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccGcfgOverflowSpec;
impl crate::RegisterSpec for RingaccGcfgOverflowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_gcfg_overflow::R`](R) reader structure"]
impl crate::Readable for RingaccGcfgOverflowSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_gcfg_overflow::W`](W) writer structure"]
impl crate::Writable for RingaccGcfgOverflowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_GCFG_overflow to value 0"]
impl crate::Resettable for RingaccGcfgOverflowSpec {
    const RESET_VALUE: u32 = 0;
}
