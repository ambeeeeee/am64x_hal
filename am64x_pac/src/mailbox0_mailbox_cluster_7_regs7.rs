#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regs7_mailbox_id: Regs7MailboxId,
    _reserved1: [u8; 0x0c],
    regs7_mailbox_sysconfig: Regs7MailboxSysconfig,
    _reserved2: [u8; 0x2c],
    regs7_mailbox_message: Regs7MailboxMessage,
    _reserved3: [u8; 0x3c],
    regs7_mailbox_fifostatus: Regs7MailboxFifostatus,
    _reserved4: [u8; 0x3c],
    regs7_mailbox_msgstatus: Regs7MailboxMsgstatus,
    _reserved5: [u8; 0x3c],
    regs7_mailbox_irq_status_raw: Regs7MailboxIrqStatusRaw,
    regs7_mailbox_irq_status_clr: Regs7MailboxIrqStatusClr,
    regs7_mailbox_irq_enable_set: Regs7MailboxIrqEnableSet,
    regs7_mailbox_irq_enable_clr: Regs7MailboxIrqEnableClr,
    _reserved9: [u8; 0x30],
    regs7_mailbox_irq_eoi: Regs7MailboxIrqEoi,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
    #[inline(always)]
    pub const fn regs7_mailbox_id(&self) -> &Regs7MailboxId {
        &self.regs7_mailbox_id
    }
    #[doc = "0x10 - This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset."]
    #[inline(always)]
    pub const fn regs7_mailbox_sysconfig(&self) -> &Regs7MailboxSysconfig {
        &self.regs7_mailbox_sysconfig
    }
    #[doc = "0x40 - The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue."]
    #[inline(always)]
    pub const fn regs7_mailbox_message(&self) -> &Regs7MailboxMessage {
        &self.regs7_mailbox_message
    }
    #[doc = "0x80 - The FIFO status register has the status of the Mailbox\\[a\\]
