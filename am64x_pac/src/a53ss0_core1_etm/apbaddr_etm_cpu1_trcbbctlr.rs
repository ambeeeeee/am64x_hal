#[doc = "Register `APBADDR_ETM_CPU1_TRCBBCTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcbbctlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCBBCTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcbbctlrSpec>;
#[doc = "Field `RANGE` reader - 7:0\\]
Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each bit represents an address range comparator pair, so bit\\[n\\]
controls the selection of address range comparator pair n. If bit\\[n\\]
is: 0 The address range that address range comparator pair n defines is not selected. 1 The address range that address range comparator pair n defines is selected."]
pub type RangeR = crate::FieldReader;
#[doc = "Field `RANGE` writer - 7:0\\]
Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each bit represents an address range comparator pair, so bit\\[n\\]
controls the selection of address range comparator pair n. If bit\\[n\\]
is: 0 The address range that address range comparator pair n defines is not selected. 1 The address range that address range comparator pair n defines is selected."]
pub type RangeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODE` reader - 8:8\\]
Mode bit: 0 Exclude mode. Branch broadcasting is not enabled in the address range that RANGE defines. If RANGE==0 then branch broadcasting is enabled for the entire memory map. 1 Include mode. Branch broadcasting is enabled in the address range that RANGE defines. If RANGE==0 then the branch broadcasting behavior is UNPREDICTABLE."]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - 8:8\\]
Mode bit: 0 Exclude mode. Branch broadcasting is not enabled in the address range that RANGE defines. If RANGE==0 then branch broadcasting is enabled for the entire memory map. 1 Include mode. Branch broadcasting is enabled in the address range that RANGE defines. If RANGE==0 then the branch broadcasting behavior is UNPREDICTABLE."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCBBCTLR_31_9` reader - 31:9\\]
Reserved, RES0."]
pub type Res0Trcbbctlr31_9R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCBBCTLR_31_9` writer - 31:9\\]
Reserved, RES0."]
pub type Res0Trcbbctlr31_9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each bit represents an address range comparator pair, so bit\\[n\\]
controls the selection of address range comparator pair n. If bit\\[n\\]
is: 0 The address range that address range comparator pair n defines is not selected. 1 The address range that address range comparator pair n defines is selected."]
    #[inline(always)]
    pub fn range(&self) -> RangeR {
        RangeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Mode bit: 0 Exclude mode. Branch broadcasting is not enabled in the address range that RANGE defines. If RANGE==0 then branch broadcasting is enabled for the entire memory map. 1 Include mode. Branch broadcasting is enabled in the address range that RANGE defines. If RANGE==0 then the branch broadcasting behavior is UNPREDICTABLE."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcbbctlr_31_9(&self) -> Res0Trcbbctlr31_9R {
        Res0Trcbbctlr31_9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each bit represents an address range comparator pair, so bit\\[n\\]
controls the selection of address range comparator pair n. If bit\\[n\\]
is: 0 The address range that address range comparator pair n defines is not selected. 1 The address range that address range comparator pair n defines is selected."]
    #[inline(always)]
    #[must_use]
    pub fn range(&mut self) -> RangeW<ApbaddrEtmCpu1TrcbbctlrSpec> {
        RangeW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Mode bit: 0 Exclude mode. Branch broadcasting is not enabled in the address range that RANGE defines. If RANGE==0 then branch broadcasting is enabled for the entire memory map. 1 Include mode. Branch broadcasting is enabled in the address range that RANGE defines. If RANGE==0 then the branch broadcasting behavior is UNPREDICTABLE."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ApbaddrEtmCpu1TrcbbctlrSpec> {
        ModeW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcbbctlr_31_9(&mut self) -> Res0Trcbbctlr31_9W<ApbaddrEtmCpu1TrcbbctlrSpec> {
        Res0Trcbbctlr31_9W::new(self, 9)
    }
}
#[doc = "Branch Broadcast Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcbbctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcbbctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcbbctlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcbbctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcbbctlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcbbctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcbbctlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcbbctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCBBCTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcbbctlrSpec {
    const RESET_VALUE: u32 = 0;
}
