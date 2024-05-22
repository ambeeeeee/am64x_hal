#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0:
        QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRMap0,
    _reserved1: [u8; 0x03fc],
    qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0:
        QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWMap0,
    _reserved2: [u8; 0x07fc],
    qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0: QosRegsIemmc8ss16ffcMain0EmmcssWrMap0,
    _reserved3: [u8; 0x03fc],
    qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0: QosRegsIemmc8ss16ffcMain0EmmcssRdMap0,
    _reserved4: [u8; 0x0bfc],
    qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0: QosRegsIgic500ss1_2Main0MemWrVbusmMap0,
    _reserved5: [u8; 0x03fc],
    qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0: QosRegsIgic500ss1_2Main0MemRdVbusmMap0,
    _reserved6: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0: QosRegsIpulsarLiteMain0Cpu0RmstMap0,
    _reserved7: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0: QosRegsIpulsarLiteMain0Cpu0WmstMap0,
    _reserved8: [u8; 0x0ffc],
    qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0: QosRegsIpulsarLiteMain0Cpu1RmstMap0,
    _reserved9: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0: QosRegsIpulsarLiteMain0Cpu1WmstMap0,
    _reserved10: [u8; 0x0ffc],
    qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0: QosRegsIpulsarLiteMain1Cpu0RmstMap0,
    _reserved11: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0: QosRegsIpulsarLiteMain1Cpu0WmstMap0,
    _reserved12: [u8; 0x0ffc],
    qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0: QosRegsIpulsarLiteMain1Cpu1RmstMap0,
    _reserved13: [u8; 0x03fc],
    qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0: QosRegsIpulsarLiteMain1Cpu1WmstMap0,
    _reserved14: [u8; 0x0ffc],
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0: QosRegsIicssG16ffMain0Pr1ExtVbusmMap0,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1: QosRegsIicssG16ffMain0Pr1ExtVbusmMap1,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2: QosRegsIicssG16ffMain0Pr1ExtVbusmMap2,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3: QosRegsIicssG16ffMain0Pr1ExtVbusmMap3,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4: QosRegsIicssG16ffMain0Pr1ExtVbusmMap4,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5: QosRegsIicssG16ffMain0Pr1ExtVbusmMap5,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6: QosRegsIicssG16ffMain0Pr1ExtVbusmMap6,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7: QosRegsIicssG16ffMain0Pr1ExtVbusmMap7,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8: QosRegsIicssG16ffMain0Pr1ExtVbusmMap8,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9: QosRegsIicssG16ffMain0Pr1ExtVbusmMap9,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10: QosRegsIicssG16ffMain0Pr1ExtVbusmMap10,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11: QosRegsIicssG16ffMain0Pr1ExtVbusmMap11,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12: QosRegsIicssG16ffMain0Pr1ExtVbusmMap12,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13: QosRegsIicssG16ffMain0Pr1ExtVbusmMap13,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14: QosRegsIicssG16ffMain0Pr1ExtVbusmMap14,
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15: QosRegsIicssG16ffMain0Pr1ExtVbusmMap15,
    _reserved30: [u8; 0x03c0],
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0: QosRegsIicssG16ffMain1Pr1ExtVbusmMap0,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1: QosRegsIicssG16ffMain1Pr1ExtVbusmMap1,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2: QosRegsIicssG16ffMain1Pr1ExtVbusmMap2,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3: QosRegsIicssG16ffMain1Pr1ExtVbusmMap3,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4: QosRegsIicssG16ffMain1Pr1ExtVbusmMap4,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5: QosRegsIicssG16ffMain1Pr1ExtVbusmMap5,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6: QosRegsIicssG16ffMain1Pr1ExtVbusmMap6,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7: QosRegsIicssG16ffMain1Pr1ExtVbusmMap7,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8: QosRegsIicssG16ffMain1Pr1ExtVbusmMap8,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9: QosRegsIicssG16ffMain1Pr1ExtVbusmMap9,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10: QosRegsIicssG16ffMain1Pr1ExtVbusmMap10,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11: QosRegsIicssG16ffMain1Pr1ExtVbusmMap11,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12: QosRegsIicssG16ffMain1Pr1ExtVbusmMap12,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13: QosRegsIicssG16ffMain1Pr1ExtVbusmMap13,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14: QosRegsIicssG16ffMain1Pr1ExtVbusmMap14,
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15: QosRegsIicssG16ffMain1Pr1ExtVbusmMap15,
    _reserved46: [u8; 0x03c0],
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0: QosRegsIpcieG2x1_64Main0PcieMstRdMap0,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1: QosRegsIpcieG2x1_64Main0PcieMstRdMap1,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2: QosRegsIpcieG2x1_64Main0PcieMstRdMap2,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3: QosRegsIpcieG2x1_64Main0PcieMstRdMap3,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4: QosRegsIpcieG2x1_64Main0PcieMstRdMap4,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5: QosRegsIpcieG2x1_64Main0PcieMstRdMap5,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6: QosRegsIpcieG2x1_64Main0PcieMstRdMap6,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7: QosRegsIpcieG2x1_64Main0PcieMstRdMap7,
    _reserved54: [u8; 0x03e0],
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0: QosRegsIpcieG2x1_64Main0PcieMstWrMap0,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1: QosRegsIpcieG2x1_64Main0PcieMstWrMap1,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2: QosRegsIpcieG2x1_64Main0PcieMstWrMap2,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3: QosRegsIpcieG2x1_64Main0PcieMstWrMap3,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4: QosRegsIpcieG2x1_64Main0PcieMstWrMap4,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5: QosRegsIpcieG2x1_64Main0PcieMstWrMap5,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6: QosRegsIpcieG2x1_64Main0PcieMstWrMap6,
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7: QosRegsIpcieG2x1_64Main0PcieMstWrMap7,
    _reserved62: [u8; 0x03e0],
    qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0: QosRegsIemmcsd4ssMain0EmmcsdssWrMap0,
    _reserved63: [u8; 0x03fc],
    qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0: QosRegsIemmcsd4ssMain0EmmcsdssRdMap0,
    _reserved64: [u8; 0x03fc],
    qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0: QosRegsIsa2UlMain0CtxcachExtDmaMap0,
    _reserved65: [u8; 0x03fc],
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map0,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map1,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map2,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map3,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map4,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map5,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map6,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7: QosRegsIusb3p0ss64_16ffcMain0Mstr0Map7,
    _reserved73: [u8; 0x03e0],
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map0,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map1,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map2,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map3,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map4,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map5,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map6,
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7: QosRegsIusb3p0ss64_16ffcMain0Mstw0Map7,
    _reserved81: [u8; 0x07e0],
    qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0: QosRegsIdebugssK3WrapCv0Main0VbusmrMap0,
    _reserved82: [u8; 0x03fc],
    qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0: QosRegsIdebugssK3WrapCv0Main0VbusmwMap0,
}
impl RegisterBlock {
    #[doc = "0x500 - The Map Register defines the fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r per channel."]
    #[inline(always)]
    pub const fn qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0(
        &self,
    ) -> &QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRMap0 {
        &self.qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0
    }
    #[doc = "0x900 - The Map Register defines the fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w per channel."]
    #[inline(always)]
    pub const fn qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0(
        &self,
    ) -> &QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWMap0 {
        &self.qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0
    }
    #[doc = "0x1100 - The Map Register defines the fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0(
        &self,
    ) -> &QosRegsIemmc8ss16ffcMain0EmmcssWrMap0 {
        &self.qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0
    }
    #[doc = "0x1500 - The Map Register defines the fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0(
        &self,
    ) -> &QosRegsIemmc8ss16ffcMain0EmmcssRdMap0 {
        &self.qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0
    }
    #[doc = "0x2100 - The Map Register defines the fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0(
        &self,
    ) -> &QosRegsIgic500ss1_2Main0MemWrVbusmMap0 {
        &self.qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0
    }
    #[doc = "0x2500 - The Map Register defines the fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0(
        &self,
    ) -> &QosRegsIgic500ss1_2Main0MemRdVbusmMap0 {
        &self.qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0
    }
    #[doc = "0x2900 - The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_rmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain0Cpu0RmstMap0 {
        &self.qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0
    }
    #[doc = "0x2d00 - The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_wmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain0Cpu0WmstMap0 {
        &self.qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0
    }
    #[doc = "0x3d00 - The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_rmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain0Cpu1RmstMap0 {
        &self.qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0
    }
    #[doc = "0x4100 - The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_wmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain0Cpu1WmstMap0 {
        &self.qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0
    }
    #[doc = "0x5100 - The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_rmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain1Cpu0RmstMap0 {
        &self.qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0
    }
    #[doc = "0x5500 - The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_wmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain1Cpu0WmstMap0 {
        &self.qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0
    }
    #[doc = "0x6500 - The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_rmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain1Cpu1RmstMap0 {
        &self.qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0
    }
    #[doc = "0x6900 - The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_wmst per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0(
        &self,
    ) -> &QosRegsIpulsarLiteMain1Cpu1WmstMap0 {
        &self.qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0
    }
    #[doc = "0x7900 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap0 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0
    }
    #[doc = "0x7904 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap1 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1
    }
    #[doc = "0x7908 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap2 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2
    }
    #[doc = "0x790c - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap3 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3
    }
    #[doc = "0x7910 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap4 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4
    }
    #[doc = "0x7914 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap5 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5
    }
    #[doc = "0x7918 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap6 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6
    }
    #[doc = "0x791c - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap7 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7
    }
    #[doc = "0x7920 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap8 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8
    }
    #[doc = "0x7924 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap9 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9
    }
    #[doc = "0x7928 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap10 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10
    }
    #[doc = "0x792c - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap11 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11
    }
    #[doc = "0x7930 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap12 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12
    }
    #[doc = "0x7934 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap13 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13
    }
    #[doc = "0x7938 - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap14 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14
    }
    #[doc = "0x793c - The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15(
        &self,
    ) -> &QosRegsIicssG16ffMain0Pr1ExtVbusmMap15 {
        &self.qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15
    }
    #[doc = "0x7d00 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap0 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0
    }
    #[doc = "0x7d04 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap1 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1
    }
    #[doc = "0x7d08 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap2 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2
    }
    #[doc = "0x7d0c - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap3 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3
    }
    #[doc = "0x7d10 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap4 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4
    }
    #[doc = "0x7d14 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap5 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5
    }
    #[doc = "0x7d18 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap6 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6
    }
    #[doc = "0x7d1c - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap7 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7
    }
    #[doc = "0x7d20 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap8 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8
    }
    #[doc = "0x7d24 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap9 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9
    }
    #[doc = "0x7d28 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap10 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10
    }
    #[doc = "0x7d2c - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap11 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11
    }
    #[doc = "0x7d30 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap12 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12
    }
    #[doc = "0x7d34 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap13 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13
    }
    #[doc = "0x7d38 - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap14 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14
    }
    #[doc = "0x7d3c - The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
    #[inline(always)]
    pub const fn qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15(
        &self,
    ) -> &QosRegsIicssG16ffMain1Pr1ExtVbusmMap15 {
        &self.qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15
    }
    #[doc = "0x8100 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap0 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0
    }
    #[doc = "0x8104 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap1 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1
    }
    #[doc = "0x8108 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap2 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2
    }
    #[doc = "0x810c - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap3 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3
    }
    #[doc = "0x8110 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap4 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4
    }
    #[doc = "0x8114 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap5 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5
    }
    #[doc = "0x8118 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap6 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6
    }
    #[doc = "0x811c - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstRdMap7 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7
    }
    #[doc = "0x8500 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap0 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0
    }
    #[doc = "0x8504 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap1 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1
    }
    #[doc = "0x8508 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap2 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2
    }
    #[doc = "0x850c - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap3 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3
    }
    #[doc = "0x8510 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap4 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4
    }
    #[doc = "0x8514 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap5 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5
    }
    #[doc = "0x8518 - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap6 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6
    }
    #[doc = "0x851c - The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7(
        &self,
    ) -> &QosRegsIpcieG2x1_64Main0PcieMstWrMap7 {
        &self.qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7
    }
    #[doc = "0x8900 - The Map Register defines the fields for the master Iemmcsd4ss_main_0.emmcsdss_wr per channel."]
    #[inline(always)]
    pub const fn qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0(
        &self,
    ) -> &QosRegsIemmcsd4ssMain0EmmcsdssWrMap0 {
        &self.qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0
    }
    #[doc = "0x8d00 - The Map Register defines the fields for the master Iemmcsd4ss_main_0.emmcsdss_rd per channel."]
    #[inline(always)]
    pub const fn qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0(
        &self,
    ) -> &QosRegsIemmcsd4ssMain0EmmcsdssRdMap0 {
        &self.qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0
    }
    #[doc = "0x9100 - The Map Register defines the fields for the master Isa2_ul_main_0.ctxcach_ext_dma per channel."]
    #[inline(always)]
    pub const fn qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0(
        &self,
    ) -> &QosRegsIsa2UlMain0CtxcachExtDmaMap0 {
        &self.qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0
    }
    #[doc = "0x9500 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map0 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0
    }
    #[doc = "0x9504 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map1 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1
    }
    #[doc = "0x9508 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map2 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2
    }
    #[doc = "0x950c - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map3 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3
    }
    #[doc = "0x9510 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map4 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4
    }
    #[doc = "0x9514 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map5 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5
    }
    #[doc = "0x9518 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map6 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6
    }
    #[doc = "0x951c - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstr0Map7 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7
    }
    #[doc = "0x9900 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map0 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0
    }
    #[doc = "0x9904 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map1 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1
    }
    #[doc = "0x9908 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map2 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2
    }
    #[doc = "0x990c - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map3 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3
    }
    #[doc = "0x9910 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map4 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4
    }
    #[doc = "0x9914 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map5 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5
    }
    #[doc = "0x9918 - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map6 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6
    }
    #[doc = "0x991c - The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
    #[inline(always)]
    pub const fn qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7(
        &self,
    ) -> &QosRegsIusb3p0ss64_16ffcMain0Mstw0Map7 {
        &self.qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7
    }
    #[doc = "0xa100 - The Map Register defines the fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr per channel."]
    #[inline(always)]
    pub const fn qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0(
        &self,
    ) -> &QosRegsIdebugssK3WrapCv0Main0VbusmrMap0 {
        &self.qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0
    }
    #[doc = "0xa500 - The Map Register defines the fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw per channel."]
    #[inline(always)]
    pub const fn qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0(
        &self,
    ) -> &QosRegsIdebugssK3WrapCv0Main0VbusmwMap0 {
        &self.qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0
    }
}
#[doc = "QOS_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0 (rw) register accessor: The Map Register defines the fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0`]
module"]
#[doc(alias = "QOS_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0")]
pub type QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRMap0 = crate :: Reg < qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0 :: QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRMap0Spec > ;
#[doc = "The Map Register defines the fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r per channel."]
pub mod qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_map0;
#[doc = "QOS_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0 (rw) register accessor: The Map Register defines the fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0`]
module"]
#[doc(alias = "QOS_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0")]
pub type QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWMap0 = crate :: Reg < qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0 :: QosRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWMap0Spec > ;
#[doc = "The Map Register defines the fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w per channel."]
pub mod qos_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_map0;
#[doc = "QOS_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_map0 (rw) register accessor: The Map Register defines the fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0`]
module"]
#[doc(alias = "QOS_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_map0")]
pub type QosRegsIemmc8ss16ffcMain0EmmcssWrMap0 = crate::Reg<
    qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0::QosRegsIemmc8ss16ffcMain0EmmcssWrMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr per channel."]
