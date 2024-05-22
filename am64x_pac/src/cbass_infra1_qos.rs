#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0: QosRegsIpulsarLiteMain0Cpu0PmstMap0,
    _reserved1: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0: QosRegsIpulsarLiteMain0Cpu1PmstMap0,
    _reserved2: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0: QosRegsIpulsarLiteMain1Cpu0PmstMap0,
    _reserved3: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0: QosRegsIpulsarLiteMain1Cpu1PmstMap0,
}
impl RegisterBlock {
    #[doc = "0x100 - The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_pmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain0Cpu0PmstMap0 {
        &self.qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0
    }
    #[doc = "0x500 - The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_pmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain0Cpu1PmstMap0 {
        &self.qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0
    }
    #[doc = "0x900 - The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_pmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain1Cpu0PmstMap0 {
        &self.qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0
    }
    #[doc = "0xd00 - The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_pmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain1Cpu1PmstMap0 {
        &self.qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0
    }
}
#[doc = "QOS_REGS_Ipulsar_lite_main_0_cpu0_pmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_pmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_0_cpu0_pmst_map0")]
pub type QosRegsIpulsarLiteMain0Cpu0PmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0::QosRegsIpulsarLiteMain0Cpu0PmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_pmst per channel."]
pub mod qos_regs_ipulsar_lite_main_0_cpu0_pmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_0_cpu1_pmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_pmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_0_cpu1_pmst_map0")]
pub type QosRegsIpulsarLiteMain0Cpu1PmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0::QosRegsIpulsarLiteMain0Cpu1PmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_pmst per channel."]
pub mod qos_regs_ipulsar_lite_main_0_cpu1_pmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_1_cpu0_pmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_pmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_1_cpu0_pmst_map0")]
pub type QosRegsIpulsarLiteMain1Cpu0PmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0::QosRegsIpulsarLiteMain1Cpu0PmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_pmst per channel."]
pub mod qos_regs_ipulsar_lite_main_1_cpu0_pmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_1_cpu1_pmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_pmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_1_cpu1_pmst_map0")]
pub type QosRegsIpulsarLiteMain1Cpu1PmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0::QosRegsIpulsarLiteMain1Cpu1PmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_pmst per channel."]
pub mod qos_regs_ipulsar_lite_main_1_cpu1_pmst_map0;
