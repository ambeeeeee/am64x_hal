#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_tx_master_ctrl: CfgTxMasterCtrl,
    cfg_reserved_1: CfgReserved1,
    cfg_tx_clk_ctrl: CfgTxClkCtrl,
    cfg_reserved_2: CfgReserved2,
    cfg_tx_oper_ctrl_lo_alt1_: CfgTxOperCtrlLoAlt1_,
    cfg_tx_oper_ctrl_hi_alt1_: CfgTxOperCtrlHiAlt1_,
    cfg_tx_frame_ctrl: CfgTxFrameCtrl,
    cfg_tx_frame_tag_udata: CfgTxFrameTagUdata,
    cfg_tx_buf_ptr_load: CfgTxBufPtrLoad,
    cfg_tx_buf_ptr_sts: CfgTxBufPtrSts,
    cfg_tx_ping_ctrl_alt1_: CfgTxPingCtrlAlt1_,
    cfg_tx_ping_tag: CfgTxPingTag,
    cfg_tx_ping_to_ref: CfgTxPingToRef,
    cfg_tx_ping_to_cnt: CfgTxPingToCnt,
    cfg_tx_int_ctrl: CfgTxIntCtrl,
    cfg_tx_dma_ctrl: CfgTxDmaCtrl,
    cfg_tx_lock_ctrl: CfgTxLockCtrl,
    cfg_reserved_3: CfgReserved3,
    cfg_tx_evt_sts: CfgTxEvtSts,
    cfg_reserved_4: CfgReserved4,
    cfg_tx_evt_clr: CfgTxEvtClr,
    cfg_tx_evt_frc: CfgTxEvtFrc,
    cfg_tx_user_crc: CfgTxUserCrc,
    cfg_reserved_5: CfgReserved5,
    cfg_reserved_6: CfgReserved6,
    cfg_reserved_7: CfgReserved7,
    cfg_reserved_8: CfgReserved8,
    cfg_tx_ecc_data: CfgTxEccData,
    cfg_tx_ecc_val: CfgTxEccVal,
    cfg_reserved_9: CfgReserved9,
    _reserved30: [u8; 0x38],
    cfg_tx_buf_base: CfgTxBufBase,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit master control register"]
    #[inline(always)]
    pub const fn cfg_tx_master_ctrl(&self) -> &CfgTxMasterCtrl {
        &self.cfg_tx_master_ctrl
    }
    #[doc = "0x02 - CFG_Reserved_1"]
    #[inline(always)]
    pub const fn cfg_reserved_1(&self) -> &CfgReserved1 {
        &self.cfg_reserved_1
    }
    #[doc = "0x04 - Transmit clock control register. Protected by LOCK field in TX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_tx_clk_ctrl(&self) -> &CfgTxClkCtrl {
        &self.cfg_tx_clk_ctrl
    }
    #[doc = "0x06 - CFG_Reserved_2"]
    #[inline(always)]
    pub const fn cfg_reserved_2(&self) -> &CfgReserved2 {
        &self.cfg_reserved_2
    }
    #[doc = "0x08 - Transmit operation control register low. Protected by LOCK field in TX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_tx_oper_ctrl_lo_alt1_(&self) -> &CfgTxOperCtrlLoAlt1_ {
        &self.cfg_tx_oper_ctrl_lo_alt1_
    }
    #[doc = "0x0a - Transmit operation control register high. Protected by LOCK field in TX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_tx_oper_ctrl_hi_alt1_(&self) -> &CfgTxOperCtrlHiAlt1_ {
        &self.cfg_tx_oper_ctrl_hi_alt1_
    }
    #[doc = "0x0c - Transmit frame control register"]
    #[inline(always)]
    pub const fn cfg_tx_frame_ctrl(&self) -> &CfgTxFrameCtrl {
        &self.cfg_tx_frame_ctrl
    }
    #[doc = "0x0e - Transmit frame tag and user data register"]
    #[inline(always)]
    pub const fn cfg_tx_frame_tag_udata(&self) -> &CfgTxFrameTagUdata {
        &self.cfg_tx_frame_tag_udata
    }
    #[doc = "0x10 - Transmit buffer pointer control load register"]
    #[inline(always)]
    pub const fn cfg_tx_buf_ptr_load(&self) -> &CfgTxBufPtrLoad {
        &self.cfg_tx_buf_ptr_load
    }
    #[doc = "0x12 - Transmit buffer pointer control status register"]
    #[inline(always)]
    pub const fn cfg_tx_buf_ptr_sts(&self) -> &CfgTxBufPtrSts {
        &self.cfg_tx_buf_ptr_sts
    }
    #[doc = "0x14 - Transmit ping control register. Protected by LOCK field in TX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_tx_ping_ctrl_alt1_(&self) -> &CfgTxPingCtrlAlt1_ {
        &self.cfg_tx_ping_ctrl_alt1_
    }
    #[doc = "0x16 - Transmit ping tag register"]
    #[inline(always)]
    pub const fn cfg_tx_ping_tag(&self) -> &CfgTxPingTag {
        &self.cfg_tx_ping_tag
    }
    #[doc = "0x18 - Transmit ping timeout counter reference. Protected by LOCK field in TX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_tx_ping_to_ref(&self) -> &CfgTxPingToRef {
        &self.cfg_tx_ping_to_ref
    }
    #[doc = "0x1c - Transmit ping timeout current count"]
    #[inline(always)]
    pub const fn cfg_tx_ping_to_cnt(&self) -> &CfgTxPingToCnt {
        &self.cfg_tx_ping_to_cnt
    }
    #[doc = "0x20 - Transmit interrupt event control register. Protected by LOCK field in TX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_tx_int_ctrl(&self) -> &CfgTxIntCtrl {
        &self.cfg_tx_int_ctrl
    }
    #[doc = "0x22 - Transmit DMA event control register. Protected by LOCK field in TX_LOCK_CTRL register."]
    #[inline(always)]
    pub const fn cfg_tx_dma_ctrl(&self) -> &CfgTxDmaCtrl {
        &self.cfg_tx_dma_ctrl
    }
    #[doc = "0x24 - Transmit lock control register"]
    #[inline(always)]
    pub const fn cfg_tx_lock_ctrl(&self) -> &CfgTxLockCtrl {
        &self.cfg_tx_lock_ctrl
    }
    #[doc = "0x26 - CFG_Reserved_3"]
    #[inline(always)]
    pub const fn cfg_reserved_3(&self) -> &CfgReserved3 {
        &self.cfg_reserved_3
    }
    #[doc = "0x28 - Transmit event and error status flag register"]
    #[inline(always)]
    pub const fn cfg_tx_evt_sts(&self) -> &CfgTxEvtSts {
        &self.cfg_tx_evt_sts
    }
    #[doc = "0x2a - CFG_Reserved_4"]
    #[inline(always)]
    pub const fn cfg_reserved_4(&self) -> &CfgReserved4 {
        &self.cfg_reserved_4
    }
    #[doc = "0x2c - Transmit event and error clear register"]
    #[inline(always)]
    pub const fn cfg_tx_evt_clr(&self) -> &CfgTxEvtClr {
        &self.cfg_tx_evt_clr
    }
    #[doc = "0x2e - Transmit event and error flag force register"]
    #[inline(always)]
    pub const fn cfg_tx_evt_frc(&self) -> &CfgTxEvtFrc {
        &self.cfg_tx_evt_frc
    }
    #[doc = "0x30 - Transmit user-defined CRC register"]
    #[inline(always)]
    pub const fn cfg_tx_user_crc(&self) -> &CfgTxUserCrc {
        &self.cfg_tx_user_crc
    }
    #[doc = "0x32 - CFG_Reserved_5"]
    #[inline(always)]
    pub const fn cfg_reserved_5(&self) -> &CfgReserved5 {
        &self.cfg_reserved_5
    }
    #[doc = "0x34 - CFG_Reserved_6"]
    #[inline(always)]
    pub const fn cfg_reserved_6(&self) -> &CfgReserved6 {
        &self.cfg_reserved_6
    }
    #[doc = "0x38 - CFG_Reserved_7"]
    #[inline(always)]
    pub const fn cfg_reserved_7(&self) -> &CfgReserved7 {
        &self.cfg_reserved_7
    }
    #[doc = "0x3c - CFG_Reserved_8"]
    #[inline(always)]
    pub const fn cfg_reserved_8(&self) -> &CfgReserved8 {
        &self.cfg_reserved_8
    }
    #[doc = "0x40 - Transmit ECC data register"]
    #[inline(always)]
    pub const fn cfg_tx_ecc_data(&self) -> &CfgTxEccData {
        &self.cfg_tx_ecc_data
    }
    #[doc = "0x44 - Transmit ECC value register"]
    #[inline(always)]
    pub const fn cfg_tx_ecc_val(&self) -> &CfgTxEccVal {
        &self.cfg_tx_ecc_val
    }
    #[doc = "0x46 - CFG_Reserved_9"]
    #[inline(always)]
    pub const fn cfg_reserved_9(&self) -> &CfgReserved9 {
        &self.cfg_reserved_9
    }
    #[doc = "0x80 - Base address for transmit buffer"]
    #[inline(always)]
    pub const fn cfg_tx_buf_base(&self) -> &CfgTxBufBase {
        &self.cfg_tx_buf_base
    }
}
#[doc = "CFG_TX_MASTER_CTRL (rw) register accessor: Transmit master control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_master_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_master_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_master_ctrl`]
module"]
#[doc(alias = "CFG_TX_MASTER_CTRL")]
pub type CfgTxMasterCtrl = crate::Reg<cfg_tx_master_ctrl::CfgTxMasterCtrlSpec>;
#[doc = "Transmit master control register"]
pub mod cfg_tx_master_ctrl;
#[doc = "CFG_Reserved_1 (rw) register accessor: CFG_Reserved_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_1`]
module"]
#[doc(alias = "CFG_Reserved_1")]
pub type CfgReserved1 = crate::Reg<cfg_reserved_1::CfgReserved1Spec>;
#[doc = "CFG_Reserved_1"]
pub mod cfg_reserved_1;
#[doc = "CFG_TX_CLK_CTRL (rw) register accessor: Transmit clock control register. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_clk_ctrl`]
module"]
#[doc(alias = "CFG_TX_CLK_CTRL")]
pub type CfgTxClkCtrl = crate::Reg<cfg_tx_clk_ctrl::CfgTxClkCtrlSpec>;
#[doc = "Transmit clock control register. Protected by LOCK field in TX_LOCK_CTRL register."]
pub mod cfg_tx_clk_ctrl;
#[doc = "CFG_Reserved_2 (rw) register accessor: CFG_Reserved_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_2`]
module"]
#[doc(alias = "CFG_Reserved_2")]
pub type CfgReserved2 = crate::Reg<cfg_reserved_2::CfgReserved2Spec>;
#[doc = "CFG_Reserved_2"]
pub mod cfg_reserved_2;
#[doc = "CFG_TX_OPER_CTRL_LO_ALT1_ (rw) register accessor: Transmit operation control register low. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_oper_ctrl_lo_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_oper_ctrl_lo_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_oper_ctrl_lo_alt1_`]
module"]
#[doc(alias = "CFG_TX_OPER_CTRL_LO_ALT1_")]
pub type CfgTxOperCtrlLoAlt1_ = crate::Reg<cfg_tx_oper_ctrl_lo_alt1_::CfgTxOperCtrlLoAlt1_Spec>;
#[doc = "Transmit operation control register low. Protected by LOCK field in TX_LOCK_CTRL register."]
pub mod cfg_tx_oper_ctrl_lo_alt1_;
#[doc = "CFG_TX_OPER_CTRL_HI_ALT1_ (rw) register accessor: Transmit operation control register high. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_oper_ctrl_hi_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_oper_ctrl_hi_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_oper_ctrl_hi_alt1_`]
module"]
#[doc(alias = "CFG_TX_OPER_CTRL_HI_ALT1_")]
pub type CfgTxOperCtrlHiAlt1_ = crate::Reg<cfg_tx_oper_ctrl_hi_alt1_::CfgTxOperCtrlHiAlt1_Spec>;
#[doc = "Transmit operation control register high. Protected by LOCK field in TX_LOCK_CTRL register."]
pub mod cfg_tx_oper_ctrl_hi_alt1_;
#[doc = "CFG_TX_FRAME_CTRL (rw) register accessor: Transmit frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_frame_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_frame_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_frame_ctrl`]
module"]
#[doc(alias = "CFG_TX_FRAME_CTRL")]
pub type CfgTxFrameCtrl = crate::Reg<cfg_tx_frame_ctrl::CfgTxFrameCtrlSpec>;
#[doc = "Transmit frame control register"]
pub mod cfg_tx_frame_ctrl;
#[doc = "CFG_TX_FRAME_TAG_UDATA (rw) register accessor: Transmit frame tag and user data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_frame_tag_udata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_frame_tag_udata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_frame_tag_udata`]
module"]
#[doc(alias = "CFG_TX_FRAME_TAG_UDATA")]
pub type CfgTxFrameTagUdata = crate::Reg<cfg_tx_frame_tag_udata::CfgTxFrameTagUdataSpec>;
#[doc = "Transmit frame tag and user data register"]
pub mod cfg_tx_frame_tag_udata;
#[doc = "CFG_TX_BUF_PTR_LOAD (rw) register accessor: Transmit buffer pointer control load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_buf_ptr_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_buf_ptr_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_buf_ptr_load`]
module"]
#[doc(alias = "CFG_TX_BUF_PTR_LOAD")]
pub type CfgTxBufPtrLoad = crate::Reg<cfg_tx_buf_ptr_load::CfgTxBufPtrLoadSpec>;
#[doc = "Transmit buffer pointer control load register"]
pub mod cfg_tx_buf_ptr_load;
#[doc = "CFG_TX_BUF_PTR_STS (rw) register accessor: Transmit buffer pointer control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_buf_ptr_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_buf_ptr_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_buf_ptr_sts`]
module"]
#[doc(alias = "CFG_TX_BUF_PTR_STS")]
pub type CfgTxBufPtrSts = crate::Reg<cfg_tx_buf_ptr_sts::CfgTxBufPtrStsSpec>;
#[doc = "Transmit buffer pointer control status register"]
pub mod cfg_tx_buf_ptr_sts;
#[doc = "CFG_TX_PING_CTRL_ALT1_ (rw) register accessor: Transmit ping control register. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_ctrl_alt1_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_ctrl_alt1_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_ping_ctrl_alt1_`]
module"]
#[doc(alias = "CFG_TX_PING_CTRL_ALT1_")]
pub type CfgTxPingCtrlAlt1_ = crate::Reg<cfg_tx_ping_ctrl_alt1_::CfgTxPingCtrlAlt1_Spec>;
#[doc = "Transmit ping control register. Protected by LOCK field in TX_LOCK_CTRL register."]
pub mod cfg_tx_ping_ctrl_alt1_;
#[doc = "CFG_TX_PING_TAG (rw) register accessor: Transmit ping tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_ping_tag`]
module"]
#[doc(alias = "CFG_TX_PING_TAG")]
pub type CfgTxPingTag = crate::Reg<cfg_tx_ping_tag::CfgTxPingTagSpec>;
#[doc = "Transmit ping tag register"]
pub mod cfg_tx_ping_tag;
#[doc = "CFG_TX_PING_TO_REF (rw) register accessor: Transmit ping timeout counter reference. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_to_ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_to_ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_ping_to_ref`]
module"]
#[doc(alias = "CFG_TX_PING_TO_REF")]
pub type CfgTxPingToRef = crate::Reg<cfg_tx_ping_to_ref::CfgTxPingToRefSpec>;
#[doc = "Transmit ping timeout counter reference. Protected by LOCK field in TX_LOCK_CTRL register."]
pub mod cfg_tx_ping_to_ref;
#[doc = "CFG_TX_PING_TO_CNT (rw) register accessor: Transmit ping timeout current count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_ping_to_cnt`]
module"]
#[doc(alias = "CFG_TX_PING_TO_CNT")]
pub type CfgTxPingToCnt = crate::Reg<cfg_tx_ping_to_cnt::CfgTxPingToCntSpec>;
#[doc = "Transmit ping timeout current count"]
pub mod cfg_tx_ping_to_cnt;
#[doc = "CFG_TX_INT_CTRL (rw) register accessor: Transmit interrupt event control register. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_int_ctrl`]
module"]
#[doc(alias = "CFG_TX_INT_CTRL")]
pub type CfgTxIntCtrl = crate::Reg<cfg_tx_int_ctrl::CfgTxIntCtrlSpec>;
#[doc = "Transmit interrupt event control register. Protected by LOCK field in TX_LOCK_CTRL register."]
pub mod cfg_tx_int_ctrl;
#[doc = "CFG_TX_DMA_CTRL (rw) register accessor: Transmit DMA event control register. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_dma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_dma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_dma_ctrl`]
module"]
#[doc(alias = "CFG_TX_DMA_CTRL")]
pub type CfgTxDmaCtrl = crate::Reg<cfg_tx_dma_ctrl::CfgTxDmaCtrlSpec>;
#[doc = "Transmit DMA event control register. Protected by LOCK field in TX_LOCK_CTRL register."]
pub mod cfg_tx_dma_ctrl;
#[doc = "CFG_TX_LOCK_CTRL (rw) register accessor: Transmit lock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_lock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_lock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_lock_ctrl`]
module"]
#[doc(alias = "CFG_TX_LOCK_CTRL")]
pub type CfgTxLockCtrl = crate::Reg<cfg_tx_lock_ctrl::CfgTxLockCtrlSpec>;
#[doc = "Transmit lock control register"]
pub mod cfg_tx_lock_ctrl;
#[doc = "CFG_Reserved_3 (rw) register accessor: CFG_Reserved_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_3`]
module"]
#[doc(alias = "CFG_Reserved_3")]
pub type CfgReserved3 = crate::Reg<cfg_reserved_3::CfgReserved3Spec>;
#[doc = "CFG_Reserved_3"]
pub mod cfg_reserved_3;
#[doc = "CFG_TX_EVT_STS (rw) register accessor: Transmit event and error status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_evt_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_evt_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_evt_sts`]
module"]
#[doc(alias = "CFG_TX_EVT_STS")]
pub type CfgTxEvtSts = crate::Reg<cfg_tx_evt_sts::CfgTxEvtStsSpec>;
#[doc = "Transmit event and error status flag register"]
pub mod cfg_tx_evt_sts;
#[doc = "CFG_Reserved_4 (rw) register accessor: CFG_Reserved_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_4`]
module"]
#[doc(alias = "CFG_Reserved_4")]
pub type CfgReserved4 = crate::Reg<cfg_reserved_4::CfgReserved4Spec>;
#[doc = "CFG_Reserved_4"]
pub mod cfg_reserved_4;
#[doc = "CFG_TX_EVT_CLR (rw) register accessor: Transmit event and error clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_evt_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_evt_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_evt_clr`]
module"]
#[doc(alias = "CFG_TX_EVT_CLR")]
pub type CfgTxEvtClr = crate::Reg<cfg_tx_evt_clr::CfgTxEvtClrSpec>;
#[doc = "Transmit event and error clear register"]
pub mod cfg_tx_evt_clr;
#[doc = "CFG_TX_EVT_FRC (rw) register accessor: Transmit event and error flag force register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_evt_frc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_evt_frc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_evt_frc`]
module"]
#[doc(alias = "CFG_TX_EVT_FRC")]
pub type CfgTxEvtFrc = crate::Reg<cfg_tx_evt_frc::CfgTxEvtFrcSpec>;
#[doc = "Transmit event and error flag force register"]
pub mod cfg_tx_evt_frc;
#[doc = "CFG_TX_USER_CRC (rw) register accessor: Transmit user-defined CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_user_crc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_user_crc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_user_crc`]
module"]
#[doc(alias = "CFG_TX_USER_CRC")]
pub type CfgTxUserCrc = crate::Reg<cfg_tx_user_crc::CfgTxUserCrcSpec>;
#[doc = "Transmit user-defined CRC register"]
pub mod cfg_tx_user_crc;
#[doc = "CFG_Reserved_5 (rw) register accessor: CFG_Reserved_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_5`]
module"]
#[doc(alias = "CFG_Reserved_5")]
pub type CfgReserved5 = crate::Reg<cfg_reserved_5::CfgReserved5Spec>;
#[doc = "CFG_Reserved_5"]
pub mod cfg_reserved_5;
#[doc = "CFG_Reserved_6 (rw) register accessor: CFG_Reserved_6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_6`]
module"]
#[doc(alias = "CFG_Reserved_6")]
pub type CfgReserved6 = crate::Reg<cfg_reserved_6::CfgReserved6Spec>;
#[doc = "CFG_Reserved_6"]
pub mod cfg_reserved_6;
#[doc = "CFG_Reserved_7 (rw) register accessor: CFG_Reserved_7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_7`]
module"]
#[doc(alias = "CFG_Reserved_7")]
pub type CfgReserved7 = crate::Reg<cfg_reserved_7::CfgReserved7Spec>;
#[doc = "CFG_Reserved_7"]
pub mod cfg_reserved_7;
#[doc = "CFG_Reserved_8 (rw) register accessor: CFG_Reserved_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_8`]
module"]
#[doc(alias = "CFG_Reserved_8")]
pub type CfgReserved8 = crate::Reg<cfg_reserved_8::CfgReserved8Spec>;
#[doc = "CFG_Reserved_8"]
pub mod cfg_reserved_8;
#[doc = "CFG_TX_ECC_DATA (rw) register accessor: Transmit ECC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ecc_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ecc_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_ecc_data`]
module"]
#[doc(alias = "CFG_TX_ECC_DATA")]
pub type CfgTxEccData = crate::Reg<cfg_tx_ecc_data::CfgTxEccDataSpec>;
#[doc = "Transmit ECC data register"]
pub mod cfg_tx_ecc_data;
#[doc = "CFG_TX_ECC_VAL (rw) register accessor: Transmit ECC value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ecc_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ecc_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_ecc_val`]
module"]
#[doc(alias = "CFG_TX_ECC_VAL")]
pub type CfgTxEccVal = crate::Reg<cfg_tx_ecc_val::CfgTxEccValSpec>;
#[doc = "Transmit ECC value register"]
pub mod cfg_tx_ecc_val;
#[doc = "CFG_Reserved_9 (rw) register accessor: CFG_Reserved_9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_reserved_9`]
module"]
#[doc(alias = "CFG_Reserved_9")]
pub type CfgReserved9 = crate::Reg<cfg_reserved_9::CfgReserved9Spec>;
#[doc = "CFG_Reserved_9"]
pub mod cfg_reserved_9;
#[doc = "CFG_TX_BUF_BASE (rw) register accessor: Base address for transmit buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_buf_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_buf_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx_buf_base`]
module"]
#[doc(alias = "CFG_TX_BUF_BASE")]
pub type CfgTxBufBase = crate::Reg<cfg_tx_buf_base::CfgTxBufBaseSpec>;
#[doc = "Base address for transmit buffer"]
pub mod cfg_tx_buf_base;