pub mod qos_regs_iemmc8ss_16ffc_main_0_emmcss_wr_map0;
#[doc = "QOS_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_map0 (rw) register accessor: The Map Register defines the fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0`]
module"]
#[doc(alias = "QOS_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_map0")]
pub type QosRegsIemmc8ss16ffcMain0EmmcssRdMap0 = crate::Reg<
    qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0::QosRegsIemmc8ss16ffcMain0EmmcssRdMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd per channel."]
pub mod qos_regs_iemmc8ss_16ffc_main_0_emmcss_rd_map0;
#[doc = "QOS_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_map0 (rw) register accessor: The Map Register defines the fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0`]
module"]
#[doc(alias = "QOS_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_map0")]
pub type QosRegsIgic500ss1_2Main0MemWrVbusmMap0 = crate::Reg<
    qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0::QosRegsIgic500ss1_2Main0MemWrVbusmMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm per channel."]
pub mod qos_regs_igic500ss_1_2_main_0_mem_wr_vbusm_map0;
#[doc = "QOS_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_map0 (rw) register accessor: The Map Register defines the fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0`]
module"]
#[doc(alias = "QOS_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_map0")]
pub type QosRegsIgic500ss1_2Main0MemRdVbusmMap0 = crate::Reg<
    qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0::QosRegsIgic500ss1_2Main0MemRdVbusmMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm per channel."]
