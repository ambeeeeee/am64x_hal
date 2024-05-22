#[doc = "Register `INTR_ROUTER_CFG_INTR_MUXCNTL` reader"]
pub type R = crate::R<IntrRouterCfgIntrMuxcntlSpec>;
#[doc = "Register `INTR_ROUTER_CFG_INTR_MUXCNTL` writer"]
pub type W = crate::W<IntrRouterCfgIntrMuxcntlSpec>;
#[doc = "Field `MUX_CNTL` reader - 4:0\\]
Mux control for interrupt N"]
pub type MuxCntlR = crate::FieldReader;
#[doc = "Field `MUX_CNTL` writer - 4:0\\]
Mux control for interrupt N"]
pub type MuxCntlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INT_ENABLE` reader - 16:16\\]
interrupt output enable for interrupt N"]
pub type IntEnableR = crate::BitReader;
#[doc = "Field `INT_ENABLE` writer - 16:16\\]
interrupt output enable for interrupt N"]
pub type IntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Mux control for interrupt N"]
    #[inline(always)]
    pub fn mux_cntl(&self) -> MuxCntlR {
        MuxCntlR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
interrupt output enable for interrupt N"]
    #[inline(always)]
    pub fn int_enable(&self) -> IntEnableR {
        IntEnableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Mux control for interrupt N"]
    #[inline(always)]
    #[must_use]
    pub fn mux_cntl(&mut self) -> MuxCntlW<IntrRouterCfgIntrMuxcntlSpec> {
        MuxCntlW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
interrupt output enable for interrupt N"]
    #[inline(always)]
    #[must_use]
    pub fn int_enable(&mut self) -> IntEnableW<IntrRouterCfgIntrMuxcntlSpec> {
        IntEnableW::new(self, 16)
    }
}
#[doc = "Interrupt mux control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_router_cfg_intr_muxcntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_router_cfg_intr_muxcntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRouterCfgIntrMuxcntlSpec;
impl crate::RegisterSpec for IntrRouterCfgIntrMuxcntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_router_cfg_intr_muxcntl::R`](R) reader structure"]
impl crate::Readable for IntrRouterCfgIntrMuxcntlSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_router_cfg_intr_muxcntl::W`](W) writer structure"]
impl crate::Writable for IntrRouterCfgIntrMuxcntlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_ROUTER_CFG_INTR_MUXCNTL to value 0"]
impl crate::Resettable for IntrRouterCfgIntrMuxcntlSpec {
    const RESET_VALUE: u32 = 0;
}
