#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config_tm_id: ConfigTmId,
    config_tm_cntl: ConfigTmCntl,
    config_timer_counter: ConfigTimerCounter,
    _reserved3: [u8; 0x94],
    config_timeout_status0: ConfigTimeoutStatus0,
    config_timeout_status1: ConfigTimeoutStatus1,
    config_timeout_status_bank0: ConfigTimeoutStatusBank0,
    _reserved6: [u8; 0x54],
    config_timer_status: ConfigTimerStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
    #[inline(always)]
    pub const fn config_tm_id(&self) -> &ConfigTmId {
        &self.config_tm_id
    }
    #[doc = "0x04 - This register controls the overall behavior of the timer manager module"]
    #[inline(always)]
    pub const fn config_tm_cntl(&self) -> &ConfigTmCntl {
        &self.config_tm_cntl
    }
    #[doc = "0x08 - This register contains the current counter value"]
    #[inline(always)]
    pub const fn config_timer_counter(&self) -> &ConfigTimerCounter {
        &self.config_timer_counter
    }
    #[doc = "0xa0 - This register should be read whenever the timer interrupt fires. It indicates the total number of timers that have expired and the ID of the first timer to expire. If NUM_EXPIRED_TIMERS is 1, this is the only MMR that needs to be read. Depending on the value of NUM_EXPIRED_TIMERS, either TIMEOUT_STATUS_1 or TIMEOUT_STATUS_BANK may be read by the software to avoid needing to read all 32 TIMEOUT_STATUS_N registers."]
    #[inline(always)]
    pub const fn config_timeout_status0(&self) -> &ConfigTimeoutStatus0 {
        &self.config_timeout_status0
    }
    #[doc = "0xa4 - This register contains the IDs of the second and third timers to expire. It is indended as a more efficient way of finding the first few timers to expire rather than needing to read the status of all 1024 timers."]
    #[inline(always)]
    pub const fn config_timeout_status1(&self) -> &ConfigTimeoutStatus1 {
        &self.config_timeout_status1
    }
    #[doc = "0xa8 - This register contains the status of each timer bank for banks 31:0. When servicing the timer interrupt, if the num_expired_timers bit is greater than 3, this register may be read to see which banks contain expired timers. The TIMER_STATUS_N registers corresponding to those banks may then be read."]
    #[inline(always)]
    pub const fn config_timeout_status_bank0(&self) -> &ConfigTimeoutStatusBank0 {
        &self.config_timeout_status_bank0
    }
    #[doc = "0x100 - Each bit is the timeout status for an individual timer. 0 = timer has not timed out or is disabled, 1 = timer has timed out"]
    #[inline(always)]
    pub const fn config_timer_status(&self) -> &ConfigTimerStatus {
        &self.config_timer_status
    }
}
#[doc = "CONFIG_TM_ID (rw) register accessor: This is the standard TI peripheral ID register that exists at address 0 in the peripheral space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_tm_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_tm_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_tm_id`]
module"]
#[doc(alias = "CONFIG_TM_ID")]
pub type ConfigTmId = crate::Reg<config_tm_id::ConfigTmIdSpec>;
#[doc = "This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
pub mod config_tm_id;
#[doc = "CONFIG_TM_CNTL (rw) register accessor: This register controls the overall behavior of the timer manager module\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_tm_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_tm_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_tm_cntl`]
module"]
#[doc(alias = "CONFIG_TM_CNTL")]
pub type ConfigTmCntl = crate::Reg<config_tm_cntl::ConfigTmCntlSpec>;
#[doc = "This register controls the overall behavior of the timer manager module"]
pub mod config_tm_cntl;
#[doc = "CONFIG_TIMER_COUNTER (rw) register accessor: This register contains the current counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timer_counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timer_counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_timer_counter`]
module"]
#[doc(alias = "CONFIG_TIMER_COUNTER")]
pub type ConfigTimerCounter = crate::Reg<config_timer_counter::ConfigTimerCounterSpec>;
#[doc = "This register contains the current counter value"]
pub mod config_timer_counter;
#[doc = "CONFIG_TIMEOUT_STATUS0 (rw) register accessor: This register should be read whenever the timer interrupt fires. It indicates the total number of timers that have expired and the ID of the first timer to expire. If NUM_EXPIRED_TIMERS is 1, this is the only MMR that needs to be read. Depending on the value of NUM_EXPIRED_TIMERS, either TIMEOUT_STATUS_1 or TIMEOUT_STATUS_BANK may be read by the software to avoid needing to read all 32 TIMEOUT_STATUS_N registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timeout_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timeout_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_timeout_status0`]
module"]
#[doc(alias = "CONFIG_TIMEOUT_STATUS0")]
pub type ConfigTimeoutStatus0 = crate::Reg<config_timeout_status0::ConfigTimeoutStatus0Spec>;
#[doc = "This register should be read whenever the timer interrupt fires. It indicates the total number of timers that have expired and the ID of the first timer to expire. If NUM_EXPIRED_TIMERS is 1, this is the only MMR that needs to be read. Depending on the value of NUM_EXPIRED_TIMERS, either TIMEOUT_STATUS_1 or TIMEOUT_STATUS_BANK may be read by the software to avoid needing to read all 32 TIMEOUT_STATUS_N registers."]
pub mod config_timeout_status0;
#[doc = "CONFIG_TIMEOUT_STATUS1 (rw) register accessor: This register contains the IDs of the second and third timers to expire. It is indended as a more efficient way of finding the first few timers to expire rather than needing to read the status of all 1024 timers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timeout_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timeout_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_timeout_status1`]
module"]
#[doc(alias = "CONFIG_TIMEOUT_STATUS1")]
pub type ConfigTimeoutStatus1 = crate::Reg<config_timeout_status1::ConfigTimeoutStatus1Spec>;
#[doc = "This register contains the IDs of the second and third timers to expire. It is indended as a more efficient way of finding the first few timers to expire rather than needing to read the status of all 1024 timers."]
pub mod config_timeout_status1;
#[doc = "CONFIG_TIMEOUT_STATUS_BANK0 (rw) register accessor: This register contains the status of each timer bank for banks 31:0. When servicing the timer interrupt, if the num_expired_timers bit is greater than 3, this register may be read to see which banks contain expired timers. The TIMER_STATUS_N registers corresponding to those banks may then be read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timeout_status_bank0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timeout_status_bank0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_timeout_status_bank0`]
module"]
#[doc(alias = "CONFIG_TIMEOUT_STATUS_BANK0")]
pub type ConfigTimeoutStatusBank0 =
    crate::Reg<config_timeout_status_bank0::ConfigTimeoutStatusBank0Spec>;
#[doc = "This register contains the status of each timer bank for banks 31:0. When servicing the timer interrupt, if the num_expired_timers bit is greater than 3, this register may be read to see which banks contain expired timers. The TIMER_STATUS_N registers corresponding to those banks may then be read."]
pub mod config_timeout_status_bank0;
#[doc = "CONFIG_TIMER_STATUS (rw) register accessor: Each bit is the timeout status for an individual timer. 0 = timer has not timed out or is disabled, 1 = timer has timed out\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_timer_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_timer_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_timer_status`]
module"]
#[doc(alias = "CONFIG_TIMER_STATUS")]
pub type ConfigTimerStatus = crate::Reg<config_timer_status::ConfigTimerStatusSpec>;
#[doc = "Each bit is the timeout status for an individual timer. 0 = timer has not timed out or is disabled, 1 = timer has timed out"]
pub mod config_timer_status;
