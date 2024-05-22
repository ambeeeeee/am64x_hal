#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fsas__fsas_mmr_cfg__fsas_genregs_revision: Fsas_FsasMmrCfg_FsasGenregsRevision,
    fsas__fsas_mmr_cfg__fsas_genregs_sysconfig: Fsas_FsasMmrCfg_FsasGenregsSysconfig,
    fsas__fsas_mmr_cfg__fsas_genregs_frag_adr: Fsas_FsasMmrCfg_FsasGenregsFragAdr,
    fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl: Fsas_FsasMmrCfg_FsasGenregsFragCtl,
    fsas__fsas_mmr_cfg__fsas_genregs_eoi: Fsas_FsasMmrCfg_FsasGenregsEoi,
    fsas__fsas_mmr_cfg__fsas_genregs_status_raw: Fsas_FsasMmrCfg_FsasGenregsStatusRaw,
    fsas__fsas_mmr_cfg__fsas_genregs_status: Fsas_FsasMmrCfg_FsasGenregsStatus,
    fsas__fsas_mmr_cfg__fsas_genregs_enable_set: Fsas_FsasMmrCfg_FsasGenregsEnableSet,
    fsas__fsas_mmr_cfg__fsas_genregs_enable_clr: Fsas_FsasMmrCfg_FsasGenregsEnableClr,
    _reserved9: [u8; 0x0c],
    fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt: Fsas_FsasMmrCfg_FsasGenregsEccRgstrt,
    fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz: Fsas_FsasMmrCfg_FsasGenregsEccRgsiz,
    _reserved11: [u8; 0x38],
    fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr: Fsas_FsasMmrCfg_FsasGenregsEccBlockAdr,
    fsas__fsas_mmr_cfg__fsas_genregs_ecc_type: Fsas_FsasMmrCfg_FsasGenregsEccType,
    fsas__fsas_mmr_cfg__fsas_genregs_wrt_type: Fsas_FsasMmrCfg_FsasGenregsWrtType,
}
impl RegisterBlock {
    #[doc = "0x00 - IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_revision(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsRevision {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_revision
    }
    #[doc = "0x04 - Controls various parameters of the cotroller state."]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_sysconfig(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsSysconfig {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_sysconfig
    }
    #[doc = "0x08 - This FRAG_ADR is the address of a request that frag_hi or frag_lo boundary occurs"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_frag_adr(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsFragAdr {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_frag_adr
    }
    #[doc = "0x0c - The FRAG_CTL determins which frag region is fragmented"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsFragCtl {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl
    }
    #[doc = "0x10 - The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to."]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_eoi(&self) -> &Fsas_FsasMmrCfg_FsasGenregsEoi {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_eoi
    }
    #[doc = "0x14 - The IRQ_STATUS_RAW register allows the interrupt sources to be manually set when writing a 1 to a specific bit. Write 0: No action Write 1: Set event Read 0: No event pending Read 1: Event pending"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_status_raw(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsStatusRaw {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_status_raw
    }
    #[doc = "0x18 - The IRQ_STATUS register allows the interrupt sources to be manually cleared when writing a 1 to a specific bit. Write 0: No action Write 1: Clear event Read 0: No event pending Read 1: Event pending"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_status(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsStatus {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_status
    }
    #[doc = "0x1c - The IRQ_ENABLE_SET register allows the interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_enable_set(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsEnableSet {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_enable_set
    }
    #[doc = "0x20 - The IRQ_ENABLE_CLR register allows the interrupt sources to be manually disabled when writing a 1 to a specific bit. Write 0: No action Write 1: Disable event Read 0: Event is disabled Read 1: Event is enabled"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_enable_clr(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsEnableClr {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_enable_clr
    }
    #[doc = "0x30 - This defines the start of the ECC region in 4KBytes steps."]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsEccRgstrt {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt
    }
    #[doc = "0x34 - This defines the size of the ECC region in 4KBytes steps."]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsEccRgsiz {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz
    }
    #[doc = "0x70 - The ERR_ECC_BLOCK_ADR register holds the current top of stack ECC error block address, this is only valid when the ecc_err_valid is set"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsEccBlockAdr {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr
    }
    #[doc = "0x74 - The ERR_ECC_TYPE register holds the current top of stack ECC error info, this is only valid when the ecc_err_valid is set"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_ecc_type(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsEccType {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_ecc_type
    }
    #[doc = "0x78 - The ERR_WRT_TYPE register holds the current top of stack write error info. this is only valid when the wrt_err_valid is set"]
    #[inline(always)]
    pub const fn fsas__fsas_mmr_cfg__fsas_genregs_wrt_type(
        &self,
    ) -> &Fsas_FsasMmrCfg_FsasGenregsWrtType {
        &self.fsas__fsas_mmr_cfg__fsas_genregs_wrt_type
    }
}
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_REVISION (rw) register accessor: IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_revision`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_REVISION")]
pub type Fsas_FsasMmrCfg_FsasGenregsRevision =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_revision::Fsas_FsasMmrCfg_FsasGenregsRevisionSpec>;
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_revision;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_SYSCONFIG (rw) register accessor: Controls various parameters of the cotroller state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_sysconfig`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_SYSCONFIG")]
pub type Fsas_FsasMmrCfg_FsasGenregsSysconfig = crate::Reg<
    fsas__fsas_mmr_cfg__fsas_genregs_sysconfig::Fsas_FsasMmrCfg_FsasGenregsSysconfigSpec,
