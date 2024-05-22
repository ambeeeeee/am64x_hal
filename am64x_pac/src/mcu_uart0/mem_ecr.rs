#[doc = "Register `MEM_ECR` reader"]
pub type R = crate::R<MemEcrSpec>;
#[doc = "Register `MEM_ECR` writer"]
pub type W = crate::W<MemEcrSpec>;
#[doc = "Field `A_MULTIDROP` reader - 0:0\\]
In multi-drop mode, when written with the value '1' causes the next byte written into THR to be transmitted with the parity bit set, signaling an address"]
pub type AMultidropR = crate::BitReader;
#[doc = "Field `A_MULTIDROP` writer - 0:0\\]
In multi-drop mode, when written with the value '1' causes the next byte written into THR to be transmitted with the parity bit set, signaling an address"]
pub type AMultidropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RST` reader - 1:1\\]
Writing '1' resets the receiver"]
pub type RxRstR = crate::BitReader;
#[doc = "Field `RX_RST` writer - 1:1\\]
Writing '1' resets the receiver"]
pub type RxRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RST` reader - 2:2\\]
Writing '1' resets the transmitter"]
pub type TxRstR = crate::BitReader;
#[doc = "Field `TX_RST` writer - 2:2\\]
Writing '1' resets the transmitter"]
pub type TxRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EN` reader - 3:3\\]
Enables/Disables the receiver"]
pub type RxEnR = crate::BitReader;
#[doc = "Field `RX_EN` writer - 3:3\\]
Enables/Disables the receiver"]
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EN` reader - 4:4\\]
Enables/Disables the transmitter"]
pub type TxEnR = crate::BitReader;
#[doc = "Field `TX_EN` writer - 4:4\\]
Enables/Disables the transmitter"]
pub type TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_TX_PE` reader - 5:5\\]
Write 1 to clear parity error from the Transmitter to allow it to continue to try sending data \\[ISO7816 transmit only\\]"]
pub type ClearTxPeR = crate::BitReader;
#[doc = "Field `CLEAR_TX_PE` writer - 5:5\\]
Write 1 to clear parity error from the Transmitter to allow it to continue to try sending data \\[ISO7816 transmit only\\]"]
pub type ClearTxPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
In multi-drop mode, when written with the value '1' causes the next byte written into THR to be transmitted with the parity bit set, signaling an address"]
    #[inline(always)]
    pub fn a_multidrop(&self) -> AMultidropR {
        AMultidropR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing '1' resets the receiver"]
    #[inline(always)]
    pub fn rx_rst(&self) -> RxRstR {
        RxRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing '1' resets the transmitter"]
    #[inline(always)]
    pub fn tx_rst(&self) -> TxRstR {
        TxRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables/Disables the receiver"]
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables/Disables the transmitter"]
    #[inline(always)]
    pub fn tx_en(&self) -> TxEnR {
        TxEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear parity error from the Transmitter to allow it to continue to try sending data \\[ISO7816 transmit only\\]"]
    #[inline(always)]
    pub fn clear_tx_pe(&self) -> ClearTxPeR {
        ClearTxPeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
In multi-drop mode, when written with the value '1' causes the next byte written into THR to be transmitted with the parity bit set, signaling an address"]
    #[inline(always)]
    #[must_use]
    pub fn a_multidrop(&mut self) -> AMultidropW<MemEcrSpec> {
        AMultidropW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing '1' resets the receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rst(&mut self) -> RxRstW<MemEcrSpec> {
        RxRstW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing '1' resets the transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rst(&mut self) -> TxRstW<MemEcrSpec> {
        TxRstW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables/Disables the receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RxEnW<MemEcrSpec> {
        RxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables/Disables the transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TxEnW<MemEcrSpec> {
        TxEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear parity error from the Transmitter to allow it to continue to try sending data \\[ISO7816 transmit only\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clear_tx_pe(&mut self) -> ClearTxPeW<MemEcrSpec> {
        ClearTxPeW::new(self, 5)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MemEcrSpec> {
        Reserved1W::new(self, 8)
    }
}
#[doc = "Enhanced Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemEcrSpec;
impl crate::RegisterSpec for MemEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ecr::R`](R) reader structure"]
impl crate::Readable for MemEcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ecr::W`](W) writer structure"]
impl crate::Writable for MemEcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ECR to value 0x18"]
impl crate::Resettable for MemEcrSpec {
    const RESET_VALUE: u32 = 0x18;
}
