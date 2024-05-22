#[doc = "Register `CFG0_POR_BANDGAP_CTRL` reader"]
pub type R = crate::R<Cfg0PorBandgapCtrlSpec>;
#[doc = "Register `CFG0_POR_BANDGAP_CTRL` writer"]
pub type W = crate::W<Cfg0PorBandgapCtrlSpec>;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPC` reader - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
pub type PorBandgapCtrlBgapcR = crate::FieldReader;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPC` writer - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
pub type PorBandgapCtrlBgapcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPV` reader - 15:8\\]
Bandgap output voltage magnitude trim bits"]
pub type PorBandgapCtrlBgapvR = crate::FieldReader;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPV` writer - 15:8\\]
Bandgap output voltage magnitude trim bits"]
pub type PorBandgapCtrlBgapvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPI` reader - 19:16\\]
Bandgap output current trim bits"]
pub type PorBandgapCtrlBgapiR = crate::FieldReader;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPI` writer - 19:16\\]
Bandgap output current trim bits"]
pub type PorBandgapCtrlBgapiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
    #[inline(always)]
    pub fn por_bandgap_ctrl_bgapc(&self) -> PorBandgapCtrlBgapcR {
        PorBandgapCtrlBgapcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bandgap output voltage magnitude trim bits"]
    #[inline(always)]
    pub fn por_bandgap_ctrl_bgapv(&self) -> PorBandgapCtrlBgapvR {
        PorBandgapCtrlBgapvR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Bandgap output current trim bits"]
    #[inline(always)]
    pub fn por_bandgap_ctrl_bgapi(&self) -> PorBandgapCtrlBgapiR {
        PorBandgapCtrlBgapiR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
    #[inline(always)]
    #[must_use]
    pub fn por_bandgap_ctrl_bgapc(&mut self) -> PorBandgapCtrlBgapcW<Cfg0PorBandgapCtrlSpec> {
        PorBandgapCtrlBgapcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bandgap output voltage magnitude trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn por_bandgap_ctrl_bgapv(&mut self) -> PorBandgapCtrlBgapvW<Cfg0PorBandgapCtrlSpec> {
        PorBandgapCtrlBgapvW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Bandgap output current trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn por_bandgap_ctrl_bgapi(&mut self) -> PorBandgapCtrlBgapiW<Cfg0PorBandgapCtrlSpec> {
        PorBandgapCtrlBgapiW::new(self, 16)
    }
}
#[doc = "CFG0_POR_BANDGAP_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_bandgap_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_bandgap_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PorBandgapCtrlSpec;
impl crate::RegisterSpec for Cfg0PorBandgapCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_por_bandgap_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0PorBandgapCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_por_bandgap_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0PorBandgapCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POR_BANDGAP_CTRL to value 0"]
impl crate::Resettable for Cfg0PorBandgapCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