pub mod qos_regs_igic500ss_1_2_main_0_mem_rd_vbusm_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_0_cpu0_rmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_rmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_0_cpu0_rmst_map0")]
pub type QosRegsIpulsarLiteMain0Cpu0RmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0::QosRegsIpulsarLiteMain0Cpu0RmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_rmst per channel."]
pub mod qos_regs_ipulsar_lite_main_0_cpu0_rmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_0_cpu0_wmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_wmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_0_cpu0_wmst_map0")]
pub type QosRegsIpulsarLiteMain0Cpu0WmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0::QosRegsIpulsarLiteMain0Cpu0WmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu0_wmst per channel."]
pub mod qos_regs_ipulsar_lite_main_0_cpu0_wmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_0_cpu1_rmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_rmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_0_cpu1_rmst_map0")]
pub type QosRegsIpulsarLiteMain0Cpu1RmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0::QosRegsIpulsarLiteMain0Cpu1RmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_rmst per channel."]
pub mod qos_regs_ipulsar_lite_main_0_cpu1_rmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_0_cpu1_wmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_wmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_0_cpu1_wmst_map0")]
pub type QosRegsIpulsarLiteMain0Cpu1WmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0::QosRegsIpulsarLiteMain0Cpu1WmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_0.cpu1_wmst per channel."]
pub mod qos_regs_ipulsar_lite_main_0_cpu1_wmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_1_cpu0_rmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_rmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_1_cpu0_rmst_map0")]
pub type QosRegsIpulsarLiteMain1Cpu0RmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0::QosRegsIpulsarLiteMain1Cpu0RmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_rmst per channel."]
pub mod qos_regs_ipulsar_lite_main_1_cpu0_rmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_1_cpu0_wmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_wmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_1_cpu0_wmst_map0")]
pub type QosRegsIpulsarLiteMain1Cpu0WmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0::QosRegsIpulsarLiteMain1Cpu0WmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu0_wmst per channel."]
pub mod qos_regs_ipulsar_lite_main_1_cpu0_wmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_1_cpu1_rmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_rmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_1_cpu1_rmst_map0")]
pub type QosRegsIpulsarLiteMain1Cpu1RmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0::QosRegsIpulsarLiteMain1Cpu1RmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_rmst per channel."]
pub mod qos_regs_ipulsar_lite_main_1_cpu1_rmst_map0;
#[doc = "QOS_REGS_Ipulsar_lite_main_1_cpu1_wmst_map0 (rw) register accessor: The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_wmst per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipulsar_lite_main_1_cpu1_wmst_map0")]
pub type QosRegsIpulsarLiteMain1Cpu1WmstMap0 = crate::Reg<
    qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0::QosRegsIpulsarLiteMain1Cpu1WmstMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipulsar_lite_main_1.cpu1_wmst per channel."]
