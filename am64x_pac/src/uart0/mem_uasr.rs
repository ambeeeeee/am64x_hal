#[doc = "Register `MEM_UASR` reader"]
pub type R = crate::R<MemUasrSpec>;
#[doc = "Register `MEM_UASR` writer"]
pub type W = crate::W<MemUasrSpec>;
#[doc = "Field `SPEED` reader - 4:0\\]
Used to report the speed identified. 00000 => No speed identified. 00001 => 115200 bauds. 00010 => 57600 bauds. 00011 => 38400 bauds. 00100 => 28800 bauds. 00101 => 19200 bauds. 00110 => 14400 bauds. 00111 => 9600 bauds. 01000 => 4800 bauds. 01001 => 2400 bauds. 01010 => 1200 bauds"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `SPEED` writer - 4:0\\]
Used to report the speed identified. 00000 => No speed identified. 00001 => 115200 bauds. 00010 => 57600 bauds. 00011 => 38400 bauds. 00100 => 28800 bauds. 00101 => 19200 bauds. 00110 => 14400 bauds. 00111 => 9600 bauds. 01000 => 4800 bauds. 01001 => 2400 bauds. 01010 => 1200 bauds"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_BY_CHAR` reader - 5:5\\]
0 => 7 bits character identified. 1 => 8 bits character identified"]
pub type BitByCharR = crate::BitReader;
#[doc = "Field `BIT_BY_CHAR` writer - 5:5\\]
0 => 7 bits character identified. 1 => 8 bits character identified"]
pub type BitByCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_TYPE` reader - 7:6\\]
00 => No Parity identified. 01 => Parity space. 10 => Even Parity. 11 => Odd Parity"]
pub type ParityTypeR = crate::FieldReader;
#[doc = "Field `PARITY_TYPE` writer - 7:6\\]
00 => No Parity identified. 01 => Parity space. 10 => Even Parity. 11 => Odd Parity"]
pub type ParityTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Used to report the speed identified. 00000 => No speed identified. 00001 => 115200 bauds. 00010 => 57600 bauds. 00011 => 38400 bauds. 00100 => 28800 bauds. 00101 => 19200 bauds. 00110 => 14400 bauds. 00111 => 9600 bauds. 01000 => 4800 bauds. 01001 => 2400 bauds. 01010 => 1200 bauds"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
0 => 7 bits character identified. 1 => 8 bits character identified"]
    #[inline(always)]
    pub fn bit_by_char(&self) -> BitByCharR {
        BitByCharR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
00 => No Parity identified. 01 => Parity space. 10 => Even Parity. 11 => Odd Parity"]
    #[inline(always)]
    pub fn parity_type(&self) -> ParityTypeR {
        ParityTypeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Used to report the speed identified. 00000 => No speed identified. 00001 => 115200 bauds. 00010 => 57600 bauds. 00011 => 38400 bauds. 00100 => 28800 bauds. 00101 => 19200 bauds. 00110 => 14400 bauds. 00111 => 9600 bauds. 01000 => 4800 bauds. 01001 => 2400 bauds. 01010 => 1200 bauds"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<MemUasrSpec> {
        SpeedW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0 => 7 bits character identified. 1 => 8 bits character identified"]
    #[inline(always)]
    #[must_use]
    pub fn bit_by_char(&mut self) -> BitByCharW<MemUasrSpec> {
        BitByCharW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
00 => No Parity identified. 01 => Parity space. 10 => Even Parity. 11 => Odd Parity"]
    #[inline(always)]
    #[must_use]
    pub fn parity_type(&mut self) -> ParityTypeW<MemUasrSpec> {
        ParityTypeW::new(self, 6)
    }
}
#[doc = "UART Autobauding Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_uasr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_uasr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemUasrSpec;
impl crate::RegisterSpec for MemUasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_uasr::R`](R) reader structure"]
impl crate::Readable for MemUasrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_uasr::W`](W) writer structure"]
impl crate::Writable for MemUasrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_UASR to value 0"]
impl crate::Resettable for MemUasrSpec {
    const RESET_VALUE: u32 = 0;
}
