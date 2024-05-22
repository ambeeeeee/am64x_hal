#[doc = "Register `REGS7_MAILBOX_IRQ_EOI` reader"]
pub type R = crate::R<Regs7MailboxIrqEoiSpec>;
#[doc = "Register `REGS7_MAILBOX_IRQ_EOI` writer"]
pub type W = crate::W<Regs7MailboxIrqEoiSpec>;
#[doc = "Field `EOI0` reader - 0:0\\]
Software EOI signal for the user 0 interrupt"]
pub type Eoi0R = crate::BitReader;
#[doc = "Field `EOI0` writer - 0:0\\]
Software EOI signal for the user 0 interrupt"]
pub type Eoi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOI1` reader - 1:1\\]
Software EOI signal for the user 1 interrupt"]
pub type Eoi1R = crate::BitReader;
#[doc = "Field `EOI1` writer - 1:1\\]
Software EOI signal for the user 1 interrupt"]
pub type Eoi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOI2` reader - 2:2\\]
Software EOI signal for the user 2 interrupt"]
pub type Eoi2R = crate::BitReader;
#[doc = "Field `EOI2` writer - 2:2\\]
Software EOI signal for the user 2 interrupt"]
pub type Eoi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOI3` reader - 3:3\\]
Software EOI signal for the user 3 interrupt"]
pub type Eoi3R = crate::BitReader;
#[doc = "Field `EOI3` writer - 3:3\\]
Software EOI signal for the user 3 interrupt"]
pub type Eoi3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software EOI signal for the user 0 interrupt"]
    #[inline(always)]
    pub fn eoi0(&self) -> Eoi0R {
        Eoi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software EOI signal for the user 1 interrupt"]
    #[inline(always)]
    pub fn eoi1(&self) -> Eoi1R {
        Eoi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software EOI signal for the user 2 interrupt"]
    #[inline(always)]
    pub fn eoi2(&self) -> Eoi2R {
        Eoi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software EOI signal for the user 3 interrupt"]
    #[inline(always)]
    pub fn eoi3(&self) -> Eoi3R {
        Eoi3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software EOI signal for the user 0 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eoi0(&mut self) -> Eoi0W<Regs7MailboxIrqEoiSpec> {
        Eoi0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software EOI signal for the user 1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eoi1(&mut self) -> Eoi1W<Regs7MailboxIrqEoiSpec> {
        Eoi1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Software EOI signal for the user 2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eoi2(&mut self) -> Eoi2W<Regs7MailboxIrqEoiSpec> {
        Eoi2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software EOI signal for the user 3 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eoi3(&mut self) -> Eoi3W<Regs7MailboxIrqEoiSpec> {
        Eoi3W::new(self, 3)
    }
}
#[doc = "This is the EOI register with which the software is enabled to do the interrupt clearance.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_irq_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_irq_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs7MailboxIrqEoiSpec;
impl crate::RegisterSpec for Regs7MailboxIrqEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs7_mailbox_irq_eoi::R`](R) reader structure"]
impl crate::Readable for Regs7MailboxIrqEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`regs7_mailbox_irq_eoi::W`](W) writer structure"]
impl crate::Writable for Regs7MailboxIrqEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS7_MAILBOX_IRQ_EOI to value 0"]
impl crate::Resettable for Regs7MailboxIrqEoiSpec {
    const RESET_VALUE: u32 = 0;
}