pub mod qos_regs_ipulsar_lite_main_1_cpu1_wmst_map0;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map0 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map0")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap0 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0::QosRegsIicssG16ffMain0Pr1ExtVbusmMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map0;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map1 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map1")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap1 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1::QosRegsIicssG16ffMain0Pr1ExtVbusmMap1Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map1;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map2 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map2")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap2 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2::QosRegsIicssG16ffMain0Pr1ExtVbusmMap2Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map2;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map3 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map3")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap3 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3::QosRegsIicssG16ffMain0Pr1ExtVbusmMap3Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map3;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map4 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map4")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap4 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4::QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map5 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map5")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap5 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5::QosRegsIicssG16ffMain0Pr1ExtVbusmMap5Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map5;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map6 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map6")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap6 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6::QosRegsIicssG16ffMain0Pr1ExtVbusmMap6Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map6;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map7 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map7")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap7 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7::QosRegsIicssG16ffMain0Pr1ExtVbusmMap7Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map7;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map8 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map8")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap8 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8::QosRegsIicssG16ffMain0Pr1ExtVbusmMap8Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map8;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map9 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map9")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap9 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9::QosRegsIicssG16ffMain0Pr1ExtVbusmMap9Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map9;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map10 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map10")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap10 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10::QosRegsIicssG16ffMain0Pr1ExtVbusmMap10Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map10;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map11 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map11")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap11 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11::QosRegsIicssG16ffMain0Pr1ExtVbusmMap11Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map11;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map12 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map12")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap12 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12::QosRegsIicssG16ffMain0Pr1ExtVbusmMap12Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map12;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map13 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map13")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap13 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13::QosRegsIicssG16ffMain0Pr1ExtVbusmMap13Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map13;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map14 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map14")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap14 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14::QosRegsIicssG16ffMain0Pr1ExtVbusmMap14Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map14;
#[doc = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map15 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map15")]
pub type QosRegsIicssG16ffMain0Pr1ExtVbusmMap15 = crate::Reg<
    qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15::QosRegsIicssG16ffMain0Pr1ExtVbusmMap15Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map15;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map0 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map0")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap0 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0::QosRegsIicssG16ffMain1Pr1ExtVbusmMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map0;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map1 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map1")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap1 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1::QosRegsIicssG16ffMain1Pr1ExtVbusmMap1Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map1;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map2 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map2")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap2 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2::QosRegsIicssG16ffMain1Pr1ExtVbusmMap2Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map2;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map3 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map3")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap3 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3::QosRegsIicssG16ffMain1Pr1ExtVbusmMap3Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map3;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map4 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map4")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap4 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4::QosRegsIicssG16ffMain1Pr1ExtVbusmMap4Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map4;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map5 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map5")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap5 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5::QosRegsIicssG16ffMain1Pr1ExtVbusmMap5Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map5;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map6 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map6")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap6 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6::QosRegsIicssG16ffMain1Pr1ExtVbusmMap6Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map6;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map7 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map7")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap7 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7::QosRegsIicssG16ffMain1Pr1ExtVbusmMap7Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map7;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map8 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map8")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap8 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8::QosRegsIicssG16ffMain1Pr1ExtVbusmMap8Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map8;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map9 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map9")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap9 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9::QosRegsIicssG16ffMain1Pr1ExtVbusmMap9Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map9;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map10 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map10")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap10 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10::QosRegsIicssG16ffMain1Pr1ExtVbusmMap10Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map10;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map11 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map11")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap11 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11::QosRegsIicssG16ffMain1Pr1ExtVbusmMap11Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map11;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map12 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map12")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap12 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12::QosRegsIicssG16ffMain1Pr1ExtVbusmMap12Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map12;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map13 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map13")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap13 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13::QosRegsIicssG16ffMain1Pr1ExtVbusmMap13Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map13;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map14 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map14")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap14 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14::QosRegsIicssG16ffMain1Pr1ExtVbusmMap14Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map14;
