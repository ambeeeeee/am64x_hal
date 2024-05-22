#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_unmap_map: IntaggrUnmapMap,
    _reserved1: [u8; 0x7ff8],
    intaggr_unmap_map_: IntaggrUnmapMap_,
    _reserved2: [u8; 0x0ff8],
    intaggr_unmap_map__: IntaggrUnmapMap_,
    _reserved3: [u8; 0x0ff8],
    intaggr_unmap_map___: IntaggrUnmapMap_,
    _reserved4: [u8; 0x0ff8],
    intaggr_unmap_map____: IntaggrUnmapMap_,
    _reserved5: [u8; 0x0ff8],
    intaggr_unmap_map_____: IntaggrUnmapMap_,
    _reserved6: [u8; 0x0ff8],
    intaggr_unmap_map______: IntaggrUnmapMap_,
    _reserved7: [u8; 0x2ff8],
    intaggr_unmap_map_______: IntaggrUnmapMap_,
    _reserved8: [u8; 0x0ff8],
    intaggr_unmap_map________: IntaggrUnmapMap_,
    _reserved9: [u8; 0x0ff8],
    intaggr_unmap_map_________: IntaggrUnmapMap_,
    _reserved10: [u8; 0x0ff8],
    intaggr_unmap_map__________: IntaggrUnmapMap_,
    _reserved11: [u8; 0x0ff8],
    intaggr_unmap_map___________: IntaggrUnmapMap_,
    _reserved12: [u8; 0x0ff8],
    intaggr_unmap_map____________: IntaggrUnmapMap_,
    _reserved13: [u8; 0x0ff8],
    intaggr_unmap_map_____________: IntaggrUnmapMap_,
    _reserved14: [u8; 0x0ff8],
    intaggr_unmap_map______________: IntaggrUnmapMap_,
    _reserved15: [u8; 0x0ff8],
    intaggr_unmap_map_______________: IntaggrUnmapMap_,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map(&self) -> &IntaggrUnmapMap {
        &self.intaggr_unmap_map
    }
    #[doc = "0x8000..0x8008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map_(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map_
    }
    #[doc = "0x9000..0x9008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map__(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map__
    }
    #[doc = "0xa000..0xa008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map___(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map___
    }
    #[doc = "0xb000..0xb008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map____(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map____
    }
    #[doc = "0xc000..0xc008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map_____(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map_____
    }
    #[doc = "0xd000..0xd008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map______(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map______
    }
    #[doc = "0x10000..0x10008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map_______(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map_______
    }
    #[doc = "0x11000..0x11008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map________
    }
    #[doc = "0x12000..0x12008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map_________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map_________
    }
    #[doc = "0x13000..0x13008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map__________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map__________
    }
    #[doc = "0x14000..0x14008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map___________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map___________
    }
    #[doc = "0x15000..0x15008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map____________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map____________
    }
    #[doc = "0x16000..0x16008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map_____________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map_____________
    }
    #[doc = "0x17000..0x17008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map______________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map______________
    }
    #[doc = "0x18000..0x18008 - The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_unmap_map_______________(&self) -> &IntaggrUnmapMap_ {
        &self.intaggr_unmap_map_______________
    }
}
#[doc = "INTAGGR_UNMAP_map (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map")]
pub type IntaggrUnmapMap = crate::Reg<intaggr_unmap_map::IntaggrUnmapMapSpec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map;
#[doc = "INTAGGR_UNMAP_map_ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map_`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map_")]
pub type IntaggrUnmapMap_ = crate::Reg<intaggr_unmap_map_::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map_;
#[doc = "INTAGGR_UNMAP_map__ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map__::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map__::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map__`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map__")]
pub type IntaggrUnmapMap__ = crate::Reg<intaggr_unmap_map__::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map__;
#[doc = "INTAGGR_UNMAP_map___ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map___::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map___::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map___`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map___")]
pub type IntaggrUnmapMap___ = crate::Reg<intaggr_unmap_map___::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map___;
#[doc = "INTAGGR_UNMAP_map____ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map____::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map____::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map____`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map____")]
pub type IntaggrUnmapMap____ = crate::Reg<intaggr_unmap_map____::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map____;
#[doc = "INTAGGR_UNMAP_map_____ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map_____::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map_____::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map_____`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map_____")]
pub type IntaggrUnmapMap_____ = crate::Reg<intaggr_unmap_map_____::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map_____;
#[doc = "INTAGGR_UNMAP_map______ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map______::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map______::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map______`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map______")]
pub type IntaggrUnmapMap______ = crate::Reg<intaggr_unmap_map______::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map______;
#[doc = "INTAGGR_UNMAP_map_______ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map_______::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map_______::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map_______`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map_______")]
pub type IntaggrUnmapMap_______ = crate::Reg<intaggr_unmap_map_______::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map_______;
#[doc = "INTAGGR_UNMAP_map________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map________")]
pub type IntaggrUnmapMap________ = crate::Reg<intaggr_unmap_map________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map________;
#[doc = "INTAGGR_UNMAP_map_________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map_________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map_________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map_________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map_________")]
pub type IntaggrUnmapMap_________ = crate::Reg<intaggr_unmap_map_________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map_________;
#[doc = "INTAGGR_UNMAP_map__________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map__________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map__________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map__________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map__________")]
pub type IntaggrUnmapMap__________ = crate::Reg<intaggr_unmap_map__________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map__________;
#[doc = "INTAGGR_UNMAP_map___________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map___________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map___________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map___________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map___________")]
pub type IntaggrUnmapMap___________ =
    crate::Reg<intaggr_unmap_map___________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map___________;
#[doc = "INTAGGR_UNMAP_map____________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map____________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map____________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map____________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map____________")]
pub type IntaggrUnmapMap____________ =
    crate::Reg<intaggr_unmap_map____________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map____________;
#[doc = "INTAGGR_UNMAP_map_____________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map_____________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map_____________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map_____________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map_____________")]
pub type IntaggrUnmapMap_____________ =
    crate::Reg<intaggr_unmap_map_____________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map_____________;
#[doc = "INTAGGR_UNMAP_map______________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map______________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map______________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map______________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map______________")]
pub type IntaggrUnmapMap______________ =
    crate::Reg<intaggr_unmap_map______________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map______________;
#[doc = "INTAGGR_UNMAP_map_______________ (rw) register accessor: The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_unmap_map_______________::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_unmap_map_______________::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_unmap_map_______________`]
module"]
#[doc(alias = "INTAGGR_UNMAP_map_______________")]
pub type IntaggrUnmapMap_______________ =
    crate::Reg<intaggr_unmap_map_______________::IntaggrUnmapMap_Spec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this unmapped event. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_unmap_map_______________;
