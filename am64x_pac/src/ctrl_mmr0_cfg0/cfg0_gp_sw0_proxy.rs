#[doc = "Register `CFG0_GP_SW0_PROXY` reader"]
pub type R = crate::R<Cfg0GpSw0ProxySpec>;
#[doc = "Register `CFG0_GP_SW0_PROXY` writer"]
pub type W = crate::W<Cfg0GpSw0ProxySpec>;
#[doc = "Field `GP_SW0_VAL_PROXY` reader - 31:0\\]
general purpose value"]
pub type GpSw0ValProxyR = crate::FieldReader<u32>;
#[doc = "Field `GP_SW0_VAL_PROXY` writer - 31:0\\]
general purpose value"]
pub type GpSw0ValProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
general purpose value"]
    #[inline(always)]
    pub fn gp_sw0_val_proxy(&self) -> GpSw0ValProxyR {
        GpSw0ValProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
general purpose value"]
    #[inline(always)]
    #[must_use]
    pub fn gp_sw0_val_proxy(&mut self) -> GpSw0ValProxyW<Cfg0GpSw0ProxySpec> {
        GpSw0ValProxyW::new(self, 0)
    }
}
#[doc = "CFG0_GP_SW0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw0_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw0_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GpSw0ProxySpec;
impl crate::RegisterSpec for Cfg0GpSw0ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gp_sw0_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0GpSw0ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gp_sw0_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0GpSw0ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GP_SW0_PROXY to value 0"]
impl crate::Resettable for Cfg0GpSw0ProxySpec {
    const RESET_VALUE: u32 = 0;
}