#[doc = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map15 (rw) register accessor: The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15`]
module"]
#[doc(alias = "QOS_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_map15")]
pub type QosRegsIicssG16ffMain1Pr1ExtVbusmMap15 = crate::Reg<
    qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15::QosRegsIicssG16ffMain1Pr1ExtVbusmMap15Spec,
>;
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm per channel."]
pub mod qos_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_map15;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map0 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map0")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap0 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0::QosRegsIpcieG2x1_64Main0PcieMstRdMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map0;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map1 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map1")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap1 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1::QosRegsIpcieG2x1_64Main0PcieMstRdMap1Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map1;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map2 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map2")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap2 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2::QosRegsIpcieG2x1_64Main0PcieMstRdMap2Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map2;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map3 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map3")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap3 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3::QosRegsIpcieG2x1_64Main0PcieMstRdMap3Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map3;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map4 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map4")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap4 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4::QosRegsIpcieG2x1_64Main0PcieMstRdMap4Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map4;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map5 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map5")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap5 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5::QosRegsIpcieG2x1_64Main0PcieMstRdMap5Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map5;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map6 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map6")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap6 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6::QosRegsIpcieG2x1_64Main0PcieMstRdMap6Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map6;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map7 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_map7")]
pub type QosRegsIpcieG2x1_64Main0PcieMstRdMap7 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7::QosRegsIpcieG2x1_64Main0PcieMstRdMap7Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_map7;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map0 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map0")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap0 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0::QosRegsIpcieG2x1_64Main0PcieMstWrMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map0;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map1 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map1")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap1 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1::QosRegsIpcieG2x1_64Main0PcieMstWrMap1Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map1;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map2 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map2")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap2 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2::QosRegsIpcieG2x1_64Main0PcieMstWrMap2Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map2;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map3 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map3")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap3 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3::QosRegsIpcieG2x1_64Main0PcieMstWrMap3Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map3;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map4 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map4")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap4 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4::QosRegsIpcieG2x1_64Main0PcieMstWrMap4Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map4;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map5 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map5")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap5 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5::QosRegsIpcieG2x1_64Main0PcieMstWrMap5Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map5;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map6 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map6")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap6 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6::QosRegsIpcieG2x1_64Main0PcieMstWrMap6Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map6;
#[doc = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map7 (rw) register accessor: The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7`]
module"]
#[doc(alias = "QOS_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_map7")]
pub type QosRegsIpcieG2x1_64Main0PcieMstWrMap7 = crate::Reg<
    qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7::QosRegsIpcieG2x1_64Main0PcieMstWrMap7Spec,
