#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timers_timer_setup_value: TimersTimerSetupValue,
    timers_timer_control: TimersTimerControl,
}
impl RegisterBlock {
    #[doc = "0x00 - This reprograms timer N with the written value. This number will be the number of ticks of the timer_clock before the timer expires, if timer N and the timer manager itself are both enabled via TM_CTRL and TIMER_CONTROL_N"]
    #[inline(always)]
    pub const fn timers_timer_setup_value(&self) -> &TimersTimerSetupValue {
        &self.timers_timer_setup_value
    }
    #[doc = "0x04 - Modifies the behavior of timer N with control signals below"]
    #[inline(always)]
    pub const fn timers_timer_control(&self) -> &TimersTimerControl {
        &self.timers_timer_control
    }
}
#[doc = "TIMERS_TIMER_SETUP_VALUE (rw) register accessor: This reprograms timer N with the written value. This number will be the number of ticks of the timer_clock before the timer expires, if timer N and the timer manager itself are both enabled via TM_CTRL and TIMER_CONTROL_N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timers_timer_setup_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timers_timer_setup_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timers_timer_setup_value`]
module"]
#[doc(alias = "TIMERS_TIMER_SETUP_VALUE")]
pub type TimersTimerSetupValue = crate::Reg<timers_timer_setup_value::TimersTimerSetupValueSpec>;
#[doc = "This reprograms timer N with the written value. This number will be the number of ticks of the timer_clock before the timer expires, if timer N and the timer manager itself are both enabled via TM_CTRL and TIMER_CONTROL_N"]
pub mod timers_timer_setup_value;
#[doc = "TIMERS_TIMER_CONTROL (rw) register accessor: Modifies the behavior of timer N with control signals below\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timers_timer_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timers_timer_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timers_timer_control`]
module"]
#[doc(alias = "TIMERS_TIMER_CONTROL")]
pub type TimersTimerControl = crate::Reg<timers_timer_control::TimersTimerControlSpec>;
#[doc = "Modifies the behavior of timer N with control signals below"]
pub mod timers_timer_control;
