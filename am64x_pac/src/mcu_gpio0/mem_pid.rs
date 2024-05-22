#[doc = "Register `MEM_pid` reader"]
pub type R = crate::R<MemPidSpec>;
#[doc = "Register `MEM_pid` writer"]
pub type W = crate::W<MemPidSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor revision Y code"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor revision Y code"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom version code"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom version code"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major revision X code"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major revision X code"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
RTL Version R code"]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
RTL Version R code"]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Function code assigned to TCP3"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Function code assigned to TCP3"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Current scheme"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Current scheme"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision Y code"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom version code"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision X code"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version R code"]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function code assigned to TCP3"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Current scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision Y code"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<MemPidSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom version code"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<MemPidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision X code"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<MemPidSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version R code"]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<MemPidSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function code assigned to TCP3"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<MemPidSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Current scheme"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<MemPidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "GPIO Periperal ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemPidSpec;
impl crate::RegisterSpec for MemPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_pid::R`](R) reader structure"]
impl crate::Readable for MemPidSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_pid::W`](W) writer structure"]
impl crate::Writable for MemPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_pid to value 0x5155_2905"]
impl crate::Resettable for MemPidSpec {
    const RESET_VALUE: u32 = 0x5155_2905;
}
