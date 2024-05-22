#[doc = "Register `MCRC64_REGS_CRC_REGL4` reader"]
pub type R = crate::R<Mcrc64RegsCrcRegl4Spec>;
#[doc = "Register `MCRC64_REGS_CRC_REGL4` writer"]
pub type W = crate::W<Mcrc64RegsCrcRegl4Spec>;
#[doc = "Field `CRC4` reader - 31:0\\]
Channel 4 CRC Value Low Register."]
pub type Crc4R = crate::FieldReader<u32>;
#[doc = "Field `CRC4` writer - 31:0\\]
Channel 4 CRC Value Low Register."]
pub type Crc4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 4 CRC Value Low Register."]
    #[inline(always)]
    pub fn crc4(&self) -> Crc4R {
        Crc4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 4 CRC Value Low Register."]
    #[inline(always)]
    #[must_use]
    pub fn crc4(&mut self) -> Crc4W<Mcrc64RegsCrcRegl4Spec> {
        Crc4W::new(self, 0)
    }
}
#[doc = "Channel 4 CRC value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcRegl4Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcRegl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_regl4::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcRegl4Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_regl4::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcRegl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_REGL4 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcRegl4Spec {
    const RESET_VALUE: u32 = 0;
}
