#[doc = "Register `CFG0_ENET1_CTRL` reader"]
pub type R = crate::R<Cfg0Enet1CtrlSpec>;
#[doc = "Register `CFG0_ENET1_CTRL` writer"]
pub type W = crate::W<Cfg0Enet1CtrlSpec>;
#[doc = "Field `ENET1_CTRL_PORT_MODE_SEL` reader - 2:0\\]
Selects Ethernet switch Port1 interface"]
pub type Enet1CtrlPortModeSelR = crate::FieldReader;
#[doc = "Field `ENET1_CTRL_PORT_MODE_SEL` writer - 2:0\\]
Selects Ethernet switch Port1 interface"]
pub type Enet1CtrlPortModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENET1_CTRL_RGMII_ID_MODE` reader - 4:4\\]
Port1 RGMII internal transmit delay selection"]
pub type Enet1CtrlRgmiiIdModeR = crate::BitReader;
#[doc = "Field `ENET1_CTRL_RGMII_ID_MODE` writer - 4:4\\]
Port1 RGMII internal transmit delay selection"]
pub type Enet1CtrlRgmiiIdModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects Ethernet switch Port1 interface"]
    #[inline(always)]
    pub fn enet1_ctrl_port_mode_sel(&self) -> Enet1CtrlPortModeSelR {
        Enet1CtrlPortModeSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Port1 RGMII internal transmit delay selection"]
    #[inline(always)]
    pub fn enet1_ctrl_rgmii_id_mode(&self) -> Enet1CtrlRgmiiIdModeR {
        Enet1CtrlRgmiiIdModeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects Ethernet switch Port1 interface"]
    #[inline(always)]
    #[must_use]
    pub fn enet1_ctrl_port_mode_sel(&mut self) -> Enet1CtrlPortModeSelW<Cfg0Enet1CtrlSpec> {
        Enet1CtrlPortModeSelW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Port1 RGMII internal transmit delay selection"]
    #[inline(always)]
    #[must_use]
    pub fn enet1_ctrl_rgmii_id_mode(&mut self) -> Enet1CtrlRgmiiIdModeW<Cfg0Enet1CtrlSpec> {
        Enet1CtrlRgmiiIdModeW::new(self, 4)
    }
}
#[doc = "CFG0_ENET1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_enet1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_enet1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Enet1CtrlSpec;
impl crate::RegisterSpec for Cfg0Enet1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_enet1_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Enet1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_enet1_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Enet1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ENET1_CTRL to value 0x02"]
impl crate::Resettable for Cfg0Enet1CtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
