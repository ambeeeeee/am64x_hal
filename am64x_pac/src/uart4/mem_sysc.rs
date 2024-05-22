#[doc = "Register `MEM_SYSC` reader"]
pub type R = crate::R<MemSyscSpec>;
#[doc = "Register `MEM_SYSC` writer"]
pub type W = crate::W<MemSyscSpec>;
#[doc = "Field `AUTOIDLE` reader - 0:0\\]
Internal OCP clock gating strategy"]
pub type AutoidleR = crate::BitReader;
#[doc = "Field `AUTOIDLE` writer - 0:0\\]
Internal OCP clock gating strategy"]
pub type AutoidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRESET` reader - 1:1\\]
Software reset. Set this bit to 1 to trigger a module reset. This bit is automatically reset by the hardware. During reads it always returns a 0."]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - 1:1\\]
Software reset. Set this bit to 1 to trigger a module reset. This bit is automatically reset by the hardware. During reads it always returns a 0."]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENAWAKEUP` reader - 2:2\\]
WAKE UP FEATURE CONTROL"]
pub type EnawakeupR = crate::BitReader;
#[doc = "Field `ENAWAKEUP` writer - 2:2\\]
WAKE UP FEATURE CONTROL"]
pub type EnawakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEMODE` reader - 4:3\\]
POWER MANAGEMENT REQ/ACK CONTROL REF: OCP DESIGN GUIDELINES VERSION 1.1"]
pub type IdlemodeR = crate::FieldReader;
#[doc = "Field `IDLEMODE` writer - 4:3\\]
POWER MANAGEMENT REQ/ACK CONTROL REF: OCP DESIGN GUIDELINES VERSION 1.1"]
pub type IdlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP clock gating strategy"]
    #[inline(always)]
    pub fn autoidle(&self) -> AutoidleR {
        AutoidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software reset. Set this bit to 1 to trigger a module reset. This bit is automatically reset by the hardware. During reads it always returns a 0."]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
WAKE UP FEATURE CONTROL"]
    #[inline(always)]
    pub fn enawakeup(&self) -> EnawakeupR {
        EnawakeupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
POWER MANAGEMENT REQ/ACK CONTROL REF: OCP DESIGN GUIDELINES VERSION 1.1"]
    #[inline(always)]
    pub fn idlemode(&self) -> IdlemodeR {
        IdlemodeR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP clock gating strategy"]
    #[inline(always)]
    #[must_use]
    pub fn autoidle(&mut self) -> AutoidleW<MemSyscSpec> {
        AutoidleW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software reset. Set this bit to 1 to trigger a module reset. This bit is automatically reset by the hardware. During reads it always returns a 0."]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SoftresetW<MemSyscSpec> {
        SoftresetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
WAKE UP FEATURE CONTROL"]
    #[inline(always)]
    #[must_use]
    pub fn enawakeup(&mut self) -> EnawakeupW<MemSyscSpec> {
        EnawakeupW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
POWER MANAGEMENT REQ/ACK CONTROL REF: OCP DESIGN GUIDELINES VERSION 1.1"]
    #[inline(always)]
    #[must_use]
    pub fn idlemode(&mut self) -> IdlemodeW<MemSyscSpec> {
        IdlemodeW::new(self, 3)
    }
}
#[doc = "The auto idle bit controls a power saving technique to reduce the logic power consumption of the OCP interface. That is to say when the feature is enabled, the clock will be gated off until an OCP command for this device has been detected. When the software reset bit is set high it causes a full device reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sysc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sysc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSyscSpec;
impl crate::RegisterSpec for MemSyscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sysc::R`](R) reader structure"]
impl crate::Readable for MemSyscSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_sysc::W`](W) writer structure"]
impl crate::Writable for MemSyscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SYSC to value 0"]
impl crate::Resettable for MemSyscSpec {
    const RESET_VALUE: u32 = 0;
}