>;
#[doc = "Controls various parameters of the cotroller state."]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_sysconfig;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_ADR (rw) register accessor: This FRAG_ADR is the address of a request that frag_hi or frag_lo boundary occurs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_frag_adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_frag_adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_frag_adr`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_ADR")]
pub type Fsas_FsasMmrCfg_FsasGenregsFragAdr =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_frag_adr::Fsas_FsasMmrCfg_FsasGenregsFragAdrSpec>;
#[doc = "This FRAG_ADR is the address of a request that frag_hi or frag_lo boundary occurs"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_frag_adr;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_CTL (rw) register accessor: The FRAG_CTL determins which frag region is fragmented\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_FRAG_CTL")]
pub type Fsas_FsasMmrCfg_FsasGenregsFragCtl =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl::Fsas_FsasMmrCfg_FsasGenregsFragCtlSpec>;
#[doc = "The FRAG_CTL determins which frag region is fragmented"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_frag_ctl;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_EOI (rw) register accessor: The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_eoi`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_EOI")]
pub type Fsas_FsasMmrCfg_FsasGenregsEoi =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_eoi::Fsas_FsasMmrCfg_FsasGenregsEoiSpec>;
#[doc = "The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to."]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_eoi;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_STATUS_RAW (rw) register accessor: The IRQ_STATUS_RAW register allows the interrupt sources to be manually set when writing a 1 to a specific bit. Write 0: No action Write 1: Set event Read 0: No event pending Read 1: Event pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_status_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_status_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_status_raw`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_STATUS_RAW")]
pub type Fsas_FsasMmrCfg_FsasGenregsStatusRaw = crate::Reg<
    fsas__fsas_mmr_cfg__fsas_genregs_status_raw::Fsas_FsasMmrCfg_FsasGenregsStatusRawSpec,
>;
#[doc = "The IRQ_STATUS_RAW register allows the interrupt sources to be manually set when writing a 1 to a specific bit. Write 0: No action Write 1: Set event Read 0: No event pending Read 1: Event pending"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_status_raw;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_STATUS (rw) register accessor: The IRQ_STATUS register allows the interrupt sources to be manually cleared when writing a 1 to a specific bit. Write 0: No action Write 1: Clear event Read 0: No event pending Read 1: Event pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_status`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_STATUS")]
pub type Fsas_FsasMmrCfg_FsasGenregsStatus =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_status::Fsas_FsasMmrCfg_FsasGenregsStatusSpec>;
#[doc = "The IRQ_STATUS register allows the interrupt sources to be manually cleared when writing a 1 to a specific bit. Write 0: No action Write 1: Clear event Read 0: No event pending Read 1: Event pending"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_status;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ENABLE_SET (rw) register accessor: The IRQ_ENABLE_SET register allows the interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_enable_set`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ENABLE_SET")]
pub type Fsas_FsasMmrCfg_FsasGenregsEnableSet = crate::Reg<
    fsas__fsas_mmr_cfg__fsas_genregs_enable_set::Fsas_FsasMmrCfg_FsasGenregsEnableSetSpec,
