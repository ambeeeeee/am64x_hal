#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regs6_mailbox_id: Regs6MailboxId,
    _reserved1: [u8; 0x0c],
    regs6_mailbox_sysconfig: Regs6MailboxSysconfig,
    _reserved2: [u8; 0x2c],
    regs6_mailbox_message: Regs6MailboxMessage,
    _reserved3: [u8; 0x3c],
    regs6_mailbox_fifostatus: Regs6MailboxFifostatus,
    _reserved4: [u8; 0x3c],
    regs6_mailbox_msgstatus: Regs6MailboxMsgstatus,
    _reserved5: [u8; 0x3c],
    regs6_mailbox_irq_status_raw: Regs6MailboxIrqStatusRaw,
    regs6_mailbox_irq_status_clr: Regs6MailboxIrqStatusClr,
    regs6_mailbox_irq_enable_set: Regs6MailboxIrqEnableSet,
    regs6_mailbox_irq_enable_clr: Regs6MailboxIrqEnableClr,
    _reserved9: [u8; 0x30],
    regs6_mailbox_irq_eoi: Regs6MailboxIrqEoi,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
    #[inline(always)]
    pub const fn regs6_mailbox_id(&self) -> &Regs6MailboxId {
        &self.regs6_mailbox_id
    }
    #[doc = "0x10 - This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset."]
    #[inline(always)]
    pub const fn regs6_mailbox_sysconfig(&self) -> &Regs6MailboxSysconfig {
        &self.regs6_mailbox_sysconfig
    }
    #[doc = "0x40 - The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue."]
    #[inline(always)]
    pub const fn regs6_mailbox_message(&self) -> &Regs6MailboxMessage {
        &self.regs6_mailbox_message
    }
    #[doc = "0x80 - The FIFO status register has the status of the Mailbox\\[a\\]
