#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adcregs_revision: AdcregsRevision,
    _reserved1: [u8; 0x1c],
    adcregs_eoi: AdcregsEoi,
    adcregs_status_raw: AdcregsStatusRaw,
    adcregs_status: AdcregsStatus,
    adcregs_enable_set: AdcregsEnableSet,
    adcregs_enable_clr: AdcregsEnableClr,
    _reserved6: [u8; 0x04],
    adcregs_dmaenable_set: AdcregsDmaenableSet,
    adcregs_dmaenable_clr: AdcregsDmaenableClr,
    adcregs_control: AdcregsControl,
    adcregs_sequencer_stat: AdcregsSequencerStat,
    adcregs_range: AdcregsRange,
    _reserved11: [u8; 0x04],
    adcregs_misc: AdcregsMisc,
    adcregs_stepenable: AdcregsStepenable,
    _reserved13: [u8; 0x0c],
    adcregs_stepconfig_0: AdcregsStepconfig0,
    adcregs_stepdelay_0: AdcregsStepdelay0,
    adcregs_stepconfig_1: AdcregsStepconfig1,
    adcregs_stepdelay_1: AdcregsStepdelay1,
    adcregs_stepconfig_2: AdcregsStepconfig2,
    adcregs_stepdelay_2: AdcregsStepdelay2,
    adcregs_stepconfig_3: AdcregsStepconfig3,
    adcregs_stepdelay_3: AdcregsStepdelay3,
    adcregs_stepconfig_4: AdcregsStepconfig4,
    adcregs_stepdelay_4: AdcregsStepdelay4,
    adcregs_stepconfig_5: AdcregsStepconfig5,
    adcregs_stepdelay_5: AdcregsStepdelay5,
    adcregs_stepconfig_6: AdcregsStepconfig6,
    adcregs_stepdelay_6: AdcregsStepdelay6,
    adcregs_stepconfig_7: AdcregsStepconfig7,
    adcregs_stepdelay_7: AdcregsStepdelay7,
    adcregs_stepconfig_8: AdcregsStepconfig8,
    adcregs_stepdelay_8: AdcregsStepdelay8,
    adcregs_stepconfig_9: AdcregsStepconfig9,
    adcregs_stepdelay_9: AdcregsStepdelay9,
    adcregs_stepconfig_10: AdcregsStepconfig10,
    adcregs_stepdelay_10: AdcregsStepdelay10,
    adcregs_stepconfig_11: AdcregsStepconfig11,
    adcregs_stepdelay_11: AdcregsStepdelay11,
    adcregs_stepconfig_12: AdcregsStepconfig12,
    adcregs_stepdelay_12: AdcregsStepdelay12,
    adcregs_stepconfig_13: AdcregsStepconfig13,
    adcregs_stepdelay_13: AdcregsStepdelay13,
    adcregs_stepconfig_14: AdcregsStepconfig14,
    adcregs_stepdelay_14: AdcregsStepdelay14,
    adcregs_stepconfig_15: AdcregsStepconfig15,
    adcregs_stepdelay_15: AdcregsStepdelay15,
    adcregs_fifo0wc: AdcregsFifo0wc,
    adcregs_fifo0threshold: AdcregsFifo0threshold,
    adcregs_fifo0dmareq: AdcregsFifo0dmareq,
    adcregs_fifo1wc: AdcregsFifo1wc,
    adcregs_fifo1threshold: AdcregsFifo1threshold,
    adcregs_fifo1dmareq: AdcregsFifo1dmareq,
    _reserved51: [u8; 0x04],
    adcregs_fifo0data: AdcregsFifo0data,
    _reserved52: [u8; 0xfc],
    adcregs_fifo1data: AdcregsFifo1data,
}
impl RegisterBlock {
    #[doc = "0x00 - IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
    #[inline(always)]
    pub const fn adcregs_revision(&self) -> &AdcregsRevision {
        &self.adcregs_revision
    }
    #[doc = "0x20 - The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to."]
    #[inline(always)]
    pub const fn adcregs_eoi(&self) -> &AdcregsEoi {
        &self.adcregs_eoi
    }
    #[doc = "0x24 - The IRQ_STATUS_RAW register allows the adc12 interrupt sources to be manually set when writing a 1 to a specific bit. Write 0: No action Write 1: Set event Read 0: No event pending Read 1: Event pending"]
    #[inline(always)]
    pub const fn adcregs_status_raw(&self) -> &AdcregsStatusRaw {
        &self.adcregs_status_raw
    }
    #[doc = "0x28 - The IRQ_STATUS register allows the adc12 interrupt sources to be manually cleared when writing a 1 to a specific bit. Write 0: No action Write 1: Clear event Read 0: No event pending Read 1: Event pending"]
    #[inline(always)]
    pub const fn adcregs_status(&self) -> &AdcregsStatus {
        &self.adcregs_status
    }
    #[doc = "0x2c - The IRQ_ENABLE_SET register allows the adc12 interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled"]
    #[inline(always)]
    pub const fn adcregs_enable_set(&self) -> &AdcregsEnableSet {
        &self.adcregs_enable_set
    }
    #[doc = "0x30 - The IRQ_ENABLE_CLR register allows the adc12 interrupt sources to be manually disabled when writing a 1 to a specific bit. Write 0: No action Write 1: Disable event Read 0: Event is disabled Read 1: Event is enabled"]
    #[inline(always)]
    pub const fn adcregs_enable_clr(&self) -> &AdcregsEnableClr {
        &self.adcregs_enable_clr
    }
    #[doc = "0x38 - The DMAENABLE_SET register allows the enabling of DMA requests"]
    #[inline(always)]
    pub const fn adcregs_dmaenable_set(&self) -> &AdcregsDmaenableSet {
        &self.adcregs_dmaenable_set
    }
    #[doc = "0x3c - The DMAENABLE_CLR register allows the disabling of DMA requests"]
    #[inline(always)]
    pub const fn adcregs_dmaenable_clr(&self) -> &AdcregsDmaenableClr {
        &self.adcregs_dmaenable_clr
    }
    #[doc = "0x40 - Controls various parameters of the cotroller state."]
    #[inline(always)]
    pub const fn adcregs_control(&self) -> &AdcregsControl {
        &self.adcregs_control
    }
    #[doc = "0x44 - SW can read this register to find out the currently scheduled step id being converted on the ADC port. If you want to turn the controller off and then back on, the step_id bit should be checked and compared to IDLE before enabling the ADC12_SS module again. Also, before enabling the controller again, the user should wait for the FSM bit 5 to read idl."]
    #[inline(always)]
    pub const fn adcregs_sequencer_stat(&self) -> &AdcregsSequencerStat {
        &self.adcregs_sequencer_stat
    }
    #[doc = "0x48 - This feature requires the range check interrupt bit to be enabled first. The user can decide which channel input is compared by programming the RangeCheck bit of the StepConfig Registers. It is up to software to sort through FIFO data to determine which channel data was out of range. Software should only use LSB 10 bits for comparison and make sure bits 11,12 are 0."]
    #[inline(always)]
    pub const fn adcregs_range(&self) -> &AdcregsRange {
        &self.adcregs_range
    }
    #[doc = "0x50 - Spare inputs of the AFE are driven by this register, spare outputs from the AFE are read."]
    #[inline(always)]
    pub const fn adcregs_misc(&self) -> &AdcregsMisc {
        &self.adcregs_misc
    }
    #[doc = "0x54 - Contains the enable bit for each step in the sequencer. When all steps are disabled, the FSM will stay in IDLE state. These bits can be enabled or disabled dynamically during operation. When a write to this register occurs during operational mode, the HW will make sure the new settings are updated after the END_OF_SEQUENCE event"]
    #[inline(always)]
    pub const fn adcregs_stepenable(&self) -> &AdcregsStepenable {
        &self.adcregs_stepenable
    }
    #[doc = "0x64 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_0(&self) -> &AdcregsStepconfig0 {
        &self.adcregs_stepconfig_0
    }
    #[doc = "0x68 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_0(&self) -> &AdcregsStepdelay0 {
        &self.adcregs_stepdelay_0
    }
    #[doc = "0x6c - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_1(&self) -> &AdcregsStepconfig1 {
        &self.adcregs_stepconfig_1
    }
    #[doc = "0x70 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_1(&self) -> &AdcregsStepdelay1 {
        &self.adcregs_stepdelay_1
    }
    #[doc = "0x74 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_2(&self) -> &AdcregsStepconfig2 {
        &self.adcregs_stepconfig_2
    }
    #[doc = "0x78 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_2(&self) -> &AdcregsStepdelay2 {
        &self.adcregs_stepdelay_2
    }
    #[doc = "0x7c - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_3(&self) -> &AdcregsStepconfig3 {
        &self.adcregs_stepconfig_3
    }
    #[doc = "0x80 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_3(&self) -> &AdcregsStepdelay3 {
        &self.adcregs_stepdelay_3
    }
    #[doc = "0x84 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_4(&self) -> &AdcregsStepconfig4 {
        &self.adcregs_stepconfig_4
    }
    #[doc = "0x88 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_4(&self) -> &AdcregsStepdelay4 {
        &self.adcregs_stepdelay_4
    }
    #[doc = "0x8c - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_5(&self) -> &AdcregsStepconfig5 {
        &self.adcregs_stepconfig_5
    }
    #[doc = "0x90 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_5(&self) -> &AdcregsStepdelay5 {
        &self.adcregs_stepdelay_5
    }
    #[doc = "0x94 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_6(&self) -> &AdcregsStepconfig6 {
        &self.adcregs_stepconfig_6
    }
    #[doc = "0x98 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_6(&self) -> &AdcregsStepdelay6 {
        &self.adcregs_stepdelay_6
    }
    #[doc = "0x9c - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_7(&self) -> &AdcregsStepconfig7 {
        &self.adcregs_stepconfig_7
    }
    #[doc = "0xa0 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_7(&self) -> &AdcregsStepdelay7 {
        &self.adcregs_stepdelay_7
    }
    #[doc = "0xa4 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_8(&self) -> &AdcregsStepconfig8 {
        &self.adcregs_stepconfig_8
    }
    #[doc = "0xa8 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_8(&self) -> &AdcregsStepdelay8 {
        &self.adcregs_stepdelay_8
    }
    #[doc = "0xac - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_9(&self) -> &AdcregsStepconfig9 {
        &self.adcregs_stepconfig_9
    }
    #[doc = "0xb0 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_9(&self) -> &AdcregsStepdelay9 {
        &self.adcregs_stepdelay_9
    }
    #[doc = "0xb4 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_10(&self) -> &AdcregsStepconfig10 {
        &self.adcregs_stepconfig_10
    }
    #[doc = "0xb8 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_10(&self) -> &AdcregsStepdelay10 {
        &self.adcregs_stepdelay_10
    }
    #[doc = "0xbc - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_11(&self) -> &AdcregsStepconfig11 {
        &self.adcregs_stepconfig_11
    }
    #[doc = "0xc0 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_11(&self) -> &AdcregsStepdelay11 {
        &self.adcregs_stepdelay_11
    }
    #[doc = "0xc4 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_12(&self) -> &AdcregsStepconfig12 {
        &self.adcregs_stepconfig_12
    }
    #[doc = "0xc8 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_12(&self) -> &AdcregsStepdelay12 {
        &self.adcregs_stepdelay_12
    }
    #[doc = "0xcc - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_13(&self) -> &AdcregsStepconfig13 {
        &self.adcregs_stepconfig_13
    }
    #[doc = "0xd0 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_13(&self) -> &AdcregsStepdelay13 {
        &self.adcregs_stepdelay_13
    }
    #[doc = "0xd4 - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_14(&self) -> &AdcregsStepconfig14 {
        &self.adcregs_stepconfig_14
    }
    #[doc = "0xd8 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_14(&self) -> &AdcregsStepdelay14 {
        &self.adcregs_stepdelay_14
    }
    #[doc = "0xdc - The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
    #[inline(always)]
    pub const fn adcregs_stepconfig_15(&self) -> &AdcregsStepconfig15 {
        &self.adcregs_stepconfig_15
    }
    #[doc = "0xe0 - Controls number of clock periods to sample and delay"]
    #[inline(always)]
    pub const fn adcregs_stepdelay_15(&self) -> &AdcregsStepdelay15 {
        &self.adcregs_stepdelay_15
    }
    #[doc = "0xe4 - FIFO word count status"]
    #[inline(always)]
    pub const fn adcregs_fifo0wc(&self) -> &AdcregsFifo0wc {
        &self.adcregs_fifo0wc
    }
    #[doc = "0xe8 - FIFO threshold"]
    #[inline(always)]
    pub const fn adcregs_fifo0threshold(&self) -> &AdcregsFifo0threshold {
        &self.adcregs_fifo0threshold
    }
    #[doc = "0xec - dma request."]
    #[inline(always)]
    pub const fn adcregs_fifo0dmareq(&self) -> &AdcregsFifo0dmareq {
        &self.adcregs_fifo0dmareq
    }
    #[doc = "0xf0 - FIFO word count status"]
    #[inline(always)]
    pub const fn adcregs_fifo1wc(&self) -> &AdcregsFifo1wc {
        &self.adcregs_fifo1wc
    }
    #[doc = "0xf4 - FIFO threshold"]
    #[inline(always)]
    pub const fn adcregs_fifo1threshold(&self) -> &AdcregsFifo1threshold {
        &self.adcregs_fifo1threshold
    }
    #[doc = "0xf8 - dma request."]
    #[inline(always)]
    pub const fn adcregs_fifo1dmareq(&self) -> &AdcregsFifo1dmareq {
        &self.adcregs_fifo1dmareq
    }
    #[doc = "0x100 - A read from this register will auto increment the FIFO read pointer. If you read when FIFO is empty,it will trigger an underflow interrupt"]
    #[inline(always)]
    pub const fn adcregs_fifo0data(&self) -> &AdcregsFifo0data {
        &self.adcregs_fifo0data
    }
    #[doc = "0x200 - A read from this register will auto increment the FIFO read pointer. If you read when FIFO is empty,it will trigger an underflow interrupt"]
    #[inline(always)]
    pub const fn adcregs_fifo1data(&self) -> &AdcregsFifo1data {
        &self.adcregs_fifo1data
    }
}
#[doc = "ADCREGS_REVISION (rw) register accessor: IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_revision`]
module"]
#[doc(alias = "ADCREGS_REVISION")]
pub type AdcregsRevision = crate::Reg<adcregs_revision::AdcregsRevisionSpec>;
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
pub mod adcregs_revision;
#[doc = "ADCREGS_DMAENABLE_SET (rw) register accessor: The DMAENABLE_SET register allows the enabling of DMA requests\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_dmaenable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_dmaenable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_dmaenable_set`]
module"]
#[doc(alias = "ADCREGS_DMAENABLE_SET")]
pub type AdcregsDmaenableSet = crate::Reg<adcregs_dmaenable_set::AdcregsDmaenableSetSpec>;
#[doc = "The DMAENABLE_SET register allows the enabling of DMA requests"]
pub mod adcregs_dmaenable_set;
#[doc = "ADCREGS_DMAENABLE_CLR (rw) register accessor: The DMAENABLE_CLR register allows the disabling of DMA requests\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_dmaenable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_dmaenable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_dmaenable_clr`]
module"]
#[doc(alias = "ADCREGS_DMAENABLE_CLR")]
pub type AdcregsDmaenableClr = crate::Reg<adcregs_dmaenable_clr::AdcregsDmaenableClrSpec>;
#[doc = "The DMAENABLE_CLR register allows the disabling of DMA requests"]
pub mod adcregs_dmaenable_clr;
#[doc = "ADCREGS_CONTROL (rw) register accessor: Controls various parameters of the cotroller state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_control`]
module"]
#[doc(alias = "ADCREGS_CONTROL")]
pub type AdcregsControl = crate::Reg<adcregs_control::AdcregsControlSpec>;
#[doc = "Controls various parameters of the cotroller state."]
pub mod adcregs_control;
#[doc = "ADCREGS_SEQUENCER_STAT (rw) register accessor: SW can read this register to find out the currently scheduled step id being converted on the ADC port. If you want to turn the controller off and then back on, the step_id bit should be checked and compared to IDLE before enabling the ADC12_SS module again. Also, before enabling the controller again, the user should wait for the FSM bit 5 to read idl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_sequencer_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_sequencer_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_sequencer_stat`]
module"]
#[doc(alias = "ADCREGS_SEQUENCER_STAT")]
pub type AdcregsSequencerStat = crate::Reg<adcregs_sequencer_stat::AdcregsSequencerStatSpec>;
#[doc = "SW can read this register to find out the currently scheduled step id being converted on the ADC port. If you want to turn the controller off and then back on, the step_id bit should be checked and compared to IDLE before enabling the ADC12_SS module again. Also, before enabling the controller again, the user should wait for the FSM bit 5 to read idl."]
pub mod adcregs_sequencer_stat;
#[doc = "ADCREGS_RANGE (rw) register accessor: This feature requires the range check interrupt bit to be enabled first. The user can decide which channel input is compared by programming the RangeCheck bit of the StepConfig Registers. It is up to software to sort through FIFO data to determine which channel data was out of range. Software should only use LSB 10 bits for comparison and make sure bits 11,12 are 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_range::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_range::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_range`]
module"]
#[doc(alias = "ADCREGS_RANGE")]
pub type AdcregsRange = crate::Reg<adcregs_range::AdcregsRangeSpec>;
#[doc = "This feature requires the range check interrupt bit to be enabled first. The user can decide which channel input is compared by programming the RangeCheck bit of the StepConfig Registers. It is up to software to sort through FIFO data to determine which channel data was out of range. Software should only use LSB 10 bits for comparison and make sure bits 11,12 are 0."]
pub mod adcregs_range;
#[doc = "ADCREGS_MISC (rw) register accessor: Spare inputs of the AFE are driven by this register, spare outputs from the AFE are read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_misc`]
module"]
#[doc(alias = "ADCREGS_MISC")]
pub type AdcregsMisc = crate::Reg<adcregs_misc::AdcregsMiscSpec>;
#[doc = "Spare inputs of the AFE are driven by this register, spare outputs from the AFE are read."]
pub mod adcregs_misc;
#[doc = "ADCREGS_STEPENABLE (rw) register accessor: Contains the enable bit for each step in the sequencer. When all steps are disabled, the FSM will stay in IDLE state. These bits can be enabled or disabled dynamically during operation. When a write to this register occurs during operational mode, the HW will make sure the new settings are updated after the END_OF_SEQUENCE event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepenable`]
module"]
#[doc(alias = "ADCREGS_STEPENABLE")]
pub type AdcregsStepenable = crate::Reg<adcregs_stepenable::AdcregsStepenableSpec>;
#[doc = "Contains the enable bit for each step in the sequencer. When all steps are disabled, the FSM will stay in IDLE state. These bits can be enabled or disabled dynamically during operation. When a write to this register occurs during operational mode, the HW will make sure the new settings are updated after the END_OF_SEQUENCE event"]
pub mod adcregs_stepenable;
#[doc = "ADCREGS_FIFO0WC (rw) register accessor: FIFO word count status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo0wc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo0wc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo0wc`]
module"]
#[doc(alias = "ADCREGS_FIFO0WC")]
pub type AdcregsFifo0wc = crate::Reg<adcregs_fifo0wc::AdcregsFifo0wcSpec>;
#[doc = "FIFO word count status"]
pub mod adcregs_fifo0wc;
#[doc = "ADCREGS_FIFO0THRESHOLD (rw) register accessor: FIFO threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo0threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo0threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo0threshold`]
module"]
#[doc(alias = "ADCREGS_FIFO0THRESHOLD")]
pub type AdcregsFifo0threshold = crate::Reg<adcregs_fifo0threshold::AdcregsFifo0thresholdSpec>;
#[doc = "FIFO threshold"]
pub mod adcregs_fifo0threshold;
#[doc = "ADCREGS_FIFO0DMAREQ (rw) register accessor: dma request.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo0dmareq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo0dmareq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo0dmareq`]
module"]
#[doc(alias = "ADCREGS_FIFO0DMAREQ")]
pub type AdcregsFifo0dmareq = crate::Reg<adcregs_fifo0dmareq::AdcregsFifo0dmareqSpec>;
#[doc = "dma request."]
pub mod adcregs_fifo0dmareq;
#[doc = "ADCREGS_FIFO1WC (rw) register accessor: FIFO word count status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo1wc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo1wc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo1wc`]
module"]
#[doc(alias = "ADCREGS_FIFO1WC")]
pub type AdcregsFifo1wc = crate::Reg<adcregs_fifo1wc::AdcregsFifo1wcSpec>;
#[doc = "FIFO word count status"]
pub mod adcregs_fifo1wc;
#[doc = "ADCREGS_FIFO1THRESHOLD (rw) register accessor: FIFO threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo1threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo1threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo1threshold`]
module"]
#[doc(alias = "ADCREGS_FIFO1THRESHOLD")]
pub type AdcregsFifo1threshold = crate::Reg<adcregs_fifo1threshold::AdcregsFifo1thresholdSpec>;
#[doc = "FIFO threshold"]
pub mod adcregs_fifo1threshold;
#[doc = "ADCREGS_FIFO1DMAREQ (rw) register accessor: dma request.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo1dmareq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo1dmareq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo1dmareq`]
module"]
#[doc(alias = "ADCREGS_FIFO1DMAREQ")]
pub type AdcregsFifo1dmareq = crate::Reg<adcregs_fifo1dmareq::AdcregsFifo1dmareqSpec>;
#[doc = "dma request."]
pub mod adcregs_fifo1dmareq;
#[doc = "ADCREGS_FIFO0DATA (rw) register accessor: A read from this register will auto increment the FIFO read pointer. If you read when FIFO is empty,it will trigger an underflow interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo0data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo0data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo0data`]
module"]
#[doc(alias = "ADCREGS_FIFO0DATA")]
pub type AdcregsFifo0data = crate::Reg<adcregs_fifo0data::AdcregsFifo0dataSpec>;
#[doc = "A read from this register will auto increment the FIFO read pointer. If you read when FIFO is empty,it will trigger an underflow interrupt"]
pub mod adcregs_fifo0data;
#[doc = "ADCREGS_FIFO1DATA (rw) register accessor: A read from this register will auto increment the FIFO read pointer. If you read when FIFO is empty,it will trigger an underflow interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_fifo1data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_fifo1data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_fifo1data`]
module"]
#[doc(alias = "ADCREGS_FIFO1DATA")]
pub type AdcregsFifo1data = crate::Reg<adcregs_fifo1data::AdcregsFifo1dataSpec>;
#[doc = "A read from this register will auto increment the FIFO read pointer. If you read when FIFO is empty,it will trigger an underflow interrupt"]
pub mod adcregs_fifo1data;
#[doc = "ADCREGS_EOI (rw) register accessor: The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_eoi`]
module"]
#[doc(alias = "ADCREGS_EOI")]
pub type AdcregsEoi = crate::Reg<adcregs_eoi::AdcregsEoiSpec>;
#[doc = "The End of Interrupt (EOI) MISC Register allows the CPU to acknowledge completion of an interrupt by writing to the EOI for MISC interrupt sources. An eoi_write signal will be generated and another interrupt will be triggered if interrupt sources remain. This register will be reset one cycle after it has been written to."]
pub mod adcregs_eoi;
#[doc = "ADCREGS_STATUS_RAW (rw) register accessor: The IRQ_STATUS_RAW register allows the adc12 interrupt sources to be manually set when writing a 1 to a specific bit. Write 0: No action Write 1: Set event Read 0: No event pending Read 1: Event pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_status_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_status_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_status_raw`]
module"]
#[doc(alias = "ADCREGS_STATUS_RAW")]
pub type AdcregsStatusRaw = crate::Reg<adcregs_status_raw::AdcregsStatusRawSpec>;
#[doc = "The IRQ_STATUS_RAW register allows the adc12 interrupt sources to be manually set when writing a 1 to a specific bit. Write 0: No action Write 1: Set event Read 0: No event pending Read 1: Event pending"]
pub mod adcregs_status_raw;
#[doc = "ADCREGS_STATUS (rw) register accessor: The IRQ_STATUS register allows the adc12 interrupt sources to be manually cleared when writing a 1 to a specific bit. Write 0: No action Write 1: Clear event Read 0: No event pending Read 1: Event pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_status`]
module"]
#[doc(alias = "ADCREGS_STATUS")]
pub type AdcregsStatus = crate::Reg<adcregs_status::AdcregsStatusSpec>;
#[doc = "The IRQ_STATUS register allows the adc12 interrupt sources to be manually cleared when writing a 1 to a specific bit. Write 0: No action Write 1: Clear event Read 0: No event pending Read 1: Event pending"]
pub mod adcregs_status;
#[doc = "ADCREGS_ENABLE_SET (rw) register accessor: The IRQ_ENABLE_SET register allows the adc12 interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_enable_set`]
module"]
#[doc(alias = "ADCREGS_ENABLE_SET")]
pub type AdcregsEnableSet = crate::Reg<adcregs_enable_set::AdcregsEnableSetSpec>;
#[doc = "The IRQ_ENABLE_SET register allows the adc12 interrupt sources to be manually enabled when writing a 1 to a specific bit. Write 0: No action Write 1: Enable event Read 0: Event is disabled Read 1: Event is enabled"]
pub mod adcregs_enable_set;
#[doc = "ADCREGS_ENABLE_CLR (rw) register accessor: The IRQ_ENABLE_CLR register allows the adc12 interrupt sources to be manually disabled when writing a 1 to a specific bit. Write 0: No action Write 1: Disable event Read 0: Event is disabled Read 1: Event is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_enable_clr`]
module"]
#[doc(alias = "ADCREGS_ENABLE_CLR")]
pub type AdcregsEnableClr = crate::Reg<adcregs_enable_clr::AdcregsEnableClrSpec>;
#[doc = "The IRQ_ENABLE_CLR register allows the adc12 interrupt sources to be manually disabled when writing a 1 to a specific bit. Write 0: No action Write 1: Disable event Read 0: Event is disabled Read 1: Event is enabled"]
pub mod adcregs_enable_clr;
#[doc = "ADCREGS_STEPCONFIG_0 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_0`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_0")]
pub type AdcregsStepconfig0 = crate::Reg<adcregs_stepconfig_0::AdcregsStepconfig0Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_0;
#[doc = "ADCREGS_STEPDELAY_0 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_0`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_0")]
pub type AdcregsStepdelay0 = crate::Reg<adcregs_stepdelay_0::AdcregsStepdelay0Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_0;
#[doc = "ADCREGS_STEPCONFIG_1 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_1`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_1")]
pub type AdcregsStepconfig1 = crate::Reg<adcregs_stepconfig_1::AdcregsStepconfig1Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_1;
#[doc = "ADCREGS_STEPDELAY_1 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_1`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_1")]
pub type AdcregsStepdelay1 = crate::Reg<adcregs_stepdelay_1::AdcregsStepdelay1Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_1;
#[doc = "ADCREGS_STEPCONFIG_2 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_2`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_2")]
pub type AdcregsStepconfig2 = crate::Reg<adcregs_stepconfig_2::AdcregsStepconfig2Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_2;
#[doc = "ADCREGS_STEPDELAY_2 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_2`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_2")]
pub type AdcregsStepdelay2 = crate::Reg<adcregs_stepdelay_2::AdcregsStepdelay2Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_2;
#[doc = "ADCREGS_STEPCONFIG_3 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_3`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_3")]
pub type AdcregsStepconfig3 = crate::Reg<adcregs_stepconfig_3::AdcregsStepconfig3Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_3;
#[doc = "ADCREGS_STEPDELAY_3 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_3`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_3")]
pub type AdcregsStepdelay3 = crate::Reg<adcregs_stepdelay_3::AdcregsStepdelay3Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_3;
#[doc = "ADCREGS_STEPCONFIG_4 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_4`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_4")]
pub type AdcregsStepconfig4 = crate::Reg<adcregs_stepconfig_4::AdcregsStepconfig4Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_4;
#[doc = "ADCREGS_STEPDELAY_4 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_4`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_4")]
pub type AdcregsStepdelay4 = crate::Reg<adcregs_stepdelay_4::AdcregsStepdelay4Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_4;
#[doc = "ADCREGS_STEPCONFIG_5 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_5`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_5")]
pub type AdcregsStepconfig5 = crate::Reg<adcregs_stepconfig_5::AdcregsStepconfig5Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_5;
#[doc = "ADCREGS_STEPDELAY_5 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_5`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_5")]
pub type AdcregsStepdelay5 = crate::Reg<adcregs_stepdelay_5::AdcregsStepdelay5Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_5;
#[doc = "ADCREGS_STEPCONFIG_6 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_6`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_6")]
pub type AdcregsStepconfig6 = crate::Reg<adcregs_stepconfig_6::AdcregsStepconfig6Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_6;
#[doc = "ADCREGS_STEPDELAY_6 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_6`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_6")]
pub type AdcregsStepdelay6 = crate::Reg<adcregs_stepdelay_6::AdcregsStepdelay6Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_6;
#[doc = "ADCREGS_STEPCONFIG_7 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_7`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_7")]
pub type AdcregsStepconfig7 = crate::Reg<adcregs_stepconfig_7::AdcregsStepconfig7Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_7;
#[doc = "ADCREGS_STEPDELAY_7 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_7`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_7")]
pub type AdcregsStepdelay7 = crate::Reg<adcregs_stepdelay_7::AdcregsStepdelay7Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_7;
#[doc = "ADCREGS_STEPCONFIG_8 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_8`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_8")]
pub type AdcregsStepconfig8 = crate::Reg<adcregs_stepconfig_8::AdcregsStepconfig8Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_8;
#[doc = "ADCREGS_STEPDELAY_8 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_8`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_8")]
pub type AdcregsStepdelay8 = crate::Reg<adcregs_stepdelay_8::AdcregsStepdelay8Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_8;
#[doc = "ADCREGS_STEPCONFIG_9 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_9`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_9")]
pub type AdcregsStepconfig9 = crate::Reg<adcregs_stepconfig_9::AdcregsStepconfig9Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_9;
#[doc = "ADCREGS_STEPDELAY_9 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_9`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_9")]
pub type AdcregsStepdelay9 = crate::Reg<adcregs_stepdelay_9::AdcregsStepdelay9Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_9;
#[doc = "ADCREGS_STEPCONFIG_10 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_10`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_10")]
pub type AdcregsStepconfig10 = crate::Reg<adcregs_stepconfig_10::AdcregsStepconfig10Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_10;
#[doc = "ADCREGS_STEPDELAY_10 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_10`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_10")]
pub type AdcregsStepdelay10 = crate::Reg<adcregs_stepdelay_10::AdcregsStepdelay10Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_10;
#[doc = "ADCREGS_STEPCONFIG_11 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_11`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_11")]
pub type AdcregsStepconfig11 = crate::Reg<adcregs_stepconfig_11::AdcregsStepconfig11Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_11;
#[doc = "ADCREGS_STEPDELAY_11 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_11`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_11")]
pub type AdcregsStepdelay11 = crate::Reg<adcregs_stepdelay_11::AdcregsStepdelay11Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_11;
#[doc = "ADCREGS_STEPCONFIG_12 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_12`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_12")]
pub type AdcregsStepconfig12 = crate::Reg<adcregs_stepconfig_12::AdcregsStepconfig12Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_12;
#[doc = "ADCREGS_STEPDELAY_12 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_12`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_12")]
pub type AdcregsStepdelay12 = crate::Reg<adcregs_stepdelay_12::AdcregsStepdelay12Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_12;
#[doc = "ADCREGS_STEPCONFIG_13 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_13`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_13")]
pub type AdcregsStepconfig13 = crate::Reg<adcregs_stepconfig_13::AdcregsStepconfig13Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_13;
#[doc = "ADCREGS_STEPDELAY_13 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_13`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_13")]
pub type AdcregsStepdelay13 = crate::Reg<adcregs_stepdelay_13::AdcregsStepdelay13Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_13;
#[doc = "ADCREGS_STEPCONFIG_14 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_14`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_14")]
pub type AdcregsStepconfig14 = crate::Reg<adcregs_stepconfig_14::AdcregsStepconfig14Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_14;
#[doc = "ADCREGS_STEPDELAY_14 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_14`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_14")]
pub type AdcregsStepdelay14 = crate::Reg<adcregs_stepdelay_14::AdcregsStepdelay14Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_14;
#[doc = "ADCREGS_STEPCONFIG_15 (rw) register accessor: The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepconfig_15`]
module"]
#[doc(alias = "ADCREGS_STEPCONFIG_15")]
pub type AdcregsStepconfig15 = crate::Reg<adcregs_stepconfig_15::AdcregsStepconfig15Spec>;
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur."]
pub mod adcregs_stepconfig_15;
#[doc = "ADCREGS_STEPDELAY_15 (rw) register accessor: Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcregs_stepdelay_15`]
module"]
#[doc(alias = "ADCREGS_STEPDELAY_15")]
pub type AdcregsStepdelay15 = crate::Reg<adcregs_stepdelay_15::AdcregsStepdelay15Spec>;
#[doc = "Controls number of clock periods to sample and delay"]
pub mod adcregs_stepdelay_15;
