#[doc = "Register `CFG0_ENET1_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Enet1CtrlProxySpec>;
#[doc = "Register `CFG0_ENET1_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Enet1CtrlProxySpec>;
#[doc = "Field `ENET1_CTRL_PORT_MODE_SEL_PROXY` reader - 2:0\\]
Selects Ethernet switch Port1 interface"]
pub type Enet1CtrlPortModeSelProxyR = crate::FieldReader;
#[doc = "Field `ENET1_CTRL_PORT_MODE_SEL_PROXY` writer - 2:0\\]
Selects Ethernet switch Port1 interface"]
pub type Enet1CtrlPortModeSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENET1_CTRL_RGMII_ID_MODE_PROXY` reader - 4:4\\]
Port1 RGMII internal transmit delay selection"]
pub type Enet1CtrlRgmiiIdModeProxyR = crate::BitReader;
#[doc = "Field `ENET1_CTRL_RGMII_ID_MODE_PROXY` writer - 4:4\\]
Port1 RGMII internal transmit delay selection"]
pub type Enet1CtrlRgmiiIdModeProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects Ethernet switch Port1 interface"]
    #[inline(always)]
    pub fn enet1_ctrl_port_mode_sel_proxy(&self) -> Enet1CtrlPortModeSelProxyR {
        Enet1CtrlPortModeSelProxyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Port1 RGMII internal transmit delay selection"]
    #[inline(always)]
    pub fn enet1_ctrl_rgmii_id_mode_proxy(&self) -> Enet1CtrlRgmiiIdModeProxyR {
        Enet1CtrlRgmiiIdModeProxyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects Ethernet switch Port1 interface"]
    #[inline(always)]
    #[must_use]
    pub fn enet1_ctrl_port_mode_sel_proxy(
        &mut self,
    ) -> Enet1CtrlPortModeSelProxyW<Cfg0Enet1CtrlProxySpec> {
        Enet1CtrlPortModeSelProxyW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Port1 RGMII internal transmit delay selection"]
    #[inline(always)]
    #[must_use]
    pub fn enet1_ctrl_rgmii_id_mode_proxy(
        &mut self,
    ) -> Enet1CtrlRgmiiIdModeProxyW<Cfg0Enet1CtrlProxySpec> {
        Enet1CtrlRgmiiIdModeProxyW::new(self, 4)
    }
}
#[doc = "CFG0_ENET1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_enet1_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_enet1_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Enet1CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Enet1CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_enet1_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Enet1CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_enet1_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Enet1CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ENET1_CTRL_PROXY to value 0x02"]
impl crate::Resettable for Cfg0Enet1CtrlProxySpec {
    const RESET_VALUE: u32 = 0x02;
}