>;
#[doc = "The Map Register defines the fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr per channel."]
pub mod qos_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_map7;
#[doc = "QOS_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_map0 (rw) register accessor: The Map Register defines the fields for the master Iemmcsd4ss_main_0.emmcsdss_wr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0`]
module"]
#[doc(alias = "QOS_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_map0")]
pub type QosRegsIemmcsd4ssMain0EmmcsdssWrMap0 = crate::Reg<
    qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0::QosRegsIemmcsd4ssMain0EmmcsdssWrMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iemmcsd4ss_main_0.emmcsdss_wr per channel."]
pub mod qos_regs_iemmcsd4ss_main_0_emmcsdss_wr_map0;
#[doc = "QOS_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_map0 (rw) register accessor: The Map Register defines the fields for the master Iemmcsd4ss_main_0.emmcsdss_rd per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0`]
module"]
#[doc(alias = "QOS_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_map0")]
pub type QosRegsIemmcsd4ssMain0EmmcsdssRdMap0 = crate::Reg<
    qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0::QosRegsIemmcsd4ssMain0EmmcsdssRdMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iemmcsd4ss_main_0.emmcsdss_rd per channel."]
pub mod qos_regs_iemmcsd4ss_main_0_emmcsdss_rd_map0;
#[doc = "QOS_REGS_Isa2_ul_main_0_ctxcach_ext_dma_map0 (rw) register accessor: The Map Register defines the fields for the master Isa2_ul_main_0.ctxcach_ext_dma per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0`]
module"]
#[doc(alias = "QOS_REGS_Isa2_ul_main_0_ctxcach_ext_dma_map0")]
pub type QosRegsIsa2UlMain0CtxcachExtDmaMap0 = crate::Reg<
    qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0::QosRegsIsa2UlMain0CtxcachExtDmaMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Isa2_ul_main_0.ctxcach_ext_dma per channel."]
