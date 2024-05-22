#[doc = "Register `CFG0_fault_attr_status_PROXY` reader"]
pub type R = crate::R<Cfg0FaultAttrStatusProxySpec>;
#[doc = "Register `CFG0_fault_attr_status_PROXY` writer"]
pub type W = crate::W<Cfg0FaultAttrStatusProxySpec>;
#[doc = "Field `FAULT_PRIVID_PROXY` reader - 7:0\\]
Privilege ID."]
pub type FaultPrividProxyR = crate::FieldReader;
#[doc = "Field `FAULT_PRIVID_PROXY` writer - 7:0\\]
Privilege ID."]
pub type FaultPrividProxyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FAULT_ROUTEID_PROXY` reader - 19:8\\]
Route ID."]
pub type FaultRouteidProxyR = crate::FieldReader<u16>;
#[doc = "Field `FAULT_ROUTEID_PROXY` writer - 19:8\\]
Route ID."]
pub type FaultRouteidProxyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FAULT_XID_PROXY` reader - 31:20\\]
XID."]
pub type FaultXidProxyR = crate::FieldReader<u16>;
#[doc = "Field `FAULT_XID_PROXY` writer - 31:20\\]
XID."]
pub type FaultXidProxyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Privilege ID."]
    #[inline(always)]
    pub fn fault_privid_proxy(&self) -> FaultPrividProxyR {
        FaultPrividProxyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Route ID."]
    #[inline(always)]
    pub fn fault_routeid_proxy(&self) -> FaultRouteidProxyR {
        FaultRouteidProxyR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
XID."]
    #[inline(always)]
    pub fn fault_xid_proxy(&self) -> FaultXidProxyR {
        FaultXidProxyR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Privilege ID."]
    #[inline(always)]
    #[must_use]
    pub fn fault_privid_proxy(&mut self) -> FaultPrividProxyW<Cfg0FaultAttrStatusProxySpec> {
        FaultPrividProxyW::new(self, 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Route ID."]
    #[inline(always)]
    #[must_use]
    pub fn fault_routeid_proxy(&mut self) -> FaultRouteidProxyW<Cfg0FaultAttrStatusProxySpec> {
        FaultRouteidProxyW::new(self, 8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
XID."]
    #[inline(always)]
    #[must_use]
    pub fn fault_xid_proxy(&mut self) -> FaultXidProxyW<Cfg0FaultAttrStatusProxySpec> {
        FaultXidProxyW::new(self, 20)
    }
}
#[doc = "CFG0_fault_attr_status_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_attr_status_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_attr_status_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultAttrStatusProxySpec;
impl crate::RegisterSpec for Cfg0FaultAttrStatusProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_attr_status_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultAttrStatusProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_attr_status_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultAttrStatusProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_attr_status_PROXY to value 0"]
impl crate::Resettable for Cfg0FaultAttrStatusProxySpec {
    const RESET_VALUE: u32 = 0;
}
