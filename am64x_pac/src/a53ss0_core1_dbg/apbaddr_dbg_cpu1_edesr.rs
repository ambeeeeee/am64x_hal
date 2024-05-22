#[doc = "Register `APBADDR_DBG_CPU1_EDESR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EdesrSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDESR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EdesrSpec>;
#[doc = "Field `OSUC` reader - 0:0\\]
OS unlock debug event pending. Possible values of this field are: 0 Reading this means that an OS unlock catch debug event is not pending. Writing this means no action. 1 Reading this means that an OS unlock catch debug event is pending. Writing this clears the pending OS unlock catch debug event."]
pub type OsucR = crate::BitReader;
#[doc = "Field `OSUC` writer - 0:0\\]
OS unlock debug event pending. Possible values of this field are: 0 Reading this means that an OS unlock catch debug event is not pending. Writing this means no action. 1 Reading this means that an OS unlock catch debug event is pending. Writing this clears the pending OS unlock catch debug event."]
pub type OsucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC` reader - 1:1\\]
Reset catch debug event pending. Possible values of this field are: 0 Reading this means that a Reset catch debug event is not pending. Writing this means no action. 1 Reading this means that a Reset catch debug event is pending. Writing this clears the pending Reset catch debug event."]
pub type RcR = crate::BitReader;
#[doc = "Field `RC` writer - 1:1\\]
Reset catch debug event pending. Possible values of this field are: 0 Reading this means that a Reset catch debug event is not pending. Writing this means no action. 1 Reading this means that a Reset catch debug event is pending. Writing this clears the pending Reset catch debug event."]
pub type RcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS` reader - 2:2\\]
Halting step debug event pending. Possible values of this field are: 0 Reading this means that a Halting step debug event is not pending. Writing this means no action. 1 Reading this means that a Halting step debug event is pending. Writing this clears the pending Halting step debug event."]
pub type SsR = crate::BitReader;
#[doc = "Field `SS` writer - 2:2\\]
Halting step debug event pending. Possible values of this field are: 0 Reading this means that a Halting step debug event is not pending. Writing this means no action. 1 Reading this means that a Halting step debug event is pending. Writing this clears the pending Halting step debug event."]
pub type SsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDESR_31_3` reader - 31:3\\]
Reserved, RES0."]
pub type Res0Edesr31_3R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDESR_31_3` writer - 31:3\\]
Reserved, RES0."]
pub type Res0Edesr31_3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
OS unlock debug event pending. Possible values of this field are: 0 Reading this means that an OS unlock catch debug event is not pending. Writing this means no action. 1 Reading this means that an OS unlock catch debug event is pending. Writing this clears the pending OS unlock catch debug event."]
    #[inline(always)]
    pub fn osuc(&self) -> OsucR {
        OsucR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset catch debug event pending. Possible values of this field are: 0 Reading this means that a Reset catch debug event is not pending. Writing this means no action. 1 Reading this means that a Reset catch debug event is pending. Writing this clears the pending Reset catch debug event."]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Halting step debug event pending. Possible values of this field are: 0 Reading this means that a Halting step debug event is not pending. Writing this means no action. 1 Reading this means that a Halting step debug event is pending. Writing this clears the pending Halting step debug event."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edesr_31_3(&self) -> Res0Edesr31_3R {
        Res0Edesr31_3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
OS unlock debug event pending. Possible values of this field are: 0 Reading this means that an OS unlock catch debug event is not pending. Writing this means no action. 1 Reading this means that an OS unlock catch debug event is pending. Writing this clears the pending OS unlock catch debug event."]
    #[inline(always)]
    #[must_use]
    pub fn osuc(&mut self) -> OsucW<ApbaddrDbgCpu1EdesrSpec> {
        OsucW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset catch debug event pending. Possible values of this field are: 0 Reading this means that a Reset catch debug event is not pending. Writing this means no action. 1 Reading this means that a Reset catch debug event is pending. Writing this clears the pending Reset catch debug event."]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RcW<ApbaddrDbgCpu1EdesrSpec> {
        RcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Halting step debug event pending. Possible values of this field are: 0 Reading this means that a Halting step debug event is not pending. Writing this means no action. 1 Reading this means that a Halting step debug event is pending. Writing this clears the pending Halting step debug event."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<ApbaddrDbgCpu1EdesrSpec> {
        SsW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edesr_31_3(&mut self) -> Res0Edesr31_3W<ApbaddrDbgCpu1EdesrSpec> {
        Res0Edesr31_3W::new(self, 3)
    }
}
#[doc = "External Debug Event Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edesr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edesr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EdesrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EdesrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edesr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EdesrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edesr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EdesrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDESR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1EdesrSpec {
    const RESET_VALUE: u32 = 0;
}
