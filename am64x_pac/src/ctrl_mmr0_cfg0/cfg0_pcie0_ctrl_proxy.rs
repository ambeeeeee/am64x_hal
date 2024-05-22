#[doc = "Register `CFG0_PCIE0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Pcie0CtrlProxySpec>;
#[doc = "Register `CFG0_PCIE0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Pcie0CtrlProxySpec>;
#[doc = "Field `PCIE0_CTRL_GENERATION_SEL_PROXY` reader - 1:0\\]
Configures the PCIe generation support in the PCIe capabilities linked-list"]
pub type Pcie0CtrlGenerationSelProxyR = crate::FieldReader;
#[doc = "Field `PCIE0_CTRL_GENERATION_SEL_PROXY` writer - 1:0\\]
Configures the PCIe generation support in the PCIe capabilities linked-list"]
pub type Pcie0CtrlGenerationSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCIE0_CTRL_MODE_SEL_PROXY` reader - 7:7\\]
Selects the operating mode"]
pub type Pcie0CtrlModeSelProxyR = crate::BitReader;
#[doc = "Field `PCIE0_CTRL_MODE_SEL_PROXY` writer - 7:7\\]
Selects the operating mode"]
pub type Pcie0CtrlModeSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Configures the PCIe generation support in the PCIe capabilities linked-list"]
    #[inline(always)]
    pub fn pcie0_ctrl_generation_sel_proxy(&self) -> Pcie0CtrlGenerationSelProxyR {
        Pcie0CtrlGenerationSelProxyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects the operating mode"]
    #[inline(always)]
    pub fn pcie0_ctrl_mode_sel_proxy(&self) -> Pcie0CtrlModeSelProxyR {
        Pcie0CtrlModeSelProxyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Configures the PCIe generation support in the PCIe capabilities linked-list"]
    #[inline(always)]
    #[must_use]
    pub fn pcie0_ctrl_generation_sel_proxy(
        &mut self,
    ) -> Pcie0CtrlGenerationSelProxyW<Cfg0Pcie0CtrlProxySpec> {
        Pcie0CtrlGenerationSelProxyW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects the operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcie0_ctrl_mode_sel_proxy(&mut self) -> Pcie0CtrlModeSelProxyW<Cfg0Pcie0CtrlProxySpec> {
        Pcie0CtrlModeSelProxyW::new(self, 7)
    }
}
#[doc = "CFG0_PCIE0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pcie0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pcie0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Pcie0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Pcie0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pcie0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Pcie0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pcie0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Pcie0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PCIE0_CTRL_PROXY to value 0x01"]
impl crate::Resettable for Cfg0Pcie0CtrlProxySpec {
    const RESET_VALUE: u32 = 0x01;
}
