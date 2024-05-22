#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSTRT` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSTRT` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec>;
#[doc = "Field `R_START` reader - 19:0\\]
This defines the start of the ECC region in 4KBytes steps. Address start = {start\\[19:0\\], 0x000} 0x0 means the start is 0x0000_0000 0x1 means the start is 0x0000_1000 0xA means the start is 0x0000_A000 Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
pub type RStartR = crate::FieldReader<u32>;
#[doc = "Field `R_START` writer - 19:0\\]
This defines the start of the ECC region in 4KBytes steps. Address start = {start\\[19:0\\], 0x000} 0x0 means the start is 0x0000_0000 0x1 means the start is 0x0000_1000 0xA means the start is 0x0000_A000 Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
pub type RStartW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This defines the start of the ECC region in 4KBytes steps. Address start = {start\\[19:0\\], 0x000} 0x0 means the start is 0x0000_0000 0x1 means the start is 0x0000_1000 0xA means the start is 0x0000_A000 Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
    #[inline(always)]
    pub fn r_start(&self) -> RStartR {
        RStartR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This defines the start of the ECC region in 4KBytes steps. Address start = {start\\[19:0\\], 0x000} 0x0 means the start is 0x0000_0000 0x1 means the start is 0x0000_1000 0xA means the start is 0x0000_A000 Note the offset + size should be &lt;= 4GBytes, wrap around is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn r_start(&mut self) -> RStartW<Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec> {
        RStartW::new(self, 0)
    }
}
#[doc = "This defines the start of the ECC region in 4KBytes steps.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSTRT to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec {
    const RESET_VALUE: u32 = 0;
}
