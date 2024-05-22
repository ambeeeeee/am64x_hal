#[doc = "Register `CFG_CAFRC1` reader"]
pub type R = crate::R<CfgCafrc1Spec>;
#[doc = "Register `CFG_CAFRC1` writer"]
pub type W = crate::W<CfgCafrc1Spec>;
#[doc = "Field `CAFRC1` reader - 31:0\\]
This registers captures the current value of the Free Running Counter 1 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 1 on a capture event"]
pub type Cafrc1R = crate::FieldReader<u32>;
#[doc = "Field `CAFRC1` writer - 31:0\\]
This registers captures the current value of the Free Running Counter 1 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 1 on a capture event"]
pub type Cafrc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers captures the current value of the Free Running Counter 1 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 1 on a capture event"]
    #[inline(always)]
    pub fn cafrc1(&self) -> Cafrc1R {
        Cafrc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers captures the current value of the Free Running Counter 1 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 1 on a capture event"]
    #[inline(always)]
    #[must_use]
    pub fn cafrc1(&mut self) -> Cafrc1W<CfgCafrc1Spec> {
        Cafrc1W::new(self, 0)
    }
}
#[doc = "CFG_CAFRC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cafrc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cafrc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCafrc1Spec;
impl crate::RegisterSpec for CfgCafrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cafrc1::R`](R) reader structure"]
impl crate::Readable for CfgCafrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cafrc1::W`](W) writer structure"]
impl crate::Writable for CfgCafrc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CAFRC1 to value 0"]
impl crate::Resettable for CfgCafrc1Spec {
    const RESET_VALUE: u32 = 0;
}
