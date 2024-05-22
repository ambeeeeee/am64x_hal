#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSIZ` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSIZ` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec>;
#[doc = "Field `R_SIZE` reader - 19:0\\]
This defines the size of the ECC region in 4KBytes steps 0x0 means the size is zero and disabled 0x1 means the size is 4KBytes 0xA means the size is 40KBytes 0xF_FFFF means the size is 4GBytes Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
pub type RSizeR = crate::FieldReader<u32>;
#[doc = "Field `R_SIZE` writer - 19:0\\]
This defines the size of the ECC region in 4KBytes steps 0x0 means the size is zero and disabled 0x1 means the size is 4KBytes 0xA means the size is 40KBytes 0xF_FFFF means the size is 4GBytes Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
pub type RSizeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This defines the size of the ECC region in 4KBytes steps 0x0 means the size is zero and disabled 0x1 means the size is 4KBytes 0xA means the size is 40KBytes 0xF_FFFF means the size is 4GBytes Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
    #[inline(always)]
    pub fn r_size(&self) -> RSizeR {
        RSizeR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This defines the size of the ECC region in 4KBytes steps 0x0 means the size is zero and disabled 0x1 means the size is 4KBytes 0xA means the size is 40KBytes 0xF_FFFF means the size is 4GBytes Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn r_size(&mut self) -> RSizeW<Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec> {
        RSizeW::new(self, 0)
    }
}
#[doc = "This defines the size of the ECC region in 4KBytes steps.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSIZ to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec {
    const RESET_VALUE: u32 = 0;
}
