#[doc = "Register `CFG0_ICSSG1_CTRL0_PROXY` reader"]
pub type R = crate::R<Cfg0Icssg1Ctrl0ProxySpec>;
#[doc = "Register `CFG0_ICSSG1_CTRL0_PROXY` writer"]
pub type W = crate::W<Cfg0Icssg1Ctrl0ProxySpec>;
#[doc = "Field `ICSSG1_CTRL0_GPM_BIDI_PROXY` reader - 19:0\\]
Controls operation of the ICSS_G1 PRU0_GPO pins. Each bit n controls the corresponding PRG1_PRU0GPOn I/O"]
pub type Icssg1Ctrl0GpmBidiProxyR = crate::FieldReader<u32>;
#[doc = "Field `ICSSG1_CTRL0_GPM_BIDI_PROXY` writer - 19:0\\]
Controls operation of the ICSS_G1 PRU0_GPO pins. Each bit n controls the corresponding PRG1_PRU0GPOn I/O"]
pub type Icssg1Ctrl0GpmBidiProxyW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `ICSSG1_CTRL0_RGMII0_ID_MODE_PROXY` reader - 24:24\\]
Controls the ICSS_G1 RGMII0 port internal transmit delay"]
pub type Icssg1Ctrl0Rgmii0IdModeProxyR = crate::BitReader;
#[doc = "Field `ICSSG1_CTRL0_RGMII0_ID_MODE_PROXY` writer - 24:24\\]
Controls the ICSS_G1 RGMII0 port internal transmit delay"]
pub type Icssg1Ctrl0Rgmii0IdModeProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Controls operation of the ICSS_G1 PRU0_GPO pins. Each bit n controls the corresponding PRG1_PRU0GPOn I/O"]
    #[inline(always)]
    pub fn icssg1_ctrl0_gpm_bidi_proxy(&self) -> Icssg1Ctrl0GpmBidiProxyR {
        Icssg1Ctrl0GpmBidiProxyR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls the ICSS_G1 RGMII0 port internal transmit delay"]
    #[inline(always)]
    pub fn icssg1_ctrl0_rgmii0_id_mode_proxy(&self) -> Icssg1Ctrl0Rgmii0IdModeProxyR {
        Icssg1Ctrl0Rgmii0IdModeProxyR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Controls operation of the ICSS_G1 PRU0_GPO pins. Each bit n controls the corresponding PRG1_PRU0GPOn I/O"]
    #[inline(always)]
    #[must_use]
    pub fn icssg1_ctrl0_gpm_bidi_proxy(
        &mut self,
    ) -> Icssg1Ctrl0GpmBidiProxyW<Cfg0Icssg1Ctrl0ProxySpec> {
        Icssg1Ctrl0GpmBidiProxyW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls the ICSS_G1 RGMII0 port internal transmit delay"]
    #[inline(always)]
    #[must_use]
    pub fn icssg1_ctrl0_rgmii0_id_mode_proxy(
        &mut self,
    ) -> Icssg1Ctrl0Rgmii0IdModeProxyW<Cfg0Icssg1Ctrl0ProxySpec> {
        Icssg1Ctrl0Rgmii0IdModeProxyW::new(self, 24)
    }
}
#[doc = "CFG0_ICSSG1_CTRL0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_ctrl0_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_ctrl0_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Icssg1Ctrl0ProxySpec;
impl crate::RegisterSpec for Cfg0Icssg1Ctrl0ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_icssg1_ctrl0_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Icssg1Ctrl0ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_icssg1_ctrl0_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Icssg1Ctrl0ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ICSSG1_CTRL0_PROXY to value 0"]
impl crate::Resettable for Cfg0Icssg1Ctrl0ProxySpec {
    const RESET_VALUE: u32 = 0;
}
