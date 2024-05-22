#[doc = "Register `CFG0_GP_SW0` reader"]
pub type R = crate::R<Cfg0GpSw0Spec>;
#[doc = "Register `CFG0_GP_SW0` writer"]
pub type W = crate::W<Cfg0GpSw0Spec>;
#[doc = "Field `GP_SW0_VAL` reader - 31:0\\]
general purpose value"]
pub type GpSw0ValR = crate::FieldReader<u32>;
#[doc = "Field `GP_SW0_VAL` writer - 31:0\\]
general purpose value"]
pub type GpSw0ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
general purpose value"]
    #[inline(always)]
    pub fn gp_sw0_val(&self) -> GpSw0ValR {
        GpSw0ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
general purpose value"]
    #[inline(always)]
    #[must_use]
    pub fn gp_sw0_val(&mut self) -> GpSw0ValW<Cfg0GpSw0Spec> {
        GpSw0ValW::new(self, 0)
    }
}
#[doc = "CFG0_GP_SW0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GpSw0Spec;
impl crate::RegisterSpec for Cfg0GpSw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gp_sw0::R`](R) reader structure"]
impl crate::Readable for Cfg0GpSw0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gp_sw0::W`](W) writer structure"]
impl crate::Writable for Cfg0GpSw0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GP_SW0 to value 0"]
impl crate::Resettable for Cfg0GpSw0Spec {
    const RESET_VALUE: u32 = 0;
}
