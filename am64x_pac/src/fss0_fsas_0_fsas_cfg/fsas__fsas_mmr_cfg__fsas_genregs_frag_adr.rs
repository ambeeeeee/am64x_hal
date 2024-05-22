#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_ADR` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_ADR` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec>;
#[doc = "Field `FRAG_ADDR` reader - 31:0\\]
This address is used to determine the boundary of frag_hi and flag_lo"]
pub type FragAddrR = crate::FieldReader<u32>;
#[doc = "Field `FRAG_ADDR` writer - 31:0\\]
This address is used to determine the boundary of frag_hi and flag_lo"]
pub type FragAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This address is used to determine the boundary of frag_hi and flag_lo"]
    #[inline(always)]
    pub fn frag_addr(&self) -> FragAddrR {
        FragAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This address is used to determine the boundary of frag_hi and flag_lo"]
    #[inline(always)]
    #[must_use]
    pub fn frag_addr(&mut self) -> FragAddrW<Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec> {
        FragAddrW::new(self, 0)
    }
}
#[doc = "This FRAG_ADR is the address of a request that frag_hi or frag_lo boundary occurs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_frag_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_frag_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_frag_adr::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_frag_adr::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_ADR to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec {
    const RESET_VALUE: u32 = 0;
}
