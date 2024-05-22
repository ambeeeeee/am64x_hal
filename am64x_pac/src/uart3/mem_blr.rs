#[doc = "Register `MEM_BLR` reader"]
pub type R = crate::R<MemBlrSpec>;
#[doc = "Register `MEM_BLR` writer"]
pub type W = crate::W<MemBlrSpec>;
#[doc = "Field `XBOF_TYPE` reader - 6:6\\]
SIR xBOF select."]
pub type XbofTypeR = crate::BitReader;
#[doc = "Field `XBOF_TYPE` writer - 6:6\\]
SIR xBOF select."]
pub type XbofTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STS_FIFO_RESET` reader - 7:7\\]
Status FIFO reset. This bit is self-clearing"]
pub type StsFifoResetR = crate::BitReader;
#[doc = "Field `STS_FIFO_RESET` writer - 7:7\\]
Status FIFO reset. This bit is self-clearing"]
pub type StsFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - 6:6\\]
SIR xBOF select."]
    #[inline(always)]
    pub fn xbof_type(&self) -> XbofTypeR {
        XbofTypeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Status FIFO reset. This bit is self-clearing"]
    #[inline(always)]
    pub fn sts_fifo_reset(&self) -> StsFifoResetR {
        StsFifoResetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - 6:6\\]
SIR xBOF select."]
    #[inline(always)]
    #[must_use]
    pub fn xbof_type(&mut self) -> XbofTypeW<MemBlrSpec> {
        XbofTypeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Status FIFO reset. This bit is self-clearing"]
    #[inline(always)]
    #[must_use]
    pub fn sts_fifo_reset(&mut self) -> StsFifoResetW<MemBlrSpec> {
        StsFifoResetW::new(self, 7)
    }
}
#[doc = "IrDA modes only. Note that BLR\\[6\\]
is used to select whether 0xC0 or 0xFF start patterns are to be used, when multiple start flags are required in SIR Mode. If only one start flag is required, this will always be 0xC0. If n start flags are required, then either (n-1) 0xC0 or (n-1) 0xFF flags are sent, followed by a single 0xC0 flag (immediately preceding the first data byte).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_blr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_blr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemBlrSpec;
impl crate::RegisterSpec for MemBlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_blr::R`](R) reader structure"]
impl crate::Readable for MemBlrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_blr::W`](W) writer structure"]
impl crate::Writable for MemBlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_BLR to value 0x40"]
impl crate::Resettable for MemBlrSpec {
    const RESET_VALUE: u32 = 0x40;
}
