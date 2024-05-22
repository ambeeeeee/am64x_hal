#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    qos_regs_iblazar_mcu_0_vbusp_m_map0: QosRegsIblazarMcu0VbuspMMap0,
}
impl RegisterBlock {
    #[doc = "0x100 - The Map Register defines the fields for the master Iblazar_mcu_0.vbusp_m per channel."]
    #[inline(always)]
    pub const fn qos_regs_iblazar_mcu_0_vbusp_m_map0(&self) -> &QosRegsIblazarMcu0VbuspMMap0 {
        &self.qos_regs_iblazar_mcu_0_vbusp_m_map0
    }
}
#[doc = "QOS_REGS_Iblazar_mcu_0_vbusp_m_map0 (rw) register accessor: The Map Register defines the fields for the master Iblazar_mcu_0.vbusp_m per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iblazar_mcu_0_vbusp_m_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iblazar_mcu_0_vbusp_m_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iblazar_mcu_0_vbusp_m_map0`]
module"]
#[doc(alias = "QOS_REGS_Iblazar_mcu_0_vbusp_m_map0")]
pub type QosRegsIblazarMcu0VbuspMMap0 =
    crate::Reg<qos_regs_iblazar_mcu_0_vbusp_m_map0::QosRegsIblazarMcu0VbuspMMap0Spec>;
#[doc = "The Map Register defines the fields for the master Iblazar_mcu_0.vbusp_m per channel."]
pub mod qos_regs_iblazar_mcu_0_vbusp_m_map0;
