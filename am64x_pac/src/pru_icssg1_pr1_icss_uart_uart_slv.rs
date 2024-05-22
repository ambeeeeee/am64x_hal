#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_icss_uart__uart_slv__regs_rbr_tbr: Pr1IcssUart_UartSlv_RegsRbrTbr,
    pr1_icss_uart__uart_slv__regs_int_en: Pr1IcssUart_UartSlv_RegsIntEn,
    pr1_icss_uart__uart_slv__regs_int_fifo: Pr1IcssUart_UartSlv_RegsIntFifo,
    pr1_icss_uart__uart_slv__regs_lctr: Pr1IcssUart_UartSlv_RegsLctr,
    pr1_icss_uart__uart_slv__regs_mctr: Pr1IcssUart_UartSlv_RegsMctr,
    pr1_icss_uart__uart_slv__regs_lsr1: Pr1IcssUart_UartSlv_RegsLsr1,
    pr1_icss_uart__uart_slv__regs_msr: Pr1IcssUart_UartSlv_RegsMsr,
    pr1_icss_uart__uart_slv__regs_scratch: Pr1IcssUart_UartSlv_RegsScratch,
    pr1_icss_uart__uart_slv__regs_divlsb: Pr1IcssUart_UartSlv_RegsDivlsb,
    pr1_icss_uart__uart_slv__regs_divmsb: Pr1IcssUart_UartSlv_RegsDivmsb,
    pr1_icss_uart__uart_slv__regs_pid: Pr1IcssUart_UartSlv_RegsPid,
    _reserved11: [u8; 0x04],
    pr1_icss_uart__uart_slv__regs_pwr: Pr1IcssUart_UartSlv_RegsPwr,
    pr1_icss_uart__uart_slv__regs_mode: Pr1IcssUart_UartSlv_RegsMode,
}
impl RegisterBlock {
    #[doc = "0x00 - RBR_TBR Registers"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_rbr_tbr(&self) -> &Pr1IcssUart_UartSlv_RegsRbrTbr {
        &self.pr1_icss_uart__uart_slv__regs_rbr_tbr
    }
    #[doc = "0x04 - UART Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_int_en(&self) -> &Pr1IcssUart_UartSlv_RegsIntEn {
        &self.pr1_icss_uart__uart_slv__regs_int_en
    }
    #[doc = "0x08 - Interrupt Identification Register / FIFO Control Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_int_fifo(&self) -> &Pr1IcssUart_UartSlv_RegsIntFifo {
        &self.pr1_icss_uart__uart_slv__regs_int_fifo
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_lctr(&self) -> &Pr1IcssUart_UartSlv_RegsLctr {
        &self.pr1_icss_uart__uart_slv__regs_lctr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_mctr(&self) -> &Pr1IcssUart_UartSlv_RegsMctr {
        &self.pr1_icss_uart__uart_slv__regs_mctr
    }
    #[doc = "0x14 - Line Status Register1"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_lsr1(&self) -> &Pr1IcssUart_UartSlv_RegsLsr1 {
        &self.pr1_icss_uart__uart_slv__regs_lsr1
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_msr(&self) -> &Pr1IcssUart_UartSlv_RegsMsr {
        &self.pr1_icss_uart__uart_slv__regs_msr
    }
    #[doc = "0x1c - UART Scratch Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_scratch(&self) -> &Pr1IcssUart_UartSlv_RegsScratch {
        &self.pr1_icss_uart__uart_slv__regs_scratch
    }
    #[doc = "0x20 - UART Divisor Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_divlsb(&self) -> &Pr1IcssUart_UartSlv_RegsDivlsb {
        &self.pr1_icss_uart__uart_slv__regs_divlsb
    }
    #[doc = "0x24 - UART Divisor Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_divmsb(&self) -> &Pr1IcssUart_UartSlv_RegsDivmsb {
        &self.pr1_icss_uart__uart_slv__regs_divmsb
    }
    #[doc = "0x28 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_pid(&self) -> &Pr1IcssUart_UartSlv_RegsPid {
        &self.pr1_icss_uart__uart_slv__regs_pid
    }
    #[doc = "0x30 - UART PowerManagement and Emulation Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_pwr(&self) -> &Pr1IcssUart_UartSlv_RegsPwr {
        &self.pr1_icss_uart__uart_slv__regs_pwr
    }
    #[doc = "0x34 - UART Mode Definition Register"]
    #[inline(always)]
    pub const fn pr1_icss_uart__uart_slv__regs_mode(&self) -> &Pr1IcssUart_UartSlv_RegsMode {
        &self.pr1_icss_uart__uart_slv__regs_mode
    }
}
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_RBR_TBR (rw) register accessor: RBR_TBR Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_rbr_tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_rbr_tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_rbr_tbr`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_RBR_TBR")]
pub type Pr1IcssUart_UartSlv_RegsRbrTbr =
    crate::Reg<pr1_icss_uart__uart_slv__regs_rbr_tbr::Pr1IcssUart_UartSlv_RegsRbrTbrSpec>;
#[doc = "RBR_TBR Registers"]
pub mod pr1_icss_uart__uart_slv__regs_rbr_tbr;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_INT_EN (rw) register accessor: UART Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_int_en`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_INT_EN")]
pub type Pr1IcssUart_UartSlv_RegsIntEn =
    crate::Reg<pr1_icss_uart__uart_slv__regs_int_en::Pr1IcssUart_UartSlv_RegsIntEnSpec>;
