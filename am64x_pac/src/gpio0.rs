#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mem_pid: MemPid,
    mem_pcr: MemPcr,
    mem_binten: MemBinten,
    _reserved3: [u8; 0x04],
    mem_dir01: MemDir01,
    mem_out_data01: MemOutData01,
    mem_set_data01: MemSetData01,
    mem_clr_data01: MemClrData01,
    mem_in_data01: MemInData01,
    mem_set_ris_trig01: MemSetRisTrig01,
    mem_clr_ris_trig01: MemClrRisTrig01,
    mem_set_fal_trig01: MemSetFalTrig01,
    mem_clr_fal_trig01: MemClrFalTrig01,
    mem_intstat01: MemIntstat01,
    mem_dir23: MemDir23,
    mem_out_data23: MemOutData23,
    mem_set_data23: MemSetData23,
    mem_clr_data23: MemClrData23,
    mem_in_data23: MemInData23,
    mem_set_ris_trig23: MemSetRisTrig23,
    mem_clr_ris_trig23: MemClrRisTrig23,
    mem_set_fal_trig23: MemSetFalTrig23,
    mem_clr_fal_trig23: MemClrFalTrig23,
    mem_intstat23: MemIntstat23,
    mem_dir45: MemDir45,
    mem_out_data45: MemOutData45,
    mem_set_data45: MemSetData45,
    mem_clr_data45: MemClrData45,
    mem_in_data45: MemInData45,
    mem_set_ris_trig45: MemSetRisTrig45,
    mem_clr_ris_trig45: MemClrRisTrig45,
    mem_set_fal_trig45: MemSetFalTrig45,
    mem_clr_fal_trig45: MemClrFalTrig45,
    mem_intstat45: MemIntstat45,
    mem_dir67: MemDir67,
    mem_out_data67: MemOutData67,
    mem_set_data67: MemSetData67,
    mem_clr_data67: MemClrData67,
    mem_in_data67: MemInData67,
    mem_set_ris_trig67: MemSetRisTrig67,
    mem_clr_ris_trig67: MemClrRisTrig67,
    mem_set_fal_trig67: MemSetFalTrig67,
    mem_clr_fal_trig67: MemClrFalTrig67,
    mem_intstat67: MemIntstat67,
    mem_dir8: MemDir8,
    mem_out_data8: MemOutData8,
    mem_set_data8: MemSetData8,
    mem_clr_data8: MemClrData8,
    mem_in_data8: MemInData8,
    mem_set_ris_trig8: MemSetRisTrig8,
    mem_clr_ris_trig8: MemClrRisTrig8,
    mem_set_fal_trig8: MemSetFalTrig8,
    mem_clr_fal_trig8: MemClrFalTrig8,
    mem_intstat8: MemIntstat8,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO Periperal ID Register"]
    #[inline(always)]
    pub const fn mem_pid(&self) -> &MemPid {
        &self.mem_pid
    }
    #[doc = "0x04 - Peripheral Control Register"]
    #[inline(always)]
    pub const fn mem_pcr(&self) -> &MemPcr {
        &self.mem_pcr
    }
    #[doc = "0x08 - Bit Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mem_binten(&self) -> &MemBinten {
        &self.mem_binten
    }
    #[doc = "0x10 - Direction Register"]
    #[inline(always)]
    pub const fn mem_dir01(&self) -> &MemDir01 {
        &self.mem_dir01
    }
    #[doc = "0x14 - Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_out_data01(&self) -> &MemOutData01 {
        &self.mem_out_data01
    }
    #[doc = "0x18 - Set Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_set_data01(&self) -> &MemSetData01 {
        &self.mem_set_data01
    }
    #[doc = "0x1c - Clear Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_clr_data01(&self) -> &MemClrData01 {
        &self.mem_clr_data01
    }
    #[doc = "0x20 - Bank Status Register"]
    #[inline(always)]
    pub const fn mem_in_data01(&self) -> &MemInData01 {
        &self.mem_in_data01
    }
    #[doc = "0x24 - Set Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_ris_trig01(&self) -> &MemSetRisTrig01 {
        &self.mem_set_ris_trig01
    }
    #[doc = "0x28 - Clear Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_ris_trig01(&self) -> &MemClrRisTrig01 {
        &self.mem_clr_ris_trig01
    }
    #[doc = "0x2c - Set Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_fal_trig01(&self) -> &MemSetFalTrig01 {
        &self.mem_set_fal_trig01
    }
    #[doc = "0x30 - Clear Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_fal_trig01(&self) -> &MemClrFalTrig01 {
        &self.mem_clr_fal_trig01
    }
    #[doc = "0x34 - Bank Interrupt Status Register"]
    #[inline(always)]
    pub const fn mem_intstat01(&self) -> &MemIntstat01 {
        &self.mem_intstat01
    }
    #[doc = "0x38 - Direction Register"]
    #[inline(always)]
    pub const fn mem_dir23(&self) -> &MemDir23 {
        &self.mem_dir23
    }
    #[doc = "0x3c - Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_out_data23(&self) -> &MemOutData23 {
        &self.mem_out_data23
    }
    #[doc = "0x40 - Set Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_set_data23(&self) -> &MemSetData23 {
        &self.mem_set_data23
    }
    #[doc = "0x44 - Clear Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_clr_data23(&self) -> &MemClrData23 {
        &self.mem_clr_data23
    }
    #[doc = "0x48 - Bank Status Register"]
    #[inline(always)]
    pub const fn mem_in_data23(&self) -> &MemInData23 {
        &self.mem_in_data23
    }
    #[doc = "0x4c - Set Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_ris_trig23(&self) -> &MemSetRisTrig23 {
        &self.mem_set_ris_trig23
    }
    #[doc = "0x50 - Clear Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_ris_trig23(&self) -> &MemClrRisTrig23 {
        &self.mem_clr_ris_trig23
    }
    #[doc = "0x54 - Set Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_fal_trig23(&self) -> &MemSetFalTrig23 {
        &self.mem_set_fal_trig23
    }
    #[doc = "0x58 - Clear Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_fal_trig23(&self) -> &MemClrFalTrig23 {
        &self.mem_clr_fal_trig23
    }
    #[doc = "0x5c - Bank Interrupt Status Register"]
    #[inline(always)]
    pub const fn mem_intstat23(&self) -> &MemIntstat23 {
        &self.mem_intstat23
    }
    #[doc = "0x60 - Direction Register"]
    #[inline(always)]
    pub const fn mem_dir45(&self) -> &MemDir45 {
        &self.mem_dir45
    }
    #[doc = "0x64 - Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_out_data45(&self) -> &MemOutData45 {
        &self.mem_out_data45
    }
    #[doc = "0x68 - Set Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_set_data45(&self) -> &MemSetData45 {
        &self.mem_set_data45
    }
    #[doc = "0x6c - Clear Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_clr_data45(&self) -> &MemClrData45 {
        &self.mem_clr_data45
    }
    #[doc = "0x70 - Bank Status Register"]
    #[inline(always)]
    pub const fn mem_in_data45(&self) -> &MemInData45 {
        &self.mem_in_data45
    }
    #[doc = "0x74 - Set Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_ris_trig45(&self) -> &MemSetRisTrig45 {
        &self.mem_set_ris_trig45
    }
    #[doc = "0x78 - Clear Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_ris_trig45(&self) -> &MemClrRisTrig45 {
        &self.mem_clr_ris_trig45
    }
    #[doc = "0x7c - Set Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_fal_trig45(&self) -> &MemSetFalTrig45 {
        &self.mem_set_fal_trig45
    }
    #[doc = "0x80 - Clear Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_fal_trig45(&self) -> &MemClrFalTrig45 {
        &self.mem_clr_fal_trig45
    }
    #[doc = "0x84 - Bank Interrupt Status Register"]
    #[inline(always)]
    pub const fn mem_intstat45(&self) -> &MemIntstat45 {
        &self.mem_intstat45
    }
    #[doc = "0x88 - Direction Register"]
    #[inline(always)]
    pub const fn mem_dir67(&self) -> &MemDir67 {
        &self.mem_dir67
    }
    #[doc = "0x8c - Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_out_data67(&self) -> &MemOutData67 {
        &self.mem_out_data67
    }
    #[doc = "0x90 - Set Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_set_data67(&self) -> &MemSetData67 {
        &self.mem_set_data67
    }
    #[doc = "0x94 - Clear Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_clr_data67(&self) -> &MemClrData67 {
        &self.mem_clr_data67
    }
    #[doc = "0x98 - Bank Status Register"]
    #[inline(always)]
    pub const fn mem_in_data67(&self) -> &MemInData67 {
        &self.mem_in_data67
    }
    #[doc = "0x9c - Set Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_ris_trig67(&self) -> &MemSetRisTrig67 {
        &self.mem_set_ris_trig67
    }
    #[doc = "0xa0 - Clear Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_ris_trig67(&self) -> &MemClrRisTrig67 {
        &self.mem_clr_ris_trig67
    }
    #[doc = "0xa4 - Set Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_fal_trig67(&self) -> &MemSetFalTrig67 {
        &self.mem_set_fal_trig67
    }
    #[doc = "0xa8 - Clear Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_fal_trig67(&self) -> &MemClrFalTrig67 {
        &self.mem_clr_fal_trig67
    }
    #[doc = "0xac - Bank Interrupt Status Register"]
    #[inline(always)]
    pub const fn mem_intstat67(&self) -> &MemIntstat67 {
        &self.mem_intstat67
    }
    #[doc = "0xb0 - Direction Register"]
    #[inline(always)]
    pub const fn mem_dir8(&self) -> &MemDir8 {
        &self.mem_dir8
    }
    #[doc = "0xb4 - Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_out_data8(&self) -> &MemOutData8 {
        &self.mem_out_data8
    }
    #[doc = "0xb8 - Set Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_set_data8(&self) -> &MemSetData8 {
        &self.mem_set_data8
    }
    #[doc = "0xbc - Clear Output Drive State Register"]
    #[inline(always)]
    pub const fn mem_clr_data8(&self) -> &MemClrData8 {
        &self.mem_clr_data8
    }
    #[doc = "0xc0 - Bank Status Register"]
    #[inline(always)]
    pub const fn mem_in_data8(&self) -> &MemInData8 {
        &self.mem_in_data8
    }
    #[doc = "0xc4 - Set Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_ris_trig8(&self) -> &MemSetRisTrig8 {
        &self.mem_set_ris_trig8
    }
    #[doc = "0xc8 - Clear Rising Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_ris_trig8(&self) -> &MemClrRisTrig8 {
        &self.mem_clr_ris_trig8
    }
    #[doc = "0xcc - Set Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_set_fal_trig8(&self) -> &MemSetFalTrig8 {
        &self.mem_set_fal_trig8
    }
    #[doc = "0xd0 - Clear Falling Edge Detection Register"]
    #[inline(always)]
    pub const fn mem_clr_fal_trig8(&self) -> &MemClrFalTrig8 {
        &self.mem_clr_fal_trig8
    }
    #[doc = "0xd4 - Bank Interrupt Status Register"]
    #[inline(always)]
    pub const fn mem_intstat8(&self) -> &MemIntstat8 {
        &self.mem_intstat8
    }
}
#[doc = "MEM_pid (rw) register accessor: GPIO Periperal ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_pid`]
module"]
#[doc(alias = "MEM_pid")]
pub type MemPid = crate::Reg<mem_pid::MemPidSpec>;
#[doc = "GPIO Periperal ID Register"]
pub mod mem_pid;
#[doc = "MEM_PCR (rw) register accessor: Peripheral Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_pcr`]
module"]
#[doc(alias = "MEM_PCR")]
pub type MemPcr = crate::Reg<mem_pcr::MemPcrSpec>;
#[doc = "Peripheral Control Register"]
pub mod mem_pcr;
#[doc = "MEM_BINTEN (rw) register accessor: Bit Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_binten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_binten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_binten`]
module"]
#[doc(alias = "MEM_BINTEN")]
pub type MemBinten = crate::Reg<mem_binten::MemBintenSpec>;
#[doc = "Bit Interrupt Enable Register"]
pub mod mem_binten;
#[doc = "MEM_DIR01 (rw) register accessor: Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dir01`]
module"]
#[doc(alias = "MEM_DIR01")]
pub type MemDir01 = crate::Reg<mem_dir01::MemDir01Spec>;
#[doc = "Direction Register"]
pub mod mem_dir01;
#[doc = "MEM_OUT_DATA01 (rw) register accessor: Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_out_data01`]
module"]
#[doc(alias = "MEM_OUT_DATA01")]
pub type MemOutData01 = crate::Reg<mem_out_data01::MemOutData01Spec>;
#[doc = "Output Drive State Register"]
pub mod mem_out_data01;
#[doc = "MEM_SET_DATA01 (rw) register accessor: Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_data01`]
module"]
#[doc(alias = "MEM_SET_DATA01")]
pub type MemSetData01 = crate::Reg<mem_set_data01::MemSetData01Spec>;
#[doc = "Set Output Drive State Register"]
pub mod mem_set_data01;
#[doc = "MEM_CLR_DATA01 (rw) register accessor: Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_data01`]
module"]
#[doc(alias = "MEM_CLR_DATA01")]
pub type MemClrData01 = crate::Reg<mem_clr_data01::MemClrData01Spec>;
#[doc = "Clear Output Drive State Register"]
pub mod mem_clr_data01;
#[doc = "MEM_IN_DATA01 (rw) register accessor: Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_in_data01`]
module"]
#[doc(alias = "MEM_IN_DATA01")]
pub type MemInData01 = crate::Reg<mem_in_data01::MemInData01Spec>;
#[doc = "Bank Status Register"]
pub mod mem_in_data01;
#[doc = "MEM_SET_RIS_TRIG01 (rw) register accessor: Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_ris_trig01`]
module"]
#[doc(alias = "MEM_SET_RIS_TRIG01")]
pub type MemSetRisTrig01 = crate::Reg<mem_set_ris_trig01::MemSetRisTrig01Spec>;
#[doc = "Set Rising Edge Detection Register"]
pub mod mem_set_ris_trig01;
#[doc = "MEM_CLR_RIS_TRIG01 (rw) register accessor: Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_ris_trig01`]
module"]
#[doc(alias = "MEM_CLR_RIS_TRIG01")]
pub type MemClrRisTrig01 = crate::Reg<mem_clr_ris_trig01::MemClrRisTrig01Spec>;
#[doc = "Clear Rising Edge Detection Register"]
pub mod mem_clr_ris_trig01;
#[doc = "MEM_SET_FAL_TRIG01 (rw) register accessor: Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_fal_trig01`]
module"]
#[doc(alias = "MEM_SET_FAL_TRIG01")]
pub type MemSetFalTrig01 = crate::Reg<mem_set_fal_trig01::MemSetFalTrig01Spec>;
#[doc = "Set Falling Edge Detection Register"]
pub mod mem_set_fal_trig01;
#[doc = "MEM_CLR_FAL_TRIG01 (rw) register accessor: Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_fal_trig01`]
module"]
#[doc(alias = "MEM_CLR_FAL_TRIG01")]
pub type MemClrFalTrig01 = crate::Reg<mem_clr_fal_trig01::MemClrFalTrig01Spec>;
#[doc = "Clear Falling Edge Detection Register"]
pub mod mem_clr_fal_trig01;
#[doc = "MEM_INTSTAT01 (rw) register accessor: Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_intstat01`]
module"]
#[doc(alias = "MEM_INTSTAT01")]
pub type MemIntstat01 = crate::Reg<mem_intstat01::MemIntstat01Spec>;
#[doc = "Bank Interrupt Status Register"]
pub mod mem_intstat01;
#[doc = "MEM_DIR23 (rw) register accessor: Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dir23`]
module"]
#[doc(alias = "MEM_DIR23")]
pub type MemDir23 = crate::Reg<mem_dir23::MemDir23Spec>;
#[doc = "Direction Register"]
pub mod mem_dir23;
#[doc = "MEM_OUT_DATA23 (rw) register accessor: Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_out_data23`]
module"]
#[doc(alias = "MEM_OUT_DATA23")]
pub type MemOutData23 = crate::Reg<mem_out_data23::MemOutData23Spec>;
#[doc = "Output Drive State Register"]
pub mod mem_out_data23;
#[doc = "MEM_SET_DATA23 (rw) register accessor: Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_data23`]
module"]
#[doc(alias = "MEM_SET_DATA23")]
pub type MemSetData23 = crate::Reg<mem_set_data23::MemSetData23Spec>;
#[doc = "Set Output Drive State Register"]
pub mod mem_set_data23;
#[doc = "MEM_CLR_DATA23 (rw) register accessor: Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_data23`]
module"]
#[doc(alias = "MEM_CLR_DATA23")]
pub type MemClrData23 = crate::Reg<mem_clr_data23::MemClrData23Spec>;
#[doc = "Clear Output Drive State Register"]
pub mod mem_clr_data23;
#[doc = "MEM_IN_DATA23 (rw) register accessor: Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_in_data23`]
module"]
#[doc(alias = "MEM_IN_DATA23")]
pub type MemInData23 = crate::Reg<mem_in_data23::MemInData23Spec>;
#[doc = "Bank Status Register"]
pub mod mem_in_data23;
#[doc = "MEM_SET_RIS_TRIG23 (rw) register accessor: Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_ris_trig23`]
module"]
#[doc(alias = "MEM_SET_RIS_TRIG23")]
pub type MemSetRisTrig23 = crate::Reg<mem_set_ris_trig23::MemSetRisTrig23Spec>;
#[doc = "Set Rising Edge Detection Register"]
pub mod mem_set_ris_trig23;
#[doc = "MEM_CLR_RIS_TRIG23 (rw) register accessor: Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_ris_trig23`]
module"]
#[doc(alias = "MEM_CLR_RIS_TRIG23")]
pub type MemClrRisTrig23 = crate::Reg<mem_clr_ris_trig23::MemClrRisTrig23Spec>;
#[doc = "Clear Rising Edge Detection Register"]
pub mod mem_clr_ris_trig23;
#[doc = "MEM_SET_FAL_TRIG23 (rw) register accessor: Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_fal_trig23`]
module"]
#[doc(alias = "MEM_SET_FAL_TRIG23")]
pub type MemSetFalTrig23 = crate::Reg<mem_set_fal_trig23::MemSetFalTrig23Spec>;
#[doc = "Set Falling Edge Detection Register"]
pub mod mem_set_fal_trig23;
#[doc = "MEM_CLR_FAL_TRIG23 (rw) register accessor: Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_fal_trig23`]
module"]
#[doc(alias = "MEM_CLR_FAL_TRIG23")]
pub type MemClrFalTrig23 = crate::Reg<mem_clr_fal_trig23::MemClrFalTrig23Spec>;
#[doc = "Clear Falling Edge Detection Register"]
pub mod mem_clr_fal_trig23;
#[doc = "MEM_INTSTAT23 (rw) register accessor: Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_intstat23`]
module"]
#[doc(alias = "MEM_INTSTAT23")]
pub type MemIntstat23 = crate::Reg<mem_intstat23::MemIntstat23Spec>;
#[doc = "Bank Interrupt Status Register"]
pub mod mem_intstat23;
#[doc = "MEM_DIR45 (rw) register accessor: Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dir45`]
module"]
#[doc(alias = "MEM_DIR45")]
pub type MemDir45 = crate::Reg<mem_dir45::MemDir45Spec>;
#[doc = "Direction Register"]
pub mod mem_dir45;
#[doc = "MEM_OUT_DATA45 (rw) register accessor: Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_out_data45`]
module"]
#[doc(alias = "MEM_OUT_DATA45")]
pub type MemOutData45 = crate::Reg<mem_out_data45::MemOutData45Spec>;
#[doc = "Output Drive State Register"]
pub mod mem_out_data45;
#[doc = "MEM_SET_DATA45 (rw) register accessor: Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_data45`]
module"]
#[doc(alias = "MEM_SET_DATA45")]
pub type MemSetData45 = crate::Reg<mem_set_data45::MemSetData45Spec>;
#[doc = "Set Output Drive State Register"]
pub mod mem_set_data45;
#[doc = "MEM_CLR_DATA45 (rw) register accessor: Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_data45`]
module"]
#[doc(alias = "MEM_CLR_DATA45")]
pub type MemClrData45 = crate::Reg<mem_clr_data45::MemClrData45Spec>;
#[doc = "Clear Output Drive State Register"]
pub mod mem_clr_data45;
#[doc = "MEM_IN_DATA45 (rw) register accessor: Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_in_data45`]
module"]
#[doc(alias = "MEM_IN_DATA45")]
pub type MemInData45 = crate::Reg<mem_in_data45::MemInData45Spec>;
#[doc = "Bank Status Register"]
pub mod mem_in_data45;
#[doc = "MEM_SET_RIS_TRIG45 (rw) register accessor: Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_ris_trig45`]
module"]
#[doc(alias = "MEM_SET_RIS_TRIG45")]
pub type MemSetRisTrig45 = crate::Reg<mem_set_ris_trig45::MemSetRisTrig45Spec>;
#[doc = "Set Rising Edge Detection Register"]
pub mod mem_set_ris_trig45;
#[doc = "MEM_CLR_RIS_TRIG45 (rw) register accessor: Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_ris_trig45`]
module"]
#[doc(alias = "MEM_CLR_RIS_TRIG45")]
pub type MemClrRisTrig45 = crate::Reg<mem_clr_ris_trig45::MemClrRisTrig45Spec>;
#[doc = "Clear Rising Edge Detection Register"]
pub mod mem_clr_ris_trig45;
#[doc = "MEM_SET_FAL_TRIG45 (rw) register accessor: Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_fal_trig45`]
module"]
#[doc(alias = "MEM_SET_FAL_TRIG45")]
pub type MemSetFalTrig45 = crate::Reg<mem_set_fal_trig45::MemSetFalTrig45Spec>;
#[doc = "Set Falling Edge Detection Register"]
pub mod mem_set_fal_trig45;
#[doc = "MEM_CLR_FAL_TRIG45 (rw) register accessor: Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_fal_trig45`]
module"]
#[doc(alias = "MEM_CLR_FAL_TRIG45")]
pub type MemClrFalTrig45 = crate::Reg<mem_clr_fal_trig45::MemClrFalTrig45Spec>;
#[doc = "Clear Falling Edge Detection Register"]
pub mod mem_clr_fal_trig45;
#[doc = "MEM_INTSTAT45 (rw) register accessor: Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_intstat45`]
module"]
#[doc(alias = "MEM_INTSTAT45")]
pub type MemIntstat45 = crate::Reg<mem_intstat45::MemIntstat45Spec>;
#[doc = "Bank Interrupt Status Register"]
pub mod mem_intstat45;
#[doc = "MEM_DIR67 (rw) register accessor: Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dir67`]
module"]
#[doc(alias = "MEM_DIR67")]
pub type MemDir67 = crate::Reg<mem_dir67::MemDir67Spec>;
#[doc = "Direction Register"]
pub mod mem_dir67;
#[doc = "MEM_OUT_DATA67 (rw) register accessor: Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_out_data67`]
module"]
#[doc(alias = "MEM_OUT_DATA67")]
pub type MemOutData67 = crate::Reg<mem_out_data67::MemOutData67Spec>;
#[doc = "Output Drive State Register"]
pub mod mem_out_data67;
#[doc = "MEM_SET_DATA67 (rw) register accessor: Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_data67`]
module"]
#[doc(alias = "MEM_SET_DATA67")]
pub type MemSetData67 = crate::Reg<mem_set_data67::MemSetData67Spec>;
#[doc = "Set Output Drive State Register"]
pub mod mem_set_data67;
#[doc = "MEM_CLR_DATA67 (rw) register accessor: Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_data67`]
module"]
#[doc(alias = "MEM_CLR_DATA67")]
pub type MemClrData67 = crate::Reg<mem_clr_data67::MemClrData67Spec>;
#[doc = "Clear Output Drive State Register"]
pub mod mem_clr_data67;
#[doc = "MEM_IN_DATA67 (rw) register accessor: Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_in_data67`]
module"]
#[doc(alias = "MEM_IN_DATA67")]
pub type MemInData67 = crate::Reg<mem_in_data67::MemInData67Spec>;
#[doc = "Bank Status Register"]
pub mod mem_in_data67;
#[doc = "MEM_SET_RIS_TRIG67 (rw) register accessor: Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_ris_trig67`]
module"]
#[doc(alias = "MEM_SET_RIS_TRIG67")]
pub type MemSetRisTrig67 = crate::Reg<mem_set_ris_trig67::MemSetRisTrig67Spec>;
#[doc = "Set Rising Edge Detection Register"]
pub mod mem_set_ris_trig67;
#[doc = "MEM_CLR_RIS_TRIG67 (rw) register accessor: Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_ris_trig67`]
module"]
#[doc(alias = "MEM_CLR_RIS_TRIG67")]
pub type MemClrRisTrig67 = crate::Reg<mem_clr_ris_trig67::MemClrRisTrig67Spec>;
#[doc = "Clear Rising Edge Detection Register"]
pub mod mem_clr_ris_trig67;
#[doc = "MEM_SET_FAL_TRIG67 (rw) register accessor: Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_fal_trig67`]
module"]
#[doc(alias = "MEM_SET_FAL_TRIG67")]
pub type MemSetFalTrig67 = crate::Reg<mem_set_fal_trig67::MemSetFalTrig67Spec>;
#[doc = "Set Falling Edge Detection Register"]
pub mod mem_set_fal_trig67;
#[doc = "MEM_CLR_FAL_TRIG67 (rw) register accessor: Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_fal_trig67`]
module"]
#[doc(alias = "MEM_CLR_FAL_TRIG67")]
pub type MemClrFalTrig67 = crate::Reg<mem_clr_fal_trig67::MemClrFalTrig67Spec>;
#[doc = "Clear Falling Edge Detection Register"]
pub mod mem_clr_fal_trig67;
#[doc = "MEM_INTSTAT67 (rw) register accessor: Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_intstat67`]
module"]
#[doc(alias = "MEM_INTSTAT67")]
pub type MemIntstat67 = crate::Reg<mem_intstat67::MemIntstat67Spec>;
#[doc = "Bank Interrupt Status Register"]
pub mod mem_intstat67;
#[doc = "MEM_DIR8 (rw) register accessor: Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dir8`]
module"]
#[doc(alias = "MEM_DIR8")]
pub type MemDir8 = crate::Reg<mem_dir8::MemDir8Spec>;
#[doc = "Direction Register"]
pub mod mem_dir8;
#[doc = "MEM_OUT_DATA8 (rw) register accessor: Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_out_data8`]
module"]
#[doc(alias = "MEM_OUT_DATA8")]
pub type MemOutData8 = crate::Reg<mem_out_data8::MemOutData8Spec>;
#[doc = "Output Drive State Register"]
pub mod mem_out_data8;
#[doc = "MEM_SET_DATA8 (rw) register accessor: Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_data8`]
module"]
#[doc(alias = "MEM_SET_DATA8")]
pub type MemSetData8 = crate::Reg<mem_set_data8::MemSetData8Spec>;
#[doc = "Set Output Drive State Register"]
pub mod mem_set_data8;
#[doc = "MEM_CLR_DATA8 (rw) register accessor: Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_data8`]
module"]
#[doc(alias = "MEM_CLR_DATA8")]
pub type MemClrData8 = crate::Reg<mem_clr_data8::MemClrData8Spec>;
#[doc = "Clear Output Drive State Register"]
pub mod mem_clr_data8;
#[doc = "MEM_IN_DATA8 (rw) register accessor: Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_in_data8`]
module"]
#[doc(alias = "MEM_IN_DATA8")]
pub type MemInData8 = crate::Reg<mem_in_data8::MemInData8Spec>;
#[doc = "Bank Status Register"]
pub mod mem_in_data8;
#[doc = "MEM_SET_RIS_TRIG8 (rw) register accessor: Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_ris_trig8`]
module"]
#[doc(alias = "MEM_SET_RIS_TRIG8")]
pub type MemSetRisTrig8 = crate::Reg<mem_set_ris_trig8::MemSetRisTrig8Spec>;
#[doc = "Set Rising Edge Detection Register"]
pub mod mem_set_ris_trig8;
#[doc = "MEM_CLR_RIS_TRIG8 (rw) register accessor: Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_ris_trig8`]
module"]
#[doc(alias = "MEM_CLR_RIS_TRIG8")]
pub type MemClrRisTrig8 = crate::Reg<mem_clr_ris_trig8::MemClrRisTrig8Spec>;
#[doc = "Clear Rising Edge Detection Register"]
pub mod mem_clr_ris_trig8;
#[doc = "MEM_SET_FAL_TRIG8 (rw) register accessor: Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_set_fal_trig8`]
module"]
#[doc(alias = "MEM_SET_FAL_TRIG8")]
pub type MemSetFalTrig8 = crate::Reg<mem_set_fal_trig8::MemSetFalTrig8Spec>;
#[doc = "Set Falling Edge Detection Register"]
pub mod mem_set_fal_trig8;
#[doc = "MEM_CLR_FAL_TRIG8 (rw) register accessor: Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clr_fal_trig8`]
module"]
#[doc(alias = "MEM_CLR_FAL_TRIG8")]
pub type MemClrFalTrig8 = crate::Reg<mem_clr_fal_trig8::MemClrFalTrig8Spec>;
#[doc = "Clear Falling Edge Detection Register"]
pub mod mem_clr_fal_trig8;
#[doc = "MEM_INTSTAT8 (rw) register accessor: Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_intstat8`]
module"]
#[doc(alias = "MEM_INTSTAT8")]
pub type MemIntstat8 = crate::Reg<mem_intstat8::MemIntstat8Spec>;
#[doc = "Bank Interrupt Status Register"]
pub mod mem_intstat8;
