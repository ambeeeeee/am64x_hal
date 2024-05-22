#[doc = "Register `CFG_pll12_PID` reader"]
pub type R = crate::R<CfgPll12PidSpec>;
#[doc = "Register `CFG_pll12_PID` writer"]
pub type W = crate::W<CfgPll12PidSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor revision number"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor revision number"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
custom revision number"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
custom revision number"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major revision number"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major revision number"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MISC` reader - 15:11\\]
Misc revision number"]
pub type MiscR = crate::FieldReader;
#[doc = "Field `MISC` writer - 15:11\\]
Misc revision number"]
pub type MiscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MODULE` reader - 27:16\\]
Module functional identifier"]
pub type ModuleR = crate::FieldReader<u16>;
#[doc = "Field `MODULE` writer - 27:16\\]
Module functional identifier"]
pub type ModuleW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
Business Unit - Processors"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
Business Unit - Processors"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
PID follows new scheme"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
PID follows new scheme"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
custom revision number"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Misc revision number"]
    #[inline(always)]
    pub fn misc(&self) -> MiscR {
        MiscR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module functional identifier"]
    #[inline(always)]
    pub fn module(&self) -> ModuleR {
        ModuleR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business Unit - Processors"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID follows new scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<CfgPll12PidSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
custom revision number"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<CfgPll12PidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<CfgPll12PidSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Misc revision number"]
    #[inline(always)]
    #[must_use]
    pub fn misc(&mut self) -> MiscW<CfgPll12PidSpec> {
        MiscW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module functional identifier"]
    #[inline(always)]
    #[must_use]
    pub fn module(&mut self) -> ModuleW<CfgPll12PidSpec> {
        ModuleW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business Unit - Processors"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<CfgPll12PidSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID follows new scheme"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<CfgPll12PidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "CFG_pll12_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll12PidSpec;
impl crate::RegisterSpec for CfgPll12PidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll12_pid::R`](R) reader structure"]
impl crate::Readable for CfgPll12PidSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll12_pid::W`](W) writer structure"]
impl crate::Writable for CfgPll12PidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll12_PID to value 0x6384_1001"]
impl crate::Resettable for CfgPll12PidSpec {
    const RESET_VALUE: u32 = 0x6384_1001;
}
