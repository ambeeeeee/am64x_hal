#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regs2_mailbox_id: Regs2MailboxId,
    _reserved1: [u8; 0x0c],
    regs2_mailbox_sysconfig: Regs2MailboxSysconfig,
    _reserved2: [u8; 0x2c],
    regs2_mailbox_message: Regs2MailboxMessage,
    _reserved3: [u8; 0x3c],
    regs2_mailbox_fifostatus: Regs2MailboxFifostatus,
    _reserved4: [u8; 0x3c],
    regs2_mailbox_msgstatus: Regs2MailboxMsgstatus,
    _reserved5: [u8; 0x3c],
    regs2_mailbox_irq_status_raw: Regs2MailboxIrqStatusRaw,
    regs2_mailbox_irq_status_clr: Regs2MailboxIrqStatusClr,
    regs2_mailbox_irq_enable_set: Regs2MailboxIrqEnableSet,
    regs2_mailbox_irq_enable_clr: Regs2MailboxIrqEnableClr,
    _reserved9: [u8; 0x30],
    regs2_mailbox_irq_eoi: Regs2MailboxIrqEoi,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
    #[inline(always)]
    pub const fn regs2_mailbox_id(&self) -> &Regs2MailboxId {
        &self.regs2_mailbox_id
    }
    #[doc = "0x10 - This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset."]
    #[inline(always)]
    pub const fn regs2_mailbox_sysconfig(&self) -> &Regs2MailboxSysconfig {
        &self.regs2_mailbox_sysconfig
    }
    #[doc = "0x40 - The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue."]
    #[inline(always)]
    pub const fn regs2_mailbox_message(&self) -> &Regs2MailboxMessage {
        &self.regs2_mailbox_message
    }
    #[doc = "0x80 - The FIFO status register has the status of the Mailbox\\[a\\]
FIFO"]
    #[inline(always)]
    pub const fn regs2_mailbox_fifostatus(&self) -> &Regs2MailboxFifostatus {
        &self.regs2_mailbox_fifostatus
    }
    #[doc = "0xc0 - The message status register has the status of the messages in Mailbox\\[a\\]"]
    #[inline(always)]
    pub const fn regs2_mailbox_msgstatus(&self) -> &Regs2MailboxMsgstatus {
        &self.regs2_mailbox_msgstatus
    }
    #[doc = "0x100 - The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs2_mailbox_irq_status_raw(&self) -> &Regs2MailboxIrqStatusRaw {
        &self.regs2_mailbox_irq_status_raw
    }
    #[doc = "0x104 - The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs2_mailbox_irq_status_clr(&self) -> &Regs2MailboxIrqStatusClr {
        &self.regs2_mailbox_irq_status_clr
    }
    #[doc = "0x108 - The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs2_mailbox_irq_enable_set(&self) -> &Regs2MailboxIrqEnableSet {
        &self.regs2_mailbox_irq_enable_set
    }
    #[doc = "0x10c - The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect."]
    #[inline(always)]
    pub const fn regs2_mailbox_irq_enable_clr(&self) -> &Regs2MailboxIrqEnableClr {
        &self.regs2_mailbox_irq_enable_clr
    }
    #[doc = "0x140 - This is the EOI register with which the software is enabled to do the interrupt clearance."]
    #[inline(always)]
    pub const fn regs2_mailbox_irq_eoi(&self) -> &Regs2MailboxIrqEoi {
        &self.regs2_mailbox_irq_eoi
    }
}
#[doc = "REGS2_MAILBOX_ID (rw) register accessor: This is the standard TI peripheral ID register that exists at address 0 in the peripheral space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_id`]
module"]
#[doc(alias = "REGS2_MAILBOX_ID")]
pub type Regs2MailboxId = crate::Reg<regs2_mailbox_id::Regs2MailboxIdSpec>;
#[doc = "This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
pub mod regs2_mailbox_id;
#[doc = "REGS2_MAILBOX_SYSCONFIG (rw) register accessor: This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_sysconfig`]
module"]
#[doc(alias = "REGS2_MAILBOX_SYSCONFIG")]
pub type Regs2MailboxSysconfig = crate::Reg<regs2_mailbox_sysconfig::Regs2MailboxSysconfigSpec>;
#[doc = "This register contains parameters to control the whole Mailbox system. Provided for backwards compatibility with OMAP Mailbox. Only contains the soft reset."]
pub mod regs2_mailbox_sysconfig;
#[doc = "REGS2_MAILBOX_MESSAGE (rw) register accessor: The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_message::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_message::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_message`]
module"]
#[doc(alias = "REGS2_MAILBOX_MESSAGE")]
pub type Regs2MailboxMessage = crate::Reg<regs2_mailbox_message::Regs2MailboxMessageSpec>;
#[doc = "The message register stores the next to-be-read message of the mailbox. Read: Reads the next available message. Write: Add a message to this mailbox queue."]
pub mod regs2_mailbox_message;
#[doc = "REGS2_MAILBOX_FIFOSTATUS (rw) register accessor: The FIFO status register has the status of the Mailbox\\[a\\]
FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_fifostatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_fifostatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_fifostatus`]
module"]
#[doc(alias = "REGS2_MAILBOX_FIFOSTATUS")]
pub type Regs2MailboxFifostatus = crate::Reg<regs2_mailbox_fifostatus::Regs2MailboxFifostatusSpec>;
#[doc = "The FIFO status register has the status of the Mailbox\\[a\\]
FIFO"]
pub mod regs2_mailbox_fifostatus;
#[doc = "REGS2_MAILBOX_MSGSTATUS (rw) register accessor: The message status register has the status of the messages in Mailbox\\[a\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_msgstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_msgstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_msgstatus`]
module"]
#[doc(alias = "REGS2_MAILBOX_MSGSTATUS")]
pub type Regs2MailboxMsgstatus = crate::Reg<regs2_mailbox_msgstatus::Regs2MailboxMsgstatusSpec>;
#[doc = "The message status register has the status of the messages in Mailbox\\[a\\]"]
pub mod regs2_mailbox_msgstatus;
#[doc = "REGS2_MAILBOX_IRQ_EOI (rw) register accessor: This is the EOI register with which the software is enabled to do the interrupt clearance.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_irq_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_irq_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_irq_eoi`]
module"]
#[doc(alias = "REGS2_MAILBOX_IRQ_EOI")]
pub type Regs2MailboxIrqEoi = crate::Reg<regs2_mailbox_irq_eoi::Regs2MailboxIrqEoiSpec>;
#[doc = "This is the EOI register with which the software is enabled to do the interrupt clearance."]
pub mod regs2_mailbox_irq_eoi;
#[doc = "REGS2_MAILBOX_IRQ_STATUS_RAW (rw) register accessor: The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_irq_status_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_irq_status_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_irq_status_raw`]
module"]
#[doc(alias = "REGS2_MAILBOX_IRQ_STATUS_RAW")]
pub type Regs2MailboxIrqStatusRaw =
    crate::Reg<regs2_mailbox_irq_status_raw::Regs2MailboxIrqStatusRawSpec>;
