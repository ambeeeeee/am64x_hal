#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mmr__vbusp__cfg1_vtm_pid: Mmr_Vbusp_Cfg1VtmPid,
    mmr__vbusp__cfg1_vtm_devinfo_pwr0: Mmr_Vbusp_Cfg1VtmDevinfoPwr0,
    _reserved2: [u8; 0xf8],
    mmr__vbusp__cfg1_devinfo: Mmr_Vbusp_Cfg1Devinfo,
    mmr__vbusp__cfg1_oppvid: Mmr_Vbusp_Cfg1Oppvid,
    mmr__vbusp__cfg1_evt_stat: Mmr_Vbusp_Cfg1EvtStat,
    mmr__vbusp__cfg1_evt_sel_set: Mmr_Vbusp_Cfg1EvtSelSet,
    mmr__vbusp__cfg1_evt_sel_clr: Mmr_Vbusp_Cfg1EvtSelClr,
    _reserved7: [u8; 0xf0],
    mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set: Mmr_Vbusp_Cfg1VtmGtTh1IntRawStatSet,
    mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr: Mmr_Vbusp_Cfg1VtmGtTh1IntEnStatClr,
    _reserved9: [u8; 0x08],
    mmr__vbusp__cfg1_vtm_gt_th1_int_en_set: Mmr_Vbusp_Cfg1VtmGtTh1IntEnSet,
    mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr: Mmr_Vbusp_Cfg1VtmGtTh1IntEnClr,
    _reserved11: [u8; 0x08],
    mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set: Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSet,
    mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr: Mmr_Vbusp_Cfg1VtmGtTh2IntEnStatClr,
    _reserved13: [u8; 0x08],
    mmr__vbusp__cfg1_vtm_gt_th2_int_en_set: Mmr_Vbusp_Cfg1VtmGtTh2IntEnSet,
    mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr: Mmr_Vbusp_Cfg1VtmGtTh2IntEnClr,
    _reserved15: [u8; 0x08],
    mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set: Mmr_Vbusp_Cfg1VtmLtTh0IntRawStatSet,
    mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr: Mmr_Vbusp_Cfg1VtmLtTh0IntEnStatClr,
    _reserved17: [u8; 0x08],
    mmr__vbusp__cfg1_vtm_lt_th0_int_en_set: Mmr_Vbusp_Cfg1VtmLtTh0IntEnSet,
    mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr: Mmr_Vbusp_Cfg1VtmLtTh0IntEnClr,
    _reserved19: [u8; 0xa4],
    mmr__vbusp__cfg1_ctrl: Mmr_Vbusp_Cfg1Ctrl,
    _reserved20: [u8; 0x04],
    mmr__vbusp__cfg1_stat: Mmr_Vbusp_Cfg1Stat,
    mmr__vbusp__cfg1_th: Mmr_Vbusp_Cfg1Th,
    mmr__vbusp__cfg1_th2: Mmr_Vbusp_Cfg1Th2,
}
impl RegisterBlock {
    #[doc = "0x00 - VTM Peripheral Identification Register"]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_pid(&self) -> &Mmr_Vbusp_Cfg1VtmPid {
        &self.mmr__vbusp__cfg1_vtm_pid
    }
    #[doc = "0x04 - Device specific voltage domain and temp sensor information register."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_devinfo_pwr0(&self) -> &Mmr_Vbusp_Cfg1VtmDevinfoPwr0 {
        &self.mmr__vbusp__cfg1_vtm_devinfo_pwr0
    }
    #[doc = "0x100 - Voltage domain a information register. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_devinfo(&self) -> &Mmr_Vbusp_Cfg1Devinfo {
        &self.mmr__vbusp__cfg1_devinfo
    }
    #[doc = "0x104 - Voltage domain a VID actual code used as reference by Firmware to set the various voltage domain supply voltages. Reset defaults are sourced from efuse for each OPP. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_oppvid(&self) -> &Mmr_Vbusp_Cfg1Oppvid {
        &self.mmr__vbusp__cfg1_oppvid
    }
    #[doc = "0x108 - Voltage domain a event and control status register."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_evt_stat(&self) -> &Mmr_Vbusp_Cfg1EvtStat {
        &self.mmr__vbusp__cfg1_evt_stat
    }
    #[doc = "0x10c - Voltage domain a event select and control set register. NOTE: This MMR and the companion MMR VTM_VD\\[a\\]_EVT_SEL_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_evt_sel_set(&self) -> &Mmr_Vbusp_Cfg1EvtSelSet {
        &self.mmr__vbusp__cfg1_evt_sel_set
    }
    #[doc = "0x110 - Voltage domain a event select and control clear register. NOTE: This MMR and the companion MMR VTM_VD\\[a\\]_EVT_SEL_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_evt_sel_clr(&self) -> &Mmr_Vbusp_Cfg1EvtSelClr {
        &self.mmr__vbusp__cfg1_evt_sel_clr
    }
    #[doc = "0x204 - Interrupt RAW event status and set MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set(
        &self,
    ) -> &Mmr_Vbusp_Cfg1VtmGtTh1IntRawStatSet {
        &self.mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set
    }
    #[doc = "0x208 - Enabled interrupt event status and clear MMR for interrupt GT_TH1 per voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr(
        &self,
    ) -> &Mmr_Vbusp_Cfg1VtmGtTh1IntEnStatClr {
        &self.mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr
    }
    #[doc = "0x214 - Enable set MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th1_int_en_set(&self) -> &Mmr_Vbusp_Cfg1VtmGtTh1IntEnSet {
        &self.mmr__vbusp__cfg1_vtm_gt_th1_int_en_set
    }
    #[doc = "0x218 - Enable clear MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr(&self) -> &Mmr_Vbusp_Cfg1VtmGtTh1IntEnClr {
        &self.mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr
    }
    #[doc = "0x224 - Interrupt RAW event status and set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set(
        &self,
    ) -> &Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSet {
        &self.mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set
    }
    #[doc = "0x228 - Enabled interrupt event status and clear MMR for interrupt GT_TH2 per voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr(
        &self,
    ) -> &Mmr_Vbusp_Cfg1VtmGtTh2IntEnStatClr {
        &self.mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr
    }
    #[doc = "0x234 - Enable set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th2_int_en_set(&self) -> &Mmr_Vbusp_Cfg1VtmGtTh2IntEnSet {
        &self.mmr__vbusp__cfg1_vtm_gt_th2_int_en_set
    }
    #[doc = "0x238 - Enable clear MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr(&self) -> &Mmr_Vbusp_Cfg1VtmGtTh2IntEnClr {
        &self.mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr
    }
    #[doc = "0x244 - Interrupt RAW event status and set MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set(
        &self,
    ) -> &Mmr_Vbusp_Cfg1VtmLtTh0IntRawStatSet {
        &self.mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set
    }
    #[doc = "0x248 - Enabled interrupt event status and clear MMR for interrupt LT_TH0 per voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr(
        &self,
    ) -> &Mmr_Vbusp_Cfg1VtmLtTh0IntEnStatClr {
        &self.mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr
    }
    #[doc = "0x254 - Enable set MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_lt_th0_int_en_set(&self) -> &Mmr_Vbusp_Cfg1VtmLtTh0IntEnSet {
        &self.mmr__vbusp__cfg1_vtm_lt_th0_int_en_set
    }
    #[doc = "0x258 - Enable clear MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr(&self) -> &Mmr_Vbusp_Cfg1VtmLtTh0IntEnClr {
        &self.mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr
    }
    #[doc = "0x300 - Temperature Sensor Band-gap control register for sensor a."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_ctrl(&self) -> &Mmr_Vbusp_Cfg1Ctrl {
        &self.mmr__vbusp__cfg1_ctrl
    }
    #[doc = "0x308 - Temperature Sensor Band-gap Status register for sensor a."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_stat(&self) -> &Mmr_Vbusp_Cfg1Stat {
        &self.mmr__vbusp__cfg1_stat
    }
    #[doc = "0x30c - Temperature Sensor Band-gap Threshold register for sensor a."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_th(&self) -> &Mmr_Vbusp_Cfg1Th {
        &self.mmr__vbusp__cfg1_th
    }
    #[doc = "0x310 - Temperature Sensor Band-gap Threshold register 2 for sensor a."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg1_th2(&self) -> &Mmr_Vbusp_Cfg1Th2 {
        &self.mmr__vbusp__cfg1_th2
    }
}
#[doc = "MMR__VBUSP__CFG1_VTM_PID (rw) register accessor: VTM Peripheral Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_pid`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_PID")]
pub type Mmr_Vbusp_Cfg1VtmPid = crate::Reg<mmr__vbusp__cfg1_vtm_pid::Mmr_Vbusp_Cfg1VtmPidSpec>;
#[doc = "VTM Peripheral Identification Register"]
pub mod mmr__vbusp__cfg1_vtm_pid;
#[doc = "MMR__VBUSP__CFG1_VTM_DEVINFO_PWR0 (rw) register accessor: Device specific voltage domain and temp sensor information register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_devinfo_pwr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_devinfo_pwr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_devinfo_pwr0`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_DEVINFO_PWR0")]
pub type Mmr_Vbusp_Cfg1VtmDevinfoPwr0 =
    crate::Reg<mmr__vbusp__cfg1_vtm_devinfo_pwr0::Mmr_Vbusp_Cfg1VtmDevinfoPwr0Spec>;
