#[doc = "Register `CFG0_fault_type_status_PROXY` reader"]
pub type R = crate::R<Cfg0FaultTypeStatusProxySpec>;
#[doc = "Register `CFG0_fault_type_status_PROXY` writer"]
pub type W = crate::W<Cfg0FaultTypeStatusProxySpec>;
#[doc = "Field `FAULT_TYPE_PROXY` reader - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
pub type FaultTypeProxyR = crate::FieldReader;
#[doc = "Field `FAULT_TYPE_PROXY` writer - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
pub type FaultTypeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FAULT_NS_PROXY` reader - 6:6\\]
Non-secure access."]
pub type FaultNsProxyR = crate::BitReader;
#[doc = "Field `FAULT_NS_PROXY` writer - 6:6\\]
Non-secure access."]
pub type FaultNsProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
    #[inline(always)]
    pub fn fault_type_proxy(&self) -> FaultTypeProxyR {
        FaultTypeProxyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Non-secure access."]
    #[inline(always)]
    pub fn fault_ns_proxy(&self) -> FaultNsProxyR {
        FaultNsProxyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Fault Type 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 01_0000 = Supervisor write fault - priv = 1 dir = 0 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 00_0010 = User write fault - priv = 0 dir = 0 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 00_0000 = No fault"]
    #[inline(always)]
    #[must_use]
    pub fn fault_type_proxy(&mut self) -> FaultTypeProxyW<Cfg0FaultTypeStatusProxySpec> {
        FaultTypeProxyW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Non-secure access."]
    #[inline(always)]
    #[must_use]
    pub fn fault_ns_proxy(&mut self) -> FaultNsProxyW<Cfg0FaultTypeStatusProxySpec> {
        FaultNsProxyW::new(self, 6)
    }
}
#[doc = "CFG0_fault_type_status_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_type_status_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_type_status_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultTypeStatusProxySpec;
impl crate::RegisterSpec for Cfg0FaultTypeStatusProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_type_status_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultTypeStatusProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_type_status_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultTypeStatusProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_type_status_PROXY to value 0"]
impl crate::Resettable for Cfg0FaultTypeStatusProxySpec {
    const RESET_VALUE: u32 = 0;
}
