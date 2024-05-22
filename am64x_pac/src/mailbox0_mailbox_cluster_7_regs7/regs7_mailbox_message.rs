#[doc = "Register `REGS7_MAILBOX_MESSAGE` reader"]
pub type R = crate::R<Regs7MailboxMessageSpec>;
#[doc = "Register `REGS7_MAILBOX_MESSAGE` writer"]
pub type W = crate::W<Regs7MailboxMessageSpec>;
#[doc = "Field `MESSAGE_VALUE` reader - 31:0\\]
Message in Mailbox \\[a\\]"]
pub type MessageValueR = crate::FieldReader<u32>;
#[doc = "Field `MESSAGE_VALUE` writer - 31:0\\]
Message in Mailbox \\[a\\]"]
pub type MessageValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Message in Mailbox \\[a\\]"]
    #[inline(always)]
    pub fn message_value(&self) -> MessageValueR {
        MessageValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Message in Mailbox \\[a\\]"]
    #[inline(always)]
    #[must_use]
    pub fn message_value(&mut self) -> MessageValueW<Regs7MailboxMessageSpec> {
        MessageValueW::new(self, 0)
    }
}
#[doc = "The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_message::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_message::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs7MailboxMessageSpec;
impl crate::RegisterSpec for Regs7MailboxMessageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs7_mailbox_message::R`](R) reader structure"]
impl crate::Readable for Regs7MailboxMessageSpec {}
#[doc = "`write(|w| ..)` method takes [`regs7_mailbox_message::W`](W) writer structure"]
impl crate::Writable for Regs7MailboxMessageSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS7_MAILBOX_MESSAGE to value 0"]
impl crate::Resettable for Regs7MailboxMessageSpec {
    const RESET_VALUE: u32 = 0;
}
