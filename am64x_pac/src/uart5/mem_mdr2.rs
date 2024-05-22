#[doc = "Register `MEM_MDR2` reader"]
pub type R = crate::R<MemMdr2Spec>;
#[doc = "Register `MEM_MDR2` writer"]
pub type W = crate::W<MemMdr2Spec>;
#[doc = "Field `IRTX_UNDERRUN` reader - 0:0\\]
IRDA Transmission status interrupt.When the IIR\\[5\\]
interrupt occurs, the meaning of the interrupt is :"]
pub type IrtxUnderrunR = crate::BitReader;
#[doc = "Field `IRTX_UNDERRUN` writer - 0:0\\]
IRDA Transmission status interrupt.When the IIR\\[5\\]
interrupt occurs, the meaning of the interrupt is :"]
pub type IrtxUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STS_FIFO_TRIG` reader - 2:1\\]
Only for IR-IRDA mode. Frame Status FIFO Threshold select:"]
pub type StsFifoTrigR = crate::FieldReader;
#[doc = "Field `STS_FIFO_TRIG` writer - 2:1\\]
Only for IR-IRDA mode. Frame Status FIFO Threshold select:"]
pub type StsFifoTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART_PULSE` reader - 3:3\\]
UART mode only. Used to allow pulse shaping in UART mode."]
pub type UartPulseR = crate::BitReader;
#[doc = "Field `UART_PULSE` writer - 3:3\\]
UART mode only. Used to allow pulse shaping in UART mode."]
pub type UartPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIR_PULSE_MODE` reader - 5:4\\]
CIR Pulse modulation definition. It defines high level of the pulse width associated with a digit:"]
pub type CirPulseModeR = crate::FieldReader;
#[doc = "Field `CIR_PULSE_MODE` writer - 5:4\\]
CIR Pulse modulation definition. It defines high level of the pulse width associated with a digit:"]
pub type CirPulseModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRRXINVERT` reader - 6:6\\]
Only for IR mode \\[IRDA &amp; CIR\\]Invert RX pin inside the module before the voting or sampling system logic of the infra red block. This will not affect the RX path in UART Modem modes."]
pub type IrrxinvertR = crate::BitReader;
#[doc = "Field `IRRXINVERT` writer - 6:6\\]
Only for IR mode \\[IRDA &amp; CIR\\]Invert RX pin inside the module before the voting or sampling system logic of the infra red block. This will not affect the RX path in UART Modem modes."]
pub type IrrxinvertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_TXIR_ALT` reader - 7:7\\]
Provide alternate functionnality for MDR1\\[4\\]
\\[SET_TXIR\\]"]
pub type SetTxirAltR = crate::BitReader;
#[doc = "Field `SET_TXIR_ALT` writer - 7:7\\]
Provide alternate functionnality for MDR1\\[4\\]
\\[SET_TXIR\\]"]
pub type SetTxirAltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
IRDA Transmission status interrupt.When the IIR\\[5\\]
interrupt occurs, the meaning of the interrupt is :"]
    #[inline(always)]
    pub fn irtx_underrun(&self) -> IrtxUnderrunR {
        IrtxUnderrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Only for IR-IRDA mode. Frame Status FIFO Threshold select:"]
    #[inline(always)]
    pub fn sts_fifo_trig(&self) -> StsFifoTrigR {
        StsFifoTrigR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
UART mode only. Used to allow pulse shaping in UART mode."]
    #[inline(always)]
    pub fn uart_pulse(&self) -> UartPulseR {
        UartPulseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
CIR Pulse modulation definition. It defines high level of the pulse width associated with a digit:"]
    #[inline(always)]
    pub fn cir_pulse_mode(&self) -> CirPulseModeR {
        CirPulseModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Only for IR mode \\[IRDA &amp; CIR\\]Invert RX pin inside the module before the voting or sampling system logic of the infra red block. This will not affect the RX path in UART Modem modes."]
    #[inline(always)]
    pub fn irrxinvert(&self) -> IrrxinvertR {
        IrrxinvertR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Provide alternate functionnality for MDR1\\[4\\]
\\[SET_TXIR\\]"]
    #[inline(always)]
    pub fn set_txir_alt(&self) -> SetTxirAltR {
        SetTxirAltR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
IRDA Transmission status interrupt.When the IIR\\[5\\]
interrupt occurs, the meaning of the interrupt is :"]
    #[inline(always)]
    #[must_use]
    pub fn irtx_underrun(&mut self) -> IrtxUnderrunW<MemMdr2Spec> {
        IrtxUnderrunW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Only for IR-IRDA mode. Frame Status FIFO Threshold select:"]
    #[inline(always)]
    #[must_use]
    pub fn sts_fifo_trig(&mut self) -> StsFifoTrigW<MemMdr2Spec> {
        StsFifoTrigW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
UART mode only. Used to allow pulse shaping in UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn uart_pulse(&mut self) -> UartPulseW<MemMdr2Spec> {
        UartPulseW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
CIR Pulse modulation definition. It defines high level of the pulse width associated with a digit:"]
    #[inline(always)]
    #[must_use]
    pub fn cir_pulse_mode(&mut self) -> CirPulseModeW<MemMdr2Spec> {
        CirPulseModeW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Only for IR mode \\[IRDA &amp; CIR\\]Invert RX pin inside the module before the voting or sampling system logic of the infra red block. This will not affect the RX path in UART Modem modes."]
    #[inline(always)]
    #[must_use]
    pub fn irrxinvert(&mut self) -> IrrxinvertW<MemMdr2Spec> {
        IrrxinvertW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Provide alternate functionnality for MDR1\\[4\\]
\\[SET_TXIR\\]"]
    #[inline(always)]
    #[must_use]
    pub fn set_txir_alt(&mut self) -> SetTxirAltW<MemMdr2Spec> {
        SetTxirAltW::new(self, 7)
    }
}
#[doc = "IR-IrDA and IR-CIR modes only. MDR2\\[0\\]
describes the status of the interrupt in IIR\\[5\\]. The IRTX_UNDERRUN bit should be read after an IIR\\[5\\]
TX_STATUS_IT interrupt has occurred. The bits \\[2:1\\]
of this register sets the trigger level for the frame status FIFO (8 entries) and must be programmed before the mode is programmed in MDR1\\[2:0\\]. Note: The MDR2\\[6\\]
gives the flexibility to invert the RX pin inside the UART module to ensure that the protocol at the input of the transceiver module has the same polarity at module level. By default, the RX pin is inverted because most of transceiver invert the IR receive pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMdr2Spec;
impl crate::RegisterSpec for MemMdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mdr2::R`](R) reader structure"]
impl crate::Readable for MemMdr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_mdr2::W`](W) writer structure"]
impl crate::Writable for MemMdr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MDR2 to value 0"]
impl crate::Resettable for MemMdr2Spec {
    const RESET_VALUE: u32 = 0;
}
