#[doc = "Register `CFG_COMP3CLR` reader"]
pub type R = crate::R<CfgComp3clrSpec>;
#[doc = "Register `CFG_COMP3CLR` writer"]
pub type W = crate::W<CfgComp3clrSpec>;
#[doc = "Field `COMP3CLR` reader - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 3 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp3clrR = crate::FieldReader<u32>;
#[doc = "Field `COMP3CLR` writer - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 3 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp3clrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 3 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    pub fn comp3clr(&self) -> Comp3clrR {
        Comp3clrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 3 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp3clr(&mut self) -> Comp3clrW<CfgComp3clrSpec> {
        Comp3clrW::new(self, 0)
    }
}
#[doc = "CFG_COMP3CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp3clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp3clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgComp3clrSpec;
impl crate::RegisterSpec for CfgComp3clrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_comp3clr::R`](R) reader structure"]
impl crate::Readable for CfgComp3clrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_comp3clr::W`](W) writer structure"]
impl crate::Writable for CfgComp3clrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_COMP3CLR to value 0"]
impl crate::Resettable for CfgComp3clrSpec {
    const RESET_VALUE: u32 = 0;
}
