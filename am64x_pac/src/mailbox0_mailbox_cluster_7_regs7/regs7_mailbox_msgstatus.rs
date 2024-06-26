#[doc = "Register `REGS7_MAILBOX_MSGSTATUS` reader"]
pub type R = crate::R<Regs7MailboxMsgstatusSpec>;
#[doc = "Register `REGS7_MAILBOX_MSGSTATUS` writer"]
pub type W = crate::W<Regs7MailboxMsgstatusSpec>;
#[doc = "Field `NUM_MESSAGES` reader - 2:0\\]
Number of messages in Mailbox\\[a\\]"]
pub type NumMessagesR = crate::FieldReader;
#[doc = "Field `NUM_MESSAGES` writer - 2:0\\]
Number of messages in Mailbox\\[a\\]"]
pub type NumMessagesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of messages in Mailbox\\[a\\]"]
    #[inline(always)]
    pub fn num_messages(&self) -> NumMessagesR {
        NumMessagesR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of messages in Mailbox\\[a\\]"]
    #[inline(always)]
    #[must_use]
    pub fn num_messages(&mut self) -> NumMessagesW<Regs7MailboxMsgstatusSpec> {
        NumMessagesW::new(self, 0)
    }
}
#[doc = "The message status register has the status of the messages in Mailbox\\[a\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_msgstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_msgstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs7MailboxMsgstatusSpec;
impl crate::RegisterSpec for Regs7MailboxMsgstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs7_mailbox_msgstatus::R`](R) reader structure"]
impl crate::Readable for Regs7MailboxMsgstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`regs7_mailbox_msgstatus::W`](W) writer structure"]
impl crate::Writable for Regs7MailboxMsgstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS7_MAILBOX_MSGSTATUS to value 0"]
impl crate::Resettable for Regs7MailboxMsgstatusSpec {
    const RESET_VALUE: u32 = 0;
}
