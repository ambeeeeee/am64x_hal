#[doc = "Register `MEM_LCR` reader"]
pub type R = crate::R<MemLcrSpec>;
#[doc = "Register `MEM_LCR` writer"]
pub type W = crate::W<MemLcrSpec>;
#[doc = "Field `CHAR_LENGTH` reader - 1:0\\]
Specifies the word length to be transmitted or received."]
pub type CharLengthR = crate::FieldReader;
#[doc = "Field `CHAR_LENGTH` writer - 1:0\\]
Specifies the word length to be transmitted or received."]
pub type CharLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NB_STOP` reader - 2:2\\]
Specifies the number of stop bits:"]
pub type NbStopR = crate::BitReader;
#[doc = "Field `NB_STOP` writer - 2:2\\]
Specifies the number of stop bits:"]
pub type NbStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_EN` reader - "]
pub type ParityEnR = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - "]
pub type ParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_TYPE1` reader - "]
pub type ParityType1R = crate::BitReader;
#[doc = "Field `PARITY_TYPE1` writer - "]
pub type ParityType1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_TYPE2` reader - 5:5\\]
Selects the forced parity format \\[if LCR\\[3\\]
= 1\\]. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 0, the parity bit is forced to 1 in the transmitted and received data. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 1, the parity bit is forced to 0 in the transmitted and received data."]
pub type ParityType2R = crate::BitReader;
#[doc = "Field `PARITY_TYPE2` writer - 5:5\\]
Selects the forced parity format \\[if LCR\\[3\\]
= 1\\]. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 0, the parity bit is forced to 1 in the transmitted and received data. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 1, the parity bit is forced to 0 in the transmitted and received data."]
pub type ParityType2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_EN` reader - 6:6\\]
Break control bit."]
pub type BreakEnR = crate::BitReader;
#[doc = "Field `BREAK_EN` writer - 6:6\\]
Break control bit."]
pub type BreakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_EN` reader - "]
pub type DivEnR = crate::BitReader;
#[doc = "Field `DIV_EN` writer - "]
pub type DivEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies the word length to be transmitted or received."]
    #[inline(always)]
    pub fn char_length(&self) -> CharLengthR {
        CharLengthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Specifies the number of stop bits:"]
    #[inline(always)]
    pub fn nb_stop(&self) -> NbStopR {
        NbStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn parity_en(&self) -> ParityEnR {
        ParityEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn parity_type1(&self) -> ParityType1R {
        ParityType1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Selects the forced parity format \\[if LCR\\[3\\]
= 1\\]. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 0, the parity bit is forced to 1 in the transmitted and received data. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 1, the parity bit is forced to 0 in the transmitted and received data."]
    #[inline(always)]
    pub fn parity_type2(&self) -> ParityType2R {
        ParityType2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Break control bit."]
    #[inline(always)]
    pub fn break_en(&self) -> BreakEnR {
        BreakEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn div_en(&self) -> DivEnR {
        DivEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies the word length to be transmitted or received."]
    #[inline(always)]
    #[must_use]
    pub fn char_length(&mut self) -> CharLengthW<MemLcrSpec> {
        CharLengthW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Specifies the number of stop bits:"]
    #[inline(always)]
    #[must_use]
    pub fn nb_stop(&mut self) -> NbStopW<MemLcrSpec> {
        NbStopW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> ParityEnW<MemLcrSpec> {
        ParityEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn parity_type1(&mut self) -> ParityType1W<MemLcrSpec> {
        ParityType1W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Selects the forced parity format \\[if LCR\\[3\\]
= 1\\]. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 0, the parity bit is forced to 1 in the transmitted and received data. If LCR\\[5\\]
= 1 and LCR\\[4\\]
= 1, the parity bit is forced to 0 in the transmitted and received data."]
    #[inline(always)]
    #[must_use]
    pub fn parity_type2(&mut self) -> ParityType2W<MemLcrSpec> {
        ParityType2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Break control bit."]
    #[inline(always)]
    #[must_use]
    pub fn break_en(&mut self) -> BreakEnW<MemLcrSpec> {
        BreakEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn div_en(&mut self) -> DivEnW<MemLcrSpec> {
        DivEnW::new(self, 7)
    }
}
#[doc = "LCR\\[6:0\\]
define parameters of the transmission and reception. Note: As soon as LCR\\[6\\]
is set to 1, the TX line is forced to 0 and remains in this state as long as LCR\\[6\\]
= 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemLcrSpec;
impl crate::RegisterSpec for MemLcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_lcr::R`](R) reader structure"]
impl crate::Readable for MemLcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_lcr::W`](W) writer structure"]
impl crate::Writable for MemLcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_LCR to value 0"]
impl crate::Resettable for MemLcrSpec {
    const RESET_VALUE: u32 = 0;
}