FIFO"]
    #[inline(always)]
    pub const fn regs6_mailbox_fifostatus(&self) -> &Regs6MailboxFifostatus {
        &self.regs6_mailbox_fifostatus
    }
    #[doc = "0xc0 - The message status register has the status of the messages in Mailbox\\[a\\]"]
    #[inline(always)]
    pub const fn regs6_mailbox_msgstatus(&self) -> &Regs6MailboxMsgstatus {
        &self.regs6_mailbox_msgstatus
    }
    #[doc = "0x100 - The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs6_mailbox_irq_status_raw(&self) -> &Regs6MailboxIrqStatusRaw {
        &self.regs6_mailbox_irq_status_raw
    }
    #[doc = "0x104 - The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs6_mailbox_irq_status_clr(&self) -> &Regs6MailboxIrqStatusClr {
        &self.regs6_mailbox_irq_status_clr
    }
    #[doc = "0x108 - The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs6_mailbox_irq_enable_set(&self) -> &Regs6MailboxIrqEnableSet {
        &self.regs6_mailbox_irq_enable_set
    }
    #[doc = "0x10c - The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs6_mailbox_irq_enable_clr(&self) -> &Regs6MailboxIrqEnableClr {
        &self.regs6_mailbox_irq_enable_clr
    }
    #[doc = "0x140 - This is the EOI register with which the software is enabled to do the interrupt clearance."]
    #[inline(always)]
    pub const fn regs6_mailbox_irq_eoi(&self) -> &Regs6MailboxIrqEoi {
        &self.regs6_mailbox_irq_eoi
    }
}
#[doc = "REGS6_MAILBOX_ID (rw) register accessor: This is the standard TI peripheral ID register that exists at address 0 in the peripheral space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_id`]
module"]
#[doc(alias = "REGS6_MAILBOX_ID")]
pub type Regs6MailboxId = crate::Reg<regs6_mailbox_id::Regs6MailboxIdSpec>;
#[doc = "This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
pub mod regs6_mailbox_id;
#[doc = "REGS6_MAILBOX_SYSCONFIG (rw) register accessor: This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_sysconfig`]
module"]
#[doc(alias = "REGS6_MAILBOX_SYSCONFIG")]
pub type Regs6MailboxSysconfig = crate::Reg<regs6_mailbox_sysconfig::Regs6MailboxSysconfigSpec>;
#[doc = "This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset."]
pub mod regs6_mailbox_sysconfig;
#[doc = "REGS6_MAILBOX_MESSAGE (rw) register accessor: The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_message::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_message::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_message`]
module"]
#[doc(alias = "REGS6_MAILBOX_MESSAGE")]
pub type Regs6MailboxMessage = crate::Reg<regs6_mailbox_message::Regs6MailboxMessageSpec>;
#[doc = "The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue."]
pub mod regs6_mailbox_message;
#[doc = "REGS6_MAILBOX_FIFOSTATUS (rw) register accessor: The FIFO status register has the status of the Mailbox\\[a\\]
FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_fifostatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_fifostatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_fifostatus`]
module"]
#[doc(alias = "REGS6_MAILBOX_FIFOSTATUS")]
pub type Regs6MailboxFifostatus = crate::Reg<regs6_mailbox_fifostatus::Regs6MailboxFifostatusSpec>;
#[doc = "The FIFO status register has the status of the Mailbox\\[a\\]
FIFO"]
pub mod regs6_mailbox_fifostatus;
#[doc = "REGS6_MAILBOX_MSGSTATUS (rw) register accessor: The message status register has the status of the messages in Mailbox\\[a\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_msgstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_msgstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_msgstatus`]
module"]
#[doc(alias = "REGS6_MAILBOX_MSGSTATUS")]
pub type Regs6MailboxMsgstatus = crate::Reg<regs6_mailbox_msgstatus::Regs6MailboxMsgstatusSpec>;
#[doc = "The message status register has the status of the messages in Mailbox\\[a\\]"]
pub mod regs6_mailbox_msgstatus;
#[doc = "REGS6_MAILBOX_IRQ_EOI (rw) register accessor: This is the EOI register with which the software is enabled to do the interrupt clearance.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_irq_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_irq_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_irq_eoi`]
module"]
#[doc(alias = "REGS6_MAILBOX_IRQ_EOI")]
pub type Regs6MailboxIrqEoi = crate::Reg<regs6_mailbox_irq_eoi::Regs6MailboxIrqEoiSpec>;
#[doc = "This is the EOI register with which the software is enabled to do the interrupt clearance."]
pub mod regs6_mailbox_irq_eoi;
#[doc = "REGS6_MAILBOX_IRQ_STATUS_RAW (rw) register accessor: The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_irq_status_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_irq_status_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_irq_status_raw`]
module"]
#[doc(alias = "REGS6_MAILBOX_IRQ_STATUS_RAW")]
pub type Regs6MailboxIrqStatusRaw =
    crate::Reg<regs6_mailbox_irq_status_raw::Regs6MailboxIrqStatusRawSpec>;
#[doc = "The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect."]
pub mod regs6_mailbox_irq_status_raw;
#[doc = "REGS6_MAILBOX_IRQ_STATUS_CLR (rw) register accessor: The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_irq_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_irq_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_irq_status_clr`]
module"]
#[doc(alias = "REGS6_MAILBOX_IRQ_STATUS_CLR")]
pub type Regs6MailboxIrqStatusClr =
    crate::Reg<regs6_mailbox_irq_status_clr::Regs6MailboxIrqStatusClrSpec>;
#[doc = "The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect."]
pub mod regs6_mailbox_irq_status_clr;
#[doc = "REGS6_MAILBOX_IRQ_ENABLE_SET (rw) register accessor: The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_irq_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_irq_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_irq_enable_set`]
module"]
#[doc(alias = "REGS6_MAILBOX_IRQ_ENABLE_SET")]
pub type Regs6MailboxIrqEnableSet =
    crate::Reg<regs6_mailbox_irq_enable_set::Regs6MailboxIrqEnableSetSpec>;
#[doc = "The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect."]
pub mod regs6_mailbox_irq_enable_set;
#[doc = "REGS6_MAILBOX_IRQ_ENABLE_CLR (rw) register accessor: The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs6_mailbox_irq_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs6_mailbox_irq_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs6_mailbox_irq_enable_clr`]
module"]
#[doc(alias = "REGS6_MAILBOX_IRQ_ENABLE_CLR")]
pub type Regs6MailboxIrqEnableClr =
    crate::Reg<regs6_mailbox_irq_enable_clr::Regs6MailboxIrqEnableClrSpec>;
#[doc = "The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect."]
pub mod regs6_mailbox_irq_enable_clr;
