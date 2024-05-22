#[doc = "Register `APBADDR_DBG_CPU1_OSLAR_EL1` reader"]
pub type R = crate::R<ApbaddrDbgCpu1OslarEl1Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_OSLAR_EL1` writer"]
pub type W = crate::W<ApbaddrDbgCpu1OslarEl1Spec>;
#[doc = "Field `OSLK` reader - 0:0\\]
On writes to OSLAR_EL1, bit\\[0\\]
is copied to the OS lock.Use EDPRSR.OSLK to check the current status of the lock."]
pub type OslkR = crate::BitReader;
#[doc = "Field `OSLK` writer - 0:0\\]
On writes to OSLAR_EL1, bit\\[0\\]
is copied to the OS lock.Use EDPRSR.OSLK to check the current status of the lock."]
pub type OslkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_OSLAR_EL1_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0OslarEl1_31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_OSLAR_EL1_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0OslarEl1_31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
On writes to OSLAR_EL1, bit\\[0\\]
is copied to the OS lock.Use EDPRSR.OSLK to check the current status of the lock."]
    #[inline(always)]
    pub fn oslk(&self) -> OslkR {
        OslkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_oslar_el1_31_1(&self) -> Res0OslarEl1_31_1R {
        Res0OslarEl1_31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
On writes to OSLAR_EL1, bit\\[0\\]
is copied to the OS lock.Use EDPRSR.OSLK to check the current status of the lock."]
    #[inline(always)]
    #[must_use]
    pub fn oslk(&mut self) -> OslkW<ApbaddrDbgCpu1OslarEl1Spec> {
        OslkW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_oslar_el1_31_1(&mut self) -> Res0OslarEl1_31_1W<ApbaddrDbgCpu1OslarEl1Spec> {
        Res0OslarEl1_31_1W::new(self, 1)
    }
}
#[doc = "OS Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_oslar_el1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_oslar_el1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1OslarEl1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1OslarEl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_oslar_el1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1OslarEl1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_oslar_el1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1OslarEl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_OSLAR_EL1 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1OslarEl1Spec {
    const RESET_VALUE: u32 = 0;
}
