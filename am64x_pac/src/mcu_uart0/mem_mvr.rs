#[doc = "Register `MEM_MVR` reader"]
pub type R = crate::R<MemMvrSpec>;
#[doc = "Register `MEM_MVR` writer"]
pub type W = crate::W<MemMvrSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor revision number of the module."]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor revision number of the module."]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom revision number of the module."]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom revision number of the module."]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major revision number of the module."]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major revision number of the module."]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
Rtl revision number of module"]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
Rtl revision number of module"]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Function revision number of module"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Function revision number of module"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Scheme revision number of module"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Scheme revision number of module"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number of the module."]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision number of the module."]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number of the module."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Rtl revision number of module"]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function revision number of module"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme revision number of module"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number of the module."]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<MemMvrSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision number of the module."]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<MemMvrSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number of the module."]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<MemMvrSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Rtl revision number of module"]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<MemMvrSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function revision number of module"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<MemMvrSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme revision number of module"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<MemMvrSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "The reset value is fixed by hardware and corresponds to the RTL revision of this module. A reset has no effect on the value returned Notes: UART / IRDA SIR only module is revision 1.x (WMU_012_1 specification). UART / IRDA with SIR, MIR and FIR support is revision 2.x (WMU_012_2 specification). UART / IRDA with SIR, MIR and FIR / CIR support is revision 3.x (this specification). For example: MVR = 0x30 => Version 3.0 MVR = 0x38 => Version 3.8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMvrSpec;
impl crate::RegisterSpec for MemMvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mvr::R`](R) reader structure"]
impl crate::Readable for MemMvrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_mvr::W`](W) writer structure"]
impl crate::Writable for MemMvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MVR to value 0x5858_4603"]
impl crate::Resettable for MemMvrSpec {
    const RESET_VALUE: u32 = 0x5858_4603;
}
