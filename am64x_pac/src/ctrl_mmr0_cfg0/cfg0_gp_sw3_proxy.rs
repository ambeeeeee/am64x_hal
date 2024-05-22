#[doc = "Register `CFG0_GP_SW3_PROXY` reader"]
pub type R = crate::R<Cfg0GpSw3ProxySpec>;
#[doc = "Register `CFG0_GP_SW3_PROXY` writer"]
pub type W = crate::W<Cfg0GpSw3ProxySpec>;
#[doc = "Field `GP_SW3_VAL_PROXY` reader - 3:0\\]
general purpose value"]
pub type GpSw3ValProxyR = crate::FieldReader;
#[doc = "Field `GP_SW3_VAL_PROXY` writer - 3:0\\]
general purpose value"]
pub type GpSw3ValProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
general purpose value"]
    #[inline(always)]
    pub fn gp_sw3_val_proxy(&self) -> GpSw3ValProxyR {
        GpSw3ValProxyR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
general purpose value"]
    #[inline(always)]
    #[must_use]
    pub fn gp_sw3_val_proxy(&mut self) -> GpSw3ValProxyW<Cfg0GpSw3ProxySpec> {
        GpSw3ValProxyW::new(self, 0)
    }
}
#[doc = "CFG0_GP_SW3_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw3_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw3_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GpSw3ProxySpec;
impl crate::RegisterSpec for Cfg0GpSw3ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gp_sw3_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0GpSw3ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gp_sw3_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0GpSw3ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GP_SW3_PROXY to value 0"]
impl crate::Resettable for Cfg0GpSw3ProxySpec {
    const RESET_VALUE: u32 = 0;
}