#[doc = "Device specific voltage domain and temp sensor information register."]
pub mod mmr__vbusp__cfg1_vtm_devinfo_pwr0;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_RAW_STAT_SET (rw) register accessor: Interrupt RAW event status and set MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_RAW_STAT_SET")]
pub type Mmr_Vbusp_Cfg1VtmGtTh1IntRawStatSet = crate::Reg<
    mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set::Mmr_Vbusp_Cfg1VtmGtTh1IntRawStatSetSpec,
>;
#[doc = "Interrupt RAW event status and set MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
pub mod mmr__vbusp__cfg1_vtm_gt_th1_int_raw_stat_set;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_EN_STAT_CLR (rw) register accessor: Enabled interrupt event status and clear MMR for interrupt GT_TH1 per voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_EN_STAT_CLR")]
pub type Mmr_Vbusp_Cfg1VtmGtTh1IntEnStatClr =
    crate::Reg<mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr::Mmr_Vbusp_Cfg1VtmGtTh1IntEnStatClrSpec>;
#[doc = "Enabled interrupt event status and clear MMR for interrupt GT_TH1 per voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
pub mod mmr__vbusp__cfg1_vtm_gt_th1_int_en_stat_clr;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_EN_SET (rw) register accessor: Enable set MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th1_int_en_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th1_int_en_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th1_int_en_set`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_EN_SET")]
pub type Mmr_Vbusp_Cfg1VtmGtTh1IntEnSet =
    crate::Reg<mmr__vbusp__cfg1_vtm_gt_th1_int_en_set::Mmr_Vbusp_Cfg1VtmGtTh1IntEnSetSpec>;
#[doc = "Enable set MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_vtm_gt_th1_int_en_set;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_EN_CLR (rw) register accessor: Enable clear MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH1_INT_EN_CLR")]
pub type Mmr_Vbusp_Cfg1VtmGtTh1IntEnClr =
    crate::Reg<mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr::Mmr_Vbusp_Cfg1VtmGtTh1IntEnClrSpec>;
#[doc = "Enable clear MMR for interrupt GT_TH1 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH1_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_vtm_gt_th1_int_en_clr;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_RAW_STAT_SET (rw) register accessor: Interrupt RAW event status and set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_RAW_STAT_SET")]
pub type Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSet = crate::Reg<
    mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set::Mmr_Vbusp_Cfg1VtmGtTh2IntRawStatSetSpec,
>;
#[doc = "Interrupt RAW event status and set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
pub mod mmr__vbusp__cfg1_vtm_gt_th2_int_raw_stat_set;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_STAT_CLR (rw) register accessor: Enabled interrupt event status and clear MMR for interrupt GT_TH2 per voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_STAT_CLR")]
pub type Mmr_Vbusp_Cfg1VtmGtTh2IntEnStatClr =
    crate::Reg<mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr::Mmr_Vbusp_Cfg1VtmGtTh2IntEnStatClrSpec>;
#[doc = "Enabled interrupt event status and clear MMR for interrupt GT_TH2 per voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
pub mod mmr__vbusp__cfg1_vtm_gt_th2_int_en_stat_clr;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_SET (rw) register accessor: Enable set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th2_int_en_set`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_SET")]
pub type Mmr_Vbusp_Cfg1VtmGtTh2IntEnSet =
    crate::Reg<mmr__vbusp__cfg1_vtm_gt_th2_int_en_set::Mmr_Vbusp_Cfg1VtmGtTh2IntEnSetSpec>;
