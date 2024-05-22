#[doc = "Register `APBADDR_DBG_CPU0_EDRCR` reader"]
pub type R = crate::R<ApbaddrDbgCpu0EdrcrSpec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDRCR` writer"]
pub type W = crate::W<ApbaddrDbgCpu0EdrcrSpec>;
#[doc = "Field `RES0_EDRCR_1_0` reader - 1:0\\]
Reserved, RES0."]
pub type Res0Edrcr1_0R = crate::FieldReader;
#[doc = "Field `RES0_EDRCR_1_0` writer - 1:0\\]
Reserved, RES0."]
pub type Res0Edrcr1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSE` reader - 2:2\\]
Clear Sticky Error. Used to clear the EDSCR cumulative error bits to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.{TXU, RXO, ERR} bits, and, if the processor is in Debug state, the EDSCR.ITO bit, to 0."]
pub type CseR = crate::BitReader;
#[doc = "Field `CSE` writer - 2:2\\]
Clear Sticky Error. Used to clear the EDSCR cumulative error bits to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.{TXU, RXO, ERR} bits, and, if the processor is in Debug state, the EDSCR.ITO bit, to 0."]
pub type CseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPA` reader - 3:3\\]
Clear Sticky Pipeline Advance. This bit is used to clear the EDSCR.PipeAdv bit to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.PipeAdv bit to 0."]
pub type CspaR = crate::BitReader;
#[doc = "Field `CSPA` writer - 3:3\\]
Clear Sticky Pipeline Advance. This bit is used to clear the EDSCR.PipeAdv bit to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.PipeAdv bit to 0."]
pub type CspaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRRQ` reader - 4:4\\]
Allow imprecise entry to Debug state. The actions on writing to this bit are: 0 No action. 1 Allow imprecise entry to Debug state, for example by canceling pending bus accesses. Setting this bit to 1 allows a debugger to request imprecise entry to Debug state. An External Debug Request debug event must be pending before the debugger sets this bit to 1.This feature is optional. If this feature is not implemented, writes to this bit are ignored."]
pub type CbrrqR = crate::BitReader;
#[doc = "Field `CBRRQ` writer - 4:4\\]
Allow imprecise entry to Debug state. The actions on writing to this bit are: 0 No action. 1 Allow imprecise entry to Debug state, for example by canceling pending bus accesses. Setting this bit to 1 allows a debugger to request imprecise entry to Debug state. An External Debug Request debug event must be pending before the debugger sets this bit to 1.This feature is optional. If this feature is not implemented, writes to this bit are ignored."]
pub type CbrrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDRCR_31_5` reader - 31:5\\]
Reserved, RES0."]
pub type Res0Edrcr31_5R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDRCR_31_5` writer - 31:5\\]
Reserved, RES0."]
pub type Res0Edrcr31_5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edrcr_1_0(&self) -> Res0Edrcr1_0R {
        Res0Edrcr1_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear Sticky Error. Used to clear the EDSCR cumulative error bits to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.{TXU, RXO, ERR} bits, and, if the processor is in Debug state, the EDSCR.ITO bit, to 0."]
    #[inline(always)]
    pub fn cse(&self) -> CseR {
        CseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear Sticky Pipeline Advance. This bit is used to clear the EDSCR.PipeAdv bit to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.PipeAdv bit to 0."]
    #[inline(always)]
    pub fn cspa(&self) -> CspaR {
        CspaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Allow imprecise entry to Debug state. The actions on writing to this bit are: 0 No action. 1 Allow imprecise entry to Debug state, for example by canceling pending bus accesses. Setting this bit to 1 allows a debugger to request imprecise entry to Debug state. An External Debug Request debug event must be pending before the debugger sets this bit to 1.This feature is optional. If this feature is not implemented, writes to this bit are ignored."]
    #[inline(always)]
    pub fn cbrrq(&self) -> CbrrqR {
        CbrrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edrcr_31_5(&self) -> Res0Edrcr31_5R {
        Res0Edrcr31_5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edrcr_1_0(&mut self) -> Res0Edrcr1_0W<ApbaddrDbgCpu0EdrcrSpec> {
        Res0Edrcr1_0W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear Sticky Error. Used to clear the EDSCR cumulative error bits to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.{TXU, RXO, ERR} bits, and, if the processor is in Debug state, the EDSCR.ITO bit, to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cse(&mut self) -> CseW<ApbaddrDbgCpu0EdrcrSpec> {
        CseW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear Sticky Pipeline Advance. This bit is used to clear the EDSCR.PipeAdv bit to 0. The actions on writing to this bit are: 0 No action. 1 Clear the EDSCR.PipeAdv bit to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cspa(&mut self) -> CspaW<ApbaddrDbgCpu0EdrcrSpec> {
        CspaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Allow imprecise entry to Debug state. The actions on writing to this bit are: 0 No action. 1 Allow imprecise entry to Debug state, for example by canceling pending bus accesses. Setting this bit to 1 allows a debugger to request imprecise entry to Debug state. An External Debug Request debug event must be pending before the debugger sets this bit to 1.This feature is optional. If this feature is not implemented, writes to this bit are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cbrrq(&mut self) -> CbrrqW<ApbaddrDbgCpu0EdrcrSpec> {
        CbrrqW::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edrcr_31_5(&mut self) -> Res0Edrcr31_5W<ApbaddrDbgCpu0EdrcrSpec> {
        Res0Edrcr31_5W::new(self, 5)
    }
}
#[doc = "External Debug Reserve Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0EdrcrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu0EdrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_edrcr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0EdrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_edrcr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0EdrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDRCR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0EdrcrSpec {
    const RESET_VALUE: u32 = 0;
}
