#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    apbaddr_romv8_romentry0: ApbaddrRomv8Romentry0,
    apbaddr_romv8_romentry1: ApbaddrRomv8Romentry1,
    apbaddr_romv8_romentry2: ApbaddrRomv8Romentry2,
    apbaddr_romv8_romentry3: ApbaddrRomv8Romentry3,
    apbaddr_romv8_romentry4: ApbaddrRomv8Romentry4,
    apbaddr_romv8_romentry5: ApbaddrRomv8Romentry5,
    apbaddr_romv8_romentry6: ApbaddrRomv8Romentry6,
    apbaddr_romv8_romentry7: ApbaddrRomv8Romentry7,
    _reserved8: [u8; 0x0fb0],
    apbaddr_romv8_rom_periphid4_val: ApbaddrRomv8RomPeriphid4Val,
    apbaddr_romv8_rom_periphid5_val: ApbaddrRomv8RomPeriphid5Val,
    apbaddr_romv8_rom_periphid6_val: ApbaddrRomv8RomPeriphid6Val,
    apbaddr_romv8_rom_periphid7_val: ApbaddrRomv8RomPeriphid7Val,
    apbaddr_romv8_rom_periphid0_val: ApbaddrRomv8RomPeriphid0Val,
    apbaddr_romv8_rom_periphid1_val: ApbaddrRomv8RomPeriphid1Val,
    apbaddr_romv8_rom_periphid2_val: ApbaddrRomv8RomPeriphid2Val,
    apbaddr_romv8_rom_periphid3_val: ApbaddrRomv8RomPeriphid3Val,
    apbaddr_romv8_rom_componid0_val: ApbaddrRomv8RomComponid0Val,
    apbaddr_romv8_rom_componid1_val: ApbaddrRomv8RomComponid1Val,
    apbaddr_romv8_rom_componid2_val: ApbaddrRomv8RomComponid2Val,
    apbaddr_romv8_rom_componid3_val: ApbaddrRomv8RomComponid3Val,
}
impl RegisterBlock {
    #[doc = "0x00 - ROM Table Entry Register 0 (CPU 0 Debug Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry0(&self) -> &ApbaddrRomv8Romentry0 {
        &self.apbaddr_romv8_romentry0
    }
    #[doc = "0x04 - ROM Table Entry Register 1 (CPU 0 CTI Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry1(&self) -> &ApbaddrRomv8Romentry1 {
        &self.apbaddr_romv8_romentry1
    }
    #[doc = "0x08 - ROM Table Entry Register 2 (CPU 0 PMU Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry2(&self) -> &ApbaddrRomv8Romentry2 {
        &self.apbaddr_romv8_romentry2
    }
    #[doc = "0x0c - ROM Table Entry Register 3 (CPU 0 ETM Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry3(&self) -> &ApbaddrRomv8Romentry3 {
        &self.apbaddr_romv8_romentry3
    }
    #[doc = "0x10 - ROM Table Entry Register 4 (CPU 1 Debug Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry4(&self) -> &ApbaddrRomv8Romentry4 {
        &self.apbaddr_romv8_romentry4
    }
    #[doc = "0x14 - ROM Table Entry Register 5 (CPU 1 CTI Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry5(&self) -> &ApbaddrRomv8Romentry5 {
        &self.apbaddr_romv8_romentry5
    }
    #[doc = "0x18 - ROM Table Entry Register 6 (CPU 1 PMU Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry6(&self) -> &ApbaddrRomv8Romentry6 {
        &self.apbaddr_romv8_romentry6
    }
    #[doc = "0x1c - ROM Table Entry Register 7 (CPU 1 ETM Component)"]
    #[inline(always)]
    pub const fn apbaddr_romv8_romentry7(&self) -> &ApbaddrRomv8Romentry7 {
        &self.apbaddr_romv8_romentry7
    }
    #[doc = "0xfd0 - ROM Peripheral ID 4"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid4_val(&self) -> &ApbaddrRomv8RomPeriphid4Val {
        &self.apbaddr_romv8_rom_periphid4_val
    }
    #[doc = "0xfd4 - ROM Peripheral ID 5"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid5_val(&self) -> &ApbaddrRomv8RomPeriphid5Val {
        &self.apbaddr_romv8_rom_periphid5_val
    }
    #[doc = "0xfd8 - ROM Peripheral ID 6"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid6_val(&self) -> &ApbaddrRomv8RomPeriphid6Val {
        &self.apbaddr_romv8_rom_periphid6_val
    }
    #[doc = "0xfdc - ROM Peripheral ID 7"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid7_val(&self) -> &ApbaddrRomv8RomPeriphid7Val {
        &self.apbaddr_romv8_rom_periphid7_val
    }
    #[doc = "0xfe0 - ROM Peripheral ID 0"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid0_val(&self) -> &ApbaddrRomv8RomPeriphid0Val {
        &self.apbaddr_romv8_rom_periphid0_val
    }
    #[doc = "0xfe4 - ROM Peripheral ID 1"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid1_val(&self) -> &ApbaddrRomv8RomPeriphid1Val {
        &self.apbaddr_romv8_rom_periphid1_val
    }
    #[doc = "0xfe8 - ROM Peripheral ID 2"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid2_val(&self) -> &ApbaddrRomv8RomPeriphid2Val {
        &self.apbaddr_romv8_rom_periphid2_val
    }
    #[doc = "0xfec - ROM Peripheral ID 3"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_periphid3_val(&self) -> &ApbaddrRomv8RomPeriphid3Val {
        &self.apbaddr_romv8_rom_periphid3_val
    }
    #[doc = "0xff0 - ROM Component ID 0"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_componid0_val(&self) -> &ApbaddrRomv8RomComponid0Val {
        &self.apbaddr_romv8_rom_componid0_val
    }
    #[doc = "0xff4 - ROM Component ID 1"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_componid1_val(&self) -> &ApbaddrRomv8RomComponid1Val {
        &self.apbaddr_romv8_rom_componid1_val
    }
    #[doc = "0xff8 - ROM Component ID 2"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_componid2_val(&self) -> &ApbaddrRomv8RomComponid2Val {
        &self.apbaddr_romv8_rom_componid2_val
    }
    #[doc = "0xffc - ROM Component ID 3"]
    #[inline(always)]
    pub const fn apbaddr_romv8_rom_componid3_val(&self) -> &ApbaddrRomv8RomComponid3Val {
        &self.apbaddr_romv8_rom_componid3_val
    }
}
#[doc = "APBADDR_ROMV8_ROMENTRY0 (rw) register accessor: ROM Table Entry Register 0 (CPU 0 Debug Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry0`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY0")]
pub type ApbaddrRomv8Romentry0 = crate::Reg<apbaddr_romv8_romentry0::ApbaddrRomv8Romentry0Spec>;
#[doc = "ROM Table Entry Register 0 (CPU 0 Debug Component)"]
pub mod apbaddr_romv8_romentry0;
#[doc = "APBADDR_ROMV8_ROMENTRY1 (rw) register accessor: ROM Table Entry Register 1 (CPU 0 CTI Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry1`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY1")]
pub type ApbaddrRomv8Romentry1 = crate::Reg<apbaddr_romv8_romentry1::ApbaddrRomv8Romentry1Spec>;
#[doc = "ROM Table Entry Register 1 (CPU 0 CTI Component)"]
pub mod apbaddr_romv8_romentry1;
#[doc = "APBADDR_ROMV8_ROMENTRY2 (rw) register accessor: ROM Table Entry Register 2 (CPU 0 PMU Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry2`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY2")]
pub type ApbaddrRomv8Romentry2 = crate::Reg<apbaddr_romv8_romentry2::ApbaddrRomv8Romentry2Spec>;
#[doc = "ROM Table Entry Register 2 (CPU 0 PMU Component)"]
pub mod apbaddr_romv8_romentry2;
#[doc = "APBADDR_ROMV8_ROMENTRY3 (rw) register accessor: ROM Table Entry Register 3 (CPU 0 ETM Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry3`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY3")]
pub type ApbaddrRomv8Romentry3 = crate::Reg<apbaddr_romv8_romentry3::ApbaddrRomv8Romentry3Spec>;
#[doc = "ROM Table Entry Register 3 (CPU 0 ETM Component)"]
pub mod apbaddr_romv8_romentry3;
#[doc = "APBADDR_ROMV8_ROMENTRY4 (rw) register accessor: ROM Table Entry Register 4 (CPU 1 Debug Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry4`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY4")]
pub type ApbaddrRomv8Romentry4 = crate::Reg<apbaddr_romv8_romentry4::ApbaddrRomv8Romentry4Spec>;
#[doc = "ROM Table Entry Register 4 (CPU 1 Debug Component)"]
pub mod apbaddr_romv8_romentry4;
#[doc = "APBADDR_ROMV8_ROMENTRY5 (rw) register accessor: ROM Table Entry Register 5 (CPU 1 CTI Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry5`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY5")]
pub type ApbaddrRomv8Romentry5 = crate::Reg<apbaddr_romv8_romentry5::ApbaddrRomv8Romentry5Spec>;
#[doc = "ROM Table Entry Register 5 (CPU 1 CTI Component)"]
pub mod apbaddr_romv8_romentry5;
#[doc = "APBADDR_ROMV8_ROMENTRY6 (rw) register accessor: ROM Table Entry Register 6 (CPU 1 PMU Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry6`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY6")]
pub type ApbaddrRomv8Romentry6 = crate::Reg<apbaddr_romv8_romentry6::ApbaddrRomv8Romentry6Spec>;
#[doc = "ROM Table Entry Register 6 (CPU 1 PMU Component)"]
pub mod apbaddr_romv8_romentry6;
#[doc = "APBADDR_ROMV8_ROMENTRY7 (rw) register accessor: ROM Table Entry Register 7 (CPU 1 ETM Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_romentry7`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROMENTRY7")]
pub type ApbaddrRomv8Romentry7 = crate::Reg<apbaddr_romv8_romentry7::ApbaddrRomv8Romentry7Spec>;
#[doc = "ROM Table Entry Register 7 (CPU 1 ETM Component)"]
pub mod apbaddr_romv8_romentry7;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID4_VAL (rw) register accessor: ROM Peripheral ID 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid4_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid4_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid4_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID4_VAL")]
pub type ApbaddrRomv8RomPeriphid4Val =
    crate::Reg<apbaddr_romv8_rom_periphid4_val::ApbaddrRomv8RomPeriphid4ValSpec>;
