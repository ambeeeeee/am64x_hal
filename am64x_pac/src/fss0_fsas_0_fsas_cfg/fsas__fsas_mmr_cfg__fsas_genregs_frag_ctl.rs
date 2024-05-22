#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_CTL` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_CTL` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec>;
#[doc = "Field `FRAG_LO` reader - 0:0\\]
When set any address less than frag_addr will be fragmented to 16 bits"]
pub type FragLoR = crate::BitReader;
#[doc = "Field `FRAG_LO` writer - 0:0\\]
When set any address less than frag_addr will be fragmented to 16 bits"]
pub type FragLoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAG_HI` reader - 1:1\\]
When set any address greater than or equal to frag_addr will be fragmented to 16 bits"]
pub type FragHiR = crate::BitReader;
#[doc = "Field `FRAG_HI` writer - 1:1\\]
When set any address greater than or equal to frag_addr will be fragmented to 16 bits"]
pub type FragHiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set any address less than frag_addr will be fragmented to 16 bits"]
    #[inline(always)]
    pub fn frag_lo(&self) -> FragLoR {
        FragLoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set any address greater than or equal to frag_addr will be fragmented to 16 bits"]
    #[inline(always)]
    pub fn frag_hi(&self) -> FragHiR {
        FragHiR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set any address less than frag_addr will be fragmented to 16 bits"]
    #[inline(always)]
    #[must_use]
    pub fn frag_lo(&mut self) -> FragLoW<Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec> {
        FragLoW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set any address greater than or equal to frag_addr will be fragmented to 16 bits"]
    #[inline(always)]
    #[must_use]
    pub fn frag_hi(&mut self) -> FragHiW<Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec> {
        FragHiW::new(self, 1)
    }
}
#[doc = "The FRAG_CTL determins which frag region is fragmented\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_CTL to value 0x03"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec {
    const RESET_VALUE: u32 = 0x03;
}