#[doc = "The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user . Software may also write 1 to a given bit to set this bit to test interrupt generation. It will activate the status bit for two cycles. This may still be masked, however. Write 0 has no effect."]
pub mod regs2_mailbox_irq_status_raw;
#[doc = "REGS2_MAILBOX_IRQ_STATUS_CLR (rw) register accessor: The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_irq_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_irq_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_irq_status_clr`]
module"]
#[doc(alias = "REGS2_MAILBOX_IRQ_STATUS_CLR")]
pub type Regs2MailboxIrqStatusClr =
    crate::Reg<regs2_mailbox_irq_status_clr::Regs2MailboxIrqStatusClrSpec>;
#[doc = "The interrupt status register has the status for each event that may be responsible for the generation of an interrupt to the corresponding user combined with the corresponding MASK information. Software may also write 1 to a given bit to clear this bit. However, if the hardware still has pending, enabled events, the interrupt will fire again in two cycles. Write 0 has no effect."]
pub mod regs2_mailbox_irq_status_clr;
#[doc = "REGS2_MAILBOX_IRQ_ENABLE_SET (rw) register accessor: The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_irq_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_irq_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_irq_enable_set`]
module"]
#[doc(alias = "REGS2_MAILBOX_IRQ_ENABLE_SET")]
pub type Regs2MailboxIrqEnableSet =
    crate::Reg<regs2_mailbox_irq_enable_set::Regs2MailboxIrqEnableSetSpec>;
#[doc = "The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt source. Write 1 to a bit enables an interrupt source. Write 0 has no effect."]
pub mod regs2_mailbox_irq_enable_set;
#[doc = "REGS2_MAILBOX_IRQ_ENABLE_CLR (rw) register accessor: The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs2_mailbox_irq_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs2_mailbox_irq_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs2_mailbox_irq_enable_clr`]
module"]
#[doc(alias = "REGS2_MAILBOX_IRQ_ENABLE_CLR")]
pub type Regs2MailboxIrqEnableClr =
    crate::Reg<regs2_mailbox_irq_enable_clr::Regs2MailboxIrqEnableClrSpec>;
#[doc = "The interrupt enable register allows software to mask/unmask the module internal source of interrupt for the user \\[a\\]. Read value is the current enable bits for each interrupt sourc. Write 1 to a bit disables an interrupt source. Write 0 has no effect."]
pub mod regs2_mailbox_irq_enable_clr;
