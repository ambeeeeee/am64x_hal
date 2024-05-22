#[doc = "Register `CPSW_NUSS_VBUSP_RGMII1_STATUS_REG` reader"]
pub type R = crate::R<CpswNussVbuspRgmii1StatusRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_RGMII1_STATUS_REG` writer"]
pub type W = crate::W<CpswNussVbuspRgmii1StatusRegSpec>;
#[doc = "Field `LINK` reader - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
pub type LinkR = crate::BitReader;
#[doc = "Field `LINK` writer - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
pub type LinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `SPEED` writer - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FULLDUPLEX` reader - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
pub type FullduplexR = crate::BitReader;
#[doc = "Field `FULLDUPLEX` writer - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
pub type FullduplexW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
    #[inline(always)]
    pub fn fullduplex(&self) -> FullduplexR {
        FullduplexR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Rgmii1 link indicator: 0=Link is down, 1=Link is up"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LinkW<CpswNussVbuspRgmii1StatusRegSpec> {
        LinkW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Rgmii1 speed: 00=10Mbps, 01=100Mbps, 10=1000Mbps, 11=reserved"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<CpswNussVbuspRgmii1StatusRegSpec> {
        SpeedW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Rgmii1 full dulex: 0=Half-duplex, 1=Full-duplex"]
    #[inline(always)]
    #[must_use]
    pub fn fullduplex(&mut self) -> FullduplexW<CpswNussVbuspRgmii1StatusRegSpec> {
        FullduplexW::new(self, 3)
    }
}
#[doc = "RGMII1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_rgmii1_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_rgmii1_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspRgmii1StatusRegSpec;
impl crate::RegisterSpec for CpswNussVbuspRgmii1StatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_rgmii1_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspRgmii1StatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_rgmii1_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspRgmii1StatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_RGMII1_STATUS_REG to value 0"]
impl crate::Resettable for CpswNussVbuspRgmii1StatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
