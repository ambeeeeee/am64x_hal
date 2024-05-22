#[doc = "Register `CFG_COMP1` reader"]
pub type R = crate::R<CfgComp1Spec>;
#[doc = "Register `CFG_COMP1` writer"]
pub type W = crate::W<CfgComp1Spec>;
#[doc = "Field `COMP1` reader - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp1R = crate::FieldReader<u32>;
#[doc = "Field `COMP1` writer - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> Comp1W<CfgComp1Spec> {
        Comp1W::new(self, 0)
    }
}
#[doc = "CFG_COMP1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgComp1Spec;
impl crate::RegisterSpec for CfgComp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_comp1::R`](R) reader structure"]
impl crate::Readable for CfgComp1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_comp1::W`](W) writer structure"]
impl crate::Writable for CfgComp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_COMP1 to value 0"]
impl crate::Resettable for CfgComp1Spec {
    const RESET_VALUE: u32 = 0;
}
