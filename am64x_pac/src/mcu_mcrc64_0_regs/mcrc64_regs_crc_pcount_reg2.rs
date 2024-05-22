#[doc = "Register `MCRC64_REGS_CRC_PCOUNT_REG2` reader"]
pub type R = crate::R<Mcrc64RegsCrcPcountReg2Spec>;
#[doc = "Register `MCRC64_REGS_CRC_PCOUNT_REG2` writer"]
pub type W = crate::W<Mcrc64RegsCrcPcountReg2Spec>;
#[doc = "Field `CRC_PAT_COUNT2` reader - 19:0\\]
CRC Pattern Counter Preload Register 2 This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount2R = crate::FieldReader<u32>;
#[doc = "Field `CRC_PAT_COUNT2` writer - 19:0\\]
CRC Pattern Counter Preload Register 2 This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
CRC Pattern Counter Preload Register 2 This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    pub fn crc_pat_count2(&self) -> CrcPatCount2R {
        CrcPatCount2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
CRC Pattern Counter Preload Register 2 This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    #[must_use]
    pub fn crc_pat_count2(&mut self) -> CrcPatCount2W<Mcrc64RegsCrcPcountReg2Spec> {
        CrcPatCount2W::new(self, 0)
    }
}
#[doc = "CRC Pattern Counter Preload Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_pcount_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_pcount_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcPcountReg2Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcPcountReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_pcount_reg2::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcPcountReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_pcount_reg2::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcPcountReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_PCOUNT_REG2 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcPcountReg2Spec {
    const RESET_VALUE: u32 = 0;
}
