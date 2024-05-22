#[doc = "Register `CFG_UDCP3` reader"]
pub type R = crate::R<CfgUdcp3Spec>;
#[doc = "Register `CFG_UDCP3` writer"]
pub type W = crate::W<CfgUdcp3Spec>;
#[doc = "Field `UDCP3` reader - 31:0\\]
This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp3R = crate::FieldReader<u32>;
#[doc = "Field `UDCP3` writer - 31:0\\]
This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    pub fn udcp3(&self) -> Udcp3R {
        Udcp3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    #[must_use]
    pub fn udcp3(&mut self) -> Udcp3W<CfgUdcp3Spec> {
        Udcp3W::new(self, 0)
    }
}
#[doc = "CFG_UDCP3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_udcp3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_udcp3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgUdcp3Spec;
impl crate::RegisterSpec for CfgUdcp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_udcp3::R`](R) reader structure"]
impl crate::Readable for CfgUdcp3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_udcp3::W`](W) writer structure"]
impl crate::Writable for CfgUdcp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_UDCP3 to value 0"]
impl crate::Resettable for CfgUdcp3Spec {
    const RESET_VALUE: u32 = 0;
}