#[doc = "Enable set MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_vtm_gt_th2_int_en_set;
#[doc = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_CLR (rw) register accessor: Enable clear MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_GT_TH2_INT_EN_CLR")]
pub type Mmr_Vbusp_Cfg1VtmGtTh2IntEnClr =
    crate::Reg<mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr::Mmr_Vbusp_Cfg1VtmGtTh2IntEnClrSpec>;
#[doc = "Enable clear MMR for interrupt GT_TH2 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_GT_TH2_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_vtm_gt_th2_int_en_clr;
#[doc = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_RAW_STAT_SET (rw) register accessor: Interrupt RAW event status and set MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_RAW_STAT_SET")]
pub type Mmr_Vbusp_Cfg1VtmLtTh0IntRawStatSet = crate::Reg<
    mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set::Mmr_Vbusp_Cfg1VtmLtTh0IntRawStatSetSpec,
>;
#[doc = "Interrupt RAW event status and set MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_STAT_CLR are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
pub mod mmr__vbusp__cfg1_vtm_lt_th0_int_raw_stat_set;
#[doc = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_EN_STAT_CLR (rw) register accessor: Enabled interrupt event status and clear MMR for interrupt LT_TH0 per voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_EN_STAT_CLR")]
pub type Mmr_Vbusp_Cfg1VtmLtTh0IntEnStatClr =
    crate::Reg<mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr::Mmr_Vbusp_Cfg1VtmLtTh0IntEnStatClrSpec>;
