#[doc = "Register `INTAGGR_IMAP_INTMAP` reader"]
pub type R = crate::R<IntaggrImapIntmapSpec>;
#[doc = "Register `INTAGGR_IMAP_INTMAP` writer"]
pub type W = crate::W<IntaggrImapIntmapSpec>;
#[doc = "Field `BITNUM` reader - 5:0\\]
Virtual interrupt cause register bit number: this field specifies which of the 64 bits in the specified virtual interrupt cause register the event pending bit will appear in."]
pub type BitnumR = crate::FieldReader;
#[doc = "Field `BITNUM` writer - 5:0\\]
Virtual interrupt cause register bit number: this field specifies which of the 64 bits in the specified virtual interrupt cause register the event pending bit will appear in."]
pub type BitnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REGNUM` reader - 16:8\\]
Virtual interrupt status register number: this field specifies which of the potential virtual interrupt cause registers the event pending bit will appear in."]
pub type RegnumR = crate::FieldReader<u16>;
#[doc = "Field `REGNUM` writer - 16:8\\]
Virtual interrupt status register number: this field specifies which of the potential virtual interrupt cause registers the event pending bit will appear in."]
pub type RegnumW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Virtual interrupt cause register bit number: this field specifies which of the 64 bits in the specified virtual interrupt cause register the event pending bit will appear in."]
    #[inline(always)]
    pub fn bitnum(&self) -> BitnumR {
        BitnumR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Virtual interrupt status register number: this field specifies which of the potential virtual interrupt cause registers the event pending bit will appear in."]
    #[inline(always)]
    pub fn regnum(&self) -> RegnumR {
        RegnumR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Virtual interrupt cause register bit number: this field specifies which of the 64 bits in the specified virtual interrupt cause register the event pending bit will appear in."]
    #[inline(always)]
    #[must_use]
    pub fn bitnum(&mut self) -> BitnumW<IntaggrImapIntmapSpec> {
        BitnumW::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Virtual interrupt status register number: this field specifies which of the potential virtual interrupt cause registers the event pending bit will appear in."]
    #[inline(always)]
    #[must_use]
    pub fn regnum(&mut self) -> RegnumW<IntaggrImapIntmapSpec> {
        RegnumW::new(self, 8)
    }
}
#[doc = "The Interrupt Mapping Register controls which of N virtual interrupt source outputs this channels physical interrupt sources will map onto.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_imap_intmap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_imap_intmap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrImapIntmapSpec;
impl crate::RegisterSpec for IntaggrImapIntmapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_imap_intmap::R`](R) reader structure"]
impl crate::Readable for IntaggrImapIntmapSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_imap_intmap::W`](W) writer structure"]
impl crate::Writable for IntaggrImapIntmapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_IMAP_INTMAP to value 0"]
impl crate::Resettable for IntaggrImapIntmapSpec {
    const RESET_VALUE: u64 = 0;
}
