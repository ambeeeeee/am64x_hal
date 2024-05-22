#[doc = "Register `REGS0_MAILBOX_FIFOSTATUS` reader"]
pub type R = crate::R<Regs0MailboxFifostatusSpec>;
#[doc = "Register `REGS0_MAILBOX_FIFOSTATUS` writer"]
pub type W = crate::W<Regs0MailboxFifostatusSpec>;
#[doc = "Field `FIFO_FULL` reader - 0:0\\]
Full flag for Mailbox m"]
pub type FifoFullR = crate::BitReader;
#[doc = "Field `FIFO_FULL` writer - 0:0\\]
Full flag for Mailbox m"]
pub type FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Full flag for Mailbox m"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Full flag for Mailbox m"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_full(&mut self) -> FifoFullW<Regs0MailboxFifostatusSpec> {
        FifoFullW::new(self, 0)
    }
}
#[doc = "The FIFO status register has the status of the Mailbox\\[a\\]
FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs0_mailbox_fifostatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs0_mailbox_fifostatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs0MailboxFifostatusSpec;
impl crate::RegisterSpec for Regs0MailboxFifostatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs0_mailbox_fifostatus::R`](R) reader structure"]
impl crate::Readable for Regs0MailboxFifostatusSpec {}
#[doc = "`write(|w| ..)` method takes [`regs0_mailbox_fifostatus::W`](W) writer structure"]
impl crate::Writable for Regs0MailboxFifostatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS0_MAILBOX_FIFOSTATUS to value 0"]
impl crate::Resettable for Regs0MailboxFifostatusSpec {
    const RESET_VALUE: u32 = 0;
}