#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_EOI` reader"]
pub type R = crate::R<Fsas_FsasMmrCfg_FsasGenregsEoiSpec>;
#[doc = "Register `FSAS__FSAS_MMR_CFG__FSAS_GENREGS_EOI` writer"]
pub type W = crate::W<Fsas_FsasMmrCfg_FsasGenregsEoiSpec>;
#[doc = "Field `EOI_VECTOR` reader - 0:0\\]
Write with bit position of targeted interrupt. (E.g. Ext FSS ECC is bit 0). Upon write, level interrupt will clear and if un-serviced will issue another pulse interrupt"]
pub type EoiVectorR = crate::BitReader;
#[doc = "Field `EOI_VECTOR` writer - 0:0\\]
Write with bit position of targeted interrupt. (E.g. Ext FSS ECC is bit 0). Upon write, level interrupt will clear and if un-serviced will issue another pulse interrupt"]
pub type EoiVectorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write with bit position of targeted interrupt. (E.g. Ext FSS ECC is bit 0). Upon write, level interrupt will clear and if un-serviced will issue another pulse interrupt"]
    #[inline(always)]
    pub fn eoi_vector(&self) -> EoiVectorR {
        EoiVectorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write with bit position of targeted interrupt. (E.g. Ext FSS ECC is bit 0). Upon write, level interrupt will clear and if un-serviced will issue another pulse interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eoi_vector(&mut self) -> EoiVectorW<Fsas_FsasMmrCfg_FsasGenregsEoiSpec> {
        EoiVectorW::new(self, 0)
    }
}
#[doc = "The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasMmrCfg_FsasGenregsEoiSpec;
impl crate::RegisterSpec for Fsas_FsasMmrCfg_FsasGenregsEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_mmr_cfg__fsas_genregs_eoi::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasMmrCfg_FsasGenregsEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_mmr_cfg__fsas_genregs_eoi::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasMmrCfg_FsasGenregsEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_MMR_CFG__FSAS_GENREGS_EOI to value 0"]
impl crate::Resettable for Fsas_FsasMmrCfg_FsasGenregsEoiSpec {
    const RESET_VALUE: u32 = 0;
}
