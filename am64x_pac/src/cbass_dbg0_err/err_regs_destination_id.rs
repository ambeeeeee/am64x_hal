#[doc = "Register `ERR_REGS_destination_id` reader"]
pub type R = crate::R<ErrRegsDestinationIdSpec>;
#[doc = "Register `ERR_REGS_destination_id` writer"]
pub type W = crate::W<ErrRegsDestinationIdSpec>;
#[doc = "Field `DEST_ID` reader - 7:0\\]
The destination ID."]
pub type DestIdR = crate::FieldReader;
#[doc = "Field `DEST_ID` writer - 7:0\\]
The destination ID."]
pub type DestIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The destination ID."]
    #[inline(always)]
    pub fn dest_id(&self) -> DestIdR {
        DestIdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The destination ID."]
    #[inline(always)]
    #[must_use]
    pub fn dest_id(&mut self) -> DestIdW<ErrRegsDestinationIdSpec> {
        DestIdW::new(self, 0)
    }
}
#[doc = "The Destination ID Register defines the destination ID value for error messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_destination_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_destination_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRegsDestinationIdSpec;
impl crate::RegisterSpec for ErrRegsDestinationIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_regs_destination_id::R`](R) reader structure"]
impl crate::Readable for ErrRegsDestinationIdSpec {}
#[doc = "`write(|w| ..)` method takes [`err_regs_destination_id::W`](W) writer structure"]
impl crate::Writable for ErrRegsDestinationIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_REGS_destination_id to value 0"]
impl crate::Resettable for ErrRegsDestinationIdSpec {
    const RESET_VALUE: u32 = 0;
}