>;
#[doc = "The IRQ_ENABLE_SET register allows the interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_enable_set;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ENABLE_CLR (rw) register accessor: The IRQ_ENABLE_CLR register allows the interrupt sources to be manually disabled when writing a 1 to a specific bit. Write 0: No action Write 1: Disable event Read 0: Event is disabled Read 1: Event is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_enable_clr`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ENABLE_CLR")]
pub type Fsas_FsasMmrCfg_FsasGenregsEnableClr = crate::Reg<
    fsas__fsas_mmr_cfg__fsas_genregs_enable_clr::Fsas_FsasMmrCfg_FsasGenregsEnableClrSpec,
>;
#[doc = "The IRQ_ENABLE_CLR register allows the interrupt sources to be manually disabled when writing a 1 to a specific bit. Write 0: No action Write 1: Disable event Read 0: Event is disabled Read 1: Event is enabled"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_enable_clr;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSTRT (rw) register accessor: This defines the start of the ECC region in 4KBytes steps.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSTRT")]
pub type Fsas_FsasMmrCfg_FsasGenregsEccRgstrt = crate::Reg<
    fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt::Fsas_FsasMmrCfg_FsasGenregsEccRgstrtSpec,
>;
#[doc = "This defines the start of the ECC region in 4KBytes steps."]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgstrt;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSIZ (rw) register accessor: This defines the size of the ECC region in 4KBytes steps.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_RGSIZ")]
pub type Fsas_FsasMmrCfg_FsasGenregsEccRgsiz =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz::Fsas_FsasMmrCfg_FsasGenregsEccRgsizSpec>;
#[doc = "This defines the size of the ECC region in 4KBytes steps."]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_ecc_rgsiz;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_BLOCK_ADR (rw) register accessor: The ERR_ECC_BLOCK_ADR register holds the current top of stack ECC error block address, this is only valid when the ecc_err_valid is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_BLOCK_ADR")]
pub type Fsas_FsasMmrCfg_FsasGenregsEccBlockAdr = crate::Reg<
    fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr::Fsas_FsasMmrCfg_FsasGenregsEccBlockAdrSpec,
>;
#[doc = "The ERR_ECC_BLOCK_ADR register holds the current top of stack ECC error block address, this is only valid when the ecc_err_valid is set"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_ecc_block_adr;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_TYPE (rw) register accessor: The ERR_ECC_TYPE register holds the current top of stack ECC error info, this is only valid when the ecc_err_valid is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_ecc_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_ecc_type`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_ECC_TYPE")]
pub type Fsas_FsasMmrCfg_FsasGenregsEccType =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_ecc_type::Fsas_FsasMmrCfg_FsasGenregsEccTypeSpec>;
#[doc = "The ERR_ECC_TYPE register holds the current top of stack ECC error info, this is only valid when the ecc_err_valid is set"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_ecc_type;
#[doc = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_WRT_TYPE (rw) register accessor: The ERR_WRT_TYPE register holds the current top of stack write error info. this is only valid when the wrt_err_valid is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_mmr_cfg__fsas_genregs_wrt_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_mmr_cfg__fsas_genregs_wrt_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_mmr_cfg__fsas_genregs_wrt_type`]
module"]
#[doc(alias = "FSAS__FSAS_MMR_CFG__FSAS_GENREGS_WRT_TYPE")]
pub type Fsas_FsasMmrCfg_FsasGenregsWrtType =
    crate::Reg<fsas__fsas_mmr_cfg__fsas_genregs_wrt_type::Fsas_FsasMmrCfg_FsasGenregsWrtTypeSpec>;
#[doc = "The ERR_WRT_TYPE register holds the current top of stack write error info. this is only valid when the wrt_err_valid is set"]
pub mod fsas__fsas_mmr_cfg__fsas_genregs_wrt_type;
