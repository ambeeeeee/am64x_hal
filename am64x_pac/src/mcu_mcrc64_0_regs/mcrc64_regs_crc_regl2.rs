#[doc = "Register `MCRC64_REGS_CRC_REGL2` reader"]
pub type R = crate::R<Mcrc64RegsCrcRegl2Spec>;
#[doc = "Register `MCRC64_REGS_CRC_REGL2` writer"]
pub type W = crate::W<Mcrc64RegsCrcRegl2Spec>;
#[doc = "Field `CRC2` reader - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
register."]
pub type Crc2R = crate::FieldReader<u32>;
#[doc = "Field `CRC2` writer - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
register."]
pub type Crc2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
register."]
    #[inline(always)]
    pub fn crc2(&self) -> Crc2R {
        Crc2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn crc2(&mut self) -> Crc2W<Mcrc64RegsCrcRegl2Spec> {
        Crc2W::new(self, 0)
    }
}
#[doc = "Channel 2 CRC value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcRegl2Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcRegl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_regl2::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcRegl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_regl2::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcRegl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_REGL2 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcRegl2Spec {
    const RESET_VALUE: u32 = 0;
}
