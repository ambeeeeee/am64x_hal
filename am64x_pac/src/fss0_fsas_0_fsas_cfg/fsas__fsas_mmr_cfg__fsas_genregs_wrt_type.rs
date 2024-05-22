#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_WRT_TYPE` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_WRT_TYPE` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec>;
#[doc = "Field `WRT_ERR_ROUTEID` reader - 11:0\\]
Indicates the Route ID for the Master that caused the write error"]
pub type WrtErrRouteidR = crate::FieldReader<u16>;
#[doc = "Field `WRT_ERR_ROUTEID` writer - 11:0\\]
Indicates the Route ID for the Master that caused the write error"]
pub type WrtErrRouteidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WRT_ERR_ADR` reader - 12:12\\]
When set indicates that there was a write error due to a non-aligned address"]
pub type WrtErrAdrR = crate::BitReader;
#[doc = "Field `WRT_ERR_ADR` writer - 12:12\\]
When set indicates that there was a write error due to a non-aligned address"]
pub type WrtErrAdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRT_ERR_BEN` reader - 13:13\\]
When set indicates that there was a write error due to a non-contiguous byte enables"]
pub type WrtErrBenR = crate::BitReader;
#[doc = "Field `WRT_ERR_BEN` writer - 13:13\\]
When set indicates that there was a write error due to a non-contiguous byte enables"]
pub type WrtErrBenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRT_ERR_VALID` reader - 31:31\\]
When set indicates that there is valid write error information available, Writing a one to this register will pop the top of the stack"]
pub type WrtErrValidR = crate::BitReader;
#[doc = "Field `WRT_ERR_VALID` writer - 31:31\\]
When set indicates that there is valid write error information available, Writing a one to this register will pop the top of the stack"]
pub type WrtErrValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the Route ID for the Master that caused the write error"]
    #[inline(always)]
    pub fn wrt_err_routeid(&self) -> WrtErrRouteidR {
        WrtErrRouteidR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - 12:12\\]
When set indicates that there was a write error due to a non-aligned address"]
    #[inline(always)]
    pub fn wrt_err_adr(&self) -> WrtErrAdrR {
        WrtErrAdrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
When set indicates that there was a write error due to a non-contiguous byte enables"]
    #[inline(always)]
    pub fn wrt_err_ben(&self) -> WrtErrBenR {
        WrtErrBenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set indicates that there is valid write error information available, Writing a one to this register will pop the top of the stack"]
    #[inline(always)]
    pub fn wrt_err_valid(&self) -> WrtErrValidR {
        WrtErrValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the Route ID for the Master that caused the write error"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_err_routeid(&mut self) -> WrtErrRouteidW<Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec> {
        WrtErrRouteidW::new(self, 0)
    }
    #[doc = "Bit 12 - 12:12\\]
When set indicates that there was a write error due to a non-aligned address"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_err_adr(&mut self) -> WrtErrAdrW<Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec> {
        WrtErrAdrW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
When set indicates that there was a write error due to a non-contiguous byte enables"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_err_ben(&mut self) -> WrtErrBenW<Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec> {
        WrtErrBenW::new(self, 13)
    }
    #[doc = "Bit 31 - 31:31\\]
When set indicates that there is valid write error information available, Writing a one to this register will pop the top of the stack"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_err_valid(&mut self) -> WrtErrValidW<Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec> {
        WrtErrValidW::new(self, 31)
    }
}
#[doc = "The ERR_WRT_TYPE register holds the current top of stack write error info. this is only valid when the wrt_err_valid is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_wrt_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_wrt_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_wrt_type::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_wrt_type::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_WRT_TYPE to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec {
    const RESET_VALUE: u32 = 0;
}
