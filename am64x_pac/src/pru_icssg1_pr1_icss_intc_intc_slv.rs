#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_icss_intc__intc_slv__regs_revision_reg: Pr1IcssIntc_IntcSlv_RegsRevisionReg,
    pr1_icss_intc__intc_slv__regs_control_reg: Pr1IcssIntc_IntcSlv_RegsControlReg,
    _reserved2: [u8; 0x08],
    pr1_icss_intc__intc_slv__regs_global_enable_hint_reg:
        Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintReg,
    _reserved3: [u8; 0x08],
    pr1_icss_intc__intc_slv__regs_glb_nest_level_reg: Pr1IcssIntc_IntcSlv_RegsGlbNestLevelReg,
    pr1_icss_intc__intc_slv__regs_status_set_index_reg: Pr1IcssIntc_IntcSlv_RegsStatusSetIndexReg,
    pr1_icss_intc__intc_slv__regs_status_clr_index_reg: Pr1IcssIntc_IntcSlv_RegsStatusClrIndexReg,
    pr1_icss_intc__intc_slv__regs_enable_set_index_reg: Pr1IcssIntc_IntcSlv_RegsEnableSetIndexReg,
    pr1_icss_intc__intc_slv__regs_enable_clr_index_reg: Pr1IcssIntc_IntcSlv_RegsEnableClrIndexReg,
    _reserved8: [u8; 0x04],
    pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg:
        Pr1IcssIntc_IntcSlv_RegsHintEnableSetIndexReg,
    pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg:
        Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexReg,
    _reserved10: [u8; 0x44],
    pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg: Pr1IcssIntc_IntcSlv_RegsGlbPriIntrReg,
    _reserved11: [u8; 0x017c],
    pr1_icss_intc__intc_slv__regs_raw_status_reg0: Pr1IcssIntc_IntcSlv_RegsRawStatusReg0,
    pr1_icss_intc__intc_slv__regs_raw_status_reg1: Pr1IcssIntc_IntcSlv_RegsRawStatusReg1,
    pr1_icss_intc__intc_slv__regs_raw_status_reg2: Pr1IcssIntc_IntcSlv_RegsRawStatusReg2,
    pr1_icss_intc__intc_slv__regs_raw_status_reg3: Pr1IcssIntc_IntcSlv_RegsRawStatusReg3,
    pr1_icss_intc__intc_slv__regs_raw_status_reg4: Pr1IcssIntc_IntcSlv_RegsRawStatusReg4,
    _reserved16: [u8; 0x6c],
    pr1_icss_intc__intc_slv__regs_ena_status_reg0: Pr1IcssIntc_IntcSlv_RegsEnaStatusReg0,
    pr1_icss_intc__intc_slv__regs_ena_status_reg1: Pr1IcssIntc_IntcSlv_RegsEnaStatusReg1,
    pr1_icss_intc__intc_slv__regs_ena_status_reg2: Pr1IcssIntc_IntcSlv_RegsEnaStatusReg2,
    pr1_icss_intc__intc_slv__regs_ena_status_reg3: Pr1IcssIntc_IntcSlv_RegsEnaStatusReg3,
    pr1_icss_intc__intc_slv__regs_ena_status_reg4: Pr1IcssIntc_IntcSlv_RegsEnaStatusReg4,
    _reserved21: [u8; 0x6c],
    pr1_icss_intc__intc_slv__regs_enable_reg0: Pr1IcssIntc_IntcSlv_RegsEnableReg0,
    pr1_icss_intc__intc_slv__regs_enable_reg1: Pr1IcssIntc_IntcSlv_RegsEnableReg1,
    pr1_icss_intc__intc_slv__regs_enable_reg2: Pr1IcssIntc_IntcSlv_RegsEnableReg2,
    pr1_icss_intc__intc_slv__regs_enable_reg3: Pr1IcssIntc_IntcSlv_RegsEnableReg3,
    pr1_icss_intc__intc_slv__regs_enable_reg4: Pr1IcssIntc_IntcSlv_RegsEnableReg4,
    _reserved26: [u8; 0x6c],
    pr1_icss_intc__intc_slv__regs_enable_clr_reg0: Pr1IcssIntc_IntcSlv_RegsEnableClrReg0,
    pr1_icss_intc__intc_slv__regs_enable_clr_reg1: Pr1IcssIntc_IntcSlv_RegsEnableClrReg1,
    pr1_icss_intc__intc_slv__regs_enable_clr_reg2: Pr1IcssIntc_IntcSlv_RegsEnableClrReg2,
    pr1_icss_intc__intc_slv__regs_enable_clr_reg3: Pr1IcssIntc_IntcSlv_RegsEnableClrReg3,
    pr1_icss_intc__intc_slv__regs_enable_clr_reg4: Pr1IcssIntc_IntcSlv_RegsEnableClrReg4,
    _reserved31: [u8; 0x6c],
    pr1_icss_intc__intc_slv__regs_ch_map_reg0: Pr1IcssIntc_IntcSlv_RegsChMapReg0,
    pr1_icss_intc__intc_slv__regs_ch_map_reg1: Pr1IcssIntc_IntcSlv_RegsChMapReg1,
    pr1_icss_intc__intc_slv__regs_ch_map_reg2: Pr1IcssIntc_IntcSlv_RegsChMapReg2,
    pr1_icss_intc__intc_slv__regs_ch_map_reg3: Pr1IcssIntc_IntcSlv_RegsChMapReg3,
    pr1_icss_intc__intc_slv__regs_ch_map_reg4: Pr1IcssIntc_IntcSlv_RegsChMapReg4,
    pr1_icss_intc__intc_slv__regs_ch_map_reg5: Pr1IcssIntc_IntcSlv_RegsChMapReg5,
    pr1_icss_intc__intc_slv__regs_ch_map_reg6: Pr1IcssIntc_IntcSlv_RegsChMapReg6,
    pr1_icss_intc__intc_slv__regs_ch_map_reg7: Pr1IcssIntc_IntcSlv_RegsChMapReg7,
    pr1_icss_intc__intc_slv__regs_ch_map_reg8: Pr1IcssIntc_IntcSlv_RegsChMapReg8,
    pr1_icss_intc__intc_slv__regs_ch_map_reg9: Pr1IcssIntc_IntcSlv_RegsChMapReg9,
    pr1_icss_intc__intc_slv__regs_ch_map_reg10: Pr1IcssIntc_IntcSlv_RegsChMapReg10,
    pr1_icss_intc__intc_slv__regs_ch_map_reg11: Pr1IcssIntc_IntcSlv_RegsChMapReg11,
    pr1_icss_intc__intc_slv__regs_ch_map_reg12: Pr1IcssIntc_IntcSlv_RegsChMapReg12,
    pr1_icss_intc__intc_slv__regs_ch_map_reg13: Pr1IcssIntc_IntcSlv_RegsChMapReg13,
    pr1_icss_intc__intc_slv__regs_ch_map_reg14: Pr1IcssIntc_IntcSlv_RegsChMapReg14,
    pr1_icss_intc__intc_slv__regs_ch_map_reg15: Pr1IcssIntc_IntcSlv_RegsChMapReg15,
    pr1_icss_intc__intc_slv__regs_ch_map_reg16: Pr1IcssIntc_IntcSlv_RegsChMapReg16,
    pr1_icss_intc__intc_slv__regs_ch_map_reg17: Pr1IcssIntc_IntcSlv_RegsChMapReg17,
    pr1_icss_intc__intc_slv__regs_ch_map_reg18: Pr1IcssIntc_IntcSlv_RegsChMapReg18,
    pr1_icss_intc__intc_slv__regs_ch_map_reg19: Pr1IcssIntc_IntcSlv_RegsChMapReg19,
    pr1_icss_intc__intc_slv__regs_ch_map_reg20: Pr1IcssIntc_IntcSlv_RegsChMapReg20,
    pr1_icss_intc__intc_slv__regs_ch_map_reg21: Pr1IcssIntc_IntcSlv_RegsChMapReg21,
    pr1_icss_intc__intc_slv__regs_ch_map_reg22: Pr1IcssIntc_IntcSlv_RegsChMapReg22,
    pr1_icss_intc__intc_slv__regs_ch_map_reg23: Pr1IcssIntc_IntcSlv_RegsChMapReg23,
    pr1_icss_intc__intc_slv__regs_ch_map_reg24: Pr1IcssIntc_IntcSlv_RegsChMapReg24,
    pr1_icss_intc__intc_slv__regs_ch_map_reg25: Pr1IcssIntc_IntcSlv_RegsChMapReg25,
    pr1_icss_intc__intc_slv__regs_ch_map_reg26: Pr1IcssIntc_IntcSlv_RegsChMapReg26,
    pr1_icss_intc__intc_slv__regs_ch_map_reg27: Pr1IcssIntc_IntcSlv_RegsChMapReg27,
    pr1_icss_intc__intc_slv__regs_ch_map_reg28: Pr1IcssIntc_IntcSlv_RegsChMapReg28,
    pr1_icss_intc__intc_slv__regs_ch_map_reg29: Pr1IcssIntc_IntcSlv_RegsChMapReg29,
    pr1_icss_intc__intc_slv__regs_ch_map_reg30: Pr1IcssIntc_IntcSlv_RegsChMapReg30,
    pr1_icss_intc__intc_slv__regs_ch_map_reg31: Pr1IcssIntc_IntcSlv_RegsChMapReg31,
    pr1_icss_intc__intc_slv__regs_ch_map_reg32: Pr1IcssIntc_IntcSlv_RegsChMapReg32,
    pr1_icss_intc__intc_slv__regs_ch_map_reg33: Pr1IcssIntc_IntcSlv_RegsChMapReg33,
    pr1_icss_intc__intc_slv__regs_ch_map_reg34: Pr1IcssIntc_IntcSlv_RegsChMapReg34,
    pr1_icss_intc__intc_slv__regs_ch_map_reg35: Pr1IcssIntc_IntcSlv_RegsChMapReg35,
    pr1_icss_intc__intc_slv__regs_ch_map_reg36: Pr1IcssIntc_IntcSlv_RegsChMapReg36,
    pr1_icss_intc__intc_slv__regs_ch_map_reg37: Pr1IcssIntc_IntcSlv_RegsChMapReg37,
    pr1_icss_intc__intc_slv__regs_ch_map_reg38: Pr1IcssIntc_IntcSlv_RegsChMapReg38,
    pr1_icss_intc__intc_slv__regs_ch_map_reg39: Pr1IcssIntc_IntcSlv_RegsChMapReg39,
    _reserved71: [u8; 0x0360],
    pr1_icss_intc__intc_slv__regs_hint_map_reg0: Pr1IcssIntc_IntcSlv_RegsHintMapReg0,
    pr1_icss_intc__intc_slv__regs_hint_map_reg1: Pr1IcssIntc_IntcSlv_RegsHintMapReg1,
    pr1_icss_intc__intc_slv__regs_hint_map_reg2: Pr1IcssIntc_IntcSlv_RegsHintMapReg2,
    pr1_icss_intc__intc_slv__regs_hint_map_reg3: Pr1IcssIntc_IntcSlv_RegsHintMapReg3,
    pr1_icss_intc__intc_slv__regs_hint_map_reg4: Pr1IcssIntc_IntcSlv_RegsHintMapReg4,
    _reserved76: [u8; 0xec],
    pr1_icss_intc__intc_slv__regs_pri_hint_reg0: Pr1IcssIntc_IntcSlv_RegsPriHintReg0,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg1: Pr1IcssIntc_IntcSlv_RegsPriHintReg1,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg2: Pr1IcssIntc_IntcSlv_RegsPriHintReg2,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg3: Pr1IcssIntc_IntcSlv_RegsPriHintReg3,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg4: Pr1IcssIntc_IntcSlv_RegsPriHintReg4,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg5: Pr1IcssIntc_IntcSlv_RegsPriHintReg5,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg6: Pr1IcssIntc_IntcSlv_RegsPriHintReg6,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg7: Pr1IcssIntc_IntcSlv_RegsPriHintReg7,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg8: Pr1IcssIntc_IntcSlv_RegsPriHintReg8,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg9: Pr1IcssIntc_IntcSlv_RegsPriHintReg9,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg10: Pr1IcssIntc_IntcSlv_RegsPriHintReg10,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg11: Pr1IcssIntc_IntcSlv_RegsPriHintReg11,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg12: Pr1IcssIntc_IntcSlv_RegsPriHintReg12,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg13: Pr1IcssIntc_IntcSlv_RegsPriHintReg13,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg14: Pr1IcssIntc_IntcSlv_RegsPriHintReg14,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg15: Pr1IcssIntc_IntcSlv_RegsPriHintReg15,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg16: Pr1IcssIntc_IntcSlv_RegsPriHintReg16,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg17: Pr1IcssIntc_IntcSlv_RegsPriHintReg17,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg18: Pr1IcssIntc_IntcSlv_RegsPriHintReg18,
    pr1_icss_intc__intc_slv__regs_pri_hint_reg19: Pr1IcssIntc_IntcSlv_RegsPriHintReg19,
    _reserved96: [u8; 0x03b0],
    pr1_icss_intc__intc_slv__regs_polarity_reg0: Pr1IcssIntc_IntcSlv_RegsPolarityReg0,
    pr1_icss_intc__intc_slv__regs_polarity_reg1: Pr1IcssIntc_IntcSlv_RegsPolarityReg1,
    pr1_icss_intc__intc_slv__regs_polarity_reg2: Pr1IcssIntc_IntcSlv_RegsPolarityReg2,
    pr1_icss_intc__intc_slv__regs_polarity_reg3: Pr1IcssIntc_IntcSlv_RegsPolarityReg3,
    pr1_icss_intc__intc_slv__regs_polarity_reg4: Pr1IcssIntc_IntcSlv_RegsPolarityReg4,
    _reserved101: [u8; 0x6c],
    pr1_icss_intc__intc_slv__regs_type_reg0: Pr1IcssIntc_IntcSlv_RegsTypeReg0,
    pr1_icss_intc__intc_slv__regs_type_reg1: Pr1IcssIntc_IntcSlv_RegsTypeReg1,
    pr1_icss_intc__intc_slv__regs_type_reg2: Pr1IcssIntc_IntcSlv_RegsTypeReg2,
    pr1_icss_intc__intc_slv__regs_type_reg3: Pr1IcssIntc_IntcSlv_RegsTypeReg3,
    pr1_icss_intc__intc_slv__regs_type_reg4: Pr1IcssIntc_IntcSlv_RegsTypeReg4,
    _reserved106: [u8; 0x036c],
    pr1_icss_intc__intc_slv__regs_nest_level_reg0: Pr1IcssIntc_IntcSlv_RegsNestLevelReg0,
    pr1_icss_intc__intc_slv__regs_nest_level_reg1: Pr1IcssIntc_IntcSlv_RegsNestLevelReg1,
    pr1_icss_intc__intc_slv__regs_nest_level_reg2: Pr1IcssIntc_IntcSlv_RegsNestLevelReg2,
    pr1_icss_intc__intc_slv__regs_nest_level_reg3: Pr1IcssIntc_IntcSlv_RegsNestLevelReg3,
    pr1_icss_intc__intc_slv__regs_nest_level_reg4: Pr1IcssIntc_IntcSlv_RegsNestLevelReg4,
    pr1_icss_intc__intc_slv__regs_nest_level_reg5: Pr1IcssIntc_IntcSlv_RegsNestLevelReg5,
    pr1_icss_intc__intc_slv__regs_nest_level_reg6: Pr1IcssIntc_IntcSlv_RegsNestLevelReg6,
    pr1_icss_intc__intc_slv__regs_nest_level_reg7: Pr1IcssIntc_IntcSlv_RegsNestLevelReg7,
    pr1_icss_intc__intc_slv__regs_nest_level_reg8: Pr1IcssIntc_IntcSlv_RegsNestLevelReg8,
    pr1_icss_intc__intc_slv__regs_nest_level_reg9: Pr1IcssIntc_IntcSlv_RegsNestLevelReg9,
    pr1_icss_intc__intc_slv__regs_nest_level_reg10: Pr1IcssIntc_IntcSlv_RegsNestLevelReg10,
    pr1_icss_intc__intc_slv__regs_nest_level_reg11: Pr1IcssIntc_IntcSlv_RegsNestLevelReg11,
    pr1_icss_intc__intc_slv__regs_nest_level_reg12: Pr1IcssIntc_IntcSlv_RegsNestLevelReg12,
    pr1_icss_intc__intc_slv__regs_nest_level_reg13: Pr1IcssIntc_IntcSlv_RegsNestLevelReg13,
    pr1_icss_intc__intc_slv__regs_nest_level_reg14: Pr1IcssIntc_IntcSlv_RegsNestLevelReg14,
    pr1_icss_intc__intc_slv__regs_nest_level_reg15: Pr1IcssIntc_IntcSlv_RegsNestLevelReg15,
    pr1_icss_intc__intc_slv__regs_nest_level_reg16: Pr1IcssIntc_IntcSlv_RegsNestLevelReg16,
    pr1_icss_intc__intc_slv__regs_nest_level_reg17: Pr1IcssIntc_IntcSlv_RegsNestLevelReg17,
    pr1_icss_intc__intc_slv__regs_nest_level_reg18: Pr1IcssIntc_IntcSlv_RegsNestLevelReg18,
    pr1_icss_intc__intc_slv__regs_nest_level_reg19: Pr1IcssIntc_IntcSlv_RegsNestLevelReg19,
    _reserved126: [u8; 0x03b0],
    pr1_icss_intc__intc_slv__regs_enable_hint_reg0: Pr1IcssIntc_IntcSlv_RegsEnableHintReg0,
}
impl RegisterBlock {
    #[doc = "0x00 - PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_revision_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsRevisionReg {
        &self.pr1_icss_intc__intc_slv__regs_revision_reg
    }
    #[doc = "0x04 - PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_control_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsControlReg {
        &self.pr1_icss_intc__intc_slv__regs_control_reg
    }
    #[doc = "0x10 - PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_global_enable_hint_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintReg {
        &self.pr1_icss_intc__intc_slv__regs_global_enable_hint_reg
    }
    #[doc = "0x1c - PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_glb_nest_level_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsGlbNestLevelReg {
        &self.pr1_icss_intc__intc_slv__regs_glb_nest_level_reg
    }
    #[doc = "0x20 - PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_SET_INDEX_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_status_set_index_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsStatusSetIndexReg {
        &self.pr1_icss_intc__intc_slv__regs_status_set_index_reg
    }
    #[doc = "0x24 - PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_CLR_INDEX_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_status_clr_index_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsStatusClrIndexReg {
        &self.pr1_icss_intc__intc_slv__regs_status_clr_index_reg
    }
    #[doc = "0x28 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_SET_INDEX_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_set_index_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableSetIndexReg {
        &self.pr1_icss_intc__intc_slv__regs_enable_set_index_reg
    }
    #[doc = "0x2c - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_INDEX_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_clr_index_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableClrIndexReg {
        &self.pr1_icss_intc__intc_slv__regs_enable_clr_index_reg
    }
    #[doc = "0x34 - PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_SET_INDEX_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsHintEnableSetIndexReg {
        &self.pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg
    }
    #[doc = "0x38 - PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexReg {
        &self.pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg
    }
    #[doc = "0x80 - PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsGlbPriIntrReg {
        &self.pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg
    }
    #[doc = "0x200 - PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_raw_status_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsRawStatusReg0 {
        &self.pr1_icss_intc__intc_slv__regs_raw_status_reg0
    }
    #[doc = "0x204 - PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_raw_status_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsRawStatusReg1 {
        &self.pr1_icss_intc__intc_slv__regs_raw_status_reg1
    }
    #[doc = "0x208 - PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_raw_status_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsRawStatusReg2 {
        &self.pr1_icss_intc__intc_slv__regs_raw_status_reg2
    }
    #[doc = "0x20c - PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_raw_status_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsRawStatusReg3 {
        &self.pr1_icss_intc__intc_slv__regs_raw_status_reg3
    }
    #[doc = "0x210 - PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_raw_status_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsRawStatusReg4 {
        &self.pr1_icss_intc__intc_slv__regs_raw_status_reg4
    }
    #[doc = "0x280 - PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ena_status_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnaStatusReg0 {
        &self.pr1_icss_intc__intc_slv__regs_ena_status_reg0
    }
    #[doc = "0x284 - PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ena_status_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnaStatusReg1 {
        &self.pr1_icss_intc__intc_slv__regs_ena_status_reg1
    }
    #[doc = "0x288 - PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ena_status_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnaStatusReg2 {
        &self.pr1_icss_intc__intc_slv__regs_ena_status_reg2
    }
    #[doc = "0x28c - PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ena_status_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnaStatusReg3 {
        &self.pr1_icss_intc__intc_slv__regs_ena_status_reg3
    }
    #[doc = "0x290 - PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ena_status_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnaStatusReg4 {
        &self.pr1_icss_intc__intc_slv__regs_ena_status_reg4
    }
    #[doc = "0x300 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableReg0 {
        &self.pr1_icss_intc__intc_slv__regs_enable_reg0
    }
    #[doc = "0x304 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableReg1 {
        &self.pr1_icss_intc__intc_slv__regs_enable_reg1
    }
    #[doc = "0x308 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableReg2 {
        &self.pr1_icss_intc__intc_slv__regs_enable_reg2
    }
    #[doc = "0x30c - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableReg3 {
        &self.pr1_icss_intc__intc_slv__regs_enable_reg3
    }
    #[doc = "0x310 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableReg4 {
        &self.pr1_icss_intc__intc_slv__regs_enable_reg4
    }
    #[doc = "0x380 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_clr_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableClrReg0 {
        &self.pr1_icss_intc__intc_slv__regs_enable_clr_reg0
    }
    #[doc = "0x384 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_clr_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableClrReg1 {
        &self.pr1_icss_intc__intc_slv__regs_enable_clr_reg1
    }
    #[doc = "0x388 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_clr_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableClrReg2 {
        &self.pr1_icss_intc__intc_slv__regs_enable_clr_reg2
    }
    #[doc = "0x38c - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_clr_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableClrReg3 {
        &self.pr1_icss_intc__intc_slv__regs_enable_clr_reg3
    }
    #[doc = "0x390 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_clr_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableClrReg4 {
        &self.pr1_icss_intc__intc_slv__regs_enable_clr_reg4
    }
    #[doc = "0x400 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg0 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg0
    }
    #[doc = "0x404 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg1 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg1
    }
    #[doc = "0x408 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg2 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg2
    }
    #[doc = "0x40c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg3 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg3
    }
    #[doc = "0x410 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg4 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg4
    }
    #[doc = "0x414 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg5(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg5 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg5
    }
    #[doc = "0x418 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg6(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg6 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg6
    }
    #[doc = "0x41c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg7(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg7 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg7
    }
    #[doc = "0x420 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg8(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg8 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg8
    }
    #[doc = "0x424 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg9(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg9 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg9
    }
    #[doc = "0x428 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg10(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg10 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg10
    }
    #[doc = "0x42c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG11"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg11(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg11 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg11
    }
    #[doc = "0x430 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg12(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg12 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg12
    }
    #[doc = "0x434 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg13(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg13 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg13
    }
    #[doc = "0x438 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg14(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg14 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg14
    }
    #[doc = "0x43c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg15(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg15 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg15
    }
    #[doc = "0x440 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg16(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg16 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg16
    }
    #[doc = "0x444 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg17(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg17 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg17
    }
    #[doc = "0x448 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg18(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg18 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg18
    }
    #[doc = "0x44c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg19(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg19 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg19
    }
    #[doc = "0x450 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg20(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg20 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg20
    }
    #[doc = "0x454 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg21(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg21 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg21
    }
    #[doc = "0x458 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg22(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg22 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg22
    }
    #[doc = "0x45c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg23(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg23 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg23
    }
    #[doc = "0x460 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg24(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg24 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg24
    }
    #[doc = "0x464 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg25(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg25 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg25
    }
    #[doc = "0x468 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg26(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg26 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg26
    }
    #[doc = "0x46c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg27(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg27 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg27
    }
    #[doc = "0x470 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg28(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg28 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg28
    }
    #[doc = "0x474 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg29(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg29 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg29
    }
    #[doc = "0x478 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg30(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg30 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg30
    }
    #[doc = "0x47c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg31(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg31 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg31
    }
    #[doc = "0x480 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg32(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg32 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg32
    }
    #[doc = "0x484 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg33(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg33 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg33
    }
    #[doc = "0x488 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg34(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg34 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg34
    }
    #[doc = "0x48c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg35(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg35 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg35
    }
    #[doc = "0x490 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg36(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg36 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg36
    }
    #[doc = "0x494 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG37"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg37(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg37 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg37
    }
    #[doc = "0x498 - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg38(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg38 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg38
    }
    #[doc = "0x49c - PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_ch_map_reg39(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsChMapReg39 {
        &self.pr1_icss_intc__intc_slv__regs_ch_map_reg39
    }
    #[doc = "0x800 - PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_hint_map_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsHintMapReg0 {
        &self.pr1_icss_intc__intc_slv__regs_hint_map_reg0
    }
    #[doc = "0x804 - PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_hint_map_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsHintMapReg1 {
        &self.pr1_icss_intc__intc_slv__regs_hint_map_reg1
    }
    #[doc = "0x808 - PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_hint_map_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsHintMapReg2 {
        &self.pr1_icss_intc__intc_slv__regs_hint_map_reg2
    }
    #[doc = "0x80c - PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_hint_map_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsHintMapReg3 {
        &self.pr1_icss_intc__intc_slv__regs_hint_map_reg3
    }
    #[doc = "0x810 - PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_hint_map_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsHintMapReg4 {
        &self.pr1_icss_intc__intc_slv__regs_hint_map_reg4
    }
    #[doc = "0x900 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg0 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg0
    }
    #[doc = "0x904 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg1 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg1
    }
    #[doc = "0x908 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg2 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg2
    }
    #[doc = "0x90c - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg3 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg3
    }
    #[doc = "0x910 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg4 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg4
    }
    #[doc = "0x914 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG5"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg5(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg5 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg5
    }
    #[doc = "0x918 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG6"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg6(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg6 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg6
    }
    #[doc = "0x91c - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG7"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg7(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg7 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg7
    }
    #[doc = "0x920 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG8"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg8(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg8 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg8
    }
    #[doc = "0x924 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG9"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg9(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg9 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg9
    }
    #[doc = "0x928 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG10"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg10(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg10 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg10
    }
    #[doc = "0x92c - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG11"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg11(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg11 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg11
    }
    #[doc = "0x930 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG12"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg12(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg12 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg12
    }
    #[doc = "0x934 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG13"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg13(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg13 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg13
    }
    #[doc = "0x938 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG14"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg14(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg14 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg14
    }
    #[doc = "0x93c - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG15"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg15(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg15 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg15
    }
    #[doc = "0x940 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG16"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg16(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg16 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg16
    }
    #[doc = "0x944 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG17"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg17(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg17 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg17
    }
    #[doc = "0x948 - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG18"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg18(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg18 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg18
    }
    #[doc = "0x94c - PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_pri_hint_reg19(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPriHintReg19 {
        &self.pr1_icss_intc__intc_slv__regs_pri_hint_reg19
    }
    #[doc = "0xd00 - PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_polarity_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPolarityReg0 {
        &self.pr1_icss_intc__intc_slv__regs_polarity_reg0
    }
    #[doc = "0xd04 - PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_polarity_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPolarityReg1 {
        &self.pr1_icss_intc__intc_slv__regs_polarity_reg1
    }
    #[doc = "0xd08 - PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_polarity_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPolarityReg2 {
        &self.pr1_icss_intc__intc_slv__regs_polarity_reg2
    }
    #[doc = "0xd0c - PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_polarity_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPolarityReg3 {
        &self.pr1_icss_intc__intc_slv__regs_polarity_reg3
    }
    #[doc = "0xd10 - PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_polarity_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsPolarityReg4 {
        &self.pr1_icss_intc__intc_slv__regs_polarity_reg4
    }
    #[doc = "0xd80 - PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_type_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsTypeReg0 {
        &self.pr1_icss_intc__intc_slv__regs_type_reg0
    }
    #[doc = "0xd84 - PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_type_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsTypeReg1 {
        &self.pr1_icss_intc__intc_slv__regs_type_reg1
    }
    #[doc = "0xd88 - PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_type_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsTypeReg2 {
        &self.pr1_icss_intc__intc_slv__regs_type_reg2
    }
    #[doc = "0xd8c - PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_type_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsTypeReg3 {
        &self.pr1_icss_intc__intc_slv__regs_type_reg3
    }
    #[doc = "0xd90 - PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_type_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsTypeReg4 {
        &self.pr1_icss_intc__intc_slv__regs_type_reg4
    }
    #[doc = "0x1100 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg0 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg0
    }
    #[doc = "0x1104 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG1"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg1(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg1 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg1
    }
    #[doc = "0x1108 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG2"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg2(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg2 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg2
    }
    #[doc = "0x110c - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg3(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg3 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg3
    }
    #[doc = "0x1110 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG4"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg4(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg4 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg4
    }
    #[doc = "0x1114 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG5"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg5(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg5 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg5
    }
    #[doc = "0x1118 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG6"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg6(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg6 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg6
    }
    #[doc = "0x111c - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG7"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg7(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg7 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg7
    }
    #[doc = "0x1120 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG8"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg8(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg8 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg8
    }
    #[doc = "0x1124 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG9"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg9(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg9 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg9
    }
    #[doc = "0x1128 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG10"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg10(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg10 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg10
    }
    #[doc = "0x112c - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG11"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg11(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg11 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg11
    }
    #[doc = "0x1130 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG12"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg12(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg12 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg12
    }
    #[doc = "0x1134 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg13(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg13 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg13
    }
    #[doc = "0x1138 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG14"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg14(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg14 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg14
    }
    #[doc = "0x113c - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG15"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg15(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg15 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg15
    }
    #[doc = "0x1140 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG16"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg16(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg16 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg16
    }
    #[doc = "0x1144 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG17"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg17(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg17 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg17
    }
    #[doc = "0x1148 - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG18"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg18(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg18 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg18
    }
    #[doc = "0x114c - PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG19"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_nest_level_reg19(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsNestLevelReg19 {
        &self.pr1_icss_intc__intc_slv__regs_nest_level_reg19
    }
    #[doc = "0x1500 - PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_HINT_REG0"]
    #[inline(always)]
    pub const fn pr1_icss_intc__intc_slv__regs_enable_hint_reg0(
        &self,
    ) -> &Pr1IcssIntc_IntcSlv_RegsEnableHintReg0 {
        &self.pr1_icss_intc__intc_slv__regs_enable_hint_reg0
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_revision_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_revision_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_revision_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsRevisionReg =
    crate::Reg<pr1_icss_intc__intc_slv__regs_revision_reg::Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG"]
pub mod pr1_icss_intc__intc_slv__regs_revision_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_control_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_control_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_control_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsControlReg =
    crate::Reg<pr1_icss_intc__intc_slv__regs_control_reg::Pr1IcssIntc_IntcSlv_RegsControlRegSpec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG"]
pub mod pr1_icss_intc__intc_slv__regs_control_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_global_enable_hint_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_global_enable_hint_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_global_enable_hint_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintReg = crate :: Reg < pr1_icss_intc__intc_slv__regs_global_enable_hint_reg :: Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec > ;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG"]
pub mod pr1_icss_intc__intc_slv__regs_global_enable_hint_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_glb_nest_level_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_glb_nest_level_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_glb_nest_level_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsGlbNestLevelReg = crate::Reg<
    pr1_icss_intc__intc_slv__regs_glb_nest_level_reg::Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG"]
pub mod pr1_icss_intc__intc_slv__regs_glb_nest_level_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_SET_INDEX_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_SET_INDEX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_status_set_index_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_status_set_index_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_status_set_index_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_SET_INDEX_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsStatusSetIndexReg = crate :: Reg < pr1_icss_intc__intc_slv__regs_status_set_index_reg :: Pr1IcssIntc_IntcSlv_RegsStatusSetIndexRegSpec > ;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_SET_INDEX_REG"]
pub mod pr1_icss_intc__intc_slv__regs_status_set_index_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_CLR_INDEX_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_CLR_INDEX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_status_clr_index_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_status_clr_index_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_status_clr_index_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_CLR_INDEX_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsStatusClrIndexReg = crate :: Reg < pr1_icss_intc__intc_slv__regs_status_clr_index_reg :: Pr1IcssIntc_IntcSlv_RegsStatusClrIndexRegSpec > ;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_STATUS_CLR_INDEX_REG"]
pub mod pr1_icss_intc__intc_slv__regs_status_clr_index_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_SET_INDEX_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_SET_INDEX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_set_index_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_set_index_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_set_index_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_SET_INDEX_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableSetIndexReg = crate :: Reg < pr1_icss_intc__intc_slv__regs_enable_set_index_reg :: Pr1IcssIntc_IntcSlv_RegsEnableSetIndexRegSpec > ;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_SET_INDEX_REG"]
pub mod pr1_icss_intc__intc_slv__regs_enable_set_index_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_INDEX_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_INDEX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_clr_index_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_clr_index_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_clr_index_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_INDEX_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableClrIndexReg = crate :: Reg < pr1_icss_intc__intc_slv__regs_enable_clr_index_reg :: Pr1IcssIntc_IntcSlv_RegsEnableClrIndexRegSpec > ;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_INDEX_REG"]
pub mod pr1_icss_intc__intc_slv__regs_enable_clr_index_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_SET_INDEX_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_SET_INDEX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_SET_INDEX_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsHintEnableSetIndexReg = crate :: Reg < pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg :: Pr1IcssIntc_IntcSlv_RegsHintEnableSetIndexRegSpec > ;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_SET_INDEX_REG"]
pub mod pr1_icss_intc__intc_slv__regs_hint_enable_set_index_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexReg = crate :: Reg < pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg :: Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec > ;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG"]
pub mod pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG")]
pub type Pr1IcssIntc_IntcSlv_RegsGlbPriIntrReg = crate::Reg<
    pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg::Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG"]
pub mod pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_raw_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_raw_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_raw_status_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsRawStatusReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_raw_status_reg0::Pr1IcssIntc_IntcSlv_RegsRawStatusReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_raw_status_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_raw_status_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_raw_status_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_raw_status_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsRawStatusReg1 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_raw_status_reg1::Pr1IcssIntc_IntcSlv_RegsRawStatusReg1Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_raw_status_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_raw_status_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_raw_status_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_raw_status_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsRawStatusReg2 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_raw_status_reg2::Pr1IcssIntc_IntcSlv_RegsRawStatusReg2Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_raw_status_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_raw_status_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_raw_status_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_raw_status_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsRawStatusReg3 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_raw_status_reg3::Pr1IcssIntc_IntcSlv_RegsRawStatusReg3Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_raw_status_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_raw_status_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_raw_status_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_raw_status_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsRawStatusReg4 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_raw_status_reg4::Pr1IcssIntc_IntcSlv_RegsRawStatusReg4Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_RAW_STATUS_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_raw_status_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ena_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ena_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ena_status_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsEnaStatusReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_ena_status_reg0::Pr1IcssIntc_IntcSlv_RegsEnaStatusReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_ena_status_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ena_status_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ena_status_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ena_status_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsEnaStatusReg1 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_ena_status_reg1::Pr1IcssIntc_IntcSlv_RegsEnaStatusReg1Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_ena_status_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ena_status_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ena_status_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ena_status_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsEnaStatusReg2 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_ena_status_reg2::Pr1IcssIntc_IntcSlv_RegsEnaStatusReg2Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_ena_status_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ena_status_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ena_status_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ena_status_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsEnaStatusReg3 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_ena_status_reg3::Pr1IcssIntc_IntcSlv_RegsEnaStatusReg3Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_ena_status_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ena_status_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ena_status_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ena_status_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsEnaStatusReg4 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_ena_status_reg4::Pr1IcssIntc_IntcSlv_RegsEnaStatusReg4Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENA_STATUS_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_ena_status_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableReg0 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_enable_reg0::Pr1IcssIntc_IntcSlv_RegsEnableReg0Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_enable_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableReg1 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_enable_reg1::Pr1IcssIntc_IntcSlv_RegsEnableReg1Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_enable_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableReg2 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_enable_reg2::Pr1IcssIntc_IntcSlv_RegsEnableReg2Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_enable_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableReg3 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_enable_reg3::Pr1IcssIntc_IntcSlv_RegsEnableReg3Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_enable_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableReg4 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_enable_reg4::Pr1IcssIntc_IntcSlv_RegsEnableReg4Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_enable_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_clr_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableClrReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_enable_clr_reg0::Pr1IcssIntc_IntcSlv_RegsEnableClrReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_enable_clr_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_clr_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_clr_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_clr_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableClrReg1 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_enable_clr_reg1::Pr1IcssIntc_IntcSlv_RegsEnableClrReg1Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_enable_clr_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_clr_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_clr_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_clr_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableClrReg2 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_enable_clr_reg2::Pr1IcssIntc_IntcSlv_RegsEnableClrReg2Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_enable_clr_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_clr_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_clr_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_clr_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableClrReg3 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_enable_clr_reg3::Pr1IcssIntc_IntcSlv_RegsEnableClrReg3Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_enable_clr_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_clr_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_clr_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_clr_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableClrReg4 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_enable_clr_reg4::Pr1IcssIntc_IntcSlv_RegsEnableClrReg4Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_CLR_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_enable_clr_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg0 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg0::Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg1 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg1::Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg2 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg2::Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg3 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg3::Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg4 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg4::Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg5`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg5 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg5::Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg5;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg6`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg6 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg6::Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg6;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg7`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg7 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg7::Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg7;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg8`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg8 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg8::Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg8;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg9`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg9 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg9::Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg9;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg10`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg10 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg10::Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg10;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG11 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg11`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG11")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg11 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg11::Pr1IcssIntc_IntcSlv_RegsChMapReg11Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG11"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg11;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg12`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg12 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg12::Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg12;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg13`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg13 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg13::Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg13;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg14`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg14 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg14::Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg14;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg15`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg15 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg15::Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg15;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg16`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg16 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg16::Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg16;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg17`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg17 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg17::Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg17;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg18`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg18 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg18::Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg18;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg19`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg19 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg19::Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg19;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg20`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg20 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg20::Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg20;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg21`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg21 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg21::Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg21;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg22`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg22 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg22::Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg22;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg23`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg23 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg23::Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg23;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg24`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg24 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg24::Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg24;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg25`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg25 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg25::Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg25;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg26`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg26 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg26::Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg26;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg27`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg27 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg27::Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg27;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg28`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg28 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg28::Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg28;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg29`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg29 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg29::Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg29;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg30`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg30 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg30::Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg30;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg31`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg31 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg31::Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg31;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg32`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg32 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg32::Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg32;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg33`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg33 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg33::Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg33;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg34`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg34 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg34::Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg34;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg35`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg35 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg35::Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg35;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg36`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg36 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg36::Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg36;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG37 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg37`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG37")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg37 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg37::Pr1IcssIntc_IntcSlv_RegsChMapReg37Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG37"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg37;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg38`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg38 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg38::Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg38;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_ch_map_reg39`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39")]
pub type Pr1IcssIntc_IntcSlv_RegsChMapReg39 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_ch_map_reg39::Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39"]
pub mod pr1_icss_intc__intc_slv__regs_ch_map_reg39;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_hint_map_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsHintMapReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_hint_map_reg0::Pr1IcssIntc_IntcSlv_RegsHintMapReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_hint_map_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_hint_map_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsHintMapReg1 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_hint_map_reg1::Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_hint_map_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_hint_map_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsHintMapReg2 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_hint_map_reg2::Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_hint_map_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_hint_map_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsHintMapReg3 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_hint_map_reg3::Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_hint_map_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_hint_map_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsHintMapReg4 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_hint_map_reg4::Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_hint_map_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg0::Pr1IcssIntc_IntcSlv_RegsPriHintReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg1 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg1::Pr1IcssIntc_IntcSlv_RegsPriHintReg1Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg2 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg2::Pr1IcssIntc_IntcSlv_RegsPriHintReg2Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg3 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg3::Pr1IcssIntc_IntcSlv_RegsPriHintReg3Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg4 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg4::Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG5 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg5`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG5")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg5 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg5::Pr1IcssIntc_IntcSlv_RegsPriHintReg5Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG5"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg5;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG6 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg6`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG6")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg6 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg6::Pr1IcssIntc_IntcSlv_RegsPriHintReg6Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG6"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg6;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG7 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg7`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG7")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg7 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg7::Pr1IcssIntc_IntcSlv_RegsPriHintReg7Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG7"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg7;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG8 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg8`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG8")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg8 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg8::Pr1IcssIntc_IntcSlv_RegsPriHintReg8Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG8"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg8;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG9 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg9`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG9")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg9 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg9::Pr1IcssIntc_IntcSlv_RegsPriHintReg9Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG9"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg9;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG10 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg10`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG10")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg10 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg10::Pr1IcssIntc_IntcSlv_RegsPriHintReg10Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG10"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg10;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG11 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg11`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG11")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg11 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg11::Pr1IcssIntc_IntcSlv_RegsPriHintReg11Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG11"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg11;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG12 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg12`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG12")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg12 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg12::Pr1IcssIntc_IntcSlv_RegsPriHintReg12Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG12"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg12;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG13 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg13`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG13")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg13 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg13::Pr1IcssIntc_IntcSlv_RegsPriHintReg13Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG13"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg13;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG14 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg14`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG14")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg14 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg14::Pr1IcssIntc_IntcSlv_RegsPriHintReg14Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG14"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg14;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG15 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg15`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG15")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg15 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg15::Pr1IcssIntc_IntcSlv_RegsPriHintReg15Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG15"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg15;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG16 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg16`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG16")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg16 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg16::Pr1IcssIntc_IntcSlv_RegsPriHintReg16Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG16"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg16;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG17 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg17`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG17")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg17 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg17::Pr1IcssIntc_IntcSlv_RegsPriHintReg17Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG17"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg17;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG18 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg18`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG18")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg18 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg18::Pr1IcssIntc_IntcSlv_RegsPriHintReg18Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG18"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg18;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_pri_hint_reg19`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19")]
pub type Pr1IcssIntc_IntcSlv_RegsPriHintReg19 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_pri_hint_reg19::Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19"]
pub mod pr1_icss_intc__intc_slv__regs_pri_hint_reg19;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_polarity_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_polarity_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_polarity_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsPolarityReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_polarity_reg0::Pr1IcssIntc_IntcSlv_RegsPolarityReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_polarity_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_polarity_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_polarity_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_polarity_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsPolarityReg1 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_polarity_reg1::Pr1IcssIntc_IntcSlv_RegsPolarityReg1Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_polarity_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_polarity_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_polarity_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_polarity_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsPolarityReg2 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_polarity_reg2::Pr1IcssIntc_IntcSlv_RegsPolarityReg2Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_polarity_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_polarity_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_polarity_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_polarity_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsPolarityReg3 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_polarity_reg3::Pr1IcssIntc_IntcSlv_RegsPolarityReg3Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_polarity_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_polarity_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_polarity_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_polarity_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsPolarityReg4 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_polarity_reg4::Pr1IcssIntc_IntcSlv_RegsPolarityReg4Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_POLARITY_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_polarity_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_type_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_type_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_type_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsTypeReg0 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_type_reg0::Pr1IcssIntc_IntcSlv_RegsTypeReg0Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_type_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_type_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_type_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_type_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsTypeReg1 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_type_reg1::Pr1IcssIntc_IntcSlv_RegsTypeReg1Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_type_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_type_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_type_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_type_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsTypeReg2 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_type_reg2::Pr1IcssIntc_IntcSlv_RegsTypeReg2Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_type_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_type_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_type_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_type_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsTypeReg3 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_type_reg3::Pr1IcssIntc_IntcSlv_RegsTypeReg3Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_type_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_type_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_type_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_type_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsTypeReg4 =
    crate::Reg<pr1_icss_intc__intc_slv__regs_type_reg4::Pr1IcssIntc_IntcSlv_RegsTypeReg4Spec>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_TYPE_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_type_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg0::Pr1IcssIntc_IntcSlv_RegsNestLevelReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg0;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG1 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg1`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG1")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg1 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg1::Pr1IcssIntc_IntcSlv_RegsNestLevelReg1Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG1"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg1;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG2 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg2`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG2")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg2 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg2::Pr1IcssIntc_IntcSlv_RegsNestLevelReg2Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG2"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg2;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg3`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg3 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg3::Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg3;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG4 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg4`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG4")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg4 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg4::Pr1IcssIntc_IntcSlv_RegsNestLevelReg4Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG4"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg4;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG5 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg5`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG5")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg5 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg5::Pr1IcssIntc_IntcSlv_RegsNestLevelReg5Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG5"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg5;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG6 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg6`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG6")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg6 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg6::Pr1IcssIntc_IntcSlv_RegsNestLevelReg6Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG6"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg6;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG7 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg7`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG7")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg7 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg7::Pr1IcssIntc_IntcSlv_RegsNestLevelReg7Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG7"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg7;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG8 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg8`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG8")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg8 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg8::Pr1IcssIntc_IntcSlv_RegsNestLevelReg8Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG8"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg8;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG9 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg9`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG9")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg9 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg9::Pr1IcssIntc_IntcSlv_RegsNestLevelReg9Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG9"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg9;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG10 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg10`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG10")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg10 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg10::Pr1IcssIntc_IntcSlv_RegsNestLevelReg10Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG10"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg10;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG11 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg11`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG11")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg11 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg11::Pr1IcssIntc_IntcSlv_RegsNestLevelReg11Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG11"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg11;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG12 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg12`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG12")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg12 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg12::Pr1IcssIntc_IntcSlv_RegsNestLevelReg12Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG12"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg12;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg13`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg13 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg13::Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg13;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG14 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg14`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG14")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg14 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg14::Pr1IcssIntc_IntcSlv_RegsNestLevelReg14Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG14"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg14;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG15 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg15`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG15")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg15 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg15::Pr1IcssIntc_IntcSlv_RegsNestLevelReg15Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG15"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg15;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG16 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg16`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG16")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg16 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg16::Pr1IcssIntc_IntcSlv_RegsNestLevelReg16Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG16"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg16;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG17 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg17`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG17")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg17 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg17::Pr1IcssIntc_IntcSlv_RegsNestLevelReg17Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG17"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg17;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG18 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg18`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG18")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg18 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg18::Pr1IcssIntc_IntcSlv_RegsNestLevelReg18Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG18"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg18;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG19 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_nest_level_reg19`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG19")]
pub type Pr1IcssIntc_IntcSlv_RegsNestLevelReg19 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_nest_level_reg19::Pr1IcssIntc_IntcSlv_RegsNestLevelReg19Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG19"]
pub mod pr1_icss_intc__intc_slv__regs_nest_level_reg19;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_HINT_REG0 (rw) register accessor: PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_HINT_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_enable_hint_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_enable_hint_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_intc__intc_slv__regs_enable_hint_reg0`]
module"]
#[doc(alias = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_HINT_REG0")]
pub type Pr1IcssIntc_IntcSlv_RegsEnableHintReg0 = crate::Reg<
    pr1_icss_intc__intc_slv__regs_enable_hint_reg0::Pr1IcssIntc_IntcSlv_RegsEnableHintReg0Spec,
>;
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_ENABLE_HINT_REG0"]
pub mod pr1_icss_intc__intc_slv__regs_enable_hint_reg0;
