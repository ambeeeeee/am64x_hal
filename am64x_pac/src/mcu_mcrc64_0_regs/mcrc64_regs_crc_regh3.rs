#[doc = "Register `MCRC64_REGS_CRC_REGH3` reader"]
pub type R = crate::R<Mcrc64RegsCrcRegh3Spec>;
#[doc = "Register `MCRC64_REGS_CRC_REGH3` writer"]
pub type W = crate::W<Mcrc64RegsCrcRegh3Spec>;
#[doc = "Field `CRC3_63_32` reader - 31:0\\]
Channel 3 CRC Value High Register. This register contains the current known good signature value stored at CRC3\\[63:32\\]
register."]
pub type Crc3_63_32R = crate::FieldReader<u32>;
#[doc = "Field `CRC3_63_32` writer - 31:0\\]
Channel 3 CRC Value High Register. This register contains the current known good signature value stored at CRC3\\[63:32\\]
register."]
pub type Crc3_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 3 CRC Value High Register. This register contains the current known good signature value stored at CRC3\\[63:32\\]
register."]
    #[inline(always)]
    pub fn crc3_63_32(&self) -> Crc3_63_32R {
        Crc3_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 3 CRC Value High Register. This register contains the current known good signature value stored at CRC3\\[63:32\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn crc3_63_32(&mut self) -> Crc3_63_32W<Mcrc64RegsCrcRegh3Spec> {
        Crc3_63_32W::new(self, 0)
    }
}
#[doc = "Channel 3 CRC value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regh3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regh3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcRegh3Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcRegh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_regh3::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcRegh3Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_regh3::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcRegh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_REGH3 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcRegh3Spec {
    const RESET_VALUE: u32 = 0;
}
