#[doc = "Register `MEM_PCR` reader"]
pub type R = crate::R<MemPcrSpec>;
#[doc = "Register `MEM_PCR` writer"]
pub type W = crate::W<MemPcrSpec>;
#[doc = "Field `FREE` reader - 0:0\\]
For GPIO, the FREE bit is fixed at 1, which means GPIO runs free in emulation suspend"]
pub type FreeR = crate::BitReader;
#[doc = "Field `FREE` writer - 0:0\\]
For GPIO, the FREE bit is fixed at 1, which means GPIO runs free in emulation suspend"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT` reader - 1:1\\]
Used in conjunction with FREE bit to determine the emulation suspend mode"]
pub type SoftR = crate::BitReader;
#[doc = "Field `SOFT` writer - 1:1\\]
Used in conjunction with FREE bit to determine the emulation suspend mode"]
pub type SoftW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
For GPIO, the FREE bit is fixed at 1, which means GPIO runs free in emulation suspend"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used in conjunction with FREE bit to determine the emulation suspend mode"]
    #[inline(always)]
    pub fn soft(&self) -> SoftR {
        SoftR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
For GPIO, the FREE bit is fixed at 1, which means GPIO runs free in emulation suspend"]
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FreeW<MemPcrSpec> {
        FreeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used in conjunction with FREE bit to determine the emulation suspend mode"]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SoftW<MemPcrSpec> {
        SoftW::new(self, 1)
    }
}
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemPcrSpec;
impl crate::RegisterSpec for MemPcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_pcr::R`](R) reader structure"]
impl crate::Readable for MemPcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_pcr::W`](W) writer structure"]
impl crate::Writable for MemPcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_PCR to value 0x01"]
impl crate::Resettable for MemPcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