pub mod qos_regs_isa2_ul_main_0_ctxcach_ext_dma_map0;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map0 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map0")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map0 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map0;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map1 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map1")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map1 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map1Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map1;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map2 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map2")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map2 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map2Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map2;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map3 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map3")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map3 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map3Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map3;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map4 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map4")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map4 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map4Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map4;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map5 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map5")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map5 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map5Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map5;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map6 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map6")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map6 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map6Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map6;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map7 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_map7")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstr0Map7 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7::QosRegsIusb3p0ss64_16ffcMain0Mstr0Map7Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstr0_map7;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map0 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map0")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map0 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map0Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map0;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map1 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map1")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map1 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map1Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map1;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map2 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map2")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map2 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map2Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map2;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map3 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map3")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map3 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map3Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map3;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map4 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map4")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map4 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map4Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map4;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map5 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map5")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map5 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map5Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map5;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map6 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map6")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map6 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map6Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map6;
#[doc = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map7 (rw) register accessor: The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7`]
module"]
#[doc(alias = "QOS_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_map7")]
pub type QosRegsIusb3p0ss64_16ffcMain0Mstw0Map7 = crate::Reg<
    qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7::QosRegsIusb3p0ss64_16ffcMain0Mstw0Map7Spec,
>;
#[doc = "The Map Register defines the fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 per channel."]
pub mod qos_regs_iusb3p0ss64_16ffc_main_0_mstw0_map7;
#[doc = "QOS_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_map0 (rw) register accessor: The Map Register defines the fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0`]
module"]
#[doc(alias = "QOS_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_map0")]
pub type QosRegsIdebugssK3WrapCv0Main0VbusmrMap0 = crate::Reg<
    qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0::QosRegsIdebugssK3WrapCv0Main0VbusmrMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr per channel."]
pub mod qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_map0;
#[doc = "QOS_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_map0 (rw) register accessor: The Map Register defines the fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0`]
module"]
#[doc(alias = "QOS_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_map0")]
pub type QosRegsIdebugssK3WrapCv0Main0VbusmwMap0 = crate::Reg<
    qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0::QosRegsIdebugssK3WrapCv0Main0VbusmwMap0Spec,
>;
#[doc = "The Map Register defines the fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw per channel."]
pub mod qos_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_map0;
