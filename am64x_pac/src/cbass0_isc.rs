#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0Control,
    _reserved1: [u8; 0x0c],
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressL,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressH,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressL,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressH,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegionDefControl,
    _reserved6: [u8; 0x03dc],
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0Control,
    _reserved7: [u8; 0x0c],
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressL,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressH,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressL,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressH,
    isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control:
        IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegionDefControl,
    _reserved12: [u8; 0x07dc],
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control:
        IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0Control,
    _reserved13: [u8; 0x0c],
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l:
        IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressL,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h:
        IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressH,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l:
        IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressL,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h:
        IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressH,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control:
        IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegionDefControl,
    _reserved18: [u8; 0x03dc],
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control:
        IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0Control,
    _reserved19: [u8; 0x0c],
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l:
        IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressL,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h:
        IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressH,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l:
        IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressL,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h:
        IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressH,
    isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control:
        IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegionDefControl,
    _reserved24: [u8; 0x0bdc],
    isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control:
        IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0Control,
    _reserved25: [u8; 0x0c],
    isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l:
        IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressL,
    isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h:
        IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressH,
    isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l:
        IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressL,
    isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h:
        IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressH,
    isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control:
        IscRegsIgic500ss1_2Main0MemWrVbusmIscRegionDefControl,
    _reserved30: [u8; 0x03dc],
    isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control:
        IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0Control,
    _reserved31: [u8; 0x0c],
    isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l:
        IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressL,
    isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h:
        IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressH,
    isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l:
        IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressL,
    isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h:
        IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressH,
    isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control:
        IscRegsIgic500ss1_2Main0MemRdVbusmIscRegionDefControl,
    _reserved36: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control:
        IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0Control,
    _reserved37: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control:
        IscRegsIpulsarLiteMain0Cpu0RmstIscRegionDefControl,
    _reserved42: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control:
        IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0Control,
    _reserved43: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control:
        IscRegsIpulsarLiteMain0Cpu0WmstIscRegionDefControl,
    _reserved48: [u8; 0x0fdc],
    isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control:
        IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0Control,
    _reserved49: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control:
        IscRegsIpulsarLiteMain0Cpu1RmstIscRegionDefControl,
    _reserved54: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control:
        IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0Control,
    _reserved55: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control:
        IscRegsIpulsarLiteMain0Cpu1WmstIscRegionDefControl,
    _reserved60: [u8; 0x0fdc],
    isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control:
        IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0Control,
    _reserved61: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control:
        IscRegsIpulsarLiteMain1Cpu0RmstIscRegionDefControl,
    _reserved66: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control:
        IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0Control,
    _reserved67: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control:
        IscRegsIpulsarLiteMain1Cpu0WmstIscRegionDefControl,
    _reserved72: [u8; 0x0fdc],
    isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control:
        IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0Control,
    _reserved73: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control:
        IscRegsIpulsarLiteMain1Cpu1RmstIscRegionDefControl,
    _reserved78: [u8; 0x03dc],
    isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control:
        IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0Control,
    _reserved79: [u8; 0x0c],
    isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l:
        IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressL,
    isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h:
        IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressH,
    isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l:
        IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressL,
    isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h:
        IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressH,
    isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control:
        IscRegsIpulsarLiteMain1Cpu1WmstIscRegionDefControl,
    _reserved84: [u8; 0x0fdc],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0Control,
    _reserved85: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1Control,
    _reserved90: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2Control,
    _reserved95: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3Control,
    _reserved100: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4Control,
    _reserved105: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5Control,
    _reserved110: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6Control,
    _reserved115: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7Control,
    _reserved120: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8Control,
    _reserved125: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9Control,
    _reserved130: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10Control,
    _reserved135: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11Control,
    _reserved140: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12Control,
    _reserved145: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13Control,
    _reserved150: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14Control,
    _reserved155: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15Control,
    _reserved160: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressL,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressH,
    isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control:
        IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegionDefControl,
    _reserved165: [u8; 0x01fc],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0Control,
    _reserved166: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1Control,
    _reserved171: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2Control,
    _reserved176: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3Control,
    _reserved181: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4Control,
    _reserved186: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5Control,
    _reserved191: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6Control,
    _reserved196: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7Control,
    _reserved201: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8Control,
    _reserved206: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9Control,
    _reserved211: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10Control,
    _reserved216: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11Control,
    _reserved221: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12Control,
    _reserved226: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13Control,
    _reserved231: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14Control,
    _reserved236: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15Control,
    _reserved241: [u8; 0x0c],
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressL,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressH,
    isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control:
        IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegionDefControl,
    _reserved246: [u8; 0x01fc],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0Control,
    _reserved247: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1Control,
    _reserved252: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2Control,
    _reserved257: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3Control,
    _reserved262: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4Control,
    _reserved267: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5Control,
    _reserved272: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6Control,
    _reserved277: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7Control,
    _reserved282: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control:
        IscRegsIpcieG2x1_64Main0PcieMstRdIscRegionDefControl,
    _reserved287: [u8; 0x02fc],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0Control,
    _reserved288: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1Control,
    _reserved293: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2Control,
    _reserved298: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3Control,
    _reserved303: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4Control,
    _reserved308: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5Control,
    _reserved313: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6Control,
    _reserved318: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7Control,
    _reserved323: [u8; 0x0c],
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressL,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressH,
    isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control:
        IscRegsIpcieG2x1_64Main0PcieMstWrIscRegionDefControl,
    _reserved328: [u8; 0x02fc],
    isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control:
        IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0Control,
    _reserved329: [u8; 0x0c],
    isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l:
        IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressL,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h:
        IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressH,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l:
        IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressL,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h:
        IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressH,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control:
        IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegionDefControl,
    _reserved334: [u8; 0x03dc],
    isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control:
        IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0Control,
    _reserved335: [u8; 0x0c],
    isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l:
        IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressL,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h:
        IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressH,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l:
        IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressL,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h:
        IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressH,
    isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control:
        IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegionDefControl,
    _reserved340: [u8; 0x03dc],
    isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control:
        IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0Control,
    _reserved341: [u8; 0x0c],
    isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l:
        IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressL,
    isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h:
        IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressH,
    isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l:
        IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressL,
    isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h:
        IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressH,
    isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control:
        IscRegsIsa2UlMain0CtxcachExtDmaIscRegionDefControl,
    _reserved346: [u8; 0x03dc],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0Control,
    _reserved347: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1Control,
    _reserved352: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2Control,
    _reserved357: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3Control,
    _reserved362: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4Control,
    _reserved367: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5Control,
    _reserved372: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6Control,
    _reserved377: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7Control,
    _reserved382: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegionDefControl,
    _reserved387: [u8; 0x02fc],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0Control,
    _reserved388: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1Control,
    _reserved393: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2Control,
    _reserved398: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3Control,
    _reserved403: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4Control,
    _reserved408: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5Control,
    _reserved413: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6Control,
    _reserved418: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7Control,
    _reserved423: [u8; 0x0c],
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressL,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressH,
    isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control:
        IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegionDefControl,
    _reserved428: [u8; 0x02fc],
    isc_regs_ij7_led_main_0_vbusp_isc_region_0_control: IscRegsIj7LedMain0VbuspIscRegion0Control,
    _reserved429: [u8; 0x0c],
    isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l:
        IscRegsIj7LedMain0VbuspIscRegion0StartAddressL,
    isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h:
        IscRegsIj7LedMain0VbuspIscRegion0StartAddressH,
    isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l:
        IscRegsIj7LedMain0VbuspIscRegion0EndAddressL,
    isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h:
        IscRegsIj7LedMain0VbuspIscRegion0EndAddressH,
    isc_regs_ij7_led_main_0_vbusp_isc_region_def_control:
        IscRegsIj7LedMain0VbuspIscRegionDefControl,
    _reserved434: [u8; 0x03dc],
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control:
        IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0Control,
    _reserved435: [u8; 0x0c],
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l:
        IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressL,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h:
        IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressH,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l:
        IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressL,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h:
        IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressH,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control:
        IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegionDefControl,
    _reserved440: [u8; 0x03dc],
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control:
        IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0Control,
    _reserved441: [u8; 0x0c],
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l:
        IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressL,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h:
        IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressH,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l:
        IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressL,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h:
        IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressH,
    isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control:
        IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegionDefControl,
}
impl RegisterBlock {
    #[doc = "0x400 - The ISC Region 0 Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0Control {
        &self.isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control
    }
    #[doc = "0x410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressL {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l
    }
    #[doc = "0x414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressH {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h
    }
    #[doc = "0x418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressL {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l
    }
    #[doc = "0x41c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressH {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h
    }
    #[doc = "0x420 - The ISC Default Region Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegionDefControl {
        &self.isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control
    }
    #[doc = "0x800 - The ISC Region 0 Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0Control {
        &self.isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control
    }
    #[doc = "0x810 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressL {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l
    }
    #[doc = "0x814 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressH {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h
    }
    #[doc = "0x818 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressL {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l
    }
    #[doc = "0x81c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressH {
        & self . isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h
    }
    #[doc = "0x820 - The ISC Default Region Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control(
        &self,
    ) -> &IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegionDefControl {
        &self.isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control
    }
    #[doc = "0x1000 - The ISC Region 0 Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0Control {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control
    }
    #[doc = "0x1010 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressL {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l
    }
    #[doc = "0x1014 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressH {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h
    }
    #[doc = "0x1018 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressL {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l
    }
    #[doc = "0x101c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressH {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h
    }
    #[doc = "0x1020 - The ISC Default Region Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegionDefControl {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control
    }
    #[doc = "0x1400 - The ISC Region 0 Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0Control {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control
    }
    #[doc = "0x1410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressL {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l
    }
    #[doc = "0x1414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressH {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h
    }
    #[doc = "0x1418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressL {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l
    }
    #[doc = "0x141c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressH {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h
    }
    #[doc = "0x1420 - The ISC Default Region Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control(
        &self,
    ) -> &IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegionDefControl {
        &self.isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control
    }
    #[doc = "0x2000 - The ISC Region 0 Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0Control {
        &self.isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control
    }
    #[doc = "0x2010 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressL {
        &self.isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l
    }
    #[doc = "0x2014 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressH {
        &self.isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h
    }
    #[doc = "0x2018 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressL {
        &self.isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l
    }
    #[doc = "0x201c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressH {
        &self.isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h
    }
    #[doc = "0x2020 - The ISC Default Region Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemWrVbusmIscRegionDefControl {
        &self.isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control
    }
    #[doc = "0x2400 - The ISC Region 0 Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0Control {
        &self.isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control
    }
    #[doc = "0x2410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressL {
        &self.isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l
    }
    #[doc = "0x2414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressH {
        &self.isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h
    }
    #[doc = "0x2418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressL {
        &self.isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l
    }
    #[doc = "0x241c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressH {
        &self.isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h
    }
    #[doc = "0x2420 - The ISC Default Region Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control(
        &self,
    ) -> &IscRegsIgic500ss1_2Main0MemRdVbusmIscRegionDefControl {
        &self.isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control
    }
    #[doc = "0x2800 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control
    }
    #[doc = "0x2810 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l
    }
    #[doc = "0x2814 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h
    }
    #[doc = "0x2818 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l
    }
    #[doc = "0x281c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h
    }
    #[doc = "0x2820 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_rmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0RmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control
    }
    #[doc = "0x2c00 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control
    }
    #[doc = "0x2c10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l
    }
    #[doc = "0x2c14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h
    }
    #[doc = "0x2c18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l
    }
    #[doc = "0x2c1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h
    }
    #[doc = "0x2c20 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_wmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu0WmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control
    }
    #[doc = "0x3c00 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control
    }
    #[doc = "0x3c10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l
    }
    #[doc = "0x3c14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h
    }
    #[doc = "0x3c18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l
    }
    #[doc = "0x3c1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h
    }
    #[doc = "0x3c20 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_rmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1RmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control
    }
    #[doc = "0x4000 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control
    }
    #[doc = "0x4010 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l
    }
    #[doc = "0x4014 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h
    }
    #[doc = "0x4018 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l
    }
    #[doc = "0x401c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h
    }
    #[doc = "0x4020 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_wmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain0Cpu1WmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control
    }
    #[doc = "0x5000 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control
    }
    #[doc = "0x5010 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l
    }
    #[doc = "0x5014 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h
    }
    #[doc = "0x5018 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l
    }
    #[doc = "0x501c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h
    }
    #[doc = "0x5020 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_rmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0RmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control
    }
    #[doc = "0x5400 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control
    }
    #[doc = "0x5410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l
    }
    #[doc = "0x5414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h
    }
    #[doc = "0x5418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l
    }
    #[doc = "0x541c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h
    }
    #[doc = "0x5420 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_wmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu0WmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control
    }
    #[doc = "0x6400 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control
    }
    #[doc = "0x6410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l
    }
    #[doc = "0x6414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h
    }
    #[doc = "0x6418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l
    }
    #[doc = "0x641c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h
    }
    #[doc = "0x6420 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_rmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1RmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control
    }
    #[doc = "0x6800 - The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0Control {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control
    }
    #[doc = "0x6810 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l
    }
    #[doc = "0x6814 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h
    }
    #[doc = "0x6818 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressL {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l
    }
    #[doc = "0x681c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressH {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h
    }
    #[doc = "0x6820 - The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_wmst region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control(
        &self,
    ) -> &IscRegsIpulsarLiteMain1Cpu1WmstIscRegionDefControl {
        &self.isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control
    }
    #[doc = "0x7800 - The ISC Region 0 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control
    }
    #[doc = "0x7810 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l
    }
    #[doc = "0x7814 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h
    }
    #[doc = "0x7818 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l
    }
    #[doc = "0x781c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h
    }
    #[doc = "0x7820 - The ISC Region 1 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control
    }
    #[doc = "0x7830 - The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l
    }
    #[doc = "0x7834 - The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h
    }
    #[doc = "0x7838 - The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l
    }
    #[doc = "0x783c - The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h
    }
    #[doc = "0x7840 - The ISC Region 2 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control
    }
    #[doc = "0x7850 - The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l
    }
    #[doc = "0x7854 - The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h
    }
    #[doc = "0x7858 - The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l
    }
    #[doc = "0x785c - The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h
    }
    #[doc = "0x7860 - The ISC Region 3 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control
    }
    #[doc = "0x7870 - The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l
    }
    #[doc = "0x7874 - The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h
    }
    #[doc = "0x7878 - The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l
    }
    #[doc = "0x787c - The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h
    }
    #[doc = "0x7880 - The ISC Region 4 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control
    }
    #[doc = "0x7890 - The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l
    }
    #[doc = "0x7894 - The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h
    }
    #[doc = "0x7898 - The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l
    }
    #[doc = "0x789c - The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h
    }
    #[doc = "0x78a0 - The ISC Region 5 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control
    }
    #[doc = "0x78b0 - The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l
    }
    #[doc = "0x78b4 - The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h
    }
    #[doc = "0x78b8 - The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l
    }
    #[doc = "0x78bc - The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h
    }
    #[doc = "0x78c0 - The ISC Region 6 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control
    }
    #[doc = "0x78d0 - The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l
    }
    #[doc = "0x78d4 - The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h
    }
    #[doc = "0x78d8 - The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l
    }
    #[doc = "0x78dc - The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h
    }
    #[doc = "0x78e0 - The ISC Region 7 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control
    }
    #[doc = "0x78f0 - The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l
    }
    #[doc = "0x78f4 - The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h
    }
    #[doc = "0x78f8 - The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l
    }
    #[doc = "0x78fc - The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h
    }
    #[doc = "0x7900 - The ISC Region 8 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control
    }
    #[doc = "0x7910 - The ISC Region 8 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l
    }
    #[doc = "0x7914 - The ISC Region 8 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h
    }
    #[doc = "0x7918 - The ISC Region 8 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l
    }
    #[doc = "0x791c - The ISC Region 8 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h
    }
    #[doc = "0x7920 - The ISC Region 9 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control
    }
    #[doc = "0x7930 - The ISC Region 9 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l
    }
    #[doc = "0x7934 - The ISC Region 9 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h
    }
    #[doc = "0x7938 - The ISC Region 9 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l
    }
    #[doc = "0x793c - The ISC Region 9 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h
    }
    #[doc = "0x7940 - The ISC Region 10 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control
    }
    #[doc = "0x7950 - The ISC Region 10 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l
    }
    #[doc = "0x7954 - The ISC Region 10 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h
    }
    #[doc = "0x7958 - The ISC Region 10 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l
    }
    #[doc = "0x795c - The ISC Region 10 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h
    }
    #[doc = "0x7960 - The ISC Region 11 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control
    }
    #[doc = "0x7970 - The ISC Region 11 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l
    }
    #[doc = "0x7974 - The ISC Region 11 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h
    }
    #[doc = "0x7978 - The ISC Region 11 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l
    }
    #[doc = "0x797c - The ISC Region 11 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h
    }
    #[doc = "0x7980 - The ISC Region 12 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control
    }
    #[doc = "0x7990 - The ISC Region 12 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l
    }
    #[doc = "0x7994 - The ISC Region 12 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h
    }
    #[doc = "0x7998 - The ISC Region 12 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l
    }
    #[doc = "0x799c - The ISC Region 12 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h
    }
    #[doc = "0x79a0 - The ISC Region 13 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control
    }
    #[doc = "0x79b0 - The ISC Region 13 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l
    }
    #[doc = "0x79b4 - The ISC Region 13 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h
    }
    #[doc = "0x79b8 - The ISC Region 13 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l
    }
    #[doc = "0x79bc - The ISC Region 13 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h
    }
    #[doc = "0x79c0 - The ISC Region 14 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control
    }
    #[doc = "0x79d0 - The ISC Region 14 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l
    }
    #[doc = "0x79d4 - The ISC Region 14 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h
    }
    #[doc = "0x79d8 - The ISC Region 14 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l
    }
    #[doc = "0x79dc - The ISC Region 14 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h
    }
    #[doc = "0x79e0 - The ISC Region 15 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15Control {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control
    }
    #[doc = "0x79f0 - The ISC Region 15 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l
    }
    #[doc = "0x79f4 - The ISC Region 15 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h
    }
    #[doc = "0x79f8 - The ISC Region 15 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l
    }
    #[doc = "0x79fc - The ISC Region 15 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h
    }
    #[doc = "0x7a00 - The ISC Default Region Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 16 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control(
        &self,
    ) -> &IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegionDefControl {
        &self.isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control
    }
    #[doc = "0x7c00 - The ISC Region 0 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control
    }
    #[doc = "0x7c10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l
    }
    #[doc = "0x7c14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h
    }
    #[doc = "0x7c18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l
    }
    #[doc = "0x7c1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h
    }
    #[doc = "0x7c20 - The ISC Region 1 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control
    }
    #[doc = "0x7c30 - The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l
    }
    #[doc = "0x7c34 - The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h
    }
    #[doc = "0x7c38 - The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l
    }
    #[doc = "0x7c3c - The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h
    }
    #[doc = "0x7c40 - The ISC Region 2 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control
    }
    #[doc = "0x7c50 - The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l
    }
    #[doc = "0x7c54 - The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h
    }
    #[doc = "0x7c58 - The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l
    }
    #[doc = "0x7c5c - The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h
    }
    #[doc = "0x7c60 - The ISC Region 3 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control
    }
    #[doc = "0x7c70 - The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l
    }
    #[doc = "0x7c74 - The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h
    }
    #[doc = "0x7c78 - The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l
    }
    #[doc = "0x7c7c - The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h
    }
    #[doc = "0x7c80 - The ISC Region 4 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control
    }
    #[doc = "0x7c90 - The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l
    }
    #[doc = "0x7c94 - The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h
    }
    #[doc = "0x7c98 - The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l
    }
    #[doc = "0x7c9c - The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h
    }
    #[doc = "0x7ca0 - The ISC Region 5 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control
    }
    #[doc = "0x7cb0 - The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l
    }
    #[doc = "0x7cb4 - The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h
    }
    #[doc = "0x7cb8 - The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l
    }
    #[doc = "0x7cbc - The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h
    }
    #[doc = "0x7cc0 - The ISC Region 6 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control
    }
    #[doc = "0x7cd0 - The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l
    }
    #[doc = "0x7cd4 - The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h
    }
    #[doc = "0x7cd8 - The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l
    }
    #[doc = "0x7cdc - The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h
    }
    #[doc = "0x7ce0 - The ISC Region 7 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control
    }
    #[doc = "0x7cf0 - The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l
    }
    #[doc = "0x7cf4 - The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h
    }
    #[doc = "0x7cf8 - The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l
    }
    #[doc = "0x7cfc - The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h
    }
    #[doc = "0x7d00 - The ISC Region 8 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control
    }
    #[doc = "0x7d10 - The ISC Region 8 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l
    }
    #[doc = "0x7d14 - The ISC Region 8 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h
    }
    #[doc = "0x7d18 - The ISC Region 8 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l
    }
    #[doc = "0x7d1c - The ISC Region 8 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h
    }
    #[doc = "0x7d20 - The ISC Region 9 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control
    }
    #[doc = "0x7d30 - The ISC Region 9 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l
    }
    #[doc = "0x7d34 - The ISC Region 9 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h
    }
    #[doc = "0x7d38 - The ISC Region 9 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l
    }
    #[doc = "0x7d3c - The ISC Region 9 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h
    }
    #[doc = "0x7d40 - The ISC Region 10 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control
    }
    #[doc = "0x7d50 - The ISC Region 10 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l
    }
    #[doc = "0x7d54 - The ISC Region 10 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h
    }
    #[doc = "0x7d58 - The ISC Region 10 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l
    }
    #[doc = "0x7d5c - The ISC Region 10 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h
    }
    #[doc = "0x7d60 - The ISC Region 11 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control
    }
    #[doc = "0x7d70 - The ISC Region 11 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l
    }
    #[doc = "0x7d74 - The ISC Region 11 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h
    }
    #[doc = "0x7d78 - The ISC Region 11 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l
    }
    #[doc = "0x7d7c - The ISC Region 11 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h
    }
    #[doc = "0x7d80 - The ISC Region 12 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control
    }
    #[doc = "0x7d90 - The ISC Region 12 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l
    }
    #[doc = "0x7d94 - The ISC Region 12 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h
    }
    #[doc = "0x7d98 - The ISC Region 12 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l
    }
    #[doc = "0x7d9c - The ISC Region 12 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h
    }
    #[doc = "0x7da0 - The ISC Region 13 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control
    }
    #[doc = "0x7db0 - The ISC Region 13 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l
    }
    #[doc = "0x7db4 - The ISC Region 13 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h
    }
    #[doc = "0x7db8 - The ISC Region 13 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l
    }
    #[doc = "0x7dbc - The ISC Region 13 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h
    }
    #[doc = "0x7dc0 - The ISC Region 14 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control
    }
    #[doc = "0x7dd0 - The ISC Region 14 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l
    }
    #[doc = "0x7dd4 - The ISC Region 14 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h
    }
    #[doc = "0x7dd8 - The ISC Region 14 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l
    }
    #[doc = "0x7ddc - The ISC Region 14 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h
    }
    #[doc = "0x7de0 - The ISC Region 15 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15Control {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control
    }
    #[doc = "0x7df0 - The ISC Region 15 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l
    }
    #[doc = "0x7df4 - The ISC Region 15 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h
    }
    #[doc = "0x7df8 - The ISC Region 15 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressL {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l
    }
    #[doc = "0x7dfc - The ISC Region 15 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressH {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h
    }
    #[doc = "0x7e00 - The ISC Default Region Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 16 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control(
        &self,
    ) -> &IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegionDefControl {
        &self.isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control
    }
    #[doc = "0x8000 - The ISC Region 0 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control
    }
    #[doc = "0x8010 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l
    }
    #[doc = "0x8014 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h
    }
    #[doc = "0x8018 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l
    }
    #[doc = "0x801c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h
    }
    #[doc = "0x8020 - The ISC Region 1 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control
    }
    #[doc = "0x8030 - The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l
    }
    #[doc = "0x8034 - The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h
    }
    #[doc = "0x8038 - The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l
    }
    #[doc = "0x803c - The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h
    }
    #[doc = "0x8040 - The ISC Region 2 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control
    }
    #[doc = "0x8050 - The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l
    }
    #[doc = "0x8054 - The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h
    }
    #[doc = "0x8058 - The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l
    }
    #[doc = "0x805c - The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h
    }
    #[doc = "0x8060 - The ISC Region 3 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control
    }
    #[doc = "0x8070 - The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l
    }
    #[doc = "0x8074 - The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h
    }
    #[doc = "0x8078 - The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l
    }
    #[doc = "0x807c - The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h
    }
    #[doc = "0x8080 - The ISC Region 4 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control
    }
    #[doc = "0x8090 - The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l
    }
    #[doc = "0x8094 - The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h
    }
    #[doc = "0x8098 - The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l
    }
    #[doc = "0x809c - The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h
    }
    #[doc = "0x80a0 - The ISC Region 5 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control
    }
    #[doc = "0x80b0 - The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l
    }
    #[doc = "0x80b4 - The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h
    }
    #[doc = "0x80b8 - The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l
    }
    #[doc = "0x80bc - The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h
    }
    #[doc = "0x80c0 - The ISC Region 6 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control
    }
    #[doc = "0x80d0 - The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l
    }
    #[doc = "0x80d4 - The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h
    }
    #[doc = "0x80d8 - The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l
    }
    #[doc = "0x80dc - The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h
    }
    #[doc = "0x80e0 - The ISC Region 7 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control
    }
    #[doc = "0x80f0 - The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l
    }
    #[doc = "0x80f4 - The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h
    }
    #[doc = "0x80f8 - The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l
    }
    #[doc = "0x80fc - The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h
    }
    #[doc = "0x8100 - The ISC Default Region Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstRdIscRegionDefControl {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control
    }
    #[doc = "0x8400 - The ISC Region 0 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control
    }
    #[doc = "0x8410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l
    }
    #[doc = "0x8414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h
    }
    #[doc = "0x8418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l
    }
    #[doc = "0x841c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h
    }
    #[doc = "0x8420 - The ISC Region 1 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control
    }
    #[doc = "0x8430 - The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l
    }
    #[doc = "0x8434 - The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h
    }
    #[doc = "0x8438 - The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l
    }
    #[doc = "0x843c - The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h
    }
    #[doc = "0x8440 - The ISC Region 2 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control
    }
    #[doc = "0x8450 - The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l
    }
    #[doc = "0x8454 - The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h
    }
    #[doc = "0x8458 - The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l
    }
    #[doc = "0x845c - The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h
    }
    #[doc = "0x8460 - The ISC Region 3 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control
    }
    #[doc = "0x8470 - The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l
    }
    #[doc = "0x8474 - The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h
    }
    #[doc = "0x8478 - The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l
    }
    #[doc = "0x847c - The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h
    }
    #[doc = "0x8480 - The ISC Region 4 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control
    }
    #[doc = "0x8490 - The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l
    }
    #[doc = "0x8494 - The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h
    }
    #[doc = "0x8498 - The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l
    }
    #[doc = "0x849c - The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h
    }
    #[doc = "0x84a0 - The ISC Region 5 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control
    }
    #[doc = "0x84b0 - The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l
    }
    #[doc = "0x84b4 - The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h
    }
    #[doc = "0x84b8 - The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l
    }
    #[doc = "0x84bc - The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h
    }
    #[doc = "0x84c0 - The ISC Region 6 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control
    }
    #[doc = "0x84d0 - The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l
    }
    #[doc = "0x84d4 - The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h
    }
    #[doc = "0x84d8 - The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l
    }
    #[doc = "0x84dc - The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h
    }
    #[doc = "0x84e0 - The ISC Region 7 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7Control {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control
    }
    #[doc = "0x84f0 - The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l
    }
    #[doc = "0x84f4 - The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h
    }
    #[doc = "0x84f8 - The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressL {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l
    }
    #[doc = "0x84fc - The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressH {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h
    }
    #[doc = "0x8500 - The ISC Default Region Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control(
        &self,
    ) -> &IscRegsIpcieG2x1_64Main0PcieMstWrIscRegionDefControl {
        &self.isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control
    }
    #[doc = "0x8800 - The ISC Region 0 Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0Control {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control
    }
    #[doc = "0x8810 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressL {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l
    }
    #[doc = "0x8814 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressH {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h
    }
    #[doc = "0x8818 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressL {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l
    }
    #[doc = "0x881c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressH {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h
    }
    #[doc = "0x8820 - The ISC Default Region Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_wr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegionDefControl {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control
    }
    #[doc = "0x8c00 - The ISC Region 0 Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0Control {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control
    }
    #[doc = "0x8c10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressL {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l
    }
    #[doc = "0x8c14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressH {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h
    }
    #[doc = "0x8c18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressL {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l
    }
    #[doc = "0x8c1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressH {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h
    }
    #[doc = "0x8c20 - The ISC Default Region Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_rd region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control(
        &self,
    ) -> &IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegionDefControl {
        &self.isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control
    }
    #[doc = "0x9000 - The ISC Region 0 Control Register defines the control fields for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control(
        &self,
    ) -> &IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0Control {
        &self.isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control
    }
    #[doc = "0x9010 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressL {
        &self.isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l
    }
    #[doc = "0x9014 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressH {
        &self.isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h
    }
    #[doc = "0x9018 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressL {
        &self.isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l
    }
    #[doc = "0x901c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressH {
        &self.isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h
    }
    #[doc = "0x9020 - The ISC Default Region Control Register defines the control fields for the master Isa2_ul_main_0.ctxcach_ext_dma region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control(
        &self,
    ) -> &IscRegsIsa2UlMain0CtxcachExtDmaIscRegionDefControl {
        &self.isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control
    }
    #[doc = "0x9400 - The ISC Region 0 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control
    }
    #[doc = "0x9410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l
    }
    #[doc = "0x9414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h
    }
    #[doc = "0x9418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l
    }
    #[doc = "0x941c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h
    }
    #[doc = "0x9420 - The ISC Region 1 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control
    }
    #[doc = "0x9430 - The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l
    }
    #[doc = "0x9434 - The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h
    }
    #[doc = "0x9438 - The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l
    }
    #[doc = "0x943c - The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h
    }
    #[doc = "0x9440 - The ISC Region 2 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control
    }
    #[doc = "0x9450 - The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l
    }
    #[doc = "0x9454 - The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h
    }
    #[doc = "0x9458 - The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l
    }
    #[doc = "0x945c - The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h
    }
    #[doc = "0x9460 - The ISC Region 3 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control
    }
    #[doc = "0x9470 - The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l
    }
    #[doc = "0x9474 - The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h
    }
    #[doc = "0x9478 - The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l
    }
    #[doc = "0x947c - The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h
    }
    #[doc = "0x9480 - The ISC Region 4 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control
    }
    #[doc = "0x9490 - The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l
    }
    #[doc = "0x9494 - The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h
    }
    #[doc = "0x9498 - The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l
    }
    #[doc = "0x949c - The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h
    }
    #[doc = "0x94a0 - The ISC Region 5 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control
    }
    #[doc = "0x94b0 - The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l
    }
    #[doc = "0x94b4 - The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h
    }
    #[doc = "0x94b8 - The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l
    }
    #[doc = "0x94bc - The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h
    }
    #[doc = "0x94c0 - The ISC Region 6 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control
    }
    #[doc = "0x94d0 - The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l
    }
    #[doc = "0x94d4 - The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h
    }
    #[doc = "0x94d8 - The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l
    }
    #[doc = "0x94dc - The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h
    }
    #[doc = "0x94e0 - The ISC Region 7 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control
    }
    #[doc = "0x94f0 - The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l
    }
    #[doc = "0x94f4 - The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h
    }
    #[doc = "0x94f8 - The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l
    }
    #[doc = "0x94fc - The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h
    }
    #[doc = "0x9500 - The ISC Default Region Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegionDefControl {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control
    }
    #[doc = "0x9800 - The ISC Region 0 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control
    }
    #[doc = "0x9810 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l
    }
    #[doc = "0x9814 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h
    }
    #[doc = "0x9818 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l
    }
    #[doc = "0x981c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h
    }
    #[doc = "0x9820 - The ISC Region 1 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control
    }
    #[doc = "0x9830 - The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l
    }
    #[doc = "0x9834 - The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h
    }
    #[doc = "0x9838 - The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l
    }
    #[doc = "0x983c - The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h
    }
    #[doc = "0x9840 - The ISC Region 2 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control
    }
    #[doc = "0x9850 - The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l
    }
    #[doc = "0x9854 - The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h
    }
    #[doc = "0x9858 - The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l
    }
    #[doc = "0x985c - The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h
    }
    #[doc = "0x9860 - The ISC Region 3 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control
    }
    #[doc = "0x9870 - The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l
    }
    #[doc = "0x9874 - The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h
    }
    #[doc = "0x9878 - The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l
    }
    #[doc = "0x987c - The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h
    }
    #[doc = "0x9880 - The ISC Region 4 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control
    }
    #[doc = "0x9890 - The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l
    }
    #[doc = "0x9894 - The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h
    }
    #[doc = "0x9898 - The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l
    }
    #[doc = "0x989c - The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h
    }
    #[doc = "0x98a0 - The ISC Region 5 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control
    }
    #[doc = "0x98b0 - The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l
    }
    #[doc = "0x98b4 - The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h
    }
    #[doc = "0x98b8 - The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l
    }
    #[doc = "0x98bc - The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h
    }
    #[doc = "0x98c0 - The ISC Region 6 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control
    }
    #[doc = "0x98d0 - The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l
    }
    #[doc = "0x98d4 - The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h
    }
    #[doc = "0x98d8 - The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l
    }
    #[doc = "0x98dc - The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h
    }
    #[doc = "0x98e0 - The ISC Region 7 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7Control {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control
    }
    #[doc = "0x98f0 - The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l
    }
    #[doc = "0x98f4 - The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h
    }
    #[doc = "0x98f8 - The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressL {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l
    }
    #[doc = "0x98fc - The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressH {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h
    }
    #[doc = "0x9900 - The ISC Default Region Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 8 ISC."]
    #[inline(always)]
    pub const fn isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control(
        &self,
    ) -> &IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegionDefControl {
        &self.isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control
    }
    #[doc = "0x9c00 - The ISC Region 0 Control Register defines the control fields for the master Ij7_led_main_0.vbusp region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ij7_led_main_0_vbusp_isc_region_0_control(
        &self,
    ) -> &IscRegsIj7LedMain0VbuspIscRegion0Control {
        &self.isc_regs_ij7_led_main_0_vbusp_isc_region_0_control
    }
    #[doc = "0x9c10 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ij7_led_main_0.vbusp region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIj7LedMain0VbuspIscRegion0StartAddressL {
        &self.isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l
    }
    #[doc = "0x9c14 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ij7_led_main_0.vbusp region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIj7LedMain0VbuspIscRegion0StartAddressH {
        &self.isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h
    }
    #[doc = "0x9c18 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ij7_led_main_0.vbusp region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIj7LedMain0VbuspIscRegion0EndAddressL {
        &self.isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l
    }
    #[doc = "0x9c1c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ij7_led_main_0.vbusp region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIj7LedMain0VbuspIscRegion0EndAddressH {
        &self.isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h
    }
    #[doc = "0x9c20 - The ISC Default Region Control Register defines the control fields for the master Ij7_led_main_0.vbusp region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_ij7_led_main_0_vbusp_isc_region_def_control(
        &self,
    ) -> &IscRegsIj7LedMain0VbuspIscRegionDefControl {
        &self.isc_regs_ij7_led_main_0_vbusp_isc_region_def_control
    }
    #[doc = "0xa000 - The ISC Region 0 Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0Control {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control
    }
    #[doc = "0xa010 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressL {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l
    }
    #[doc = "0xa014 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressH {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h
    }
    #[doc = "0xa018 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressL {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l
    }
    #[doc = "0xa01c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressH {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h
    }
    #[doc = "0xa020 - The ISC Default Region Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegionDefControl {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control
    }
    #[doc = "0xa400 - The ISC Region 0 Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0Control {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control
    }
    #[doc = "0xa410 - The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressL {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l
    }
    #[doc = "0xa414 - The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressH {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h
    }
    #[doc = "0xa418 - The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressL {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l
    }
    #[doc = "0xa41c - The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressH {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h
    }
    #[doc = "0xa420 - The ISC Default Region Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 1 ISC."]
    #[inline(always)]
    pub const fn isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control(
        &self,
    ) -> &IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegionDefControl {
        &self.isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control
    }
}
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control")]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0Control = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_control;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressL = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressH = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressL = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressH = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegionDefControl = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiRIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_r region 1 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_r_isc_region_def_control;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control")]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0Control = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_control;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressL = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressH = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressL = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressH = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control`]
module"]
#[doc(
    alias = "ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control"
)]
pub type IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegionDefControl = crate :: Reg < isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control :: IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 1 ISC."]
pub mod isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_def_control;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0Control = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control :: IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_control;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressL = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l :: IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressH = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h :: IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressL = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l :: IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressH = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h :: IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegionDefControl = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control :: IscRegsIemmc8ss16ffcMain0EmmcssWrIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_wr region 1 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_wr_isc_region_def_control;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0Control = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control :: IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_control;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressL = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l :: IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressH = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h :: IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressL = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l :: IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressH = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h :: IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control")]
pub type IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegionDefControl = crate :: Reg < isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control :: IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 1 ISC."]
pub mod isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_def_control;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control")]
pub type IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0Control = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control :: IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_control;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l")]
pub type IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressL = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l :: IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h")]
pub type IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressH = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h :: IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l")]
pub type IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressL = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l :: IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h")]
pub type IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressH = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h :: IscRegsIgic500ss1_2Main0MemWrVbusmIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control")]
pub type IscRegsIgic500ss1_2Main0MemWrVbusmIscRegionDefControl = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control :: IscRegsIgic500ss1_2Main0MemWrVbusmIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_wr_vbusm region 1 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_wr_vbusm_isc_region_def_control;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control")]
pub type IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0Control = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control :: IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_control;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l")]
pub type IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressL = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l :: IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h")]
pub type IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressH = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h :: IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l")]
pub type IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressL = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l :: IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h")]
pub type IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressH = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h :: IscRegsIgic500ss1_2Main0MemRdVbusmIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 0 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control")]
pub type IscRegsIgic500ss1_2Main0MemRdVbusmIscRegionDefControl = crate :: Reg < isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control :: IscRegsIgic500ss1_2Main0MemRdVbusmIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Igic500ss_1_2_main_0.mem_rd_vbusm region 1 ISC."]
pub mod isc_regs_igic500ss_1_2_main_0_mem_rd_vbusm_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control :: IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain0Cpu0RmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_rmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain0Cpu0RmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control :: IscRegsIpulsarLiteMain0Cpu0RmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_rmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_rmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control :: IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain0Cpu0WmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_wmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain0Cpu0WmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control :: IscRegsIpulsarLiteMain0Cpu0WmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu0_wmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu0_wmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control :: IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain0Cpu1RmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_rmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain0Cpu1RmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control :: IscRegsIpulsarLiteMain0Cpu1RmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_rmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_rmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control :: IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain0Cpu1WmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_0.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_wmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain0Cpu1WmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control :: IscRegsIpulsarLiteMain0Cpu1WmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_0.cpu1_wmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_0_cpu1_wmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control :: IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain1Cpu0RmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_rmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain1Cpu0RmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control :: IscRegsIpulsarLiteMain1Cpu0RmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_rmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_rmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control :: IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain1Cpu0WmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu0_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_wmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain1Cpu0WmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control :: IscRegsIpulsarLiteMain1Cpu0WmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu0_wmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu0_wmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control :: IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain1Cpu1RmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_rmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_rmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain1Cpu1RmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control :: IscRegsIpulsarLiteMain1Cpu1RmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_rmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_rmst_isc_region_def_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control")]
pub type IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0Control = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control :: IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_control;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l :: IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h :: IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l")]
pub type IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressL = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l :: IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h")]
pub type IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressH = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h :: IscRegsIpulsarLiteMain1Cpu1WmstIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipulsar_lite_main_1.cpu1_wmst region 0 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_wmst region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control")]
pub type IscRegsIpulsarLiteMain1Cpu1WmstIscRegionDefControl = crate :: Reg < isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control :: IscRegsIpulsarLiteMain1Cpu1WmstIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipulsar_lite_main_1.cpu1_wmst region 1 ISC."]
pub mod isc_regs_ipulsar_lite_main_1_cpu1_wmst_isc_region_def_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control (rw) register accessor: The ISC Region 1 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1ControlSpec > ;
#[doc = "The ISC Region 1 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l (rw) register accessor: The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressLSpec > ;
#[doc = "The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h (rw) register accessor: The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1StartAddressHSpec > ;
#[doc = "The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l (rw) register accessor: The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressLSpec > ;
#[doc = "The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h (rw) register accessor: The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion1EndAddressHSpec > ;
#[doc = "The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_1_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control (rw) register accessor: The ISC Region 2 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2ControlSpec > ;
#[doc = "The ISC Region 2 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l (rw) register accessor: The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressLSpec > ;
#[doc = "The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h (rw) register accessor: The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2StartAddressHSpec > ;
#[doc = "The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l (rw) register accessor: The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressLSpec > ;
#[doc = "The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h (rw) register accessor: The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion2EndAddressHSpec > ;
#[doc = "The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_2_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control (rw) register accessor: The ISC Region 3 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3ControlSpec > ;
#[doc = "The ISC Region 3 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l (rw) register accessor: The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressLSpec > ;
#[doc = "The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h (rw) register accessor: The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3StartAddressHSpec > ;
#[doc = "The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l (rw) register accessor: The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressLSpec > ;
#[doc = "The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h (rw) register accessor: The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion3EndAddressHSpec > ;
#[doc = "The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_3_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control (rw) register accessor: The ISC Region 4 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4ControlSpec > ;
#[doc = "The ISC Region 4 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l (rw) register accessor: The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressLSpec > ;
#[doc = "The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h (rw) register accessor: The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4StartAddressHSpec > ;
#[doc = "The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l (rw) register accessor: The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressLSpec > ;
#[doc = "The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h (rw) register accessor: The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion4EndAddressHSpec > ;
#[doc = "The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_4_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control (rw) register accessor: The ISC Region 5 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5ControlSpec > ;
#[doc = "The ISC Region 5 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l (rw) register accessor: The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressLSpec > ;
#[doc = "The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h (rw) register accessor: The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5StartAddressHSpec > ;
#[doc = "The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l (rw) register accessor: The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressLSpec > ;
#[doc = "The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h (rw) register accessor: The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion5EndAddressHSpec > ;
#[doc = "The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_5_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control (rw) register accessor: The ISC Region 6 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6ControlSpec > ;
#[doc = "The ISC Region 6 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l (rw) register accessor: The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressLSpec > ;
#[doc = "The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h (rw) register accessor: The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6StartAddressHSpec > ;
#[doc = "The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l (rw) register accessor: The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressLSpec > ;
#[doc = "The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h (rw) register accessor: The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion6EndAddressHSpec > ;
#[doc = "The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_6_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control (rw) register accessor: The ISC Region 7 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7ControlSpec > ;
#[doc = "The ISC Region 7 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l (rw) register accessor: The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressLSpec > ;
#[doc = "The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h (rw) register accessor: The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7StartAddressHSpec > ;
#[doc = "The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l (rw) register accessor: The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressLSpec > ;
#[doc = "The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h (rw) register accessor: The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion7EndAddressHSpec > ;
#[doc = "The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_7_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control (rw) register accessor: The ISC Region 8 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8ControlSpec > ;
#[doc = "The ISC Region 8 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l (rw) register accessor: The ISC Region 8 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressLSpec > ;
#[doc = "The ISC Region 8 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h (rw) register accessor: The ISC Region 8 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8StartAddressHSpec > ;
#[doc = "The ISC Region 8 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l (rw) register accessor: The ISC Region 8 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressLSpec > ;
#[doc = "The ISC Region 8 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h (rw) register accessor: The ISC Region 8 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion8EndAddressHSpec > ;
#[doc = "The ISC Region 8 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_8_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control (rw) register accessor: The ISC Region 9 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9ControlSpec > ;
#[doc = "The ISC Region 9 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l (rw) register accessor: The ISC Region 9 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressLSpec > ;
#[doc = "The ISC Region 9 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h (rw) register accessor: The ISC Region 9 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9StartAddressHSpec > ;
#[doc = "The ISC Region 9 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l (rw) register accessor: The ISC Region 9 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressLSpec > ;
#[doc = "The ISC Region 9 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h (rw) register accessor: The ISC Region 9 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion9EndAddressHSpec > ;
#[doc = "The ISC Region 9 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_9_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control (rw) register accessor: The ISC Region 10 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10ControlSpec > ;
#[doc = "The ISC Region 10 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l (rw) register accessor: The ISC Region 10 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressLSpec > ;
#[doc = "The ISC Region 10 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h (rw) register accessor: The ISC Region 10 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10StartAddressHSpec > ;
#[doc = "The ISC Region 10 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l (rw) register accessor: The ISC Region 10 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressLSpec > ;
#[doc = "The ISC Region 10 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h (rw) register accessor: The ISC Region 10 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion10EndAddressHSpec > ;
#[doc = "The ISC Region 10 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_10_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control (rw) register accessor: The ISC Region 11 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11ControlSpec > ;
#[doc = "The ISC Region 11 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l (rw) register accessor: The ISC Region 11 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressLSpec > ;
#[doc = "The ISC Region 11 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h (rw) register accessor: The ISC Region 11 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11StartAddressHSpec > ;
#[doc = "The ISC Region 11 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l (rw) register accessor: The ISC Region 11 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressLSpec > ;
#[doc = "The ISC Region 11 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h (rw) register accessor: The ISC Region 11 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion11EndAddressHSpec > ;
#[doc = "The ISC Region 11 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_11_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control (rw) register accessor: The ISC Region 12 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12ControlSpec > ;
#[doc = "The ISC Region 12 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l (rw) register accessor: The ISC Region 12 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressLSpec > ;
#[doc = "The ISC Region 12 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h (rw) register accessor: The ISC Region 12 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12StartAddressHSpec > ;
#[doc = "The ISC Region 12 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l (rw) register accessor: The ISC Region 12 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressLSpec > ;
#[doc = "The ISC Region 12 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h (rw) register accessor: The ISC Region 12 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion12EndAddressHSpec > ;
#[doc = "The ISC Region 12 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_12_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control (rw) register accessor: The ISC Region 13 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13ControlSpec > ;
#[doc = "The ISC Region 13 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l (rw) register accessor: The ISC Region 13 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressLSpec > ;
#[doc = "The ISC Region 13 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h (rw) register accessor: The ISC Region 13 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13StartAddressHSpec > ;
#[doc = "The ISC Region 13 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l (rw) register accessor: The ISC Region 13 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressLSpec > ;
#[doc = "The ISC Region 13 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h (rw) register accessor: The ISC Region 13 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion13EndAddressHSpec > ;
#[doc = "The ISC Region 13 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_13_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control (rw) register accessor: The ISC Region 14 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14ControlSpec > ;
#[doc = "The ISC Region 14 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l (rw) register accessor: The ISC Region 14 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressLSpec > ;
#[doc = "The ISC Region 14 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h (rw) register accessor: The ISC Region 14 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14StartAddressHSpec > ;
#[doc = "The ISC Region 14 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l (rw) register accessor: The ISC Region 14 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressLSpec > ;
#[doc = "The ISC Region 14 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h (rw) register accessor: The ISC Region 14 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion14EndAddressHSpec > ;
#[doc = "The ISC Region 14 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_14_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control (rw) register accessor: The ISC Region 15 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15Control = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15ControlSpec > ;
#[doc = "The ISC Region 15 Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l (rw) register accessor: The ISC Region 15 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressLSpec > ;
#[doc = "The ISC Region 15 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h (rw) register accessor: The ISC Region 15 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15StartAddressHSpec > ;
#[doc = "The ISC Region 15 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l (rw) register accessor: The ISC Region 15 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressLSpec > ;
#[doc = "The ISC Region 15 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h (rw) register accessor: The ISC Region 15 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegion15EndAddressHSpec > ;
#[doc = "The ISC Region 15 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_15_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 16 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control")]
pub type IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegionDefControl = crate :: Reg < isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control :: IscRegsIicssG16ffMain0Pr1ExtVbusmIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm region 16 ISC."]
pub mod isc_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_isc_region_def_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 0 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control (rw) register accessor: The ISC Region 1 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1ControlSpec > ;
#[doc = "The ISC Region 1 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l (rw) register accessor: The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressLSpec > ;
#[doc = "The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h (rw) register accessor: The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1StartAddressHSpec > ;
#[doc = "The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l (rw) register accessor: The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressLSpec > ;
#[doc = "The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h (rw) register accessor: The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion1EndAddressHSpec > ;
#[doc = "The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 1 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_1_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control (rw) register accessor: The ISC Region 2 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2ControlSpec > ;
#[doc = "The ISC Region 2 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l (rw) register accessor: The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressLSpec > ;
#[doc = "The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h (rw) register accessor: The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2StartAddressHSpec > ;
#[doc = "The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l (rw) register accessor: The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressLSpec > ;
#[doc = "The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h (rw) register accessor: The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion2EndAddressHSpec > ;
#[doc = "The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 2 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_2_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control (rw) register accessor: The ISC Region 3 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3ControlSpec > ;
#[doc = "The ISC Region 3 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l (rw) register accessor: The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressLSpec > ;
#[doc = "The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h (rw) register accessor: The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3StartAddressHSpec > ;
#[doc = "The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l (rw) register accessor: The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressLSpec > ;
#[doc = "The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h (rw) register accessor: The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion3EndAddressHSpec > ;
#[doc = "The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 3 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_3_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control (rw) register accessor: The ISC Region 4 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4ControlSpec > ;
#[doc = "The ISC Region 4 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l (rw) register accessor: The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressLSpec > ;
#[doc = "The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h (rw) register accessor: The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4StartAddressHSpec > ;
#[doc = "The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l (rw) register accessor: The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressLSpec > ;
#[doc = "The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h (rw) register accessor: The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion4EndAddressHSpec > ;
#[doc = "The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 4 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_4_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control (rw) register accessor: The ISC Region 5 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5ControlSpec > ;
#[doc = "The ISC Region 5 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l (rw) register accessor: The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressLSpec > ;
#[doc = "The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h (rw) register accessor: The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5StartAddressHSpec > ;
#[doc = "The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l (rw) register accessor: The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressLSpec > ;
#[doc = "The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h (rw) register accessor: The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion5EndAddressHSpec > ;
#[doc = "The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 5 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_5_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control (rw) register accessor: The ISC Region 6 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6ControlSpec > ;
#[doc = "The ISC Region 6 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l (rw) register accessor: The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressLSpec > ;
#[doc = "The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h (rw) register accessor: The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6StartAddressHSpec > ;
#[doc = "The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l (rw) register accessor: The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressLSpec > ;
#[doc = "The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h (rw) register accessor: The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion6EndAddressHSpec > ;
#[doc = "The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 6 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_6_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control (rw) register accessor: The ISC Region 7 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7ControlSpec > ;
#[doc = "The ISC Region 7 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l (rw) register accessor: The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressLSpec > ;
#[doc = "The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h (rw) register accessor: The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7StartAddressHSpec > ;
#[doc = "The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l (rw) register accessor: The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressLSpec > ;
#[doc = "The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h (rw) register accessor: The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion7EndAddressHSpec > ;
#[doc = "The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 7 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_7_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control (rw) register accessor: The ISC Region 8 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8ControlSpec > ;
#[doc = "The ISC Region 8 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l (rw) register accessor: The ISC Region 8 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressLSpec > ;
#[doc = "The ISC Region 8 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h (rw) register accessor: The ISC Region 8 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8StartAddressHSpec > ;
#[doc = "The ISC Region 8 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l (rw) register accessor: The ISC Region 8 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressLSpec > ;
#[doc = "The ISC Region 8 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h (rw) register accessor: The ISC Region 8 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion8EndAddressHSpec > ;
#[doc = "The ISC Region 8 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 8 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_8_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control (rw) register accessor: The ISC Region 9 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9ControlSpec > ;
#[doc = "The ISC Region 9 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l (rw) register accessor: The ISC Region 9 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressLSpec > ;
#[doc = "The ISC Region 9 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h (rw) register accessor: The ISC Region 9 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9StartAddressHSpec > ;
#[doc = "The ISC Region 9 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l (rw) register accessor: The ISC Region 9 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressLSpec > ;
#[doc = "The ISC Region 9 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h (rw) register accessor: The ISC Region 9 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion9EndAddressHSpec > ;
#[doc = "The ISC Region 9 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 9 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_9_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control (rw) register accessor: The ISC Region 10 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10ControlSpec > ;
#[doc = "The ISC Region 10 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l (rw) register accessor: The ISC Region 10 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressLSpec > ;
#[doc = "The ISC Region 10 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h (rw) register accessor: The ISC Region 10 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10StartAddressHSpec > ;
#[doc = "The ISC Region 10 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l (rw) register accessor: The ISC Region 10 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressLSpec > ;
#[doc = "The ISC Region 10 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h (rw) register accessor: The ISC Region 10 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion10EndAddressHSpec > ;
#[doc = "The ISC Region 10 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 10 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_10_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control (rw) register accessor: The ISC Region 11 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11ControlSpec > ;
#[doc = "The ISC Region 11 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l (rw) register accessor: The ISC Region 11 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressLSpec > ;
#[doc = "The ISC Region 11 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h (rw) register accessor: The ISC Region 11 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11StartAddressHSpec > ;
#[doc = "The ISC Region 11 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l (rw) register accessor: The ISC Region 11 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressLSpec > ;
#[doc = "The ISC Region 11 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h (rw) register accessor: The ISC Region 11 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion11EndAddressHSpec > ;
#[doc = "The ISC Region 11 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 11 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_11_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control (rw) register accessor: The ISC Region 12 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12ControlSpec > ;
#[doc = "The ISC Region 12 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l (rw) register accessor: The ISC Region 12 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressLSpec > ;
#[doc = "The ISC Region 12 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h (rw) register accessor: The ISC Region 12 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12StartAddressHSpec > ;
#[doc = "The ISC Region 12 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l (rw) register accessor: The ISC Region 12 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressLSpec > ;
#[doc = "The ISC Region 12 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h (rw) register accessor: The ISC Region 12 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion12EndAddressHSpec > ;
#[doc = "The ISC Region 12 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 12 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_12_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control (rw) register accessor: The ISC Region 13 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13ControlSpec > ;
#[doc = "The ISC Region 13 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l (rw) register accessor: The ISC Region 13 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressLSpec > ;
#[doc = "The ISC Region 13 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h (rw) register accessor: The ISC Region 13 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13StartAddressHSpec > ;
#[doc = "The ISC Region 13 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l (rw) register accessor: The ISC Region 13 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressLSpec > ;
#[doc = "The ISC Region 13 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h (rw) register accessor: The ISC Region 13 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion13EndAddressHSpec > ;
#[doc = "The ISC Region 13 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 13 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_13_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control (rw) register accessor: The ISC Region 14 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14ControlSpec > ;
#[doc = "The ISC Region 14 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l (rw) register accessor: The ISC Region 14 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressLSpec > ;
#[doc = "The ISC Region 14 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h (rw) register accessor: The ISC Region 14 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14StartAddressHSpec > ;
#[doc = "The ISC Region 14 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l (rw) register accessor: The ISC Region 14 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressLSpec > ;
#[doc = "The ISC Region 14 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h (rw) register accessor: The ISC Region 14 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion14EndAddressHSpec > ;
#[doc = "The ISC Region 14 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 14 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_14_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control (rw) register accessor: The ISC Region 15 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15Control = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15ControlSpec > ;
#[doc = "The ISC Region 15 Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_control;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l (rw) register accessor: The ISC Region 15 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressLSpec > ;
#[doc = "The ISC Region 15 Start Address Low Register defines the start address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h (rw) register accessor: The ISC Region 15 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15StartAddressHSpec > ;
#[doc = "The ISC Region 15 Start Address High Register defines the start address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_start_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l (rw) register accessor: The ISC Region 15 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressL = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressLSpec > ;
#[doc = "The ISC Region 15 End Address Low Register defines the end included address bits 31 to 0 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_l;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h (rw) register accessor: The ISC Region 15 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressH = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegion15EndAddressHSpec > ;
#[doc = "The ISC Region 15 End Address High Register defines the end address bits 47 to 32 for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 15 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_15_end_address_h;
#[doc = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 16 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control")]
pub type IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegionDefControl = crate :: Reg < isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control :: IscRegsIicssG16ffMain1Pr1ExtVbusmIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iicss_g_16ff_main_1.pr1_ext_vbusm region 16 ISC."]
pub mod isc_regs_iicss_g_16ff_main_1_pr1_ext_vbusm_isc_region_def_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control (rw) register accessor: The ISC Region 1 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1ControlSpec > ;
#[doc = "The ISC Region 1 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l (rw) register accessor: The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressLSpec > ;
#[doc = "The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h (rw) register accessor: The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1StartAddressHSpec > ;
#[doc = "The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l (rw) register accessor: The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressLSpec > ;
#[doc = "The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h (rw) register accessor: The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion1EndAddressHSpec > ;
#[doc = "The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_1_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control (rw) register accessor: The ISC Region 2 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2ControlSpec > ;
#[doc = "The ISC Region 2 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l (rw) register accessor: The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressLSpec > ;
#[doc = "The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h (rw) register accessor: The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2StartAddressHSpec > ;
#[doc = "The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l (rw) register accessor: The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressLSpec > ;
#[doc = "The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h (rw) register accessor: The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion2EndAddressHSpec > ;
#[doc = "The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_2_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control (rw) register accessor: The ISC Region 3 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3ControlSpec > ;
#[doc = "The ISC Region 3 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l (rw) register accessor: The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressLSpec > ;
#[doc = "The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h (rw) register accessor: The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3StartAddressHSpec > ;
#[doc = "The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l (rw) register accessor: The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressLSpec > ;
#[doc = "The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h (rw) register accessor: The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion3EndAddressHSpec > ;
#[doc = "The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_3_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control (rw) register accessor: The ISC Region 4 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4ControlSpec > ;
#[doc = "The ISC Region 4 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l (rw) register accessor: The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressLSpec > ;
#[doc = "The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h (rw) register accessor: The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4StartAddressHSpec > ;
#[doc = "The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l (rw) register accessor: The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressLSpec > ;
#[doc = "The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h (rw) register accessor: The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion4EndAddressHSpec > ;
#[doc = "The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_4_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control (rw) register accessor: The ISC Region 5 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5ControlSpec > ;
#[doc = "The ISC Region 5 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l (rw) register accessor: The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressLSpec > ;
#[doc = "The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h (rw) register accessor: The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5StartAddressHSpec > ;
#[doc = "The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l (rw) register accessor: The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressLSpec > ;
#[doc = "The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h (rw) register accessor: The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion5EndAddressHSpec > ;
#[doc = "The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_5_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control (rw) register accessor: The ISC Region 6 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6ControlSpec > ;
#[doc = "The ISC Region 6 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l (rw) register accessor: The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressLSpec > ;
#[doc = "The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h (rw) register accessor: The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6StartAddressHSpec > ;
#[doc = "The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l (rw) register accessor: The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressLSpec > ;
#[doc = "The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h (rw) register accessor: The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion6EndAddressHSpec > ;
#[doc = "The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_6_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control (rw) register accessor: The ISC Region 7 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7ControlSpec > ;
#[doc = "The ISC Region 7 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l (rw) register accessor: The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressLSpec > ;
#[doc = "The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h (rw) register accessor: The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7StartAddressHSpec > ;
#[doc = "The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l (rw) register accessor: The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressLSpec > ;
#[doc = "The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h (rw) register accessor: The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegion7EndAddressHSpec > ;
#[doc = "The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_7_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstRdIscRegionDefControl = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control :: IscRegsIpcieG2x1_64Main0PcieMstRdIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_rd region 8 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_rd_isc_region_def_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 0 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control (rw) register accessor: The ISC Region 1 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1ControlSpec > ;
#[doc = "The ISC Region 1 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l (rw) register accessor: The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressLSpec > ;
#[doc = "The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h (rw) register accessor: The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1StartAddressHSpec > ;
#[doc = "The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l (rw) register accessor: The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressLSpec > ;
#[doc = "The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h (rw) register accessor: The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion1EndAddressHSpec > ;
#[doc = "The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 1 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_1_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control (rw) register accessor: The ISC Region 2 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2ControlSpec > ;
#[doc = "The ISC Region 2 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l (rw) register accessor: The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec > ;
#[doc = "The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h (rw) register accessor: The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressHSpec > ;
#[doc = "The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l (rw) register accessor: The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressLSpec > ;
#[doc = "The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h (rw) register accessor: The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2EndAddressHSpec > ;
#[doc = "The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control (rw) register accessor: The ISC Region 3 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3ControlSpec > ;
#[doc = "The ISC Region 3 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l (rw) register accessor: The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressLSpec > ;
#[doc = "The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h (rw) register accessor: The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3StartAddressHSpec > ;
#[doc = "The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l (rw) register accessor: The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressLSpec > ;
#[doc = "The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h (rw) register accessor: The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion3EndAddressHSpec > ;
#[doc = "The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 3 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_3_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control (rw) register accessor: The ISC Region 4 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4ControlSpec > ;
#[doc = "The ISC Region 4 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l (rw) register accessor: The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressLSpec > ;
#[doc = "The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h (rw) register accessor: The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4StartAddressHSpec > ;
#[doc = "The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l (rw) register accessor: The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressLSpec > ;
#[doc = "The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h (rw) register accessor: The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion4EndAddressHSpec > ;
#[doc = "The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 4 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_4_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control (rw) register accessor: The ISC Region 5 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5ControlSpec > ;
#[doc = "The ISC Region 5 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l (rw) register accessor: The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressLSpec > ;
#[doc = "The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h (rw) register accessor: The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5StartAddressHSpec > ;
#[doc = "The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l (rw) register accessor: The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressLSpec > ;
#[doc = "The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h (rw) register accessor: The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion5EndAddressHSpec > ;
#[doc = "The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 5 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_5_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control (rw) register accessor: The ISC Region 6 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6ControlSpec > ;
#[doc = "The ISC Region 6 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l (rw) register accessor: The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressLSpec > ;
#[doc = "The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h (rw) register accessor: The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6StartAddressHSpec > ;
#[doc = "The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l (rw) register accessor: The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressLSpec > ;
#[doc = "The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h (rw) register accessor: The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion6EndAddressHSpec > ;
#[doc = "The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 6 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_6_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control (rw) register accessor: The ISC Region 7 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7Control = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7ControlSpec > ;
#[doc = "The ISC Region 7 Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_control;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l (rw) register accessor: The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressLSpec > ;
#[doc = "The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h (rw) register accessor: The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7StartAddressHSpec > ;
#[doc = "The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_start_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l (rw) register accessor: The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressL = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressLSpec > ;
#[doc = "The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_l;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h (rw) register accessor: The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressH = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion7EndAddressHSpec > ;
#[doc = "The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 7 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_7_end_address_h;
#[doc = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control")]
pub type IscRegsIpcieG2x1_64Main0PcieMstWrIscRegionDefControl = crate :: Reg < isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control :: IscRegsIpcieG2x1_64Main0PcieMstWrIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 8 ISC."]
pub mod isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_def_control;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0Control = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control :: IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_control;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressL = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l :: IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressH = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h :: IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressL = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l :: IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressH = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h :: IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_wr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegionDefControl = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control :: IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_wr region 1 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_def_control;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0Control = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control :: IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_control;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressL = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l :: IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressH = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h :: IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressL = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l :: IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressH = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h :: IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_rd region 0 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_rd region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control")]
pub type IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegionDefControl = crate :: Reg < isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control :: IscRegsIemmcsd4ssMain0EmmcsdssRdIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iemmcsd4ss_main_0.emmcsdss_rd region 1 ISC."]
pub mod isc_regs_iemmcsd4ss_main_0_emmcsdss_rd_isc_region_def_control;
#[doc = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control")]
pub type IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0Control = crate :: Reg < isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control :: IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
pub mod isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_control;
#[doc = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l")]
pub type IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressL = crate :: Reg < isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l :: IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
pub mod isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h")]
pub type IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressH = crate :: Reg < isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h :: IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
pub mod isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l")]
pub type IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressL = crate :: Reg < isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l :: IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
pub mod isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h")]
pub type IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressH = crate :: Reg < isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h :: IscRegsIsa2UlMain0CtxcachExtDmaIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Isa2_ul_main_0.ctxcach_ext_dma region 0 ISC."]
pub mod isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Isa2_ul_main_0.ctxcach_ext_dma region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control")]
pub type IscRegsIsa2UlMain0CtxcachExtDmaIscRegionDefControl = crate :: Reg < isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control :: IscRegsIsa2UlMain0CtxcachExtDmaIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Isa2_ul_main_0.ctxcach_ext_dma region 1 ISC."]
pub mod isc_regs_isa2_ul_main_0_ctxcach_ext_dma_isc_region_def_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control (rw) register accessor: The ISC Region 1 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1ControlSpec > ;
#[doc = "The ISC Region 1 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l (rw) register accessor: The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressLSpec > ;
#[doc = "The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h (rw) register accessor: The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1StartAddressHSpec > ;
#[doc = "The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l (rw) register accessor: The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressLSpec > ;
#[doc = "The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h (rw) register accessor: The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion1EndAddressHSpec > ;
#[doc = "The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_1_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control (rw) register accessor: The ISC Region 2 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2ControlSpec > ;
#[doc = "The ISC Region 2 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l (rw) register accessor: The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressLSpec > ;
#[doc = "The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h (rw) register accessor: The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2StartAddressHSpec > ;
#[doc = "The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l (rw) register accessor: The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressLSpec > ;
#[doc = "The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h (rw) register accessor: The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion2EndAddressHSpec > ;
#[doc = "The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_2_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control (rw) register accessor: The ISC Region 3 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3ControlSpec > ;
#[doc = "The ISC Region 3 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l (rw) register accessor: The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressLSpec > ;
#[doc = "The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h (rw) register accessor: The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3StartAddressHSpec > ;
#[doc = "The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l (rw) register accessor: The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressLSpec > ;
#[doc = "The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h (rw) register accessor: The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion3EndAddressHSpec > ;
#[doc = "The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_3_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control (rw) register accessor: The ISC Region 4 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4ControlSpec > ;
#[doc = "The ISC Region 4 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l (rw) register accessor: The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressLSpec > ;
#[doc = "The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h (rw) register accessor: The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4StartAddressHSpec > ;
#[doc = "The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l (rw) register accessor: The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressLSpec > ;
#[doc = "The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h (rw) register accessor: The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec > ;
#[doc = "The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control (rw) register accessor: The ISC Region 5 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5ControlSpec > ;
#[doc = "The ISC Region 5 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l (rw) register accessor: The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressLSpec > ;
#[doc = "The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h (rw) register accessor: The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5StartAddressHSpec > ;
#[doc = "The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l (rw) register accessor: The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressLSpec > ;
#[doc = "The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h (rw) register accessor: The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion5EndAddressHSpec > ;
#[doc = "The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_5_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control (rw) register accessor: The ISC Region 6 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6ControlSpec > ;
#[doc = "The ISC Region 6 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l (rw) register accessor: The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressLSpec > ;
#[doc = "The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h (rw) register accessor: The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6StartAddressHSpec > ;
#[doc = "The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l (rw) register accessor: The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressLSpec > ;
#[doc = "The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h (rw) register accessor: The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion6EndAddressHSpec > ;
#[doc = "The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_6_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control (rw) register accessor: The ISC Region 7 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7ControlSpec > ;
#[doc = "The ISC Region 7 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l (rw) register accessor: The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressLSpec > ;
#[doc = "The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h (rw) register accessor: The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7StartAddressHSpec > ;
#[doc = "The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l (rw) register accessor: The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressLSpec > ;
#[doc = "The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h (rw) register accessor: The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion7EndAddressHSpec > ;
#[doc = "The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_7_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegionDefControl = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control :: IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 8 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_def_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 0 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control (rw) register accessor: The ISC Region 1 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1ControlSpec > ;
#[doc = "The ISC Region 1 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l (rw) register accessor: The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressLSpec > ;
#[doc = "The ISC Region 1 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h (rw) register accessor: The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1StartAddressHSpec > ;
#[doc = "The ISC Region 1 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l (rw) register accessor: The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressLSpec > ;
#[doc = "The ISC Region 1 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h (rw) register accessor: The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion1EndAddressHSpec > ;
#[doc = "The ISC Region 1 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 1 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_1_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control (rw) register accessor: The ISC Region 2 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2ControlSpec > ;
#[doc = "The ISC Region 2 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l (rw) register accessor: The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressLSpec > ;
#[doc = "The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h (rw) register accessor: The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2StartAddressHSpec > ;
#[doc = "The ISC Region 2 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l (rw) register accessor: The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressLSpec > ;
#[doc = "The ISC Region 2 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h (rw) register accessor: The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion2EndAddressHSpec > ;
#[doc = "The ISC Region 2 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 2 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_2_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control (rw) register accessor: The ISC Region 3 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3ControlSpec > ;
#[doc = "The ISC Region 3 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l (rw) register accessor: The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressLSpec > ;
#[doc = "The ISC Region 3 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h (rw) register accessor: The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3StartAddressHSpec > ;
#[doc = "The ISC Region 3 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l (rw) register accessor: The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec > ;
#[doc = "The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h (rw) register accessor: The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressHSpec > ;
#[doc = "The ISC Region 3 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control (rw) register accessor: The ISC Region 4 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4ControlSpec > ;
#[doc = "The ISC Region 4 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l (rw) register accessor: The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressLSpec > ;
#[doc = "The ISC Region 4 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h (rw) register accessor: The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4StartAddressHSpec > ;
#[doc = "The ISC Region 4 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l (rw) register accessor: The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressLSpec > ;
#[doc = "The ISC Region 4 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h (rw) register accessor: The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion4EndAddressHSpec > ;
#[doc = "The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 4 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_4_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control (rw) register accessor: The ISC Region 5 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5ControlSpec > ;
#[doc = "The ISC Region 5 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l (rw) register accessor: The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressLSpec > ;
#[doc = "The ISC Region 5 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h (rw) register accessor: The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5StartAddressHSpec > ;
#[doc = "The ISC Region 5 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l (rw) register accessor: The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressLSpec > ;
#[doc = "The ISC Region 5 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h (rw) register accessor: The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion5EndAddressHSpec > ;
#[doc = "The ISC Region 5 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 5 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_5_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control (rw) register accessor: The ISC Region 6 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6ControlSpec > ;
#[doc = "The ISC Region 6 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l (rw) register accessor: The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressLSpec > ;
#[doc = "The ISC Region 6 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h (rw) register accessor: The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6StartAddressHSpec > ;
#[doc = "The ISC Region 6 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l (rw) register accessor: The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressLSpec > ;
#[doc = "The ISC Region 6 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h (rw) register accessor: The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion6EndAddressHSpec > ;
#[doc = "The ISC Region 6 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 6 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_6_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control (rw) register accessor: The ISC Region 7 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7Control = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7ControlSpec > ;
#[doc = "The ISC Region 7 Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_control;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l (rw) register accessor: The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressLSpec > ;
#[doc = "The ISC Region 7 Start Address Low Register defines the start address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h (rw) register accessor: The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7StartAddressHSpec > ;
#[doc = "The ISC Region 7 Start Address High Register defines the start address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_start_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l (rw) register accessor: The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressL = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressLSpec > ;
#[doc = "The ISC Region 7 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_l;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h (rw) register accessor: The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressH = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion7EndAddressHSpec > ;
#[doc = "The ISC Region 7 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 7 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_7_end_address_h;
#[doc = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 8 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control")]
pub type IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegionDefControl = crate :: Reg < isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control :: IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 8 ISC."]
pub mod isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_def_control;
#[doc = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Ij7_led_main_0.vbusp region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ij7_led_main_0_vbusp_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_control")]
pub type IscRegsIj7LedMain0VbuspIscRegion0Control = crate :: Reg < isc_regs_ij7_led_main_0_vbusp_isc_region_0_control :: IscRegsIj7LedMain0VbuspIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Ij7_led_main_0.vbusp region 0 ISC."]
pub mod isc_regs_ij7_led_main_0_vbusp_isc_region_0_control;
#[doc = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ij7_led_main_0.vbusp region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_start_address_l")]
pub type IscRegsIj7LedMain0VbuspIscRegion0StartAddressL = crate :: Reg < isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l :: IscRegsIj7LedMain0VbuspIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Ij7_led_main_0.vbusp region 0 ISC."]
pub mod isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ij7_led_main_0.vbusp region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_start_address_h")]
pub type IscRegsIj7LedMain0VbuspIscRegion0StartAddressH = crate :: Reg < isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h :: IscRegsIj7LedMain0VbuspIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Ij7_led_main_0.vbusp region 0 ISC."]
pub mod isc_regs_ij7_led_main_0_vbusp_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ij7_led_main_0.vbusp region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_end_address_l")]
pub type IscRegsIj7LedMain0VbuspIscRegion0EndAddressL = crate :: Reg < isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l :: IscRegsIj7LedMain0VbuspIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Ij7_led_main_0.vbusp region 0 ISC."]
pub mod isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ij7_led_main_0.vbusp region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_0_end_address_h")]
pub type IscRegsIj7LedMain0VbuspIscRegion0EndAddressH = crate :: Reg < isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h :: IscRegsIj7LedMain0VbuspIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Ij7_led_main_0.vbusp region 0 ISC."]
pub mod isc_regs_ij7_led_main_0_vbusp_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Ij7_led_main_0.vbusp region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ij7_led_main_0_vbusp_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ij7_led_main_0_vbusp_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_ij7_led_main_0_vbusp_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Ij7_led_main_0_vbusp_isc_region_def_control")]
pub type IscRegsIj7LedMain0VbuspIscRegionDefControl = crate :: Reg < isc_regs_ij7_led_main_0_vbusp_isc_region_def_control :: IscRegsIj7LedMain0VbuspIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Ij7_led_main_0.vbusp region 1 ISC."]
pub mod isc_regs_ij7_led_main_0_vbusp_isc_region_def_control;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0Control = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control :: IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_control;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressL = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l :: IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressH = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h :: IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressL = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l :: IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressH = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h :: IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegionDefControl = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control :: IscRegsIdebugssK3WrapCv0Main0VbusmrIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmr region 1 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmr_isc_region_def_control;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control (rw) register accessor: The ISC Region 0 Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0Control = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control :: IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0ControlSpec > ;
#[doc = "The ISC Region 0 Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_control;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l (rw) register accessor: The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressL = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l :: IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressLSpec > ;
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_l;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h (rw) register accessor: The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressH = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h :: IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0StartAddressHSpec > ;
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_start_address_h;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l (rw) register accessor: The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressL = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l :: IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressLSpec > ;
#[doc = "The ISC Region 0 End Address Low Register defines the end included address bits 31 to 0 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_l;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h (rw) register accessor: The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressH = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h :: IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegion0EndAddressHSpec > ;
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 0 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_0_end_address_h;
#[doc = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control (rw) register accessor: The ISC Default Region Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 1 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control`]
module"]
#[doc(alias = "ISC_REGS_Idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control")]
pub type IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegionDefControl = crate :: Reg < isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control :: IscRegsIdebugssK3WrapCv0Main0VbusmwIscRegionDefControlSpec > ;
#[doc = "The ISC Default Region Control Register defines the control fields for the master Idebugss_k3_wrap_cv0_main_0.vbusmw region 1 ISC."]
pub mod isc_regs_idebugss_k3_wrap_cv0_main_0_vbusmw_isc_region_def_control;
