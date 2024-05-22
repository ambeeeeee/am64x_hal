#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ENABLE_SET` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ENABLE_SET` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec>;
#[doc = "Field `ECC_ERROR_1BIT` reader - 0:0\\]
ECC error on 1 bits. correctable"]
pub type EccError1bitR = crate::BitReader;
#[doc = "Field `ECC_ERROR_1BIT` writer - 0:0\\]
ECC error on 1 bits. correctable"]
pub type EccError1bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERROR_2BIT` reader - 1:1\\]
ECC error on 2 bits. Not correctable"]
pub type EccError2bitR = crate::BitReader;
#[doc = "Field `ECC_ERROR_2BIT` writer - 1:1\\]
ECC error on 2 bits. Not correctable"]
pub type EccError2bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_WRITE_NONALIGN` reader - 2:2\\]
Write is not aligned to 32B boundary or not a multiple of 32B"]
pub type EccWriteNonalignR = crate::BitReader;
#[doc = "Field `ECC_WRITE_NONALIGN` writer - 2:2\\]
Write is not aligned to 32B boundary or not a multiple of 32B"]
pub type EccWriteNonalignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ECC error on 1 bits. correctable"]
    #[inline(always)]
    pub fn ecc_error_1bit(&self) -> EccError1bitR {
        EccError1bitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ECC error on 2 bits. Not correctable"]
    #[inline(always)]
    pub fn ecc_error_2bit(&self) -> EccError2bitR {
        EccError2bitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write is not aligned to 32B boundary or not a multiple of 32B"]
    #[inline(always)]
    pub fn ecc_write_nonalign(&self) -> EccWriteNonalignR {
        EccWriteNonalignR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ECC error on 1 bits. correctable"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_error_1bit(&mut self) -> EccError1bitW<Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec> {
        EccError1bitW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ECC error on 2 bits. Not correctable"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_error_2bit(&mut self) -> EccError2bitW<Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec> {
        EccError2bitW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write is not aligned to 32B boundary or not a multiple of 32B"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_write_nonalign(
        &mut self,
    ) -> EccWriteNonalignW<Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec> {
        EccWriteNonalignW::new(self, 2)
    }
}
#[doc = "The IRQ_ENABLE_SET register allows the interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_enable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_enable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_enable_set::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_enable_set::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ENABLE_SET to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
