#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY` reader"]
pub type R = crate::R<Cfg0McuM4fss0LbistSeed1ProxySpec>;
#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY` writer"]
pub type W = crate::W<Cfg0McuM4fss0LbistSeed1ProxySpec>;
#[doc = "Field `MCU_M4FSS0_LBIST_SEED1_PRPG_DEF_PROXY` reader - 20:0\\]
Initial seed for PRPG (bits 52:32)"]
pub type McuM4fss0LbistSeed1PrpgDefProxyR = crate::FieldReader<u32>;
#[doc = "Field `MCU_M4FSS0_LBIST_SEED1_PRPG_DEF_PROXY` writer - 20:0\\]
Initial seed for PRPG (bits 52:32)"]
pub type McuM4fss0LbistSeed1PrpgDefProxyW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Initial seed for PRPG (bits 52:32)"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_seed1_prpg_def_proxy(&self) -> McuM4fss0LbistSeed1PrpgDefProxyR {
        McuM4fss0LbistSeed1PrpgDefProxyR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Initial seed for PRPG (bits 52:32)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_seed1_prpg_def_proxy(
        &mut self,
    ) -> McuM4fss0LbistSeed1PrpgDefProxyW<Cfg0McuM4fss0LbistSeed1ProxySpec> {
        McuM4fss0LbistSeed1PrpgDefProxyW::new(self, 0)
    }
}
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_seed1_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_seed1_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fss0LbistSeed1ProxySpec;
impl crate::RegisterSpec for Cfg0McuM4fss0LbistSeed1ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss0_lbist_seed1_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fss0LbistSeed1ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss0_lbist_seed1_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fss0LbistSeed1ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY to value 0"]
impl crate::Resettable for Cfg0McuM4fss0LbistSeed1ProxySpec {
    const RESET_VALUE: u32 = 0;
}
