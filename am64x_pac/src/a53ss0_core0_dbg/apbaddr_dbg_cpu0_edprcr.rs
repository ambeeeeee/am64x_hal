#[doc = "Register `APBADDR_DBG_CPU0_EDPRCR` reader"]
pub type R = crate::R<ApbaddrDbgCpu0EdprcrSpec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDPRCR` writer"]
pub type W = crate::W<ApbaddrDbgCpu0EdprcrSpec>;
#[doc = "Field `CORENPDRQ` reader - 0:0\\]
Core no powerdown request. Requests emulation of powerdown. Possible values of this bit are: 0 On a powerdown request, the system powers down the Core power domain. 1 On a powerdown request, the system emulates powerdown of the Core power domain. In this emulation mode the Core power domain is not actually powered down."]
pub type CorenpdrqR = crate::BitReader;
#[doc = "Field `CORENPDRQ` writer - 0:0\\]
Core no powerdown request. Requests emulation of powerdown. Possible values of this bit are: 0 On a powerdown request, the system powers down the Core power domain. 1 On a powerdown request, the system emulates powerdown of the Core power domain. In this emulation mode the Core power domain is not actually powered down."]
pub type CorenpdrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWRR` reader - 1:1\\]
Warm reset request. Write only bit that reads as zero. The actions on writing to this bit are: 0 No action. 1 Request Warm reset. The processor ignores writes to this bit if any of the following are the case:ExternalInvasiveDebugEnabled\\[\\]
== FALSE, EL3 is not implemented, and the processor is Non-secure.ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE and one of the following is true:EL3 is implemented.The processor is Secure.The Core power domain is either completely off or in a low-power state where the Core power domain registers cannot be accessed.DoubleLockStatus\\[\\]
== TRUE \\[OS Double Lock is set\\].OSLSR.OSLK == 1 \\[OS lock is locked\\].In an implementation that includes the recommended external debug interface, this bit drives the DBGRSTREQ signal."]
pub type CwrrR = crate::BitReader;
#[doc = "Field `CWRR` writer - 1:1\\]
Warm reset request. Write only bit that reads as zero. The actions on writing to this bit are: 0 No action. 1 Request Warm reset. The processor ignores writes to this bit if any of the following are the case:ExternalInvasiveDebugEnabled\\[\\]
== FALSE, EL3 is not implemented, and the processor is Non-secure.ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE and one of the following is true:EL3 is implemented.The processor is Secure.The Core power domain is either completely off or in a low-power state where the Core power domain registers cannot be accessed.DoubleLockStatus\\[\\]
== TRUE \\[OS Double Lock is set\\].OSLSR.OSLK == 1 \\[OS lock is locked\\].In an implementation that includes the recommended external debug interface, this bit drives the DBGRSTREQ signal."]
pub type CwrrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDPRCR_2_2` reader - 2:2\\]
Reserved, RES0."]
pub type Res0Edprcr2_2R = crate::BitReader;
#[doc = "Field `RES0_EDPRCR_2_2` writer - 2:2\\]
Reserved, RES0."]
pub type Res0Edprcr2_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPURQ` reader - 3:3\\]
Core powerup request. Allows a debugger to request that the power controller power up the core, enabling access to the debug register in the Core power domain. The actions on writing to this bit are: 0 No effect. 1 Request the power controller to powerup the core. In an implementation that includes the recommended external debug interface, this bit drives the DBGPWRUPREQ signal.This bit can be read and written when the Core power domain is powered off.The power controller must not allow the Core power domain to switch off while this bit is one."]
pub type CorepurqR = crate::BitReader;
#[doc = "Field `COREPURQ` writer - 3:3\\]
Core powerup request. Allows a debugger to request that the power controller power up the core, enabling access to the debug register in the Core power domain. The actions on writing to this bit are: 0 No effect. 1 Request the power controller to powerup the core. In an implementation that includes the recommended external debug interface, this bit drives the DBGPWRUPREQ signal.This bit can be read and written when the Core power domain is powered off.The power controller must not allow the Core power domain to switch off while this bit is one."]
pub type CorepurqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDPRCR_31_4` reader - 31:4\\]
Reserved, RES0."]
pub type Res0Edprcr31_4R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDPRCR_31_4` writer - 31:4\\]
Reserved, RES0."]
pub type Res0Edprcr31_4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Core no powerdown request. Requests emulation of powerdown. Possible values of this bit are: 0 On a powerdown request, the system powers down the Core power domain. 1 On a powerdown request, the system emulates powerdown of the Core power domain. In this emulation mode the Core power domain is not actually powered down."]
    #[inline(always)]
    pub fn corenpdrq(&self) -> CorenpdrqR {
        CorenpdrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Warm reset request. Write only bit that reads as zero. The actions on writing to this bit are: 0 No action. 1 Request Warm reset. The processor ignores writes to this bit if any of the following are the case:ExternalInvasiveDebugEnabled\\[\\]
== FALSE, EL3 is not implemented, and the processor is Non-secure.ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE and one of the following is true:EL3 is implemented.The processor is Secure.The Core power domain is either completely off or in a low-power state where the Core power domain registers cannot be accessed.DoubleLockStatus\\[\\]
== TRUE \\[OS Double Lock is set\\].OSLSR.OSLK == 1 \\[OS lock is locked\\].In an implementation that includes the recommended external debug interface, this bit drives the DBGRSTREQ signal."]
    #[inline(always)]
    pub fn cwrr(&self) -> CwrrR {
        CwrrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edprcr_2_2(&self) -> Res0Edprcr2_2R {
        Res0Edprcr2_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Core powerup request. Allows a debugger to request that the power controller power up the core, enabling access to the debug register in the Core power domain. The actions on writing to this bit are: 0 No effect. 1 Request the power controller to powerup the core. In an implementation that includes the recommended external debug interface, this bit drives the DBGPWRUPREQ signal.This bit can be read and written when the Core power domain is powered off.The power controller must not allow the Core power domain to switch off while this bit is one."]
    #[inline(always)]
    pub fn corepurq(&self) -> CorepurqR {
        CorepurqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edprcr_31_4(&self) -> Res0Edprcr31_4R {
        Res0Edprcr31_4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Core no powerdown request. Requests emulation of powerdown. Possible values of this bit are: 0 On a powerdown request, the system powers down the Core power domain. 1 On a powerdown request, the system emulates powerdown of the Core power domain. In this emulation mode the Core power domain is not actually powered down."]
    #[inline(always)]
    #[must_use]
    pub fn corenpdrq(&mut self) -> CorenpdrqW<ApbaddrDbgCpu0EdprcrSpec> {
        CorenpdrqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Warm reset request. Write only bit that reads as zero. The actions on writing to this bit are: 0 No action. 1 Request Warm reset. The processor ignores writes to this bit if any of the following are the case:ExternalInvasiveDebugEnabled\\[\\]
== FALSE, EL3 is not implemented, and the processor is Non-secure.ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE and one of the following is true:EL3 is implemented.The processor is Secure.The Core power domain is either completely off or in a low-power state where the Core power domain registers cannot be accessed.DoubleLockStatus\\[\\]
== TRUE \\[OS Double Lock is set\\].OSLSR.OSLK == 1 \\[OS lock is locked\\].In an implementation that includes the recommended external debug interface, this bit drives the DBGRSTREQ signal."]
    #[inline(always)]
    #[must_use]
    pub fn cwrr(&mut self) -> CwrrW<ApbaddrDbgCpu0EdprcrSpec> {
        CwrrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edprcr_2_2(&mut self) -> Res0Edprcr2_2W<ApbaddrDbgCpu0EdprcrSpec> {
        Res0Edprcr2_2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Core powerup request. Allows a debugger to request that the power controller power up the core, enabling access to the debug register in the Core power domain. The actions on writing to this bit are: 0 No effect. 1 Request the power controller to powerup the core. In an implementation that includes the recommended external debug interface, this bit drives the DBGPWRUPREQ signal.This bit can be read and written when the Core power domain is powered off.The power controller must not allow the Core power domain to switch off while this bit is one."]
    #[inline(always)]
    #[must_use]
    pub fn corepurq(&mut self) -> CorepurqW<ApbaddrDbgCpu0EdprcrSpec> {
        CorepurqW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edprcr_31_4(&mut self) -> Res0Edprcr31_4W<ApbaddrDbgCpu0EdprcrSpec> {
        Res0Edprcr31_4W::new(self, 4)
    }
}
#[doc = "External Debug Power/Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edprcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edprcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0EdprcrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu0EdprcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_edprcr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0EdprcrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_edprcr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0EdprcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDPRCR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0EdprcrSpec {
    const RESET_VALUE: u32 = 0;
}
