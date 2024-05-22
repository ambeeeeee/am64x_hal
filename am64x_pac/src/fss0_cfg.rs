#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fss_mmr__fss_mmr_cfg__fss_genregs_revision: FssMmr_FssMmrCfg_FssGenregsRevision,
}
impl RegisterBlock {
    #[doc = "0x00 - IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
    #[inline(always)]
    pub const fn fss_mmr__fss_mmr_cfg__fss_genregs_revision(
        &self,
    ) -> &FssMmr_FssMmrCfg_FssGenregsRevision {
        &self.fss_mmr__fss_mmr_cfg__fss_genregs_revision
    }
}
#[doc = "FSS_MMR__FSS_MMR_CFG__FSS_GENREGS_REVISION (rw) register accessor: IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fss_mmr__fss_mmr_cfg__fss_genregs_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fss_mmr__fss_mmr_cfg__fss_genregs_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fss_mmr__fss_mmr_cfg__fss_genregs_revision`]
module"]
#[doc(alias = "FSS_MMR__FSS_MMR_CFG__FSS_GENREGS_REVISION")]
pub type FssMmr_FssMmrCfg_FssGenregsRevision =
    crate::Reg<fss_mmr__fss_mmr_cfg__fss_genregs_revision::FssMmr_FssMmrCfg_FssGenregsRevisionSpec>;
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
pub mod fss_mmr__fss_mmr_cfg__fss_genregs_revision;