#[doc = "UART Interrupt Enable Register"]
pub mod pr1_icss_uart__uart_slv__regs_int_en;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_INT_FIFO (rw) register accessor: Interrupt Identification Register / FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_int_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_int_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_int_fifo`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_INT_FIFO")]
pub type Pr1IcssUart_UartSlv_RegsIntFifo =
    crate::Reg<pr1_icss_uart__uart_slv__regs_int_fifo::Pr1IcssUart_UartSlv_RegsIntFifoSpec>;
#[doc = "Interrupt Identification Register / FIFO Control Register"]
pub mod pr1_icss_uart__uart_slv__regs_int_fifo;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_LCTR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_lctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_lctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_lctr`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_LCTR")]
pub type Pr1IcssUart_UartSlv_RegsLctr =
    crate::Reg<pr1_icss_uart__uart_slv__regs_lctr::Pr1IcssUart_UartSlv_RegsLctrSpec>;
#[doc = "Line Control Register"]
pub mod pr1_icss_uart__uart_slv__regs_lctr;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_MCTR (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_mctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_mctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_mctr`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_MCTR")]
pub type Pr1IcssUart_UartSlv_RegsMctr =
    crate::Reg<pr1_icss_uart__uart_slv__regs_mctr::Pr1IcssUart_UartSlv_RegsMctrSpec>;
#[doc = "Modem Control Register"]
pub mod pr1_icss_uart__uart_slv__regs_mctr;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_LSR1 (rw) register accessor: Line Status Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_lsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_lsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_lsr1`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_LSR1")]
pub type Pr1IcssUart_UartSlv_RegsLsr1 =
    crate::Reg<pr1_icss_uart__uart_slv__regs_lsr1::Pr1IcssUart_UartSlv_RegsLsr1Spec>;
#[doc = "Line Status Register1"]
pub mod pr1_icss_uart__uart_slv__regs_lsr1;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_MSR (rw) register accessor: Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_msr`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_MSR")]
pub type Pr1IcssUart_UartSlv_RegsMsr =
    crate::Reg<pr1_icss_uart__uart_slv__regs_msr::Pr1IcssUart_UartSlv_RegsMsrSpec>;
#[doc = "Modem Status Register"]
pub mod pr1_icss_uart__uart_slv__regs_msr;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_SCRATCH (rw) register accessor: UART Scratch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_scratch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_scratch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_scratch`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_SCRATCH")]
pub type Pr1IcssUart_UartSlv_RegsScratch =
    crate::Reg<pr1_icss_uart__uart_slv__regs_scratch::Pr1IcssUart_UartSlv_RegsScratchSpec>;
#[doc = "UART Scratch Register"]
pub mod pr1_icss_uart__uart_slv__regs_scratch;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_DIVLSB (rw) register accessor: UART Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_divlsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_divlsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_divlsb`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_DIVLSB")]
pub type Pr1IcssUart_UartSlv_RegsDivlsb =
    crate::Reg<pr1_icss_uart__uart_slv__regs_divlsb::Pr1IcssUart_UartSlv_RegsDivlsbSpec>;
#[doc = "UART Divisor Register"]
pub mod pr1_icss_uart__uart_slv__regs_divlsb;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_DIVMSB (rw) register accessor: UART Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_divmsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_divmsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_divmsb`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_DIVMSB")]
pub type Pr1IcssUart_UartSlv_RegsDivmsb =
    crate::Reg<pr1_icss_uart__uart_slv__regs_divmsb::Pr1IcssUart_UartSlv_RegsDivmsbSpec>;
#[doc = "UART Divisor Register"]
pub mod pr1_icss_uart__uart_slv__regs_divmsb;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_PID (rw) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_pid`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_PID")]
pub type Pr1IcssUart_UartSlv_RegsPid =
    crate::Reg<pr1_icss_uart__uart_slv__regs_pid::Pr1IcssUart_UartSlv_RegsPidSpec>;
#[doc = "Peripheral ID Register"]
pub mod pr1_icss_uart__uart_slv__regs_pid;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_PWR (rw) register accessor: UART PowerManagement and Emulation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_pwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_pwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_pwr`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_PWR")]
pub type Pr1IcssUart_UartSlv_RegsPwr =
    crate::Reg<pr1_icss_uart__uart_slv__regs_pwr::Pr1IcssUart_UartSlv_RegsPwrSpec>;
#[doc = "UART PowerManagement and Emulation Register"]
pub mod pr1_icss_uart__uart_slv__regs_pwr;
#[doc = "PR1_ICSS_UART__UART_SLV__REGS_MODE (rw) register accessor: UART Mode Definition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_uart__uart_slv__regs_mode`]
module"]
#[doc(alias = "PR1_ICSS_UART__UART_SLV__REGS_MODE")]
pub type Pr1IcssUart_UartSlv_RegsMode =
    crate::Reg<pr1_icss_uart__uart_slv__regs_mode::Pr1IcssUart_UartSlv_RegsModeSpec>;
#[doc = "UART Mode Definition Register"]
pub mod pr1_icss_uart__uart_slv__regs_mode;