#[doc = "Enabled interrupt event status and clear MMR for interrupt LT_TH0 per voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_RAW_STAT_SET are fully linked for write operation, but partially linked for reads, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR. However the reads to these 2 MMRs don't yield the same read data. Reads to *_INT_RAW_STAT_SET return the full raw events contents of the common linked MMR, whereas reads to MMR *_INT_EN_STAT_CLR will yield the masked-content of the linked MMR. The mask for the read is defined by the contents of the related MMR *_INT_EN_SET/CLR."]
pub mod mmr__vbusp__cfg1_vtm_lt_th0_int_en_stat_clr;
#[doc = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_EN_SET (rw) register accessor: Enable set MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_lt_th0_int_en_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_lt_th0_int_en_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_lt_th0_int_en_set`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_EN_SET")]
pub type Mmr_Vbusp_Cfg1VtmLtTh0IntEnSet =
    crate::Reg<mmr__vbusp__cfg1_vtm_lt_th0_int_en_set::Mmr_Vbusp_Cfg1VtmLtTh0IntEnSetSpec>;
#[doc = "Enable set MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_vtm_lt_th0_int_en_set;
#[doc = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_EN_CLR (rw) register accessor: Enable clear MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_VTM_LT_TH0_INT_EN_CLR")]
pub type Mmr_Vbusp_Cfg1VtmLtTh0IntEnClr =
    crate::Reg<mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr::Mmr_Vbusp_Cfg1VtmLtTh0IntEnClrSpec>;
#[doc = "Enable clear MMR for interrupt LT_TH0 for each voltage domain. NOTE: This MMR and the companion MMR, VTM_LT_TH0_INT_EN_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_vtm_lt_th0_int_en_clr;
#[doc = "MMR__VBUSP__CFG1_DEVINFO (rw) register accessor: Voltage domain a information register. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_devinfo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_devinfo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_devinfo`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_DEVINFO")]
pub type Mmr_Vbusp_Cfg1Devinfo = crate::Reg<mmr__vbusp__cfg1_devinfo::Mmr_Vbusp_Cfg1DevinfoSpec>;
#[doc = "Voltage domain a information register. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary."]
pub mod mmr__vbusp__cfg1_devinfo;
#[doc = "MMR__VBUSP__CFG1_OPPVID (rw) register accessor: Voltage domain a VID actual code used as reference by Firmware to set the various voltage domain supply voltages. Reset defaults are sourced from efuse for each OPP. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_oppvid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_oppvid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_oppvid`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_OPPVID")]
pub type Mmr_Vbusp_Cfg1Oppvid = crate::Reg<mmr__vbusp__cfg1_oppvid::Mmr_Vbusp_Cfg1OppvidSpec>;
#[doc = "Voltage domain a VID actual code used as reference by Firmware to set the various voltage domain supply voltages. Reset defaults are sourced from efuse for each OPP. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary."]
pub mod mmr__vbusp__cfg1_oppvid;
#[doc = "MMR__VBUSP__CFG1_EVT_STAT (rw) register accessor: Voltage domain a event and control status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_evt_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_evt_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_evt_stat`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_EVT_STAT")]
pub type Mmr_Vbusp_Cfg1EvtStat = crate::Reg<mmr__vbusp__cfg1_evt_stat::Mmr_Vbusp_Cfg1EvtStatSpec>;
#[doc = "Voltage domain a event and control status register."]
pub mod mmr__vbusp__cfg1_evt_stat;
#[doc = "MMR__VBUSP__CFG1_EVT_SEL_SET (rw) register accessor: Voltage domain a event select and control set register. NOTE: This MMR and the companion MMR VTM_VD\\[a\\]_EVT_SEL_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_evt_sel_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_evt_sel_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_evt_sel_set`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_EVT_SEL_SET")]
pub type Mmr_Vbusp_Cfg1EvtSelSet =
    crate::Reg<mmr__vbusp__cfg1_evt_sel_set::Mmr_Vbusp_Cfg1EvtSelSetSpec>;
