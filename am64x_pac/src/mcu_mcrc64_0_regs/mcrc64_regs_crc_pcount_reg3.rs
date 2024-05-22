#[doc = "Register `MCRC64_REGS_CRC_PCOUNT_REG3` reader"]
pub type R = crate::R<Mcrc64RegsCrcPcountReg3Spec>;
#[doc = "Register `MCRC64_REGS_CRC_PCOUNT_REG3` writer"]
pub type W = crate::W<Mcrc64RegsCrcPcountReg3Spec>;
#[doc = "Field `CRC_PAT_COUNT3` reader - 19:0\\]
Channel 3 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount3R = crate::FieldReader<u32>;
#[doc = "Field `CRC_PAT_COUNT3` writer - 19:0\\]
Channel 3 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount3W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Channel 3 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    pub fn crc_pat_count3(&self) -> CrcPatCount3R {
        CrcPatCount3R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Channel 3 Pattern Counter Preload Register. This register contains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    #[must_use]
    pub fn crc_pat_count3(&mut self) -> CrcPatCount3W<Mcrc64RegsCrcPcountReg3Spec> {
        CrcPatCount3W::new(self, 0)
    }
}
#[doc = "CRC Pattern Counter Preload Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_pcount_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_pcount_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcPcountReg3Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcPcountReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_pcount_reg3::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcPcountReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_pcount_reg3::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcPcountReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_PCOUNT_REG3 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcPcountReg3Spec {
    const RESET_VALUE: u32 = 0;
}