FIFO"]
    #[inline(always)]
    pub const fn regs7_mailbox_fifostatus(&self) -> &Regs7MailboxFifostatus {
        &self.regs7_mailbox_fifostatus
    }
    #[doc = "0xc0 - The message status register has the status of the messages in Mailbox\\[a\\]"]
    #[inline(always)]
    pub const fn regs7_mailbox_msgstatus(&self) -> &Regs7MailboxMsgstatus {
        &self.regs7_mailbox_msgstatus
    }
    #[doc = "0x100 - The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs7_mailbox_irq_status_raw(&self) -> &Regs7MailboxIrqStatusRaw {
        &self.regs7_mailbox_irq_status_raw
    }
    #[doc = "0x104 - The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs7_mailbox_irq_status_clr(&self) -> &Regs7MailboxIrqStatusClr {
        &self.regs7_mailbox_irq_status_clr
    }
    #[doc = "0x108 - The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs7_mailbox_irq_enable_set(&self) -> &Regs7MailboxIrqEnableSet {
        &self.regs7_mailbox_irq_enable_set
    }
    #[doc = "0x10c - The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs7_mailbox_irq_enable_clr(&self) -> &Regs7MailboxIrqEnableClr {
        &self.regs7_mailbox_irq_enable_clr
    }
    #[doc = "0x140 - This is the EOI register with which the software is enabled to do the interrupt clearance."]
    #[inline(always)]
    pub const fn regs7_mailbox_irq_eoi(&self) -> &Regs7MailboxIrqEoi {
        &self.regs7_mailbox_irq_eoi
    }
}
#[doc = "REGS7_MAILBOX_ID (rw) register accessor: This is the standard TI peripheral ID register that exists at address 0 in the peripheral space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_id`]
module"]
#[doc(alias = "REGS7_MAILBOX_ID")]
pub type Regs7MailboxId = crate::Reg<regs7_mailbox_id::Regs7MailboxIdSpec>;
#[doc = "This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
pub mod regs7_mailbox_id;
#[doc = "REGS7_MAILBOX_SYSCONFIG (rw) register accessor: This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_sysconfig`]
module"]
#[doc(alias = "REGS7_MAILBOX_SYSCONFIG")]
pub type Regs7MailboxSysconfig = crate::Reg<regs7_mailbox_sysconfig::Regs7MailboxSysconfigSpec>;
#[doc = "This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset."]
pub mod regs7_mailbox_sysconfig;
#[doc = "REGS7_MAILBOX_MESSAGE (rw) register accessor: The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_message::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_message::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_message`]
module"]
#[doc(alias = "REGS7_MAILBOX_MESSAGE")]
pub type Regs7MailboxMessage = crate::Reg<regs7_mailbox_message::Regs7MailboxMessageSpec>;
#[doc = "The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue."]
pub mod regs7_mailbox_message;
#[doc = "REGS7_MAILBOX_FIFOSTATUS (rw) register accessor: The FIFO status register has the status of the Mailbox\\[a\\]
FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_fifostatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_fifostatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_fifostatus`]
module"]
#[doc(alias = "REGS7_MAILBOX_FIFOSTATUS")]
pub type Regs7MailboxFifostatus = crate::Reg<regs7_mailbox_fifostatus::Regs7MailboxFifostatusSpec>;
#[doc = "The FIFO status register has the status of the Mailbox\\[a\\]
FIFO"]
pub mod regs7_mailbox_fifostatus;
#[doc = "REGS7_MAILBOX_MSGSTATUS (rw) register accessor: The message status register has the status of the messages in Mailbox\\[a\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_msgstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_msgstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_msgstatus`]
module"]
#[doc(alias = "REGS7_MAILBOX_MSGSTATUS")]
pub type Regs7MailboxMsgstatus = crate::Reg<regs7_mailbox_msgstatus::Regs7MailboxMsgstatusSpec>;
#[doc = "The message status register has the status of the messages in Mailbox\\[a\\]"]
pub mod regs7_mailbox_msgstatus;
#[doc = "REGS7_MAILBOX_IRQ_EOI (rw) register accessor: This is the EOI register with which the software is enabled to do the interrupt clearance.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_irq_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_irq_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_irq_eoi`]
module"]
#[doc(alias = "REGS7_MAILBOX_IRQ_EOI")]
pub type Regs7MailboxIrqEoi = crate::Reg<regs7_mailbox_irq_eoi::Regs7MailboxIrqEoiSpec>;
#[doc = "This is the EOI register with which the software is enabled to do the interrupt clearance."]
pub mod regs7_mailbox_irq_eoi;
#[doc = "REGS7_MAILBOX_IRQ_STATUS_RAW (rw) register accessor: The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_irq_status_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_irq_status_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_irq_status_raw`]
module"]
#[doc(alias = "REGS7_MAILBOX_IRQ_STATUS_RAW")]
pub type Regs7MailboxIrqStatusRaw =
    crate::Reg<regs7_mailbox_irq_status_raw::Regs7MailboxIrqStatusRawSpec>;
#[doc = "The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect."]
pub mod regs7_mailbox_irq_status_raw;
#[doc = "REGS7_MAILBOX_IRQ_STATUS_CLR (rw) register accessor: The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_irq_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_irq_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_irq_status_clr`]
module"]
#[doc(alias = "REGS7_MAILBOX_IRQ_STATUS_CLR")]
pub type Regs7MailboxIrqStatusClr =
    crate::Reg<regs7_mailbox_irq_status_clr::Regs7MailboxIrqStatusClrSpec>;
#[doc = "The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect."]
pub mod regs7_mailbox_irq_status_clr;
#[doc = "REGS7_MAILBOX_IRQ_ENABLE_SET (rw) register accessor: The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_irq_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_irq_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_irq_enable_set`]
module"]
#[doc(alias = "REGS7_MAILBOX_IRQ_ENABLE_SET")]
pub type Regs7MailboxIrqEnableSet =
    crate::Reg<regs7_mailbox_irq_enable_set::Regs7MailboxIrqEnableSetSpec>;
#[doc = "The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect."]
pub mod regs7_mailbox_irq_enable_set;
#[doc = "REGS7_MAILBOX_IRQ_ENABLE_CLR (rw) register accessor: The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs7_mailbox_irq_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs7_mailbox_irq_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs7_mailbox_irq_enable_clr`]
module"]
#[doc(alias = "REGS7_MAILBOX_IRQ_ENABLE_CLR")]
pub type Regs7MailboxIrqEnableClr =
    crate::Reg<regs7_mailbox_irq_enable_clr::Regs7MailboxIrqEnableClrSpec>;
#[doc = "The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect."]
pub mod regs7_mailbox_irq_enable_clr;
