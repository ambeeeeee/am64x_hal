#[doc = "Register `CFG0_ICSSG0_CTRL1` reader"]
pub type R = crate::R<Cfg0Icssg0Ctrl1Spec>;
#[doc = "Register `CFG0_ICSSG0_CTRL1` writer"]
pub type W = crate::W<Cfg0Icssg0Ctrl1Spec>;
#[doc = "Field `ICSSG0_CTRL1_GPM_BIDI` reader - 19:0\\]
Controls operation of the ICSS_G0 PRU1_GPO pins. Each bit n controls the corresponding PRG0_PRU1GPOn I/O"]
pub type Icssg0Ctrl1GpmBidiR = crate::FieldReader<u32>;
#[doc = "Field `ICSSG0_CTRL1_GPM_BIDI` writer - 19:0\\]
Controls operation of the ICSS_G0 PRU1_GPO pins. Each bit n controls the corresponding PRG0_PRU1GPOn I/O"]
pub type Icssg0Ctrl1GpmBidiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `ICSSG0_CTRL1_RGMII1_ID_MODE` reader - 24:24\\]
Controls the ICSS_G0 RGMII1 port internal transmit delay"]
pub type Icssg0Ctrl1Rgmii1IdModeR = crate::BitReader;
#[doc = "Field `ICSSG0_CTRL1_RGMII1_ID_MODE` writer - 24:24\\]
Controls the ICSS_G0 RGMII1 port internal transmit delay"]
pub type Icssg0Ctrl1Rgmii1IdModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Controls operation of the ICSS_G0 PRU1_GPO pins. Each bit n controls the corresponding PRG0_PRU1GPOn I/O"]
    #[inline(always)]
    pub fn icssg0_ctrl1_gpm_bidi(&self) -> Icssg0Ctrl1GpmBidiR {
        Icssg0Ctrl1GpmBidiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls the ICSS_G0 RGMII1 port internal transmit delay"]
    #[inline(always)]
    pub fn icssg0_ctrl1_rgmii1_id_mode(&self) -> Icssg0Ctrl1Rgmii1IdModeR {
        Icssg0Ctrl1Rgmii1IdModeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Controls operation of the ICSS_G0 PRU1_GPO pins. Each bit n controls the corresponding PRG0_PRU1GPOn I/O"]
    #[inline(always)]
    #[must_use]
    pub fn icssg0_ctrl1_gpm_bidi(&mut self) -> Icssg0Ctrl1GpmBidiW<Cfg0Icssg0Ctrl1Spec> {
        Icssg0Ctrl1GpmBidiW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls the ICSS_G0 RGMII1 port internal transmit delay"]
    #[inline(always)]
    #[must_use]
    pub fn icssg0_ctrl1_rgmii1_id_mode(&mut self) -> Icssg0Ctrl1Rgmii1IdModeW<Cfg0Icssg0Ctrl1Spec> {
        Icssg0Ctrl1Rgmii1IdModeW::new(self, 24)
    }
}
#[doc = "CFG0_ICSSG0_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Icssg0Ctrl1Spec;
impl crate::RegisterSpec for Cfg0Icssg0Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_icssg0_ctrl1::R`](R) reader structure"]
impl crate::Readable for Cfg0Icssg0Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_icssg0_ctrl1::W`](W) writer structure"]
impl crate::Writable for Cfg0Icssg0Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ICSSG0_CTRL1 to value 0"]
impl crate::Resettable for Cfg0Icssg0Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