#[doc = "ROM Peripheral ID 4"]
pub mod apbaddr_romv8_rom_periphid4_val;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID5_VAL (rw) register accessor: ROM Peripheral ID 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid5_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid5_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid5_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID5_VAL")]
pub type ApbaddrRomv8RomPeriphid5Val =
    crate::Reg<apbaddr_romv8_rom_periphid5_val::ApbaddrRomv8RomPeriphid5ValSpec>;
#[doc = "ROM Peripheral ID 5"]
pub mod apbaddr_romv8_rom_periphid5_val;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID6_VAL (rw) register accessor: ROM Peripheral ID 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid6_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid6_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid6_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID6_VAL")]
pub type ApbaddrRomv8RomPeriphid6Val =
    crate::Reg<apbaddr_romv8_rom_periphid6_val::ApbaddrRomv8RomPeriphid6ValSpec>;
#[doc = "ROM Peripheral ID 6"]
pub mod apbaddr_romv8_rom_periphid6_val;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID7_VAL (rw) register accessor: ROM Peripheral ID 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid7_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid7_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid7_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID7_VAL")]
pub type ApbaddrRomv8RomPeriphid7Val =
    crate::Reg<apbaddr_romv8_rom_periphid7_val::ApbaddrRomv8RomPeriphid7ValSpec>;
