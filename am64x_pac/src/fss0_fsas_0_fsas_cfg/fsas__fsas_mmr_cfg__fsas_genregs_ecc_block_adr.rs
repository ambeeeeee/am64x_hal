#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_BLOCK_ADR` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_BLOCK_ADR` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec>;
#[doc = "Field `ECC_ERROR_BLOCK_ADDR` reader - 31:5\\]
ECC 32 byte aligned block address"]
pub type EccErrorBlockAddrR = crate::FieldReader<u32>;
#[doc = "Field `ECC_ERROR_BLOCK_ADDR` writer - 31:5\\]
ECC 32 byte aligned block address"]
pub type EccErrorBlockAddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - 31:5\\]
ECC 32 byte aligned block address"]
    #[inline(always)]
    pub fn ecc_error_block_addr(&self) -> EccErrorBlockAddrR {
        EccErrorBlockAddrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\]
ECC 32 byte aligned block address"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_error_block_addr(
        &mut self,
    ) -> EccErrorBlockAddrW<Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec> {
        EccErrorBlockAddrW::new(self, 5)
    }
}
#[doc = "The ERR_ECC_BLOCK_ADR register holds the current top of stack ECC error block address, this is only valid when the ecc_err_valid is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_BLOCK_ADR to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec {
    const RESET_VALUE: u32 = 0;
}
