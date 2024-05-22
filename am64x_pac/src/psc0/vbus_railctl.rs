#[doc = "Register `VBUS_RAILCTL` reader"]
pub type R = crate::R<VbusRailctlSpec>;
#[doc = "Register `VBUS_RAILCTL` writer"]
pub type W = crate::W<VbusRailctlSpec>;
#[doc = "Field `RAILCTR0` reader - 7:0\\]
Rail Counter Value 0"]
pub type Railctr0R = crate::FieldReader;
#[doc = "Field `RAILCTR0` writer - 7:0\\]
Rail Counter Value 0"]
pub type Railctr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RAILCTR1` reader - 15:8\\]
Rail Counter Value 1"]
pub type Railctr1R = crate::FieldReader;
#[doc = "Field `RAILCTR1` writer - 15:8\\]
Rail Counter Value 1"]
pub type Railctr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Rail Counter Value 0"]
    #[inline(always)]
    pub fn railctr0(&self) -> Railctr0R {
        Railctr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Rail Counter Value 1"]
    #[inline(always)]
    pub fn railctr1(&self) -> Railctr1R {
        Railctr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Rail Counter Value 0"]
    #[inline(always)]
    #[must_use]
    pub fn railctr0(&mut self) -> Railctr0W<VbusRailctlSpec> {
        Railctr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Rail Counter Value 1"]
    #[inline(always)]
    #[must_use]
    pub fn railctr1(&mut self) -> Railctr1W<VbusRailctlSpec> {
        Railctr1W::new(self, 8)
    }
}
#[doc = "This register is user programmable. It holds the counter values for rail counter. User can select one of the two counter values to be used for each power domain (see RAILSEL register).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_railctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_railctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusRailctlSpec;
impl crate::RegisterSpec for VbusRailctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_railctl::R`](R) reader structure"]
impl crate::Readable for VbusRailctlSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_railctl::W`](W) writer structure"]
impl crate::Writable for VbusRailctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_RAILCTL to value 0"]
impl crate::Resettable for VbusRailctlSpec {
    const RESET_VALUE: u32 = 0;
}