#[doc = "ROM Peripheral ID 7"]
pub mod apbaddr_romv8_rom_periphid7_val;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID0_VAL (rw) register accessor: ROM Peripheral ID 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid0_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid0_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid0_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID0_VAL")]
pub type ApbaddrRomv8RomPeriphid0Val =
    crate::Reg<apbaddr_romv8_rom_periphid0_val::ApbaddrRomv8RomPeriphid0ValSpec>;
#[doc = "ROM Peripheral ID 0"]
pub mod apbaddr_romv8_rom_periphid0_val;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID1_VAL (rw) register accessor: ROM Peripheral ID 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid1_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid1_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid1_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID1_VAL")]
pub type ApbaddrRomv8RomPeriphid1Val =
    crate::Reg<apbaddr_romv8_rom_periphid1_val::ApbaddrRomv8RomPeriphid1ValSpec>;
#[doc = "ROM Peripheral ID 1"]
pub mod apbaddr_romv8_rom_periphid1_val;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID2_VAL (rw) register accessor: ROM Peripheral ID 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid2_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid2_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid2_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID2_VAL")]
pub type ApbaddrRomv8RomPeriphid2Val =
    crate::Reg<apbaddr_romv8_rom_periphid2_val::ApbaddrRomv8RomPeriphid2ValSpec>;
