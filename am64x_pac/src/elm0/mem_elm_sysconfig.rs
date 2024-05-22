#[doc = "Register `MEM_ELM_SYSCONFIG` reader"]
pub type R = crate::R<MemElmSysconfigSpec>;
#[doc = "Register `MEM_ELM_SYSCONFIG` writer"]
pub type W = crate::W<MemElmSysconfigSpec>;
#[doc = "Field `AUTOGATING` reader - 0:0\\]
Internal OCP clock gating strategy \\[no module visible impact other than saving power\\]"]
pub type AutogatingR = crate::BitReader;
#[doc = "Field `AUTOGATING` writer - 0:0\\]
Internal OCP clock gating strategy \\[no module visible impact other than saving power\\]"]
pub type AutogatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRESET` reader - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware \\[During reads, it always returns 0\\]
It has same effect as the OCP Hardware reset"]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware \\[During reads, it always returns 0\\]
It has same effect as the OCP Hardware reset"]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIDLEMODE` reader - 4:3\\]
Slave interface power management \\[IDLE req/ack control\\]"]
pub type SidlemodeR = crate::FieldReader;
#[doc = "Field `SIDLEMODE` writer - 4:3\\]
Slave interface power management \\[IDLE req/ack control\\]"]
pub type SidlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLOCKACTIVITYOCP` reader - 8:8\\]
OCP Clock activity when module is in IDLE mode \\[during wake up mode period\\]"]
pub type ClockactivityocpR = crate::BitReader;
#[doc = "Field `CLOCKACTIVITYOCP` writer - 8:8\\]
OCP Clock activity when module is in IDLE mode \\[during wake up mode period\\]"]
pub type ClockactivityocpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP clock gating strategy \\[no module visible impact other than saving power\\]"]
    #[inline(always)]
    pub fn autogating(&self) -> AutogatingR {
        AutogatingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware \\[During reads, it always returns 0\\]
It has same effect as the OCP Hardware reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Slave interface power management \\[IDLE req/ack control\\]"]
    #[inline(always)]
    pub fn sidlemode(&self) -> SidlemodeR {
        SidlemodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
OCP Clock activity when module is in IDLE mode \\[during wake up mode period\\]"]
    #[inline(always)]
    pub fn clockactivityocp(&self) -> ClockactivityocpR {
        ClockactivityocpR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP clock gating strategy \\[no module visible impact other than saving power\\]"]
    #[inline(always)]
    #[must_use]
    pub fn autogating(&mut self) -> AutogatingW<MemElmSysconfigSpec> {
        AutogatingW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware \\[During reads, it always returns 0\\]
It has same effect as the OCP Hardware reset"]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SoftresetW<MemElmSysconfigSpec> {
        SoftresetW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Slave interface power management \\[IDLE req/ack control\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sidlemode(&mut self) -> SidlemodeW<MemElmSysconfigSpec> {
        SidlemodeW::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
OCP Clock activity when module is in IDLE mode \\[during wake up mode period\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clockactivityocp(&mut self) -> ClockactivityocpW<MemElmSysconfigSpec> {
        ClockactivityocpW::new(self, 8)
    }
}
#[doc = "This register allows controlling various parameters of the OCP interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_sysconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_sysconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSysconfigSpec;
impl crate::RegisterSpec for MemElmSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_sysconfig::R`](R) reader structure"]
impl crate::Readable for MemElmSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_sysconfig::W`](W) writer structure"]
impl crate::Writable for MemElmSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYSCONFIG to value 0x11"]
impl crate::Resettable for MemElmSysconfigSpec {
    const RESET_VALUE: u32 = 0x11;
}
