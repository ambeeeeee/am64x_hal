#[doc = "Register `MCRC64_REGS_CRC_SCOUNT_REG2` reader"]
pub type R = crate::R<Mcrc64RegsCrcScountReg2Spec>;
#[doc = "Register `MCRC64_REGS_CRC_SCOUNT_REG2` writer"]
pub type W = crate::W<Mcrc64RegsCrcScountReg2Spec>;
#[doc = "Field `CRC_SEC_COUNT2` reader - 15:0\\]
Channel 2 Sector Counter Preload Register. This register contains the number of sectors in one block of memory."]
pub type CrcSecCount2R = crate::FieldReader<u16>;
#[doc = "Field `CRC_SEC_COUNT2` writer - 15:0\\]
Channel 2 Sector Counter Preload Register. This register contains the number of sectors in one block of memory."]
pub type CrcSecCount2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Channel 2 Sector Counter Preload Register. This register contains the number of sectors in one block of memory."]
    #[inline(always)]
    pub fn crc_sec_count2(&self) -> CrcSecCount2R {
        CrcSecCount2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Channel 2 Sector Counter Preload Register. This register contains the number of sectors in one block of memory."]
    #[inline(always)]
    #[must_use]
    pub fn crc_sec_count2(&mut self) -> CrcSecCount2W<Mcrc64RegsCrcScountReg2Spec> {
        CrcSecCount2W::new(self, 0)
    }
}
#[doc = "CRC Sector Counter Preload Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_scount_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_scount_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcScountReg2Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcScountReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_scount_reg2::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcScountReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_scount_reg2::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcScountReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_SCOUNT_REG2 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcScountReg2Spec {
    const RESET_VALUE: u32 = 0;
}
