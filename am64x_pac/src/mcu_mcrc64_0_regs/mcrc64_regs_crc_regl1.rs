#[doc = "Register `MCRC64_REGS_CRC_REGL1` reader"]
pub type R = crate::R<Mcrc64RegsCrcRegl1Spec>;
#[doc = "Register `MCRC64_REGS_CRC_REGL1` writer"]
pub type W = crate::W<Mcrc64RegsCrcRegl1Spec>;
#[doc = "Field `CRC1` reader - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
register."]
pub type Crc1R = crate::FieldReader<u32>;
#[doc = "Field `CRC1` writer - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
register."]
pub type Crc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
register."]
    #[inline(always)]
    pub fn crc1(&self) -> Crc1R {
        Crc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn crc1(&mut self) -> Crc1W<Mcrc64RegsCrcRegl1Spec> {
        Crc1W::new(self, 0)
    }
}
#[doc = "Channel 1 CRC value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcRegl1Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcRegl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_regl1::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcRegl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_regl1::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcRegl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_REGL1 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcRegl1Spec {
    const RESET_VALUE: u32 = 0;
}
