#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c04],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr: Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastartr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr: Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastopr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr: Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastatr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr: Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlr,
    _reserved4: [u8; 0xe8],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr: Vbusp2apbWrap_CxstmCfg_StmRegsStmdmaidr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer: Vbusp2apbWrap_CxstmCfg_StmRegsStmheer,
    _reserved6: [u8; 0x1c],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter: Vbusp2apbWrap_CxstmCfg_StmRegsStmheter,
    _reserved7: [u8; 0x3c],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr: Vbusp2apbWrap_CxstmCfg_StmRegsStmhebsr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr: Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr: Vbusp2apbWrap_CxstmCfg_StmRegsStmheextmuxr,
    _reserved10: [u8; 0x88],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr: Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r: Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1r,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr: Vbusp2apbWrap_CxstmCfg_StmRegsStmheidr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper: Vbusp2apbWrap_CxstmCfg_StmRegsStmsper,
    _reserved14: [u8; 0x1c],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter: Vbusp2apbWrap_CxstmCfg_StmRegsStmspter,
    _reserved15: [u8; 0x3c],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr: Vbusp2apbWrap_CxstmCfg_StmRegsStmspscr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr: Vbusp2apbWrap_CxstmCfg_StmRegsStmspmscr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider:
        Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverrider,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider:
        Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverrider,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr: Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsr,
    _reserved20: [u8; 0x0c],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr: Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr: Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimr,
    _reserved22: [u8; 0x04],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr: Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr: Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr: Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcr,
    _reserved25: [u8; 0x08],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r: Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1r,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r: Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2r,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r: Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3r,
    _reserved28: [u8; 0x3c],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger: Vbusp2apbWrap_CxstmCfg_StmRegsStmittrigger,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0: Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2: Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid: Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbid,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0: Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0,
    _reserved33: [u8; 0x04],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl: Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrl,
    _reserved34: [u8; 0x9c],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset: Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimset,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr: Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimclr,
    _reserved36: [u8; 0x08],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar: Vbusp2apbWrap_CxstmCfg_StmRegsStmlar,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr: Vbusp2apbWrap_CxstmCfg_StmRegsStmlsr,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus: Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatus,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch: Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarch,
    _reserved40: [u8; 0x08],
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid: Vbusp2apbWrap_CxstmCfg_StmRegsStmdevid,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype: Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtype,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr5,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr6,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr7,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr0,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3: Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0: Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr0,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1: Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr1,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2: Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr2,
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3: Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3,
}
impl RegisterBlock {
    #[doc = "0xc04 - This write-only register is used to start a DMA transfer.&lt;p/>A write of one when the DMA peripheral request interface is idle starts a DMA transfer. A write of zero has no effect. A write of one when the DMA peripheral request interface is active has no effect."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastartr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr
    }
    #[doc = "0xc08 - This write-only register is used to stop a DMA transfer.&lt;p/>A write of one stops an active DMA transfer. A write of zero has no effect. A write of one when the DMA peripheral request interface is idle has no effect."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastopr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr
    }
    #[doc = "0xc0c - This read-only register is used to determine the status of the DMA peripheral request interface."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastatr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr
    }
    #[doc = "0xc10 - Controls the DMA transfer request mechanism."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr
    }
    #[doc = "0xcfc - This read-only register indicates the DMA features of the STM"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdmaidr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr
    }
    #[doc = "0xd00 - This read/write register is used to enable hardware events to generate trace.&lt;p/>The register defined one bit per hardware event. Writing 1 enables the appropriate hardware event, writing 0 disables the appropriate hardware event."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmheer {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer
    }
    #[doc = "0xd20 - This register is used to enable trigger generation on hardware events."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmheter {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter
    }
    #[doc = "0xd60 - This register is used to select the Hardware Event bank"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmhebsr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr
    }
    #[doc = "0xd64 - This register is used to control the primary functions of Hardware Event tracing."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr
    }
    #[doc = "0xd68 - This register is used to control hardware event multiplexors external to the STM"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmheextmuxr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr
    }
    #[doc = "0xdf4 - Indicates the STPv2 master number of hardware event trace. This number is the master number presented in STPv2."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr
    }
    #[doc = "0xdf8 - Indicates the features of the STM."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1r {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r
    }
    #[doc = "0xdfc - Indicates the features of hardware event tracing in the STM."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmheidr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr
    }
    #[doc = "0xe00 - This read/write only register is used to enable the stimulus registers to generate trace.&lt;p/>The register defines one bit per stimulus register. Writing 1 enables the appropriate stimulus port, writing 0 disables the appropriate stimulus port. This register is used in conjunction with the Software Enable Bank Select Register."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmsper {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper
    }
    #[doc = "0xe20 - This register is used to enable trigger generation on writes to enabled stimulus port registers."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmspter {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter
    }
    #[doc = "0xe60 - This register allows a debugger to program which stimulus ports the STMSPER and STMSPTER apply to."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmspscr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr
    }
    #[doc = "0xe64 - This register allows a debugger to program which masters the STMSPSCR applies to."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmspmscr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr
    }
    #[doc = "0xe68 - This register allows a debugger to override various features of the STM."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverrider {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider
    }
    #[doc = "0xe6c - This register allows a debugger to choose which masters the STMSPOVERRIDERR applies to."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverrider {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider
    }
    #[doc = "0xe70 - This register is used to control the STM triggers caused by STMSPTER."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr
    }
    #[doc = "0xe80 - Controls the STM settings."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr
    }
    #[doc = "0xe84 - This write-only register is used to force the next packet caused by a stimulus port write to have a timestamp output."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr
    }
    #[doc = "0xe8c - This read-write register is used to indicate the frequency of the timestamp counter. The unit of measurement is increments per second. When the STPv2 protocol is used, this register contains the value output in the FREQ and FREQ_TS packets. The timestamp frequency is output in the STPv2 protocol at every synchronization point when STMTCSR.TSEN is b1."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr
    }
    #[doc = "0xe90 - This register controls the interval between synchronization packets, in terms of the number of bytes of trace generated.&lt;p/>This register only provides a hint of the desired synchronization frequency, implementations are permitted to be inaccurate.&lt;p/>Writing a value of 0x00000000 to this register disables the synchronization counter however any other IMPLEMENTATION DEFINED synchronizations mechanism continue to operate independently."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr
    }
    #[doc = "0xe94 - Used for IMPLEMENTATION DEFINED STM controls."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr
    }
    #[doc = "0xea0 - Indicates the features of the STM."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1r {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r
    }
    #[doc = "0xea4 - Indicates the features of the STM."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2r {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r
    }
    #[doc = "0xea8 - Indicates the features of the STM."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3r {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r
    }
    #[doc = "0xee8 - Integration Test for Cross-Trigger Outputs Register"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmittrigger {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger
    }
    #[doc = "0xeec - Controls the value of the ATDATAM output in integration mode:"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0
    }
    #[doc = "0xef0 - Returns the value of the ATREADYM and AFVALIDM inputs in integration mode."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2
    }
    #[doc = "0xef4 - Controls the value of the ATIDM output in integration mode."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbid {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid
    }
    #[doc = "0xef8 - Controls the value of the ATVALIDM, AFREADYM, and ATBYTESM outputs in integration mode."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0
    }
    #[doc = "0xf00 - Used to enable topology detection. See the CoreSight Architecture Specification for more information. This register enables the component to switch between functional mode and integration mode. The default behavior is functional mode. In integration mode the inputs and outputs of the STM can be directly controlled for integration testing and topology solving. &lt;p/>Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrl {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl
    }
    #[doc = "0xfa0 - This is used in conjunction with Claim Tag Clear Register, STMCLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimset {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset
    }
    #[doc = "0xfa4 - This register is used in conjunction with Claim Tag Set Register, STMCLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimclr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr
    }
    #[doc = "0xfb0 - Enables write access to device registers."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmlar {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar
    }
    #[doc = "0xfb4 - Indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. The lock mechanism does not impact accesses to the extended stimulus port registers. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, blocks write accesses to any register, except the STMLAR. The lock mechanism is only present for accesses with the PADDRDBG31 signal LOW."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmlsr {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr
    }
    #[doc = "0xfb8 - Reports the required security level and current status of the authentication interface."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatus {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus
    }
    #[doc = "0xfbc - Indicates the architect and architecture of the STM. For the STM-500, the architect is ARM, and the architecture is STMv1.1"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarch {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch
    }
    #[doc = "0xfc8 - Indicates the capabilities of the CoreSight STM."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdevid {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid
    }
    #[doc = "0xfcc - Provides a debugger with information about the component when the part number is not recognized. The debugger can then report this information."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtype {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype
    }
    #[doc = "0xfd0 - Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4
    }
    #[doc = "0xfd4 - Reserved"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr5 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5
    }
    #[doc = "0xfd8 - Reserved"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr6 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6
    }
    #[doc = "0xfdc - Reserved"]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr7 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7
    }
    #[doc = "0xfe0 - Part of the set of Peripheral Identification registers. Contains part of the designer specific part number."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr0 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0
    }
    #[doc = "0xfe4 - Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1
    }
    #[doc = "0xfe8 - Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2
    }
    #[doc = "0xfec - Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer_Modified bit fields."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3
    }
    #[doc = "0xff0 - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr0 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0
    }
    #[doc = "0xff4 - A component identification register, that indicates that the identification registers are present. This register also indicates the component class."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr1 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1
    }
    #[doc = "0xff8 - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr2 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2
    }
    #[doc = "0xffc - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3(
        &self,
    ) -> &Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3 {
        &self.vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3
    }
}
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTARTR (rw) register accessor: This write-only register is used to start a DMA transfer.&lt;p/>A write of one when the DMA peripheral request interface is idle starts a DMA transfer. A write of zero has no effect. A write of one when the DMA peripheral request interface is active has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTARTR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastartr = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr :: Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastartrSpec > ;
#[doc = "This write-only register is used to start a DMA transfer.&lt;p/>A write of one when the DMA peripheral request interface is idle starts a DMA transfer. A write of zero has no effect. A write of one when the DMA peripheral request interface is active has no effect."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastartr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTOPR (rw) register accessor: This write-only register is used to stop a DMA transfer.&lt;p/>A write of one stops an active DMA transfer. A write of zero has no effect. A write of one when the DMA peripheral request interface is idle has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTOPR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastopr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr::Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec,
>;
#[doc = "This write-only register is used to stop a DMA transfer.&lt;p/>A write of one stops an active DMA transfer. A write of zero has no effect. A write of one when the DMA peripheral request interface is idle has no effect."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTATR (rw) register accessor: This read-only register is used to determine the status of the DMA peripheral request interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTATR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastatr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr::Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastatrSpec,
>;
#[doc = "This read-only register is used to determine the status of the DMA peripheral request interface."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastatr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMACTLR (rw) register accessor: Controls the DMA transfer request mechanism.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMACTLR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr::Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec,
>;
#[doc = "Controls the DMA transfer request mechanism."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMAIDR (rw) register accessor: This read-only register indicates the DMA features of the STM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMAIDR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdmaidr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr::Vbusp2apbWrap_CxstmCfg_StmRegsStmdmaidrSpec,
>;
#[doc = "This read-only register indicates the DMA features of the STM"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmaidr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEER (rw) register accessor: This read/write register is used to enable hardware events to generate trace.&lt;p/>The register defined one bit per hardware event. Writing 1 enables the appropriate hardware event, writing 0 disables the appropriate hardware event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEER")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmheer = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer::Vbusp2apbWrap_CxstmCfg_StmRegsStmheerSpec,
>;
#[doc = "This read/write register is used to enable hardware events to generate trace.&lt;p/>The register defined one bit per hardware event. Writing 1 enables the appropriate hardware event, writing 0 disables the appropriate hardware event."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheer;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHETER (rw) register accessor: This register is used to enable trigger generation on hardware events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHETER")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmheter = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter::Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec,
>;
#[doc = "This register is used to enable trigger generation on hardware events."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEBSR (rw) register accessor: This register is used to select the Hardware Event bank\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEBSR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmhebsr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr::Vbusp2apbWrap_CxstmCfg_StmRegsStmhebsrSpec,
>;
#[doc = "This register is used to select the Hardware Event bank"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhebsr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMCR (rw) register accessor: This register is used to control the primary functions of Hardware Event tracing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMCR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr::Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec,
>;
#[doc = "This register is used to control the primary functions of Hardware Event tracing."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEEXTMUXR (rw) register accessor: This register is used to control hardware event multiplexors external to the STM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEEXTMUXR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmheextmuxr = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr :: Vbusp2apbWrap_CxstmCfg_StmRegsStmheextmuxrSpec > ;
#[doc = "This register is used to control hardware event multiplexors external to the STM"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheextmuxr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMASTR (rw) register accessor: Indicates the STPv2 master number of hardware event trace. This number is the master number presented in STPv2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMASTR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr::Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec,
>;
#[doc = "Indicates the STPv2 master number of hardware event trace. This number is the master number presented in STPv2."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEFEAT1R (rw) register accessor: Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEFEAT1R")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1r = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r::Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec,
>;
#[doc = "Indicates the features of the STM."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEIDR (rw) register accessor: Indicates the features of hardware event tracing in the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEIDR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmheidr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr::Vbusp2apbWrap_CxstmCfg_StmRegsStmheidrSpec,
>;
#[doc = "Indicates the features of hardware event tracing in the STM."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheidr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPER (rw) register accessor: This read/write only register is used to enable the stimulus registers to generate trace.&lt;p/>The register defines one bit per stimulus register. Writing 1 enables the appropriate stimulus port, writing 0 disables the appropriate stimulus port. This register is used in conjunction with the Software Enable Bank Select Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPER")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmsper = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper::Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec,
>;
#[doc = "This read/write only register is used to enable the stimulus registers to generate trace.&lt;p/>The register defines one bit per stimulus register. Writing 1 enables the appropriate stimulus port, writing 0 disables the appropriate stimulus port. This register is used in conjunction with the Software Enable Bank Select Register."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTER (rw) register accessor: This register is used to enable trigger generation on writes to enabled stimulus port registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTER")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmspter = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter::Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec,
>;
#[doc = "This register is used to enable trigger generation on writes to enabled stimulus port registers."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPSCR (rw) register accessor: This register allows a debugger to program which stimulus ports the STMSPER and STMSPTER apply to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPSCR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmspscr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr::Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec,
>;
#[doc = "This register allows a debugger to program which stimulus ports the STMSPER and STMSPTER apply to."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPMSCR (rw) register accessor: This register allows a debugger to program which masters the STMSPSCR applies to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPMSCR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmspmscr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr::Vbusp2apbWrap_CxstmCfg_StmRegsStmspmscrSpec,
>;
#[doc = "This register allows a debugger to program which masters the STMSPSCR applies to."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmscr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPOVERRIDER (rw) register accessor: This register allows a debugger to override various features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPOVERRIDER")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverrider = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider :: Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec > ;
#[doc = "This register allows a debugger to override various features of the STM."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPMOVERRIDER (rw) register accessor: This register allows a debugger to choose which masters the STMSPOVERRIDERR applies to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPMOVERRIDER")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverrider = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider :: Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec > ;
#[doc = "This register allows a debugger to choose which masters the STMSPOVERRIDERR applies to."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTRIGCSR (rw) register accessor: This register is used to control the STM triggers caused by STMSPTER.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTRIGCSR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsr = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr :: Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec > ;
#[doc = "This register is used to control the STM triggers caused by STMSPTER."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTCSR (rw) register accessor: Controls the STM settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTCSR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr::Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec,
>;
#[doc = "Controls the STM settings."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSSTIMR (rw) register accessor: This write-only register is used to force the next packet caused by a stimulus port write to have a timestamp output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSSTIMR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr::Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec,
>;
#[doc = "This write-only register is used to force the next packet caused by a stimulus port write to have a timestamp output."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSFREQR (rw) register accessor: This read-write register is used to indicate the frequency of the timestamp counter. The unit of measurement is increments per second. When the STPv2 protocol is used, this register contains the value output in the FREQ and FREQ_TS packets. The timestamp frequency is output in the STPv2 protocol at every synchronization point when STMTCSR.TSEN is b1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSFREQR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr::Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec,
>;
#[doc = "This read-write register is used to indicate the frequency of the timestamp counter. The unit of measurement is increments per second. When the STPv2 protocol is used, this register contains the value output in the FREQ and FREQ_TS packets. The timestamp frequency is output in the STPv2 protocol at every synchronization point when STMTCSR.TSEN is b1."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSYNCR (rw) register accessor: This register controls the interval between synchronization packets, in terms of the number of bytes of trace generated.&lt;p/>This register only provides a hint of the desired synchronization frequency, implementations are permitted to be inaccurate.&lt;p/>Writing a value of 0x00000000 to this register disables the synchronization counter however any other IMPLEMENTATION DEFINED synchronizations mechanism continue to operate independently.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSYNCR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr::Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec,
>;
#[doc = "This register controls the interval between synchronization packets, in terms of the number of bytes of trace generated.&lt;p/>This register only provides a hint of the desired synchronization frequency, implementations are permitted to be inaccurate.&lt;p/>Writing a value of 0x00000000 to this register disables the synchronization counter however any other IMPLEMENTATION DEFINED synchronizations mechanism continue to operate independently."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUXCR (rw) register accessor: Used for IMPLEMENTATION DEFINED STM controls.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUXCR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr::Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec,
>;
#[doc = "Used for IMPLEMENTATION DEFINED STM controls."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT1R (rw) register accessor: Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT1R")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1r = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r::Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec,
>;
#[doc = "Indicates the features of the STM."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT2R (rw) register accessor: Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT2R")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2r = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r::Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec,
>;
#[doc = "Indicates the features of the STM."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT3R (rw) register accessor: Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT3R")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3r = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r::Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec,
>;
#[doc = "Indicates the features of the STM."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITTRIGGER (rw) register accessor: Integration Test for Cross-Trigger Outputs Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITTRIGGER")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmittrigger = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger :: Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec > ;
#[doc = "Integration Test for Cross-Trigger Outputs Register"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBDATA0 (rw) register accessor: Controls the value of the ATDATAM output in integration mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBDATA0")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0 = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0 :: Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec > ;
#[doc = "Controls the value of the ATDATAM output in integration mode:"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR2 (rw) register accessor: Returns the value of the ATREADYM and AFVALIDM inputs in integration mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR2")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2 = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2 :: Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec > ;
#[doc = "Returns the value of the ATREADYM and AFVALIDM inputs in integration mode."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBID (rw) register accessor: Controls the value of the ATIDM output in integration mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBID")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbid = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid::Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec,
>;
#[doc = "Controls the value of the ATIDM output in integration mode."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR0 (rw) register accessor: Controls the value of the ATVALIDM, AFREADYM, and ATBYTESM outputs in integration mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR0")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0 = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0 :: Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec > ;
#[doc = "Controls the value of the ATVALIDM, AFREADYM, and ATBYTESM outputs in integration mode."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITCTRL (rw) register accessor: Used to enable topology detection. See the CoreSight Architecture Specification for more information. This register enables the component to switch between functional mode and integration mode. The default behavior is functional mode. In integration mode the inputs and outputs of the STM can be directly controlled for integration testing and topology solving. &lt;p/>Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITCTRL")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrl = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl::Vbusp2apbWrap_CxstmCfg_StmRegsStmitctrlSpec,
>;
#[doc = "Used to enable topology detection. See the CoreSight Architecture Specification for more information. This register enables the component to switch between functional mode and integration mode. The default behavior is functional mode. In integration mode the inputs and outputs of the STM can be directly controlled for integration testing and topology solving. &lt;p/>Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitctrl;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCLAIMSET (rw) register accessor: This is used in conjunction with Claim Tag Clear Register, STMCLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCLAIMSET")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimset = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset::Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimsetSpec,
>;
#[doc = "This is used in conjunction with Claim Tag Clear Register, STMCLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimset;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCLAIMCLR (rw) register accessor: This register is used in conjunction with Claim Tag Set Register, STMCLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCLAIMCLR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimclr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr::Vbusp2apbWrap_CxstmCfg_StmRegsStmclaimclrSpec,
>;
#[doc = "This register is used in conjunction with Claim Tag Set Register, STMCLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmclaimclr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMLAR (rw) register accessor: Enables write access to device registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMLAR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmlar = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar::Vbusp2apbWrap_CxstmCfg_StmRegsStmlarSpec,
>;
#[doc = "Enables write access to device registers."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlar;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMLSR (rw) register accessor: Indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. The lock mechanism does not impact accesses to the extended stimulus port registers. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, blocks write accesses to any register, except the STMLAR. The lock mechanism is only present for accesses with the PADDRDBG31 signal LOW.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMLSR")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmlsr = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr::Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec,
>;
#[doc = "Indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. The lock mechanism does not impact accesses to the extended stimulus port registers. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, blocks write accesses to any register, except the STMLAR. The lock mechanism is only present for accesses with the PADDRDBG31 signal LOW."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUTHSTATUS (rw) register accessor: Reports the required security level and current status of the authentication interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUTHSTATUS")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatus = crate :: Reg < vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus :: Vbusp2apbWrap_CxstmCfg_StmRegsStmauthstatusSpec > ;
#[doc = "Reports the required security level and current status of the authentication interface."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauthstatus;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVARCH (rw) register accessor: Indicates the architect and architecture of the STM. For the STM-500, the architect is ARM, and the architecture is STMv1.1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVARCH")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarch = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch::Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec,
>;
#[doc = "Indicates the architect and architecture of the STM. For the STM-500, the architect is ARM, and the architecture is STMv1.1"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVID (rw) register accessor: Indicates the capabilities of the CoreSight STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVID")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdevid = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid::Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec,
>;
#[doc = "Indicates the capabilities of the CoreSight STM."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVTYPE (rw) register accessor: Provides a debugger with information about the component when the part number is not recognized. The debugger can then report this information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVTYPE")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtype = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype::Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec,
>;
#[doc = "Provides a debugger with information about the component when the part number is not recognized. The debugger can then report this information."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR4 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR4")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec,
>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR5 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR5")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr5 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr5Spec,
>;
#[doc = "Reserved"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr5;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR6 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR6")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr6 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr6Spec,
>;
#[doc = "Reserved"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr6;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR7 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR7")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr7 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr7Spec,
>;
#[doc = "Reserved"]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr7;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR0 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR0")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr0 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr0Spec,
>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr0;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR1 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR1")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec,
>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR2 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR2")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec,
>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR3 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer_Modified bit fields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR3")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3::Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec,
>;
#[doc = "Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer_Modified bit fields."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR0 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR0")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr0 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0::Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr0Spec,
>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr0;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR1 (rw) register accessor: A component identification register, that indicates that the identification registers are present. This register also indicates the component class.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR1")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr1 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1::Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr1Spec,
>;
#[doc = "A component identification register, that indicates that the identification registers are present. This register also indicates the component class."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr1;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR2 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR2")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr2 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2::Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr2Spec,
>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr2;
#[doc = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR3 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3`]
module"]
#[doc(alias = "VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR3")]
pub type Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3 = crate::Reg<
    vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3::Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec,
>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3;
