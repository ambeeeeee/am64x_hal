#[doc = "Register `CFG0_fault_attr_status` reader"]
pub type R = crate::R<Cfg0FaultAttrStatusSpec>;
#[doc = "Register `CFG0_fault_attr_status` writer"]
pub type W = crate::W<Cfg0FaultAttrStatusSpec>;
#[doc = "Field `FAULT_PRIVID` reader - 7:0\\]
Privilege ID."]
pub type FaultPrividR = crate::FieldReader;
#[doc = "Field `FAULT_PRIVID` writer - 7:0\\]
Privilege ID."]
pub type FaultPrividW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FAULT_ROUTEID` reader - 19:8\\]
Route ID."]
pub type FaultRouteidR = crate::FieldReader<u16>;
#[doc = "Field `FAULT_ROUTEID` writer - 19:8\\]
Route ID."]
pub type FaultRouteidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FAULT_XID` reader - 31:20\\]
XID."]
pub type FaultXidR = crate::FieldReader<u16>;
#[doc = "Field `FAULT_XID` writer - 31:20\\]
XID."]
pub type FaultXidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Privilege ID."]
    #[inline(always)]
    pub fn fault_privid(&self) -> FaultPrividR {
        FaultPrividR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Route ID."]
    #[inline(always)]
    pub fn fault_routeid(&self) -> FaultRouteidR {
        FaultRouteidR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
XID."]
    #[inline(always)]
    pub fn fault_xid(&self) -> FaultXidR {
        FaultXidR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Privilege ID."]
    #[inline(always)]
    #[must_use]
    pub fn fault_privid(&mut self) -> FaultPrividW<Cfg0FaultAttrStatusSpec> {
        FaultPrividW::new(self, 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Route ID."]
    #[inline(always)]
    #[must_use]
    pub fn fault_routeid(&mut self) -> FaultRouteidW<Cfg0FaultAttrStatusSpec> {
        FaultRouteidW::new(self, 8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
XID."]
    #[inline(always)]
    #[must_use]
    pub fn fault_xid(&mut self) -> FaultXidW<Cfg0FaultAttrStatusSpec> {
        FaultXidW::new(self, 20)
    }
}
#[doc = "CFG0_fault_attr_status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_attr_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_attr_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultAttrStatusSpec;
impl crate::RegisterSpec for Cfg0FaultAttrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_attr_status::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultAttrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_attr_status::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultAttrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_attr_status to value 0"]
impl crate::Resettable for Cfg0FaultAttrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
