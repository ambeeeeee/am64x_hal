#[doc = "Register `MEM_EFR` reader"]
pub type R = crate::R<MemEfrSpec>;
#[doc = "Register `MEM_EFR` writer"]
pub type W = crate::W<MemEfrSpec>;
#[doc = "Field `SW_FLOW_CONTROL` reader - 3:0\\]
Combinations of Software flow control can be selected by programming bit 3 - bit 0. See Software Flow Control Options"]
pub type SwFlowControlR = crate::FieldReader;
#[doc = "Field `SW_FLOW_CONTROL` writer - 3:0\\]
Combinations of Software flow control can be selected by programming bit 3 - bit 0. See Software Flow Control Options"]
pub type SwFlowControlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENHANCED_EN` reader - 4:4\\]
Enhanced functions write enable bit. 0: Disables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7. 1: Enables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7."]
pub type EnhancedEnR = crate::BitReader;
#[doc = "Field `ENHANCED_EN` writer - 4:4\\]
Enhanced functions write enable bit. 0: Disables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7. 1: Enables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7."]
pub type EnhancedEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPECIAL_CHAR_DETECT` reader - 5:5\\]
0: Normal operation. 1: Special character detect enable. Received data is compared with XOFF2 data. If a match occurs the received data is transferred to RX FIFO and IIR bit 4 is set to 1 to indicate a special character has been detected."]
pub type SpecialCharDetectR = crate::BitReader;
#[doc = "Field `SPECIAL_CHAR_DETECT` writer - 5:5\\]
0: Normal operation. 1: Special character detect enable. Received data is compared with XOFF2 data. If a match occurs the received data is transferred to RX FIFO and IIR bit 4 is set to 1 to indicate a special character has been detected."]
pub type SpecialCharDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_RTS_EN` reader - 6:6\\]
Auto-RTS enable bit. 0: Normal operation. 1: Auto- RTS flow control is enabled i.e. RTS* pin goes high (inactive) when the receiver FIFO HALT trigger level, TCR\\[3:0\\], is reached, and goes low (active) when the receiver FIFO RESTORE transmission trigger level is reached."]
pub type AutoRtsEnR = crate::BitReader;
#[doc = "Field `AUTO_RTS_EN` writer - 6:6\\]
Auto-RTS enable bit. 0: Normal operation. 1: Auto- RTS flow control is enabled i.e. RTS* pin goes high (inactive) when the receiver FIFO HALT trigger level, TCR\\[3:0\\], is reached, and goes low (active) when the receiver FIFO RESTORE transmission trigger level is reached."]
pub type AutoRtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CTS_EN` reader - 7:7\\]
Auto-CTS enable bit. 0: Normal operation. 1: Auto-CTS flow control is enabled i.e. transmission is halted when the CTS* pin is high (inactive)."]
pub type AutoCtsEnR = crate::BitReader;
#[doc = "Field `AUTO_CTS_EN` writer - 7:7\\]
Auto-CTS enable bit. 0: Normal operation. 1: Auto-CTS flow control is enabled i.e. transmission is halted when the CTS* pin is high (inactive)."]
pub type AutoCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Combinations of Software flow control can be selected by programming bit 3 - bit 0. See Software Flow Control Options"]
    #[inline(always)]
    pub fn sw_flow_control(&self) -> SwFlowControlR {
        SwFlowControlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Enhanced functions write enable bit. 0: Disables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7. 1: Enables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7."]
    #[inline(always)]
    pub fn enhanced_en(&self) -> EnhancedEnR {
        EnhancedEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Normal operation. 1: Special character detect enable. Received data is compared with XOFF2 data. If a match occurs the received data is transferred to RX FIFO and IIR bit 4 is set to 1 to indicate a special character has been detected."]
    #[inline(always)]
    pub fn special_char_detect(&self) -> SpecialCharDetectR {
        SpecialCharDetectR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Auto-RTS enable bit. 0: Normal operation. 1: Auto- RTS flow control is enabled i.e. RTS* pin goes high (inactive) when the receiver FIFO HALT trigger level, TCR\\[3:0\\], is reached, and goes low (active) when the receiver FIFO RESTORE transmission trigger level is reached."]
    #[inline(always)]
    pub fn auto_rts_en(&self) -> AutoRtsEnR {
        AutoRtsEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Auto-CTS enable bit. 0: Normal operation. 1: Auto-CTS flow control is enabled i.e. transmission is halted when the CTS* pin is high (inactive)."]
    #[inline(always)]
    pub fn auto_cts_en(&self) -> AutoCtsEnR {
        AutoCtsEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Combinations of Software flow control can be selected by programming bit 3 - bit 0. See Software Flow Control Options"]
    #[inline(always)]
    #[must_use]
    pub fn sw_flow_control(&mut self) -> SwFlowControlW<MemEfrSpec> {
        SwFlowControlW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enhanced functions write enable bit. 0: Disables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7. 1: Enables writing to IER bits 4-7, FCR bits 4-5, and MCR bits 5-7."]
    #[inline(always)]
    #[must_use]
    pub fn enhanced_en(&mut self) -> EnhancedEnW<MemEfrSpec> {
        EnhancedEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Normal operation. 1: Special character detect enable. Received data is compared with XOFF2 data. If a match occurs the received data is transferred to RX FIFO and IIR bit 4 is set to 1 to indicate a special character has been detected."]
    #[inline(always)]
    #[must_use]
    pub fn special_char_detect(&mut self) -> SpecialCharDetectW<MemEfrSpec> {
        SpecialCharDetectW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Auto-RTS enable bit. 0: Normal operation. 1: Auto- RTS flow control is enabled i.e. RTS* pin goes high (inactive) when the receiver FIFO HALT trigger level, TCR\\[3:0\\], is reached, and goes low (active) when the receiver FIFO RESTORE transmission trigger level is reached."]
    #[inline(always)]
    #[must_use]
    pub fn auto_rts_en(&mut self) -> AutoRtsEnW<MemEfrSpec> {
        AutoRtsEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Auto-CTS enable bit. 0: Normal operation. 1: Auto-CTS flow control is enabled i.e. transmission is halted when the CTS* pin is high (inactive)."]
    #[inline(always)]
    #[must_use]
    pub fn auto_cts_en(&mut self) -> AutoCtsEnW<MemEfrSpec> {
        AutoCtsEnW::new(self, 7)
    }
}
#[doc = "Enhanced Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_efr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_efr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemEfrSpec;
impl crate::RegisterSpec for MemEfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_efr::R`](R) reader structure"]
impl crate::Readable for MemEfrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_efr::W`](W) writer structure"]
impl crate::Writable for MemEfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_EFR to value 0"]
impl crate::Resettable for MemEfrSpec {
    const RESET_VALUE: u32 = 0;
}
