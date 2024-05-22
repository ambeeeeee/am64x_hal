#[doc = "Register `CFG0_fault_type_status` reader"]
pub type R = crate::R<Cfg0FaultTypeStatusSpec>;
#[doc = "Register `CFG0_fault_type_status` writer"]
pub type W = crate::W<Cfg0FaultTypeStatusSpec>;
#[doc = "Field `FAULT_TYPE` reader - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
pub type FaultTypeR = crate::FieldReader;
#[doc = "Field `FAULT_TYPE` writer - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
pub type FaultTypeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FAULT_NS` reader - 6:6\\]
Non-secure access."]
pub type FaultNsR = crate::BitReader;
#[doc = "Field `FAULT_NS` writer - 6:6\\]
Non-secure access."]
pub type FaultNsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
    #[inline(always)]
    pub fn fault_type(&self) -> FaultTypeR {
        FaultTypeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Non-secure access."]
    #[inline(always)]
    pub fn fault_ns(&self) -> FaultNsR {
        FaultNsR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
    #[inline(always)]
    #[must_use]
    pub fn fault_type(&mut self) -> FaultTypeW<Cfg0FaultTypeStatusSpec> {
        FaultTypeW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Non-secure access."]
    #[inline(always)]
    #[must_use]
    pub fn fault_ns(&mut self) -> FaultNsW<Cfg0FaultTypeStatusSpec> {
        FaultNsW::new(self, 6)
    }
}
#[doc = "CFG0_fault_type_status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_type_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_type_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultTypeStatusSpec;
impl crate::RegisterSpec for Cfg0FaultTypeStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_type_status::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultTypeStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_type_status::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultTypeStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_type_status to value 0"]
impl crate::Resettable for Cfg0FaultTypeStatusSpec {
    const RESET_VALUE: u32 = 0;
}
