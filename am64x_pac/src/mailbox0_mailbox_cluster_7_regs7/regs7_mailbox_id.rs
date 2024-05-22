#[doc = "Register `REGS7_MAILBOX_ID` reader"]
pub type R = crate::R<Regs7MailboxIdSpec>;
#[doc = "Register `REGS7_MAILBOX_ID` writer"]
pub type W = crate::W<Regs7MailboxIdSpec>;
#[doc = "Field `MINOR_REV` reader - 5:0\\]
Minor revision. Y of X.Y.R.Z"]
pub type MinorRevR = crate::FieldReader;
#[doc = "Field `MINOR_REV` writer - 5:0\\]
Minor revision. Y of X.Y.R.Z"]
pub type MinorRevW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Special version number"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Special version number"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR_REV` reader - 10:8\\]
Major revision. X of X.Y.R.Z"]
pub type MajorRevR = crate::FieldReader;
#[doc = "Field `MAJOR_REV` writer - 10:8\\]
Major revision. X of X.Y.R.Z"]
pub type MajorRevW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VER` reader - 15:11\\]
RTL version. R of X.Y.R.Z"]
pub type RtlVerR = crate::FieldReader;
#[doc = "Field `RTL_VER` writer - 15:11\\]
RTL version. R of X.Y.R.Z"]
pub type RtlVerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNCTION` reader - 27:16\\]
Module family."]
pub type FunctionR = crate::FieldReader<u16>;
#[doc = "Field `FUNCTION` writer - 27:16\\]
Module family."]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
BU identifier"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
BU identifier"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Used to distinguish which ID numbering scheme is used."]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Used to distinguish which ID numbering scheme is used."]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision. Y of X.Y.R.Z"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MinorRevR {
        MinorRevR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Special version number"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision. X of X.Y.R.Z"]
    #[inline(always)]
    pub fn major_rev(&self) -> MajorRevR {
        MajorRevR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version. R of X.Y.R.Z"]
    #[inline(always)]
    pub fn rtl_ver(&self) -> RtlVerR {
        RtlVerR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module family."]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
BU identifier"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Used to distinguish which ID numbering scheme is used."]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision. Y of X.Y.R.Z"]
    #[inline(always)]
    #[must_use]
    pub fn minor_rev(&mut self) -> MinorRevW<Regs7MailboxIdSpec> {
        MinorRevW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Special version number"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<Regs7MailboxIdSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision. X of X.Y.R.Z"]
    #[inline(always)]
    #[must_use]
    pub fn major_rev(&mut self) -> MajorRevW<Regs7MailboxIdSpec> {
        MajorRevW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version. R of X.Y.R.Z"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_ver(&mut self) -> RtlVerW<Regs7MailboxIdSpec> {
        RtlVerW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module family."]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<Regs7MailboxIdSpec> {
        FunctionW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
BU identifier"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<Regs7MailboxIdSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Used to distinguish which ID numbering scheme is used."]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<Regs7MailboxIdSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "This is the standard TI peripheral ID register that exists at address 0 in the peripheral space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs7MailboxIdSpec;
impl crate::RegisterSpec for Regs7MailboxIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs7_mailbox_id::R`](R) reader structure"]
impl crate::Readable for Regs7MailboxIdSpec {}
#[doc = "`write(|w| ..)` method takes [`regs7_mailbox_id::W`](W) writer structure"]
impl crate::Writable for Regs7MailboxIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS7_MAILBOX_ID to value 0x7788_b900"]
impl crate::Resettable for Regs7MailboxIdSpec {
    const RESET_VALUE: u32 = 0x7788_b900;
}
