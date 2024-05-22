#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control:
        IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0Control,
    _reserved1: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control:
        IscRegsIpulsarLiteMain0Cpu0PmstIscRegionDefControl,
    _reserved6: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control:
        IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0Control,
    _reserved7: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control:
        IscRegsIpulsarLiteMain0Cpu1PmstIscRegionDefControl,
    _reserved12: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control:
        IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0Control,
    _reserved13: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control:
        IscRegsIpulsarLiteMain1Cpu0PmstIscRegionDefControl,
    _reserved18: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control:
        IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0Control,
    _reserved19: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control:
        IscRegsIpulsarLiteMain1Cpu1PmstIscRegionDefControl,
}
impl RegisterBlock {
    #[doc = "0x00 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control
    }
    #[doc = "0x10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l
    }
    #[doc = "0x14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h
    }
    #[doc = "0x18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l
    }
    #[doc = "0x1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h
    }
    #[doc = "0x20 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_pmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0PmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control
    }
    #[doc = "0x400 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control
    }
    #[doc = "0x410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l
    }
    #[doc = "0x414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h
    }
    #[doc = "0x418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l
    }
    #[doc = "0x41c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h
    }
    #[doc = "0x420 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_pmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1PmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control
    }
    #[doc = "0x800 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control
    }
    #[doc = "0x810 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l
    }
    #[doc = "0x814 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h
    }
    #[doc = "0x818 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l
    }
    #[doc = "0x81c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h
    }
    #[doc = "0x820 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_pmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0PmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control
    }
    #[doc = "0xc00 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control
    }
    #[doc = "0xc10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l
    }
    #[doc = "0xc14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h
    }
    #[doc = "0xc18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l
    }
    #[doc = "0xc1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h
    }
    #[doc = "0xc20 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_pmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1PmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control
    }
}
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control :: IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain0Cpu0PmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_pmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain0Cpu0PmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control :: IscRegsIpulsarLiteMain0Cpu0PmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_pmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_pmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control :: IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain0Cpu1PmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_pmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain0Cpu1PmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control :: IscRegsIpulsarLiteMain0Cpu1PmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_pmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_pmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control :: IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain1Cpu0PmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_pmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain1Cpu0PmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control :: IscRegsIpulsarLiteMain1Cpu0PmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_pmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_pmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control :: IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain1Cpu1PmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_pmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_pmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain1Cpu1PmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control :: IscRegsIpulsarLiteMain1Cpu1PmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_pmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_pmst_isc_region_def_control;
