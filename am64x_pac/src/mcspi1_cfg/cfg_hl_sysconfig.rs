#[doc = "Register `CFG_HL_SYSCONFIG` reader"]
pub type R = crate::R<CfgHlSysconfigSpec>;
#[doc = "Register `CFG_HL_SYSCONFIG` writer"]
pub type W = crate::W<CfgHlSysconfigSpec>;
#[doc = "Field `SOFTRESET` reader - 0:0\\]
Software reset \\[Optional\\]"]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - 0:0\\]
Software reset \\[Optional\\]"]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEEMU` reader - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal"]
pub type FreeemuR = crate::BitReader;
#[doc = "Field `FREEEMU` writer - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal"]
pub type FreeemuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEMODE` reader - 3:2\\]
Configuration of the local target state management mode By definition, target can handle read/write transaction as long as it is out of IDLE state"]
pub type IdlemodeR = crate::FieldReader;
#[doc = "Field `IDLEMODE` writer - 3:2\\]
Configuration of the local target state management mode By definition, target can handle read/write transaction as long as it is out of IDLE state"]
pub type IdlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software reset \\[Optional\\]"]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal"]
    #[inline(always)]
    pub fn freeemu(&self) -> FreeemuR {
        FreeemuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Configuration of the local target state management mode By definition, target can handle read/write transaction as long as it is out of IDLE state"]
    #[inline(always)]
    pub fn idlemode(&self) -> IdlemodeR {
        IdlemodeR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset \\[Optional\\]"]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SoftresetW<CfgHlSysconfigSpec> {
        SoftresetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal"]
    #[inline(always)]
    #[must_use]
    pub fn freeemu(&mut self) -> FreeemuW<CfgHlSysconfigSpec> {
        FreeemuW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Configuration of the local target state management mode By definition, target can handle read/write transaction as long as it is out of IDLE state"]
    #[inline(always)]
    #[must_use]
    pub fn idlemode(&mut self) -> IdlemodeW<CfgHlSysconfigSpec> {
        IdlemodeW::new(self, 2)
    }
}
#[doc = "Clock management configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hl_sysconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hl_sysconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgHlSysconfigSpec;
impl crate::RegisterSpec for CfgHlSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_hl_sysconfig::R`](R) reader structure"]
impl crate::Readable for CfgHlSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_hl_sysconfig::W`](W) writer structure"]
impl crate::Writable for CfgHlSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_HL_SYSCONFIG to value 0x08"]
impl crate::Resettable for CfgHlSysconfigSpec {
    const RESET_VALUE: u32 = 0x08;
}
