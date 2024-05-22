#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ringacc__src__fifos_ring_head_data: Ringacc_Src_FifosRingHeadData,
    _reserved1: [u8; 0x01fc],
    ringacc__src__fifos_ring_tail_data: Ringacc_Src_FifosRingTailData,
    _reserved2: [u8; 0x01fc],
    ringacc__src__fifos_peek_head_data: Ringacc_Src_FifosPeekHeadData,
    _reserved3: [u8; 0x01fc],
    ringacc__src__fifos_peek_tail_data: Ringacc_Src_FifosPeekTailData,
}
impl RegisterBlock {
    #[doc = "0x00 - The Ring Head Entry Data Registers contain the data which is to be written or which was read from the ring head. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring head element for reads or writes. The data is right justified.)"]
    #[inline(always)]
    pub const fn ringacc__src__fifos_ring_head_data(&self) -> &Ringacc_Src_FifosRingHeadData {
        &self.ringacc__src__fifos_ring_head_data
    }
    #[doc = "0x200 - The Ring Tail Entry Data Registers contain the data which is to be written or which was read from the ring tail. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring tail element for reads or writes. The data is right justified.)"]
    #[inline(always)]
    pub const fn ringacc__src__fifos_ring_tail_data(&self) -> &Ringacc_Src_FifosRingTailData {
        &self.ringacc__src__fifos_ring_tail_data
    }
    #[doc = "0x400 - The Ring Peek Head Entry Data Registers contain the data which is to be read from the ring head without removing the element. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring head element for reads. Writes are ignored. The data is right justified.)"]
    #[inline(always)]
    pub const fn ringacc__src__fifos_peek_head_data(&self) -> &Ringacc_Src_FifosPeekHeadData {
        &self.ringacc__src__fifos_peek_head_data
    }
    #[doc = "0x600 - The Ring Peek Tail Entry Data Registers contain the data which is to be read from the ring tail without removing the element. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring tail element for reads. Writes are ignored. The data is right justified.)"]
    #[inline(always)]
    pub const fn ringacc__src__fifos_peek_tail_data(&self) -> &Ringacc_Src_FifosPeekTailData {
        &self.ringacc__src__fifos_peek_tail_data
    }
}
#[doc = "RINGACC__SRC__FIFOS_RING_HEAD_DATA (rw) register accessor: The Ring Head Entry Data Registers contain the data which is to be written or which was read from the ring head. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring head element for reads or writes. The data is right justified.)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc__src__fifos_ring_head_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc__src__fifos_ring_head_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc__src__fifos_ring_head_data`]
module"]
#[doc(alias = "RINGACC__SRC__FIFOS_RING_HEAD_DATA")]
pub type Ringacc_Src_FifosRingHeadData =
    crate::Reg<ringacc__src__fifos_ring_head_data::Ringacc_Src_FifosRingHeadDataSpec>;
#[doc = "The Ring Head Entry Data Registers contain the data which is to be written or which was read from the ring head. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring head element for reads or writes. The data is right justified.)"]
pub mod ringacc__src__fifos_ring_head_data;
#[doc = "RINGACC__SRC__FIFOS_RING_TAIL_DATA (rw) register accessor: The Ring Tail Entry Data Registers contain the data which is to be written or which was read from the ring tail. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring tail element for reads or writes. The data is right justified.)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc__src__fifos_ring_tail_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc__src__fifos_ring_tail_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc__src__fifos_ring_tail_data`]
module"]
#[doc(alias = "RINGACC__SRC__FIFOS_RING_TAIL_DATA")]
pub type Ringacc_Src_FifosRingTailData =
    crate::Reg<ringacc__src__fifos_ring_tail_data::Ringacc_Src_FifosRingTailDataSpec>;
#[doc = "The Ring Tail Entry Data Registers contain the data which is to be written or which was read from the ring tail. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring tail element for reads or writes. The data is right justified.)"]
pub mod ringacc__src__fifos_ring_tail_data;
#[doc = "RINGACC__SRC__FIFOS_PEEK_HEAD_DATA (rw) register accessor: The Ring Peek Head Entry Data Registers contain the data which is to be read from the ring head without removing the element. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring head element for reads. Writes are ignored. The data is right justified.)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc__src__fifos_peek_head_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc__src__fifos_peek_head_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc__src__fifos_peek_head_data`]
module"]
#[doc(alias = "RINGACC__SRC__FIFOS_PEEK_HEAD_DATA")]
pub type Ringacc_Src_FifosPeekHeadData =
    crate::Reg<ringacc__src__fifos_peek_head_data::Ringacc_Src_FifosPeekHeadDataSpec>;
#[doc = "The Ring Peek Head Entry Data Registers contain the data which is to be read from the ring head without removing the element. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring head element for reads. Writes are ignored. The data is right justified.)"]
pub mod ringacc__src__fifos_peek_head_data;
#[doc = "RINGACC__SRC__FIFOS_PEEK_TAIL_DATA (rw) register accessor: The Ring Peek Tail Entry Data Registers contain the data which is to be read from the ring tail without removing the element. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring tail element for reads. Writes are ignored. The data is right justified.)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc__src__fifos_peek_tail_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc__src__fifos_peek_tail_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc__src__fifos_peek_tail_data`]
module"]
#[doc(alias = "RINGACC__SRC__FIFOS_PEEK_TAIL_DATA")]
pub type Ringacc_Src_FifosPeekTailData =
    crate::Reg<ringacc__src__fifos_peek_tail_data::Ringacc_Src_FifosPeekTailDataSpec>;
#[doc = "The Ring Peek Tail Entry Data Registers contain the data which is to be read from the ring tail without removing the element. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring tail element for reads. Writes are ignored. The data is right justified.)"]
pub mod ringacc__src__fifos_peek_tail_data;
