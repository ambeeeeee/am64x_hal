#[doc = "Register `CPSW_NUSS_VBUSP_SUBSYSTEM_CONFIG_REG` reader"]
pub type R = crate::R<CpswNussVbuspSubsystemConfigRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SUBSYSTEM_CONFIG_REG` writer"]
pub type W = crate::W<CpswNussVbuspSubsystemConfigRegSpec>;
#[doc = "Field `NUM_PORTS` reader - 7:0\\]
The total number of ports including the host port 0"]
pub type NumPortsR = crate::FieldReader;
#[doc = "Field `NUM_PORTS` writer - 7:0\\]
The total number of ports including the host port 0"]
pub type NumPortsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUM_GENF` reader - 12:8\\]
The number of CPTS GENF outputs"]
pub type NumGenfR = crate::FieldReader;
#[doc = "Field `NUM_GENF` writer - 12:8\\]
The number of CPTS GENF outputs"]
pub type NumGenfW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RMII` reader - 16:16\\]
RMII is included in the CPSW_NUSS"]
pub type RmiiR = crate::BitReader;
#[doc = "Field `RMII` writer - 16:16\\]
RMII is included in the CPSW_NUSS"]
pub type RmiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGMII` reader - 17:17\\]
RGMII is included in the CPSW_NUSS"]
pub type RgmiiR = crate::BitReader;
#[doc = "Field `RGMII` writer - 17:17\\]
RGMII is included in the CPSW_NUSS"]
pub type RgmiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGMII` reader - 18:18\\]
SGMII is included in the CPSW_NUSS"]
pub type SgmiiR = crate::BitReader;
#[doc = "Field `SGMII` writer - 18:18\\]
SGMII is included in the CPSW_NUSS"]
pub type SgmiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSGMII` reader - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
pub type QsgmiiR = crate::BitReader;
#[doc = "Field `QSGMII` writer - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
pub type QsgmiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XGMII` reader - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
pub type XgmiiR = crate::FieldReader;
#[doc = "Field `XGMII` writer - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
pub type XgmiiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The total number of ports including the host port 0"]
    #[inline(always)]
    pub fn num_ports(&self) -> NumPortsR {
        NumPortsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    pub fn num_genf(&self) -> NumGenfR {
        NumGenfR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
RMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn rmii(&self) -> RmiiR {
        RmiiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
RGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn rgmii(&self) -> RgmiiR {
        RgmiiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
SGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn sgmii(&self) -> SgmiiR {
        SgmiiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn qsgmii(&self) -> QsgmiiR {
        QsgmiiR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn xgmii(&self) -> XgmiiR {
        XgmiiR::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The total number of ports including the host port 0"]
    #[inline(always)]
    #[must_use]
    pub fn num_ports(&mut self) -> NumPortsW<CpswNussVbuspSubsystemConfigRegSpec> {
        NumPortsW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    #[must_use]
    pub fn num_genf(&mut self) -> NumGenfW<CpswNussVbuspSubsystemConfigRegSpec> {
        NumGenfW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
RMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn rmii(&mut self) -> RmiiW<CpswNussVbuspSubsystemConfigRegSpec> {
        RmiiW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
RGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii(&mut self) -> RgmiiW<CpswNussVbuspSubsystemConfigRegSpec> {
        RgmiiW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
SGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn sgmii(&mut self) -> SgmiiW<CpswNussVbuspSubsystemConfigRegSpec> {
        SgmiiW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn qsgmii(&mut self) -> QsgmiiW<CpswNussVbuspSubsystemConfigRegSpec> {
        QsgmiiW::new(self, 19)
    }
    #[doc = "Bits 20:27 - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn xgmii(&mut self) -> XgmiiW<CpswNussVbuspSubsystemConfigRegSpec> {
        XgmiiW::new(self, 20)
    }
}
#[doc = "Subsystem Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_subsystem_config_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_subsystem_config_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSubsystemConfigRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSubsystemConfigRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_subsystem_config_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSubsystemConfigRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_subsystem_config_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSubsystemConfigRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SUBSYSTEM_CONFIG_REG to value 0x0007_0203"]
impl crate::Resettable for CpswNussVbuspSubsystemConfigRegSpec {
    const RESET_VALUE: u32 = 0x0007_0203;
}
