#[doc = "Register `APBADDR_DBG_CPU0_EDECCR` reader"]
pub type R = crate::R<ApbaddrDbgCpu0EdeccrSpec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDECCR` writer"]
pub type W = crate::W<ApbaddrDbgCpu0EdeccrSpec>;
#[doc = "Field `SE` reader - 3:0\\]
Coarse-grained Secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Secure exception levels. 0010 Exception catch debug event enabled for Secure EL1. 1000 Exception catch debug event enabled for Secure EL3. 1010 Exception catch debug event enabled for Secure EL1 and EL3. All other values are reserved. Bits \\[2,0\\]
are reserved. RES0. Ignored if ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE."]
pub type SeR = crate::FieldReader;
#[doc = "Field `SE` writer - 3:0\\]
Coarse-grained Secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Secure exception levels. 0010 Exception catch debug event enabled for Secure EL1. 1000 Exception catch debug event enabled for Secure EL3. 1010 Exception catch debug event enabled for Secure EL1 and EL3. All other values are reserved. Bits \\[2,0\\]
are reserved. RES0. Ignored if ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE."]
pub type SeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NSE` reader - 7:4\\]
Coarse-grained Non-secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Non-secure exception levels. 0010 Exception catch debug event enabled for Non-secure EL1. 0100 Exception catch debug event enabled for Non-secure EL2. 0110 Exception catch debug event enabled for Non-secure EL1 and EL2. All other values are reserved. Bits \\[7,4\\]
are reserved, RES0."]
pub type NseR = crate::FieldReader;
#[doc = "Field `NSE` writer - 7:4\\]
Coarse-grained Non-secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Non-secure exception levels. 0010 Exception catch debug event enabled for Non-secure EL1. 0100 Exception catch debug event enabled for Non-secure EL2. 0110 Exception catch debug event enabled for Non-secure EL1 and EL2. All other values are reserved. Bits \\[7,4\\]
are reserved, RES0."]
pub type NseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_EDECCR_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Edeccr31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDECCR_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Edeccr31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Coarse-grained Secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Secure exception levels. 0010 Exception catch debug event enabled for Secure EL1. 1000 Exception catch debug event enabled for Secure EL3. 1010 Exception catch debug event enabled for Secure EL1 and EL3. All other values are reserved. Bits \\[2,0\\]
are reserved. RES0. Ignored if ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE."]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Coarse-grained Non-secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Non-secure exception levels. 0010 Exception catch debug event enabled for Non-secure EL1. 0100 Exception catch debug event enabled for Non-secure EL2. 0110 Exception catch debug event enabled for Non-secure EL1 and EL2. All other values are reserved. Bits \\[7,4\\]
are reserved, RES0."]
    #[inline(always)]
    pub fn nse(&self) -> NseR {
        NseR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edeccr_31_8(&self) -> Res0Edeccr31_8R {
        Res0Edeccr31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Coarse-grained Secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Secure exception levels. 0010 Exception catch debug event enabled for Secure EL1. 1000 Exception catch debug event enabled for Secure EL3. 1010 Exception catch debug event enabled for Secure EL1 and EL3. All other values are reserved. Bits \\[2,0\\]
are reserved. RES0. Ignored if ExternalSecureInvasiveDebugEnabled\\[\\]
== FALSE."]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SeW<ApbaddrDbgCpu0EdeccrSpec> {
        SeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Coarse-grained Non-secure exception catch. Possible values of this field are: 0000 Exception catch debug event disabled for Non-secure exception levels. 0010 Exception catch debug event enabled for Non-secure EL1. 0100 Exception catch debug event enabled for Non-secure EL2. 0110 Exception catch debug event enabled for Non-secure EL1 and EL2. All other values are reserved. Bits \\[7,4\\]
are reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn nse(&mut self) -> NseW<ApbaddrDbgCpu0EdeccrSpec> {
        NseW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edeccr_31_8(&mut self) -> Res0Edeccr31_8W<ApbaddrDbgCpu0EdeccrSpec> {
        Res0Edeccr31_8W::new(self, 8)
    }
}
#[doc = "External Debug Exception Catch Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edeccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edeccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0EdeccrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu0EdeccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_edeccr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0EdeccrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_edeccr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0EdeccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDECCR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0EdeccrSpec {
    const RESET_VALUE: u32 = 0;
}
