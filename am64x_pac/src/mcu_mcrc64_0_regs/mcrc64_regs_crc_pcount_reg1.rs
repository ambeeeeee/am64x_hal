#[doc = "Register `MCRC64_REGS_CRC_PCOUNT_REG1` reader"]
pub type R = crate::R<Mcrc64RegsCrcPcountReg1Spec>;
#[doc = "Register `MCRC64_REGS_CRC_PCOUNT_REG1` writer"]
pub type W = crate::W<Mcrc64RegsCrcPcountReg1Spec>;
#[doc = "Field `CRC_PAT_COUNT1` reader - 19:0\\]
Channel 1 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount1R = crate::FieldReader<u32>;
#[doc = "Field `CRC_PAT_COUNT1` writer - 19:0\\]
Channel 1 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Channel 1 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    pub fn crc_pat_count1(&self) -> CrcPatCount1R {
        CrcPatCount1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Channel 1 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    #[must_use]
    pub fn crc_pat_count1(&mut self) -> CrcPatCount1W<Mcrc64RegsCrcPcountReg1Spec> {
        CrcPatCount1W::new(self, 0)
    }
}
#[doc = "CRC Pattern Counter Preload Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_pcount_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_pcount_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcPcountReg1Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcPcountReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_pcount_reg1::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcPcountReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_pcount_reg1::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcPcountReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_PCOUNT_REG1 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcPcountReg1Spec {
    const RESET_VALUE: u32 = 0;
}
