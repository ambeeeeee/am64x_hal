#[doc = "Register `CFG0_GP_SW3` reader"]
pub type R = crate::R<Cfg0GpSw3Spec>;
#[doc = "Register `CFG0_GP_SW3` writer"]
pub type W = crate::W<Cfg0GpSw3Spec>;
#[doc = "Field `GP_SW3_VAL` reader - 3:0\\]
general purpose value"]
pub type GpSw3ValR = crate::FieldReader;
#[doc = "Field `GP_SW3_VAL` writer - 3:0\\]
general purpose value"]
pub type GpSw3ValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
general purpose value"]
    #[inline(always)]
    pub fn gp_sw3_val(&self) -> GpSw3ValR {
        GpSw3ValR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
general purpose value"]
    #[inline(always)]
    #[must_use]
    pub fn gp_sw3_val(&mut self) -> GpSw3ValW<Cfg0GpSw3Spec> {
        GpSw3ValW::new(self, 0)
    }
}
#[doc = "CFG0_GP_SW3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GpSw3Spec;
impl crate::RegisterSpec for Cfg0GpSw3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gp_sw3::R`](R) reader structure"]
impl crate::Readable for Cfg0GpSw3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gp_sw3::W`](W) writer structure"]
impl crate::Writable for Cfg0GpSw3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GP_SW3 to value 0"]
impl crate::Resettable for Cfg0GpSw3Spec {
    const RESET_VALUE: u32 = 0;
}