#[doc = "ROM Peripheral ID 2"]
pub mod apbaddr_romv8_rom_periphid2_val;
#[doc = "APBADDR_ROMV8_ROM_PERIPHID3_VAL (rw) register accessor: ROM Peripheral ID 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid3_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid3_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_periphid3_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_PERIPHID3_VAL")]
pub type ApbaddrRomv8RomPeriphid3Val =
    crate::Reg<apbaddr_romv8_rom_periphid3_val::ApbaddrRomv8RomPeriphid3ValSpec>;
#[doc = "ROM Peripheral ID 3"]
pub mod apbaddr_romv8_rom_periphid3_val;
#[doc = "APBADDR_ROMV8_ROM_COMPONID0_VAL (rw) register accessor: ROM Component ID 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_componid0_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_componid0_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_componid0_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_COMPONID0_VAL")]
pub type ApbaddrRomv8RomComponid0Val =
    crate::Reg<apbaddr_romv8_rom_componid0_val::ApbaddrRomv8RomComponid0ValSpec>;
#[doc = "ROM Component ID 0"]
pub mod apbaddr_romv8_rom_componid0_val;
#[doc = "APBADDR_ROMV8_ROM_COMPONID1_VAL (rw) register accessor: ROM Component ID 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_componid1_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_componid1_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_componid1_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_COMPONID1_VAL")]
pub type ApbaddrRomv8RomComponid1Val =
    crate::Reg<apbaddr_romv8_rom_componid1_val::ApbaddrRomv8RomComponid1ValSpec>;
#[doc = "ROM Component ID 1"]
pub mod apbaddr_romv8_rom_componid1_val;
#[doc = "APBADDR_ROMV8_ROM_COMPONID2_VAL (rw) register accessor: ROM Component ID 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_componid2_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_componid2_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_componid2_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_COMPONID2_VAL")]
pub type ApbaddrRomv8RomComponid2Val =
    crate::Reg<apbaddr_romv8_rom_componid2_val::ApbaddrRomv8RomComponid2ValSpec>;
#[doc = "ROM Component ID 2"]
pub mod apbaddr_romv8_rom_componid2_val;
#[doc = "APBADDR_ROMV8_ROM_COMPONID3_VAL (rw) register accessor: ROM Component ID 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_componid3_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_componid3_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_romv8_rom_componid3_val`]
module"]
#[doc(alias = "APBADDR_ROMV8_ROM_COMPONID3_VAL")]
pub type ApbaddrRomv8RomComponid3Val =
    crate::Reg<apbaddr_romv8_rom_componid3_val::ApbaddrRomv8RomComponid3ValSpec>;
#[doc = "ROM Component ID 3"]
pub mod apbaddr_romv8_rom_componid3_val;
