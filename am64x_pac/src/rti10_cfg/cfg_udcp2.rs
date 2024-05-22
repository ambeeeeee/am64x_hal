#[doc = "Register `CFG_UDCP2` reader"]
pub type R = crate::R<CfgUdcp2Spec>;
#[doc = "Register `CFG_UDCP2` writer"]
pub type W = crate::W<CfgUdcp2Spec>;
#[doc = "Field `UDCP2` reader - 31:0\\]
This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp2R = crate::FieldReader<u32>;
#[doc = "Field `UDCP2` writer - 31:0\\]
This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    pub fn udcp2(&self) -> Udcp2R {
        Udcp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    #[must_use]
    pub fn udcp2(&mut self) -> Udcp2W<CfgUdcp2Spec> {
        Udcp2W::new(self, 0)
    }
}
#[doc = "CFG_UDCP2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_udcp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_udcp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgUdcp2Spec;
impl crate::RegisterSpec for CfgUdcp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_udcp2::R`](R) reader structure"]
impl crate::Readable for CfgUdcp2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_udcp2::W`](W) writer structure"]
impl crate::Writable for CfgUdcp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_UDCP2 to value 0"]
impl crate::Resettable for CfgUdcp2Spec {
    const RESET_VALUE: u32 = 0;
}
