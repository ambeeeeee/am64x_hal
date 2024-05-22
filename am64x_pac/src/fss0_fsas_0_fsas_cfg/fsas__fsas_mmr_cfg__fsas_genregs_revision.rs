#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_REVISION` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsRevisionSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_REVISION` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsRevisionSpec>;
#[doc = "Field `REVMIN` reader - 5:0\\]
Minor revision"]
pub type RevminR = crate::FieldReader;
#[doc = "Field `REVMIN` writer - 5:0\\]
Minor revision"]
pub type RevminW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REVMAJ` reader - 10:8\\]
Major revision"]
pub type RevmajR = crate::FieldReader;
#[doc = "Field `REVMAJ` writer - 10:8\\]
Major revision"]
pub type RevmajW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REVRTL` reader - 15:11\\]
RTL revision. Will vary depending on release."]
pub type RevrtlR = crate::FieldReader;
#[doc = "Field `REVRTL` writer - 15:11\\]
RTL revision. Will vary depending on release."]
pub type RevrtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MODID` reader - 31:16\\]
Module ID field"]
pub type ModidR = crate::FieldReader<u16>;
#[doc = "Field `MODID` writer - 31:16\\]
Module ID field"]
pub type ModidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    pub fn revmin(&self) -> RevminR {
        RevminR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    pub fn revmaj(&self) -> RevmajR {
        RevmajR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision. Will vary depending on release."]
    #[inline(always)]
    pub fn revrtl(&self) -> RevrtlR {
        RevrtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID field"]
    #[inline(always)]
    pub fn modid(&self) -> ModidR {
        ModidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    #[must_use]
    pub fn revmin(&mut self) -> RevminW<Fsas_FsasMmrCfg_FsasGenregsRevisionSpec> {
        RevminW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<Fsas_FsasMmrCfg_FsasGenregsRevisionSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    #[must_use]
    pub fn revmaj(&mut self) -> RevmajW<Fsas_FsasMmrCfg_FsasGenregsRevisionSpec> {
        RevmajW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision. Will vary depending on release."]
    #[inline(always)]
    #[must_use]
    pub fn revrtl(&mut self) -> RevrtlW<Fsas_FsasMmrCfg_FsasGenregsRevisionSpec> {
        RevrtlW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID field"]
    #[inline(always)]
    #[must_use]
    pub fn modid(&mut self) -> ModidW<Fsas_FsasMmrCfg_FsasGenregsRevisionSpec> {
        ModidW::new(self, 16)
    }
}
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_revision::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_revision::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsRevisionSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsRevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_revision::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsRevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_revision::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsRevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_REVISION to value 0x1901"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsRevisionSpec {
    const RESET_VALUE: u32 = 0x1901;
}