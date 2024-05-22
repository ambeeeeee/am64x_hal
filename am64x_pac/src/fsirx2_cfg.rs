#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_rx_master_ctrl_altb_: CfgRxMasterCtrlAltb_,
    cfg_reserved_1: CfgReserved1,
    cfg_reserved_2: CfgReserved2,
    cfg_reserved_3: CfgReserved3,
    cfg_rx_oper_ctrl: CfgRxOperCtrl,
    cfg_reserved_4: CfgReserved4,
    cfg_rx_frame_info: CfgRxFrameInfo,
    cfg_rx_frame_tag_udata: CfgRxFrameTagUdata,
    cfg_rx_dma_ctrl: CfgRxDmaCtrl,
    cfg_reserved_5: CfgReserved5,
    cfg_rx_evt_sts_alt1_: CfgRxEvtStsAlt1_,
    cfg_rx_crc_info: CfgRxCrcInfo,
    cfg_rx_evt_clr_alt1_: CfgRxEvtClrAlt1_,
    cfg_rx_evt_frc_alt1_: CfgRxEvtFrcAlt1_,
    cfg_rx_buf_ptr_load: CfgRxBufPtrLoad,
    cfg_rx_buf_ptr_sts: CfgRxBufPtrSts,
    cfg_rx_frame_wd_ctrl: CfgRxFrameWdCtrl,
    cfg_reserved_6: CfgReserved6,
    cfg_rx_frame_wd_ref: CfgRxFrameWdRef,
    cfg_rx_frame_wd_cnt: CfgRxFrameWdCnt,
    cfg_rx_ping_wd_ctrl: CfgRxPingWdCtrl,
    cfg_rx_ping_tag: CfgRxPingTag,
    cfg_rx_ping_wd_ref: CfgRxPingWdRef,
    cfg_rx_ping_wd_cnt: CfgRxPingWdCnt,
    cfg_rx_int1_ctrl_alt1_: CfgRxInt1CtrlAlt1_,
    cfg_rx_int2_ctrl_alt1_: CfgRxInt2CtrlAlt1_,
    cfg_rx_lock_ctrl: CfgRxLockCtrl,
    cfg_reserved_7: CfgReserved7,
    cfg_rx_ecc_data: CfgRxEccData,
    cfg_rx_ecc_val: CfgRxEccVal,
    cfg_reserved_8: CfgReserved8,
    cfg_rx_ecc_sec_data: CfgRxEccSecData,
    cfg_rx_ecc_log: CfgRxEccLog,
    cfg_reserved_9: CfgReserved9,
    cfg_rx_frame_tag_cmp: CfgRxFrameTagCmp,
    cfg_rx_ping_tag_cmp: CfgRxPingTagCmp,
    cfg_reserved_10: CfgReserved10,
    _reserved37: [u8; 0x0a],
    cfg_rx_dlyline_ctrl: CfgRxDlylineCtrl,
    cfg_reserved_11: CfgReserved11,
    _reserved39: [u8; 0x0c],
    cfg_rx_vis_1: CfgRxVis1,
    cfg_reserved_12: CfgReserved12,
    _reserved41: [u8; 0x0a],
    cfg_rx_buf_base: CfgRxBufBase,
}
impl RegisterBlock {
    #[doc = "0x00 - Receive master control register"]
    #[inline(always)]
    pub const fn cfg_rx_master_ctrl_altb_(&self) -> &CfgRxMasterCtrlAltb_ {
        &self.cfg_rx_master_ctrl_altb_
    }
    #[doc = "0x02 - CFG_Reserved_1"]
    #[inline(always)]
    pub const fn cfg_reserved_1(&self) -> &CfgReserved1 {
        &self.cfg_reserved_1
    }
    #[doc = "0x04 - CFG_Reserved_2"]
    #[inline(always)]
    pub const fn cfg_reserved_2(&self) -> &CfgReserved2 {
        &self.cfg_reserved_2
    }
    #[doc = "0x06 - CFG_Reserved_3"]
    #[inline(always)]
    pub const fn cfg_reserved_3(&self) -> &CfgReserved3 {
        &self.cfg_reserved_3
    }
    #[doc = "0x08 - Receive operation control register. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_oper_ctrl(&self) -> &CfgRxOperCtrl {
        &self.cfg_rx_oper_ctrl
    }
    #[doc = "0x0a - CFG_Reserved_4"]
    #[inline(always)]
    pub const fn cfg_reserved_4(&self) -> &CfgReserved4 {
        &self.cfg_reserved_4
    }
    #[doc = "0x0c - Receive frame control register"]
    #[inline(always)]
    pub const fn cfg_rx_frame_info(&self) -> &CfgRxFrameInfo {
        &self.cfg_rx_frame_info
    }
    #[doc = "0x0e - Receive frame tag and user data register"]
    #[inline(always)]
    pub const fn cfg_rx_frame_tag_udata(&self) -> &CfgRxFrameTagUdata {
        &self.cfg_rx_frame_tag_udata
    }
    #[doc = "0x10 - Receive DMA event control register. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_dma_ctrl(&self) -> &CfgRxDmaCtrl {
        &self.cfg_rx_dma_ctrl
    }
    #[doc = "0x12 - CFG_Reserved_5"]
    #[inline(always)]
    pub const fn cfg_reserved_5(&self) -> &CfgReserved5 {
        &self.cfg_reserved_5
    }
    #[doc = "0x14 - Receive event and error status flag register"]
    #[inline(always)]
    pub const fn cfg_rx_evt_sts_alt1_(&self) -> &CfgRxEvtStsAlt1_ {
        &self.cfg_rx_evt_sts_alt1_
    }
    #[doc = "0x16 - Receive CRC info of received and computed CRC"]
    #[inline(always)]
    pub const fn cfg_rx_crc_info(&self) -> &CfgRxCrcInfo {
        &self.cfg_rx_crc_info
    }
    #[doc = "0x18 - Receive event and error clear register"]
    #[inline(always)]
    pub const fn cfg_rx_evt_clr_alt1_(&self) -> &CfgRxEvtClrAlt1_ {
        &self.cfg_rx_evt_clr_alt1_
    }
    #[doc = "0x1a - Receive event and error flag force register"]
    #[inline(always)]
    pub const fn cfg_rx_evt_frc_alt1_(&self) -> &CfgRxEvtFrcAlt1_ {
        &self.cfg_rx_evt_frc_alt1_
    }
    #[doc = "0x1c - Receive buffer pointer load register"]
    #[inline(always)]
    pub const fn cfg_rx_buf_ptr_load(&self) -> &CfgRxBufPtrLoad {
        &self.cfg_rx_buf_ptr_load
    }
    #[doc = "0x1e - Receive buffer pointer status register"]
    #[inline(always)]
    pub const fn cfg_rx_buf_ptr_sts(&self) -> &CfgRxBufPtrSts {
        &self.cfg_rx_buf_ptr_sts
    }
    #[doc = "0x20 - Receive frame watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_frame_wd_ctrl(&self) -> &CfgRxFrameWdCtrl {
        &self.cfg_rx_frame_wd_ctrl
    }
    #[doc = "0x22 - CFG_Reserved_6"]
    #[inline(always)]
    pub const fn cfg_reserved_6(&self) -> &CfgReserved6 {
        &self.cfg_reserved_6
    }
    #[doc = "0x24 - Receive frame watchdog counter reference. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_frame_wd_ref(&self) -> &CfgRxFrameWdRef {
        &self.cfg_rx_frame_wd_ref
    }
    #[doc = "0x28 - Receive frame watchdog current count"]
    #[inline(always)]
    pub const fn cfg_rx_frame_wd_cnt(&self) -> &CfgRxFrameWdCnt {
        &self.cfg_rx_frame_wd_cnt
    }
    #[doc = "0x2c - Receive ping watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_ping_wd_ctrl(&self) -> &CfgRxPingWdCtrl {
        &self.cfg_rx_ping_wd_ctrl
    }
    #[doc = "0x2e - Receive ping tag register"]
    #[inline(always)]
    pub const fn cfg_rx_ping_tag(&self) -> &CfgRxPingTag {
        &self.cfg_rx_ping_tag
    }
    #[doc = "0x30 - Receive ping watchdog counter reference. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_ping_wd_ref(&self) -> &CfgRxPingWdRef {
        &self.cfg_rx_ping_wd_ref
    }
    #[doc = "0x34 - Receive pingwatchdog current count"]
    #[inline(always)]
    pub const fn cfg_rx_ping_wd_cnt(&self) -> &CfgRxPingWdCnt {
        &self.cfg_rx_ping_wd_cnt
    }
    #[doc = "0x38 - Receive interrupt control register for RX_INT1. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_int1_ctrl_alt1_(&self) -> &CfgRxInt1CtrlAlt1_ {
        &self.cfg_rx_int1_ctrl_alt1_
    }
    #[doc = "0x3a - Receive interrupt control register for RX_INT2. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_int2_ctrl_alt1_(&self) -> &CfgRxInt2CtrlAlt1_ {
        &self.cfg_rx_int2_ctrl_alt1_
    }
    #[doc = "0x3c - Receive lock control register"]
    #[inline(always)]
    pub const fn cfg_rx_lock_ctrl(&self) -> &CfgRxLockCtrl {
        &self.cfg_rx_lock_ctrl
    }
    #[doc = "0x3e - CFG_Reserved_7"]
    #[inline(always)]
    pub const fn cfg_reserved_7(&self) -> &CfgReserved7 {
        &self.cfg_reserved_7
    }
    #[doc = "0x40 - Receive ECC data register"]
    #[inline(always)]
    pub const fn cfg_rx_ecc_data(&self) -> &CfgRxEccData {
        &self.cfg_rx_ecc_data
    }
    #[doc = "0x44 - Receive ECC value register"]
    #[inline(always)]
    pub const fn cfg_rx_ecc_val(&self) -> &CfgRxEccVal {
        &self.cfg_rx_ecc_val
    }
    #[doc = "0x46 - CFG_Reserved_8"]
    #[inline(always)]
    pub const fn cfg_reserved_8(&self) -> &CfgReserved8 {
        &self.cfg_reserved_8
    }
    #[doc = "0x48 - Receive ECC corrected data register"]
    #[inline(always)]
    pub const fn cfg_rx_ecc_sec_data(&self) -> &CfgRxEccSecData {
        &self.cfg_rx_ecc_sec_data
    }
    #[doc = "0x4c - Receive ECC log and status register"]
    #[inline(always)]
    pub const fn cfg_rx_ecc_log(&self) -> &CfgRxEccLog {
        &self.cfg_rx_ecc_log
    }
    #[doc = "0x4e - CFG_Reserved_9"]
    #[inline(always)]
    pub const fn cfg_reserved_9(&self) -> &CfgReserved9 {
        &self.cfg_reserved_9
    }
    #[doc = "0x50 - Receive frame tag compare register. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_frame_tag_cmp(&self) -> &CfgRxFrameTagCmp {
        &self.cfg_rx_frame_tag_cmp
    }
    #[doc = "0x52 - Receive ping tag compare register. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_ping_tag_cmp(&self) -> &CfgRxPingTagCmp {
        &self.cfg_rx_ping_tag_cmp
    }
    #[doc = "0x54 - CFG_Reserved_10"]
    #[inline(always)]
    pub const fn cfg_reserved_10(&self) -> &CfgReserved10 {
        &self.cfg_reserved_10
    }
    #[doc = "0x60 - Receive delay line control register. Protected by LOCK field in RX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_rx_dlyline_ctrl(&self) -> &CfgRxDlylineCtrl {
        &self.cfg_rx_dlyline_ctrl
    }
    #[doc = "0x62 - CFG_Reserved_11"]
    #[inline(always)]
    pub const fn cfg_reserved_11(&self) -> &CfgReserved11 {
        &self.cfg_reserved_11
    }
    #[doc = "0x70 - Receive debug visibility register 1"]
    #[inline(always)]
    pub const fn cfg_rx_vis_1(&self) -> &CfgRxVis1 {
        &self.cfg_rx_vis_1
    }
    #[doc = "0x74 - CFG_Reserved_12"]
    #[inline(always)]
    pub const fn cfg_reserved_12(&self) -> &CfgReserved12 {
        &self.cfg_reserved_12
    }
    #[doc = "0x80 - Base address for receive data buffer"]
    #[inline(always)]
    pub const fn cfg_rx_buf_base(&self) -> &CfgRxBufBase {
        &self.cfg_rx_buf_base
    }
}
#[doc = "CFG_RX_MASTER_CTRL_ALTB_ (rw) register accessor: Receive master control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_master_ctrl_altb_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_master_ctrl_altb_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_master_ctrl_altb_`]
module"]
#[doc(alias = "CFG_RX_MASTER_CTRL_ALTB_")]
pub type CfgRxMasterCtrlAltb_ = crate::Reg<cfg_rx_master_ctrl_altb_::CfgRxMasterCtrlAltb_Spec>;
#[doc = "Receive master control register"]
pub mod cfg_rx_master_ctrl_altb_;
#[doc = "CFG_Reserved_1 (rw) register accessor: CFG_Reserved_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_1`]
module"]
#[doc(alias = "CFG_Reserved_1")]
pub type CfgReserved1 = crate::Reg<cfg_reserved_1::CfgReserved1Spec>;
#[doc = "CFG_Reserved_1"]
pub mod cfg_reserved_1;
#[doc = "CFG_Reserved_2 (rw) register accessor: CFG_Reserved_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_2`]
module"]
#[doc(alias = "CFG_Reserved_2")]
pub type CfgReserved2 = crate::Reg<cfg_reserved_2::CfgReserved2Spec>;
#[doc = "CFG_Reserved_2"]
pub mod cfg_reserved_2;
#[doc = "CFG_Reserved_3 (rw) register accessor: CFG_Reserved_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_3`]
module"]
#[doc(alias = "CFG_Reserved_3")]
pub type CfgReserved3 = crate::Reg<cfg_reserved_3::CfgReserved3Spec>;
#[doc = "CFG_Reserved_3"]
pub mod cfg_reserved_3;
#[doc = "CFG_RX_OPER_CTRL (rw) register accessor: Receive operation control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_oper_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_oper_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_oper_ctrl`]
module"]
#[doc(alias = "CFG_RX_OPER_CTRL")]
pub type CfgRxOperCtrl = crate::Reg<cfg_rx_oper_ctrl::CfgRxOperCtrlSpec>;
#[doc = "Receive operation control register. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_oper_ctrl;
#[doc = "CFG_Reserved_4 (rw) register accessor: CFG_Reserved_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_4`]
module"]
#[doc(alias = "CFG_Reserved_4")]
pub type CfgReserved4 = crate::Reg<cfg_reserved_4::CfgReserved4Spec>;
#[doc = "CFG_Reserved_4"]
pub mod cfg_reserved_4;
#[doc = "CFG_RX_FRAME_INFO (rw) register accessor: Receive frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_frame_info`]
module"]
#[doc(alias = "CFG_RX_FRAME_INFO")]
pub type CfgRxFrameInfo = crate::Reg<cfg_rx_frame_info::CfgRxFrameInfoSpec>;
#[doc = "Receive frame control register"]
pub mod cfg_rx_frame_info;
#[doc = "CFG_RX_FRAME_TAG_UDATA (rw) register accessor: Receive frame tag and user data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_tag_udata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_tag_udata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_frame_tag_udata`]
module"]
#[doc(alias = "CFG_RX_FRAME_TAG_UDATA")]
pub type CfgRxFrameTagUdata = crate::Reg<cfg_rx_frame_tag_udata::CfgRxFrameTagUdataSpec>;
#[doc = "Receive frame tag and user data register"]
pub mod cfg_rx_frame_tag_udata;
#[doc = "CFG_RX_DMA_CTRL (rw) register accessor: Receive DMA event control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_dma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_dma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_dma_ctrl`]
module"]
#[doc(alias = "CFG_RX_DMA_CTRL")]
pub type CfgRxDmaCtrl = crate::Reg<cfg_rx_dma_ctrl::CfgRxDmaCtrlSpec>;
#[doc = "Receive DMA event control register. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_dma_ctrl;
#[doc = "CFG_Reserved_5 (rw) register accessor: CFG_Reserved_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_5`]
module"]
#[doc(alias = "CFG_Reserved_5")]
pub type CfgReserved5 = crate::Reg<cfg_reserved_5::CfgReserved5Spec>;
#[doc = "CFG_Reserved_5"]
pub mod cfg_reserved_5;
#[doc = "CFG_RX_EVT_STS_ALT1_ (rw) register accessor: Receive event and error status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_evt_sts_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_evt_sts_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_evt_sts_alt1_`]
module"]
#[doc(alias = "CFG_RX_EVT_STS_ALT1_")]
pub type CfgRxEvtStsAlt1_ = crate::Reg<cfg_rx_evt_sts_alt1_::CfgRxEvtStsAlt1_Spec>;
#[doc = "Receive event and error status flag register"]
pub mod cfg_rx_evt_sts_alt1_;
#[doc = "CFG_RX_CRC_INFO (rw) register accessor: Receive CRC info of received and computed CRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_crc_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_crc_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_crc_info`]
module"]
#[doc(alias = "CFG_RX_CRC_INFO")]
pub type CfgRxCrcInfo = crate::Reg<cfg_rx_crc_info::CfgRxCrcInfoSpec>;
#[doc = "Receive CRC info of received and computed CRC"]
pub mod cfg_rx_crc_info;
#[doc = "CFG_RX_EVT_CLR_ALT1_ (rw) register accessor: Receive event and error clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_evt_clr_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_evt_clr_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_evt_clr_alt1_`]
module"]
#[doc(alias = "CFG_RX_EVT_CLR_ALT1_")]
pub type CfgRxEvtClrAlt1_ = crate::Reg<cfg_rx_evt_clr_alt1_::CfgRxEvtClrAlt1_Spec>;
#[doc = "Receive event and error clear register"]
pub mod cfg_rx_evt_clr_alt1_;
#[doc = "CFG_RX_EVT_FRC_ALT1_ (rw) register accessor: Receive event and error flag force register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_evt_frc_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_evt_frc_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_evt_frc_alt1_`]
module"]
#[doc(alias = "CFG_RX_EVT_FRC_ALT1_")]
pub type CfgRxEvtFrcAlt1_ = crate::Reg<cfg_rx_evt_frc_alt1_::CfgRxEvtFrcAlt1_Spec>;
#[doc = "Receive event and error flag force register"]
pub mod cfg_rx_evt_frc_alt1_;
#[doc = "CFG_RX_BUF_PTR_LOAD (rw) register accessor: Receive buffer pointer load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_buf_ptr_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_buf_ptr_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_buf_ptr_load`]
module"]
#[doc(alias = "CFG_RX_BUF_PTR_LOAD")]
pub type CfgRxBufPtrLoad = crate::Reg<cfg_rx_buf_ptr_load::CfgRxBufPtrLoadSpec>;
#[doc = "Receive buffer pointer load register"]
pub mod cfg_rx_buf_ptr_load;
#[doc = "CFG_RX_BUF_PTR_STS (rw) register accessor: Receive buffer pointer status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_buf_ptr_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_buf_ptr_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_buf_ptr_sts`]
module"]
#[doc(alias = "CFG_RX_BUF_PTR_STS")]
pub type CfgRxBufPtrSts = crate::Reg<cfg_rx_buf_ptr_sts::CfgRxBufPtrStsSpec>;
#[doc = "Receive buffer pointer status register"]
pub mod cfg_rx_buf_ptr_sts;
#[doc = "CFG_RX_FRAME_WD_CTRL (rw) register accessor: Receive frame watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_wd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_wd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_frame_wd_ctrl`]
module"]
#[doc(alias = "CFG_RX_FRAME_WD_CTRL")]
pub type CfgRxFrameWdCtrl = crate::Reg<cfg_rx_frame_wd_ctrl::CfgRxFrameWdCtrlSpec>;
#[doc = "Receive frame watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_frame_wd_ctrl;
#[doc = "CFG_Reserved_6 (rw) register accessor: CFG_Reserved_6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_6`]
module"]
#[doc(alias = "CFG_Reserved_6")]
pub type CfgReserved6 = crate::Reg<cfg_reserved_6::CfgReserved6Spec>;
#[doc = "CFG_Reserved_6"]
pub mod cfg_reserved_6;
#[doc = "CFG_RX_FRAME_WD_REF (rw) register accessor: Receive frame watchdog counter reference. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_wd_ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_wd_ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_frame_wd_ref`]
module"]
#[doc(alias = "CFG_RX_FRAME_WD_REF")]
pub type CfgRxFrameWdRef = crate::Reg<cfg_rx_frame_wd_ref::CfgRxFrameWdRefSpec>;
#[doc = "Receive frame watchdog counter reference. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_frame_wd_ref;
#[doc = "CFG_RX_FRAME_WD_CNT (rw) register accessor: Receive frame watchdog current count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_wd_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_wd_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_frame_wd_cnt`]
module"]
#[doc(alias = "CFG_RX_FRAME_WD_CNT")]
pub type CfgRxFrameWdCnt = crate::Reg<cfg_rx_frame_wd_cnt::CfgRxFrameWdCntSpec>;
#[doc = "Receive frame watchdog current count"]
pub mod cfg_rx_frame_wd_cnt;
#[doc = "CFG_RX_PING_WD_CTRL (rw) register accessor: Receive ping watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_wd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_wd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ping_wd_ctrl`]
module"]
#[doc(alias = "CFG_RX_PING_WD_CTRL")]
pub type CfgRxPingWdCtrl = crate::Reg<cfg_rx_ping_wd_ctrl::CfgRxPingWdCtrlSpec>;
#[doc = "Receive ping watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_ping_wd_ctrl;
#[doc = "CFG_RX_PING_TAG (rw) register accessor: Receive ping tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ping_tag`]
module"]
#[doc(alias = "CFG_RX_PING_TAG")]
pub type CfgRxPingTag = crate::Reg<cfg_rx_ping_tag::CfgRxPingTagSpec>;
#[doc = "Receive ping tag register"]
pub mod cfg_rx_ping_tag;
#[doc = "CFG_RX_PING_WD_REF (rw) register accessor: Receive ping watchdog counter reference. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_wd_ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_wd_ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ping_wd_ref`]
module"]
#[doc(alias = "CFG_RX_PING_WD_REF")]
pub type CfgRxPingWdRef = crate::Reg<cfg_rx_ping_wd_ref::CfgRxPingWdRefSpec>;
#[doc = "Receive ping watchdog counter reference. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_ping_wd_ref;
#[doc = "CFG_RX_PING_WD_CNT (rw) register accessor: Receive pingwatchdog current count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_wd_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_wd_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ping_wd_cnt`]
module"]
#[doc(alias = "CFG_RX_PING_WD_CNT")]
pub type CfgRxPingWdCnt = crate::Reg<cfg_rx_ping_wd_cnt::CfgRxPingWdCntSpec>;
#[doc = "Receive pingwatchdog current count"]
pub mod cfg_rx_ping_wd_cnt;
#[doc = "CFG_RX_INT1_CTRL_ALT1_ (rw) register accessor: Receive interrupt control register for RX_INT1. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_int1_ctrl_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_int1_ctrl_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_int1_ctrl_alt1_`]
module"]
#[doc(alias = "CFG_RX_INT1_CTRL_ALT1_")]
pub type CfgRxInt1CtrlAlt1_ = crate::Reg<cfg_rx_int1_ctrl_alt1_::CfgRxInt1CtrlAlt1_Spec>;
#[doc = "Receive interrupt control register for RX_INT1. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_int1_ctrl_alt1_;
#[doc = "CFG_RX_INT2_CTRL_ALT1_ (rw) register accessor: Receive interrupt control register for RX_INT2. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_int2_ctrl_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_int2_ctrl_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_int2_ctrl_alt1_`]
module"]
#[doc(alias = "CFG_RX_INT2_CTRL_ALT1_")]
pub type CfgRxInt2CtrlAlt1_ = crate::Reg<cfg_rx_int2_ctrl_alt1_::CfgRxInt2CtrlAlt1_Spec>;
#[doc = "Receive interrupt control register for RX_INT2. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_int2_ctrl_alt1_;
#[doc = "CFG_RX_LOCK_CTRL (rw) register accessor: Receive lock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_lock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_lock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_lock_ctrl`]
module"]
#[doc(alias = "CFG_RX_LOCK_CTRL")]
pub type CfgRxLockCtrl = crate::Reg<cfg_rx_lock_ctrl::CfgRxLockCtrlSpec>;
#[doc = "Receive lock control register"]
pub mod cfg_rx_lock_ctrl;
#[doc = "CFG_Reserved_7 (rw) register accessor: CFG_Reserved_7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_7`]
module"]
#[doc(alias = "CFG_Reserved_7")]
pub type CfgReserved7 = crate::Reg<cfg_reserved_7::CfgReserved7Spec>;
#[doc = "CFG_Reserved_7"]
pub mod cfg_reserved_7;
#[doc = "CFG_RX_ECC_DATA (rw) register accessor: Receive ECC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ecc_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ecc_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ecc_data`]
module"]
#[doc(alias = "CFG_RX_ECC_DATA")]
pub type CfgRxEccData = crate::Reg<cfg_rx_ecc_data::CfgRxEccDataSpec>;
#[doc = "Receive ECC data register"]
pub mod cfg_rx_ecc_data;
#[doc = "CFG_RX_ECC_VAL (rw) register accessor: Receive ECC value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ecc_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ecc_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ecc_val`]
module"]
#[doc(alias = "CFG_RX_ECC_VAL")]
pub type CfgRxEccVal = crate::Reg<cfg_rx_ecc_val::CfgRxEccValSpec>;
#[doc = "Receive ECC value register"]
pub mod cfg_rx_ecc_val;
#[doc = "CFG_Reserved_8 (rw) register accessor: CFG_Reserved_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_8`]
module"]
#[doc(alias = "CFG_Reserved_8")]
pub type CfgReserved8 = crate::Reg<cfg_reserved_8::CfgReserved8Spec>;
#[doc = "CFG_Reserved_8"]
pub mod cfg_reserved_8;
#[doc = "CFG_RX_ECC_SEC_DATA (rw) register accessor: Receive ECC corrected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ecc_sec_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ecc_sec_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ecc_sec_data`]
module"]
#[doc(alias = "CFG_RX_ECC_SEC_DATA")]
pub type CfgRxEccSecData = crate::Reg<cfg_rx_ecc_sec_data::CfgRxEccSecDataSpec>;
#[doc = "Receive ECC corrected data register"]
pub mod cfg_rx_ecc_sec_data;
#[doc = "CFG_RX_ECC_LOG (rw) register accessor: Receive ECC log and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ecc_log::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ecc_log::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ecc_log`]
module"]
#[doc(alias = "CFG_RX_ECC_LOG")]
pub type CfgRxEccLog = crate::Reg<cfg_rx_ecc_log::CfgRxEccLogSpec>;
#[doc = "Receive ECC log and status register"]
pub mod cfg_rx_ecc_log;
#[doc = "CFG_Reserved_9 (rw) register accessor: CFG_Reserved_9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_9`]
module"]
#[doc(alias = "CFG_Reserved_9")]
pub type CfgReserved9 = crate::Reg<cfg_reserved_9::CfgReserved9Spec>;
#[doc = "CFG_Reserved_9"]
pub mod cfg_reserved_9;
#[doc = "CFG_RX_FRAME_TAG_CMP (rw) register accessor: Receive frame tag compare register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_tag_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_tag_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_frame_tag_cmp`]
module"]
#[doc(alias = "CFG_RX_FRAME_TAG_CMP")]
pub type CfgRxFrameTagCmp = crate::Reg<cfg_rx_frame_tag_cmp::CfgRxFrameTagCmpSpec>;
#[doc = "Receive frame tag compare register. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_frame_tag_cmp;
#[doc = "CFG_RX_PING_TAG_CMP (rw) register accessor: Receive ping tag compare register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_tag_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_tag_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_ping_tag_cmp`]
module"]
#[doc(alias = "CFG_RX_PING_TAG_CMP")]
pub type CfgRxPingTagCmp = crate::Reg<cfg_rx_ping_tag_cmp::CfgRxPingTagCmpSpec>;
#[doc = "Receive ping tag compare register. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_ping_tag_cmp;
#[doc = "CFG_Reserved_10 (rw) register accessor: CFG_Reserved_10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_10`]
module"]
#[doc(alias = "CFG_Reserved_10")]
pub type CfgReserved10 = crate::Reg<cfg_reserved_10::CfgReserved10Spec>;
#[doc = "CFG_Reserved_10"]
pub mod cfg_reserved_10;
#[doc = "CFG_RX_DLYLINE_CTRL (rw) register accessor: Receive delay line control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_dlyline_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_dlyline_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_dlyline_ctrl`]
module"]
#[doc(alias = "CFG_RX_DLYLINE_CTRL")]
pub type CfgRxDlylineCtrl = crate::Reg<cfg_rx_dlyline_ctrl::CfgRxDlylineCtrlSpec>;
#[doc = "Receive delay line control register. Protected by LOCK field in RX_LOCK_CTRL register."]
pub mod cfg_rx_dlyline_ctrl;
#[doc = "CFG_Reserved_11 (rw) register accessor: CFG_Reserved_11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_11`]
module"]
#[doc(alias = "CFG_Reserved_11")]
pub type CfgReserved11 = crate::Reg<cfg_reserved_11::CfgReserved11Spec>;
#[doc = "CFG_Reserved_11"]
pub mod cfg_reserved_11;
#[doc = "CFG_RX_VIS_1 (rw) register accessor: Receive debug visibility register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_vis_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_vis_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_vis_1`]
module"]
#[doc(alias = "CFG_RX_VIS_1")]
pub type CfgRxVis1 = crate::Reg<cfg_rx_vis_1::CfgRxVis1Spec>;
#[doc = "Receive debug visibility register 1"]
pub mod cfg_rx_vis_1;
#[doc = "CFG_Reserved_12 (rw) register accessor: CFG_Reserved_12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_12`]
module"]
#[doc(alias = "CFG_Reserved_12")]
pub type CfgReserved12 = crate::Reg<cfg_reserved_12::CfgReserved12Spec>;
#[doc = "CFG_Reserved_12"]
pub mod cfg_reserved_12;
#[doc = "CFG_RX_BUF_BASE (rw) register accessor: Base address for receive data buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_buf_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_buf_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx_buf_base`]
module"]
#[doc(alias = "CFG_RX_BUF_BASE")]
pub type CfgRxBufBase = crate::Reg<cfg_rx_buf_base::CfgRxBufBaseSpec>;
#[doc = "Base address for receive data buffer"]
pub mod cfg_rx_buf_base;
