#[doc = "Register `CFG0_GP_SW1_PROXY` reader"]
pub type R = crate::R<Cfg0GpSw1ProxySpec>;
#[doc = "Register `CFG0_GP_SW1_PROXY` writer"]
pub type W = crate::W<Cfg0GpSw1ProxySpec>;
#[doc = "Field `GP_SW1_VAL_PROXY` reader - 31:0\\]
general purpose value"]
pub type GpSw1ValProxyR = crate::FieldReader<u32>;
#[doc = "Field `GP_SW1_VAL_PROXY` writer - 31:0\\]
general purpose value"]
pub type GpSw1ValProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
general purpose value"]
    #[inline(always)]
    pub fn gp_sw1_val_proxy(&self) -> GpSw1ValProxyR {
        GpSw1ValProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
general purpose value"]
    #[inline(always)]
    #[must_use]
    pub fn gp_sw1_val_proxy(&mut self) -> GpSw1ValProxyW<Cfg0GpSw1ProxySpec> {
        GpSw1ValProxyW::new(self, 0)
    }
}
#[doc = "CFG0_GP_SW1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw1_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw1_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GpSw1ProxySpec;
impl crate::RegisterSpec for Cfg0GpSw1ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gp_sw1_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0GpSw1ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gp_sw1_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0GpSw1ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GP_SW1_PROXY to value 0"]
impl crate::Resettable for Cfg0GpSw1ProxySpec {
    const RESET_VALUE: u32 = 0;
}
