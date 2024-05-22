#[doc = "Register `VBUS_RAILSTAT` reader"]
pub type R = crate::R<VbusRailstatSpec>;
#[doc = "Register `VBUS_RAILSTAT` writer"]
pub type W = crate::W<VbusRailstatSpec>;
#[doc = "Field `RAILCNT` reader - 7:0\\]
Indicates the current rail counter value"]
pub type RailcntR = crate::FieldReader;
#[doc = "Field `RAILCNT` writer - 7:0\\]
Indicates the current rail counter value"]
pub type RailcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RAILNUM` reader - 28:24\\]
Indicates Current Rail Requestor being processed by GPSC"]
pub type RailnumR = crate::FieldReader;
#[doc = "Field `RAILNUM` writer - 28:24\\]
Indicates Current Rail Requestor being processed by GPSC"]
pub type RailnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates the current rail counter value"]
    #[inline(always)]
    pub fn railcnt(&self) -> RailcntR {
        RailcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Indicates Current Rail Requestor being processed by GPSC"]
    #[inline(always)]
    pub fn railnum(&self) -> RailnumR {
        RailnumR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates the current rail counter value"]
    #[inline(always)]
    #[must_use]
    pub fn railcnt(&mut self) -> RailcntW<VbusRailstatSpec> {
        RailcntW::new(self, 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Indicates Current Rail Requestor being processed by GPSC"]
    #[inline(always)]
    #[must_use]
    pub fn railnum(&mut self) -> RailnumW<VbusRailstatSpec> {
        RailnumW::new(self, 24)
    }
}
#[doc = "This register is a read-only and shows the current rail requestor whose request is being granted and the current value of the counter associated with this requestor.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_railstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_railstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusRailstatSpec;
impl crate::RegisterSpec for VbusRailstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_railstat::R`](R) reader structure"]
impl crate::Readable for VbusRailstatSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_railstat::W`](W) writer structure"]
impl crate::Writable for VbusRailstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_RAILSTAT to value 0"]
impl crate::Resettable for VbusRailstatSpec {
    const RESET_VALUE: u32 = 0;
}
