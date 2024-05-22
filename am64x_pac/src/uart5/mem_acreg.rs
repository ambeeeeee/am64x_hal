#[doc = "Register `MEM_ACREG` reader"]
pub type R = crate::R<MemAcregSpec>;
#[doc = "Register `MEM_ACREG` writer"]
pub type W = crate::W<MemAcregSpec>;
#[doc = "Field `EOT_EN` reader - 0:0\\]
EOT \\[end of transmission\\]
bit. The LH writes 1 to this bit just before it writes the last byte to the TX FIFO in set-EOT bit frame closing method. This bit automatically gets cleared when the LH writes to the THR \\[TX FIFO\\]."]
pub type EotEnR = crate::BitReader;
#[doc = "Field `EOT_EN` writer - 0:0\\]
EOT \\[end of transmission\\]
bit. The LH writes 1 to this bit just before it writes the last byte to the TX FIFO in set-EOT bit frame closing method. This bit automatically gets cleared when the LH writes to the THR \\[TX FIFO\\]."]
pub type EotEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_EN` reader - 1:1\\]
Frame Abort. The LH can intentionally abort transmission of a frame by writing 1 to this bit. Neither the end flag nor the CRC bits are appended to the frame. If transmit FIFO is not empty and MDR1\\[5\\]=1, UART IrDA will start a new transfer with data of previous frame as soon as abort frame has been sent. Therefore, TX FIFO must be reset before sending an abort frame."]
pub type AbortEnR = crate::BitReader;
#[doc = "Field `ABORT_EN` writer - 1:1\\]
Frame Abort. The LH can intentionally abort transmission of a frame by writing 1 to this bit. Neither the end flag nor the CRC bits are appended to the frame. If transmit FIFO is not empty and MDR1\\[5\\]=1, UART IrDA will start a new transfer with data of previous frame as soon as abort frame has been sent. Therefore, TX FIFO must be reset before sending an abort frame."]
pub type AbortEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCTX_EN` reader - 2:2\\]
Store and controlled TX start. When MDR1\\[5\\]
= 1 and the LH writes 1 to this bit the TX state machine starts frame transmission. This bit is self-clearing."]
pub type SctxEnR = crate::BitReader;
#[doc = "Field `SCTX_EN` writer - 2:2\\]
Store and controlled TX start. When MDR1\\[5\\]
= 1 and the LH writes 1 to this bit the TX state machine starts frame transmission. This bit is self-clearing."]
pub type SctxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_SIP` reader - 3:3\\]
MIR/FIR Modes only.Send Serial Infrared Interaction Pulse \\[SIP\\]
If this bit is set during a MIR/FIR transmission, the SIP will be send at the end of it.This bit automatically gets cleared at the end of the SIP transmission."]
pub type SendSipR = crate::BitReader;
#[doc = "Field `SEND_SIP` writer - 3:3\\]
MIR/FIR Modes only.Send Serial Infrared Interaction Pulse \\[SIP\\]
If this bit is set during a MIR/FIR transmission, the SIP will be send at the end of it.This bit automatically gets cleared at the end of the SIP transmission."]
pub type SendSipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_TX_UNDERRUN` reader - 4:4\\]
It is recommended to disable TX FIFO underrun capability by masking corresponding underrun interrupt. When disabling underrun by setting ACREG\\[4\\]=1, garbage data is sent over TX line."]
pub type DisTxUnderrunR = crate::BitReader;
#[doc = "Field `DIS_TX_UNDERRUN` writer - 4:4\\]
It is recommended to disable TX FIFO underrun capability by masking corresponding underrun interrupt. When disabling underrun by setting ACREG\\[4\\]=1, garbage data is sent over TX line."]
pub type DisTxUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_IR_RX` reader - "]
pub type DisIrRxR = crate::BitReader;
#[doc = "Field `DIS_IR_RX` writer - "]
pub type DisIrRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_MOD` reader - 6:6\\]
Primary output used to configure transceivers. Connected to the SD/MODE input pin of IrDA transceivers."]
pub type SdModR = crate::BitReader;
#[doc = "Field `SD_MOD` writer - 6:6\\]
Primary output used to configure transceivers. Connected to the SD/MODE input pin of IrDA transceivers."]
pub type SdModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULSE_TYPE` reader - 7:7\\]
SIR pulse width select:"]
pub type PulseTypeR = crate::BitReader;
#[doc = "Field `PULSE_TYPE` writer - 7:7\\]
SIR pulse width select:"]
pub type PulseTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EOT \\[end of transmission\\]
bit. The LH writes 1 to this bit just before it writes the last byte to the TX FIFO in set-EOT bit frame closing method. This bit automatically gets cleared when the LH writes to the THR \\[TX FIFO\\]."]
    #[inline(always)]
    pub fn eot_en(&self) -> EotEnR {
        EotEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Abort. The LH can intentionally abort transmission of a frame by writing 1 to this bit. Neither the end flag nor the CRC bits are appended to the frame. If transmit FIFO is not empty and MDR1\\[5\\]=1, UART IrDA will start a new transfer with data of previous frame as soon as abort frame has been sent. Therefore, TX FIFO must be reset before sending an abort frame."]
    #[inline(always)]
    pub fn abort_en(&self) -> AbortEnR {
        AbortEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Store and controlled TX start. When MDR1\\[5\\]
= 1 and the LH writes 1 to this bit the TX state machine starts frame transmission. This bit is self-clearing."]
    #[inline(always)]
    pub fn sctx_en(&self) -> SctxEnR {
        SctxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
MIR/FIR Modes only.Send Serial Infrared Interaction Pulse \\[SIP\\]
If this bit is set during a MIR/FIR transmission, the SIP will be send at the end of it.This bit automatically gets cleared at the end of the SIP transmission."]
    #[inline(always)]
    pub fn send_sip(&self) -> SendSipR {
        SendSipR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
It is recommended to disable TX FIFO underrun capability by masking corresponding underrun interrupt. When disabling underrun by setting ACREG\\[4\\]=1, garbage data is sent over TX line."]
    #[inline(always)]
    pub fn dis_tx_underrun(&self) -> DisTxUnderrunR {
        DisTxUnderrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dis_ir_rx(&self) -> DisIrRxR {
        DisIrRxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Primary output used to configure transceivers. Connected to the SD/MODE input pin of IrDA transceivers."]
    #[inline(always)]
    pub fn sd_mod(&self) -> SdModR {
        SdModR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SIR pulse width select:"]
    #[inline(always)]
    pub fn pulse_type(&self) -> PulseTypeR {
        PulseTypeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EOT \\[end of transmission\\]
bit. The LH writes 1 to this bit just before it writes the last byte to the TX FIFO in set-EOT bit frame closing method. This bit automatically gets cleared when the LH writes to the THR \\[TX FIFO\\]."]
    #[inline(always)]
    #[must_use]
    pub fn eot_en(&mut self) -> EotEnW<MemAcregSpec> {
        EotEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frame Abort. The LH can intentionally abort transmission of a frame by writing 1 to this bit. Neither the end flag nor the CRC bits are appended to the frame. If transmit FIFO is not empty and MDR1\\[5\\]=1, UART IrDA will start a new transfer with data of previous frame as soon as abort frame has been sent. Therefore, TX FIFO must be reset before sending an abort frame."]
    #[inline(always)]
    #[must_use]
    pub fn abort_en(&mut self) -> AbortEnW<MemAcregSpec> {
        AbortEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Store and controlled TX start. When MDR1\\[5\\]
= 1 and the LH writes 1 to this bit the TX state machine starts frame transmission. This bit is self-clearing."]
    #[inline(always)]
    #[must_use]
    pub fn sctx_en(&mut self) -> SctxEnW<MemAcregSpec> {
        SctxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
MIR/FIR Modes only.Send Serial Infrared Interaction Pulse \\[SIP\\]
If this bit is set during a MIR/FIR transmission, the SIP will be send at the end of it.This bit automatically gets cleared at the end of the SIP transmission."]
    #[inline(always)]
    #[must_use]
    pub fn send_sip(&mut self) -> SendSipW<MemAcregSpec> {
        SendSipW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
It is recommended to disable TX FIFO underrun capability by masking corresponding underrun interrupt. When disabling underrun by setting ACREG\\[4\\]=1, garbage data is sent over TX line."]
    #[inline(always)]
    #[must_use]
    pub fn dis_tx_underrun(&mut self) -> DisTxUnderrunW<MemAcregSpec> {
        DisTxUnderrunW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dis_ir_rx(&mut self) -> DisIrRxW<MemAcregSpec> {
        DisIrRxW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Primary output used to configure transceivers. Connected to the SD/MODE input pin of IrDA transceivers."]
    #[inline(always)]
    #[must_use]
    pub fn sd_mod(&mut self) -> SdModW<MemAcregSpec> {
        SdModW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
SIR pulse width select:"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_type(&mut self) -> PulseTypeW<MemAcregSpec> {
        PulseTypeW::new(self, 7)
    }
}
#[doc = "IR-IrDA and IR-CIR modes only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_acreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_acreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAcregSpec;
impl crate::RegisterSpec for MemAcregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_acreg::R`](R) reader structure"]
impl crate::Readable for MemAcregSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_acreg::W`](W) writer structure"]
impl crate::Writable for MemAcregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ACREG to value 0"]
impl crate::Resettable for MemAcregSpec {
    const RESET_VALUE: u32 = 0;
}
