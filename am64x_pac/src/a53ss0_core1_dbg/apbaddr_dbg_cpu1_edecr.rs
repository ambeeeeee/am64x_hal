#[doc = "Register `APBADDR_DBG_CPU1_EDECR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EdecrSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDECR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EdecrSpec>;
#[doc = "Field `OSUCE` reader - 0:0\\]
OS unlock catch enabled. Possible values of this field are: 0 OS unlock catch debug event disabled. 1 OS unlock catch debug event enabled."]
pub type OsuceR = crate::BitReader;
#[doc = "Field `OSUCE` writer - 0:0\\]
OS unlock catch enabled. Possible values of this field are: 0 OS unlock catch debug event disabled. 1 OS unlock catch debug event enabled."]
pub type OsuceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCE` reader - 1:1\\]
Reset catch enable. Possible values of this field are: 0 Reset catch debug event disabled. 1 Reset catch debug event enabled."]
pub type RceR = crate::BitReader;
#[doc = "Field `RCE` writer - 1:1\\]
Reset catch enable. Possible values of this field are: 0 Reset catch debug event disabled. 1 Reset catch debug event enabled."]
pub type RceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS` reader - 2:2\\]
Halting step enable. Possible values of this field are: 0 Halting step debug event disabled. 1 Halting step debug event enabled. If the value of EDECR.SS is changed when the processor is in Non-debug state, the resulting value of EDECR.SS is UNKNOWN."]
pub type SsR = crate::BitReader;
#[doc = "Field `SS` writer - 2:2\\]
Halting step enable. Possible values of this field are: 0 Halting step debug event disabled. 1 Halting step debug event enabled. If the value of EDECR.SS is changed when the processor is in Non-debug state, the resulting value of EDECR.SS is UNKNOWN."]
pub type SsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDECR_31_3` reader - 31:3\\]
Reserved, RES0."]
pub type Res0Edecr31_3R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDECR_31_3` writer - 31:3\\]
Reserved, RES0."]
pub type Res0Edecr31_3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
OS unlock catch enabled. Possible values of this field are: 0 OS unlock catch debug event disabled. 1 OS unlock catch debug event enabled."]
    #[inline(always)]
    pub fn osuce(&self) -> OsuceR {
        OsuceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset catch enable. Possible values of this field are: 0 Reset catch debug event disabled. 1 Reset catch debug event enabled."]
    #[inline(always)]
    pub fn rce(&self) -> RceR {
        RceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Halting step enable. Possible values of this field are: 0 Halting step debug event disabled. 1 Halting step debug event enabled. If the value of EDECR.SS is changed when the processor is in Non-debug state, the resulting value of EDECR.SS is UNKNOWN."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edecr_31_3(&self) -> Res0Edecr31_3R {
        Res0Edecr31_3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
OS unlock catch enabled. Possible values of this field are: 0 OS unlock catch debug event disabled. 1 OS unlock catch debug event enabled."]
    #[inline(always)]
    #[must_use]
    pub fn osuce(&mut self) -> OsuceW<ApbaddrDbgCpu1EdecrSpec> {
        OsuceW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset catch enable. Possible values of this field are: 0 Reset catch debug event disabled. 1 Reset catch debug event enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rce(&mut self) -> RceW<ApbaddrDbgCpu1EdecrSpec> {
        RceW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Halting step enable. Possible values of this field are: 0 Halting step debug event disabled. 1 Halting step debug event enabled. If the value of EDECR.SS is changed when the processor is in Non-debug state, the resulting value of EDECR.SS is UNKNOWN."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<ApbaddrDbgCpu1EdecrSpec> {
        SsW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edecr_31_3(&mut self) -> Res0Edecr31_3W<ApbaddrDbgCpu1EdecrSpec> {
        Res0Edecr31_3W::new(self, 3)
    }
}
#[doc = "External Debug Execution Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EdecrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EdecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edecr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EdecrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edecr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EdecrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDECR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1EdecrSpec {
    const RESET_VALUE: u32 = 0;
}
