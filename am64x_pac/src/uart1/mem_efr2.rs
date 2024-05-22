#[doc = "Register `MEM_EFR2` reader"]
pub type R = crate::R<MemEfr2Spec>;
#[doc = "Register `MEM_EFR2` writer"]
pub type W = crate::W<MemEfr2Spec>;
#[doc = "Field `ENDIAN` reader - 0:0\\]
Endianness"]
pub type EndianR = crate::BitReader;
#[doc = "Field `ENDIAN` writer - 0:0\\]
Endianness"]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHR_OVERRUN` reader - 1:1\\]
RHR Overrun behaviour when buffer full"]
pub type RhrOverrunR = crate::BitReader;
#[doc = "Field `RHR_OVERRUN` writer - 1:1\\]
RHR Overrun behaviour when buffer full"]
pub type RhrOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTIDROP` reader - 2:2\\]
Enables parity Multi-drop mode \\[overrides LCR\\[5..3\\]\\]
when '1'"]
pub type MultidropR = crate::BitReader;
#[doc = "Field `MULTIDROP` writer - 2:2\\]
Enables parity Multi-drop mode \\[overrides LCR\\[5..3\\]\\]
when '1'"]
pub type MultidropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2` reader - 3:3\\]
Value for ISO 7816 reset pin \\[software controllable\\]"]
pub type C2R = crate::BitReader;
#[doc = "Field `C2` writer - 3:3\\]
Value for ISO 7816 reset pin \\[software controllable\\]"]
pub type C2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4` reader - 4:4\\]
Value for ISO 7816 C4 pin for software control"]
pub type C4R = crate::BitReader;
#[doc = "Field `C4` writer - 4:4\\]
Value for ISO 7816 C4 pin for software control"]
pub type C4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C8` reader - 5:5\\]
Value for ISO 7816 C8 pin for software control"]
pub type C8R = crate::BitReader;
#[doc = "Field `C8` writer - 5:5\\]
Value for ISO 7816 C8 pin for software control"]
pub type C8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_BEHAVE` reader - 6:6\\]
Specifies how timeout is measured"]
pub type TimeoutBehaveR = crate::BitReader;
#[doc = "Field `TIMEOUT_BEHAVE` writer - 6:6\\]
Specifies how timeout is measured"]
pub type TimeoutBehaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROADCAST` reader - 7:7\\]
Enables broadcast address matching in multi-drop address match mode"]
pub type BroadcastR = crate::BitReader;
#[doc = "Field `BROADCAST` writer - 7:7\\]
Enables broadcast address matching in multi-drop address match mode"]
pub type BroadcastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Endianness"]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RHR Overrun behaviour when buffer full"]
    #[inline(always)]
    pub fn rhr_overrun(&self) -> RhrOverrunR {
        RhrOverrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables parity Multi-drop mode \\[overrides LCR\\[5..3\\]\\]
when '1'"]
    #[inline(always)]
    pub fn multidrop(&self) -> MultidropR {
        MultidropR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Value for ISO 7816 reset pin \\[software controllable\\]"]
    #[inline(always)]
    pub fn c2(&self) -> C2R {
        C2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Value for ISO 7816 C4 pin for software control"]
    #[inline(always)]
    pub fn c4(&self) -> C4R {
        C4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Value for ISO 7816 C8 pin for software control"]
    #[inline(always)]
    pub fn c8(&self) -> C8R {
        C8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Specifies how timeout is measured"]
    #[inline(always)]
    pub fn timeout_behave(&self) -> TimeoutBehaveR {
        TimeoutBehaveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables broadcast address matching in multi-drop address match mode"]
    #[inline(always)]
    pub fn broadcast(&self) -> BroadcastR {
        BroadcastR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Endianness"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> EndianW<MemEfr2Spec> {
        EndianW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RHR Overrun behaviour when buffer full"]
    #[inline(always)]
    #[must_use]
    pub fn rhr_overrun(&mut self) -> RhrOverrunW<MemEfr2Spec> {
        RhrOverrunW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables parity Multi-drop mode \\[overrides LCR\\[5..3\\]\\]
when '1'"]
    #[inline(always)]
    #[must_use]
    pub fn multidrop(&mut self) -> MultidropW<MemEfr2Spec> {
        MultidropW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Value for ISO 7816 reset pin \\[software controllable\\]"]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2W<MemEfr2Spec> {
        C2W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Value for ISO 7816 C4 pin for software control"]
    #[inline(always)]
    #[must_use]
    pub fn c4(&mut self) -> C4W<MemEfr2Spec> {
        C4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Value for ISO 7816 C8 pin for software control"]
    #[inline(always)]
    #[must_use]
    pub fn c8(&mut self) -> C8W<MemEfr2Spec> {
        C8W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Specifies how timeout is measured"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_behave(&mut self) -> TimeoutBehaveW<MemEfr2Spec> {
        TimeoutBehaveW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables broadcast address matching in multi-drop address match mode"]
    #[inline(always)]
    #[must_use]
    pub fn broadcast(&mut self) -> BroadcastW<MemEfr2Spec> {
        BroadcastW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MemEfr2Spec> {
        Reserved1W::new(self, 8)
    }
}
#[doc = "Enhanced Features Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_efr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_efr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemEfr2Spec;
impl crate::RegisterSpec for MemEfr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_efr2::R`](R) reader structure"]
impl crate::Readable for MemEfr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_efr2::W`](W) writer structure"]
impl crate::Writable for MemEfr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_EFR2 to value 0"]
impl crate::Resettable for MemEfr2Spec {
    const RESET_VALUE: u32 = 0;
}