#[doc = "Voltage domain a event select and control set register. NOTE: This MMR and the companion MMR VTM_VD\\[a\\]_EVT_SEL_CLR are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_evt_sel_set;
#[doc = "MMR__VBUSP__CFG1_EVT_SEL_CLR (rw) register accessor: Voltage domain a event select and control clear register. NOTE: This MMR and the companion MMR VTM_VD\\[a\\]_EVT_SEL_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_evt_sel_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_evt_sel_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_evt_sel_clr`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_EVT_SEL_CLR")]
pub type Mmr_Vbusp_Cfg1EvtSelClr =
    crate::Reg<mmr__vbusp__cfg1_evt_sel_clr::Mmr_Vbusp_Cfg1EvtSelClrSpec>;
#[doc = "Voltage domain a event select and control clear register. NOTE: This MMR and the companion MMR VTM_VD\\[a\\]_EVT_SEL_SET are linked, which means that they are in fact a single common MMR, with 2 different write addresses/mechanisms, and thus the single common MMR updates with the writes to either MMR and reads to either of these 2 MMRs read the same content."]
pub mod mmr__vbusp__cfg1_evt_sel_clr;
#[doc = "MMR__VBUSP__CFG1_CTRL (rw) register accessor: Temperature Sensor Band-gap control register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_ctrl`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_CTRL")]
pub type Mmr_Vbusp_Cfg1Ctrl = crate::Reg<mmr__vbusp__cfg1_ctrl::Mmr_Vbusp_Cfg1CtrlSpec>;
#[doc = "Temperature Sensor Band-gap control register for sensor a."]
pub mod mmr__vbusp__cfg1_ctrl;
#[doc = "MMR__VBUSP__CFG1_STAT (rw) register accessor: Temperature Sensor Band-gap Status register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_stat`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_STAT")]
pub type Mmr_Vbusp_Cfg1Stat = crate::Reg<mmr__vbusp__cfg1_stat::Mmr_Vbusp_Cfg1StatSpec>;
#[doc = "Temperature Sensor Band-gap Status register for sensor a."]
pub mod mmr__vbusp__cfg1_stat;
#[doc = "MMR__VBUSP__CFG1_TH (rw) register accessor: Temperature Sensor Band-gap Threshold register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_th`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_TH")]
pub type Mmr_Vbusp_Cfg1Th = crate::Reg<mmr__vbusp__cfg1_th::Mmr_Vbusp_Cfg1ThSpec>;
#[doc = "Temperature Sensor Band-gap Threshold register for sensor a."]
pub mod mmr__vbusp__cfg1_th;
#[doc = "MMR__VBUSP__CFG1_TH2 (rw) register accessor: Temperature Sensor Band-gap Threshold register 2 for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg1_th2`]
module"]
#[doc(alias = "MMR__VBUSP__CFG1_TH2")]
pub type Mmr_Vbusp_Cfg1Th2 = crate::Reg<mmr__vbusp__cfg1_th2::Mmr_Vbusp_Cfg1Th2Spec>;
#[doc = "Temperature Sensor Band-gap Threshold register 2 for sensor a."]
pub mod mmr__vbusp__cfg1_th2;
