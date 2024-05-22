#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    ringacc_cfg_ba_lo: RingaccCfgBaLo,
    ringacc_cfg_ba_hi: RingaccCfgBaHi,
    ringacc_cfg_size: RingaccCfgSize,
    ringacc_cfg_evt: RingaccCfgEvt,
    ringacc_cfg_orderid: RingaccCfgOrderid,
}
impl RegisterBlock {
    #[doc = "0x40 - The Tx Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to the element size of the ring, or to double the element size of the ring if the qmode is CREDENTIALS or QM modes. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn ringacc_cfg_ba_lo(&self) -> &RingaccCfgBaLo {
        &self.ringacc_cfg_ba_lo
    }
    #[doc = "0x44 - The Tx Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to the element size of the ring, or to double the element size of the ring if the qmode is CREDENTIALS or QM modes. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn ringacc_cfg_ba_hi(&self) -> &RingaccCfgBaHi {
        &self.ringacc_cfg_ba_hi
    }
    #[doc = "0x48 - The Tx Ring Size Register contains the element size and element counts for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn ringacc_cfg_size(&self) -> &RingaccCfgSize {
        &self.ringacc_cfg_size
    }
    #[doc = "0x4c - The Ring Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote the occurrence of an up event \\[empty to not-empty\\]
or a down event \\[non-empty to empty\\]
for this ring."]
    #[inline(always)]
    pub const fn ringacc_cfg_evt(&self) -> &RingaccCfgEvt {
        &self.ringacc_cfg_evt
    }
    #[doc = "0x50 - The Ring OrderID Register contains the bus orderid value for the ring memory access."]
    #[inline(always)]
    pub const fn ringacc_cfg_orderid(&self) -> &RingaccCfgOrderid {
        &self.ringacc_cfg_orderid
    }
}
#[doc = "RINGACC_CFG_BA_LO (rw) register accessor: The Tx Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to the element size of the ring, or to double the element size of the ring if the qmode is CREDENTIALS or QM modes. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_ba_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_ba_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_cfg_ba_lo`]
module"]
#[doc(alias = "RINGACC_CFG_BA_LO")]
pub type RingaccCfgBaLo = crate::Reg<ringacc_cfg_ba_lo::RingaccCfgBaLoSpec>;
#[doc = "The Tx Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to the element size of the ring, or to double the element size of the ring if the qmode is CREDENTIALS or QM modes. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod ringacc_cfg_ba_lo;
#[doc = "RINGACC_CFG_BA_HI (rw) register accessor: The Tx Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to the element size of the ring, or to double the element size of the ring if the qmode is CREDENTIALS or QM modes. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_ba_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_ba_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_cfg_ba_hi`]
module"]
#[doc(alias = "RINGACC_CFG_BA_HI")]
pub type RingaccCfgBaHi = crate::Reg<ringacc_cfg_ba_hi::RingaccCfgBaHiSpec>;
#[doc = "The Tx Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to the element size of the ring, or to double the element size of the ring if the qmode is CREDENTIALS or QM modes. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod ringacc_cfg_ba_hi;
#[doc = "RINGACC_CFG_SIZE (rw) register accessor: The Tx Ring Size Register contains the element size and element counts for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_cfg_size`]
module"]
#[doc(alias = "RINGACC_CFG_SIZE")]
pub type RingaccCfgSize = crate::Reg<ringacc_cfg_size::RingaccCfgSizeSpec>;
#[doc = "The Tx Ring Size Register contains the element size and element counts for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod ringacc_cfg_size;
#[doc = "RINGACC_CFG_EVT (rw) register accessor: The Ring Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote the occurrence of an up event \\[empty to not-empty\\]
or a down event \\[non-empty to empty\\]
for this ring.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_evt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_evt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_cfg_evt`]
module"]
#[doc(alias = "RINGACC_CFG_EVT")]
pub type RingaccCfgEvt = crate::Reg<ringacc_cfg_evt::RingaccCfgEvtSpec>;
#[doc = "The Ring Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote the occurrence of an up event \\[empty to not-empty\\]
or a down event \\[non-empty to empty\\]
for this ring."]
pub mod ringacc_cfg_evt;
#[doc = "RINGACC_CFG_ORDERID (rw) register accessor: The Ring OrderID Register contains the bus orderid value for the ring memory access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_orderid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_orderid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_cfg_orderid`]
module"]
#[doc(alias = "RINGACC_CFG_ORDERID")]
pub type RingaccCfgOrderid = crate::Reg<ringacc_cfg_orderid::RingaccCfgOrderidSpec>;
#[doc = "The Ring OrderID Register contains the bus orderid value for the ring memory access."]
pub mod ringacc_cfg_orderid;
