#[doc = "Register `REGS3_MAILBOX_SYSCONFIG` reader"]
pub type R = crate::R<Regs3MailboxSysconfigSpec>;
#[doc = "Register `REGS3_MAILBOX_SYSCONFIG` writer"]
pub type W = crate::W<Regs3MailboxSysconfigSpec>;
#[doc = "Field `SOFT_RESET` reader - 0:0\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0. It has the same effect as the hardware reset. Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and empty all the mailboxes"]
pub type SoftResetR = crate::BitReader;
#[doc = "Field `SOFT_RESET` writer - 0:0\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0. It has the same effect as the hardware reset. Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and empty all the mailboxes"]
pub type SoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0. It has the same effect as the hardware reset. Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and empty all the mailboxes"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SoftResetR {
        SoftResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0. It has the same effect as the hardware reset. Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and empty all the mailboxes"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SoftResetW<Regs3MailboxSysconfigSpec> {
        SoftResetW::new(self, 0)
    }
}
#[doc = "This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs3_mailbox_sysconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs3_mailbox_sysconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs3MailboxSysconfigSpec;
impl crate::RegisterSpec for Regs3MailboxSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs3_mailbox_sysconfig::R`](R) reader structure"]
impl crate::Readable for Regs3MailboxSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`regs3_mailbox_sysconfig::W`](W) writer structure"]
impl crate::Writable for Regs3MailboxSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS3_MAILBOX_SYSCONFIG to value 0"]
impl crate::Resettable for Regs3MailboxSysconfigSpec {
    const RESET_VALUE: u32 = 0;
}
