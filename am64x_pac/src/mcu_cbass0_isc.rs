#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control: IscRegsIblazarMcu0VbuspMIscRegion0Control,
    _reserved1: [u8; 0x0c],
    isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l:
        IscRegsIblazarMcu0VbuspMIscRegion0StartAddressL,
    isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h:
        IscRegsIblazarMcu0VbuspMIscRegion0StartAddressH,
    isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l:
        IscRegsIblazarMcu0VbuspMIscRegion0EndAddressL,
    isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h:
        IscRegsIblazarMcu0VbuspMIscRegion0EndAddressH,
    isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control:
        IscRegsIblazarMcu0VbuspMIscRegionDefControl,
}
impl RegisterBlock {
    #[doc = "0x00 - The ISC Region 0 Control Register defines the control fields for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control(
        &self,
    ) -> &IscRegsIblazarMcu0VbuspMIscRegion0Control {
        &self.isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control
    }
    #[doc = "0x10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIblazarMcu0VbuspMIscRegion0StartAddressL {
        &self.isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l
    }
    #[doc = "0x14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIblazarMcu0VbuspMIscRegion0StartAddressH {
        &self.isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h
    }
    #[doc = "0x18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIblazarMcu0VbuspMIscRegion0EndAddressL {
        &self.isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l
    }
    #[doc = "0x1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIblazarMcu0VbuspMIscRegion0EndAddressH {
        &self.isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h
    }
    #[doc = "0x20 - The ISC Default Region Control Register defines the control fields for the master Iblazar_mcu_0.vbusp_m region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control(
        &self,
    ) -> &IscRegsIblazarMcu0VbuspMIscRegionDefControl {
        &self.isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control
    }
}
#[doc = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iblazar_mcu_0.vbusp_m region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_control")]
pub type IscRegsIblazarMcu0VbuspMIscRegion0Control = crate :: Reg < isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control :: IscRegsIblazarMcu0VbuspMIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
pub mod isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_control;
#[doc = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iblazar_mcu_0.vbusp_m region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l")]
pub type IscRegsIblazarMcu0VbuspMIscRegion0StartAddressL = crate :: Reg < isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l :: IscRegsIblazarMcu0VbuspMIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
pub mod isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iblazar_mcu_0.vbusp_m region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h")]
pub type IscRegsIblazarMcu0VbuspMIscRegion0StartAddressH = crate :: Reg < isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h :: IscRegsIblazarMcu0VbuspMIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
pub mod isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iblazar_mcu_0.vbusp_m region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l")]
pub type IscRegsIblazarMcu0VbuspMIscRegion0EndAddressL = crate :: Reg < isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l :: IscRegsIblazarMcu0VbuspMIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
pub mod isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iblazar_mcu_0.vbusp_m region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h")]
pub type IscRegsIblazarMcu0VbuspMIscRegion0EndAddressH = crate :: Reg < isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h :: IscRegsIblazarMcu0VbuspMIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iblazar_mcu_0.vbusp_m region 0 ISC."]
pub mod isc_regs_iblazar_mcu_0_vbusp_m_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iblazar_mcu_0.vbusp_m region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iblazar_mcu_0_vbusp_m_isc_region_def_control")]
pub type IscRegsIblazarMcu0VbuspMIscRegionDefControl = crate :: Reg < isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control :: IscRegsIblazarMcu0VbuspMIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iblazar_mcu_0.vbusp_m region 1 ISC."]
pub mod isc_regs_iblazar_mcu_0_vbusp_m_isc_region_def_control;
