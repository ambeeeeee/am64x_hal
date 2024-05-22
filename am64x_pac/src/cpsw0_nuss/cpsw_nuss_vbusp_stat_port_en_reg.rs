#[doc = "Register `CPSW_NUSS_VBUSP_STAT_PORT_EN_REG` reader"]
pub type R = crate::R<CpswNussVbuspStatPortEnRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_STAT_PORT_EN_REG` writer"]
pub type W = crate::W<CpswNussVbuspStatPortEnRegSpec>;
#[doc = "Field `P0_STAT_EN` reader - 0:0\\]
Port 0 Statistics Enable"]
pub type P0StatEnR = crate::BitReader;
#[doc = "Field `P0_STAT_EN` writer - 0:0\\]
Port 0 Statistics Enable"]
pub type P0StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1_STAT_EN` reader - 1:1\\]
Port 1 Statistics Enable"]
pub type P1StatEnR = crate::BitReader;
#[doc = "Field `P1_STAT_EN` writer - 1:1\\]
Port 1 Statistics Enable"]
pub type P1StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_STAT_EN` reader - 2:2\\]
Port 2 Statistics Enable"]
pub type P2StatEnR = crate::BitReader;
#[doc = "Field `P2_STAT_EN` writer - 2:2\\]
Port 2 Statistics Enable"]
pub type P2StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3_STAT_EN` reader - 3:3\\]
Port 3 Statistics Enable"]
pub type P3StatEnR = crate::BitReader;
#[doc = "Field `P3_STAT_EN` writer - 3:3\\]
Port 3 Statistics Enable"]
pub type P3StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4_STAT_EN` reader - 4:4\\]
Port 4 Statistics Enable"]
pub type P4StatEnR = crate::BitReader;
#[doc = "Field `P4_STAT_EN` writer - 4:4\\]
Port 4 Statistics Enable"]
pub type P4StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5_STAT_EN` reader - 5:5\\]
Port 5 Statistics Enable"]
pub type P5StatEnR = crate::BitReader;
#[doc = "Field `P5_STAT_EN` writer - 5:5\\]
Port 5 Statistics Enable"]
pub type P5StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6_STAT_EN` reader - 6:6\\]
Port 6 Statistics Enable"]
pub type P6StatEnR = crate::BitReader;
#[doc = "Field `P6_STAT_EN` writer - 6:6\\]
Port 6 Statistics Enable"]
pub type P6StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7_STAT_EN` reader - 7:7\\]
Port 7 Statistics Enable"]
pub type P7StatEnR = crate::BitReader;
#[doc = "Field `P7_STAT_EN` writer - 7:7\\]
Port 7 Statistics Enable"]
pub type P7StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8_STAT_EN` reader - 8:8\\]
Port 8 Statistics Enable"]
pub type P8StatEnR = crate::BitReader;
#[doc = "Field `P8_STAT_EN` writer - 8:8\\]
Port 8 Statistics Enable"]
pub type P8StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Port 0 Statistics Enable"]
    #[inline(always)]
    pub fn p0_stat_en(&self) -> P0StatEnR {
        P0StatEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Port 1 Statistics Enable"]
    #[inline(always)]
    pub fn p1_stat_en(&self) -> P1StatEnR {
        P1StatEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 2 Statistics Enable"]
    #[inline(always)]
    pub fn p2_stat_en(&self) -> P2StatEnR {
        P2StatEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 3 Statistics Enable"]
    #[inline(always)]
    pub fn p3_stat_en(&self) -> P3StatEnR {
        P3StatEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 4 Statistics Enable"]
    #[inline(always)]
    pub fn p4_stat_en(&self) -> P4StatEnR {
        P4StatEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 5 Statistics Enable"]
    #[inline(always)]
    pub fn p5_stat_en(&self) -> P5StatEnR {
        P5StatEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 6 Statistics Enable"]
    #[inline(always)]
    pub fn p6_stat_en(&self) -> P6StatEnR {
        P6StatEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 7 Statistics Enable"]
    #[inline(always)]
    pub fn p7_stat_en(&self) -> P7StatEnR {
        P7StatEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 8 Statistics Enable"]
    #[inline(always)]
    pub fn p8_stat_en(&self) -> P8StatEnR {
        P8StatEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Port 0 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p0_stat_en(&mut self) -> P0StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P0StatEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Port 1 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p1_stat_en(&mut self) -> P1StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P1StatEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 2 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p2_stat_en(&mut self) -> P2StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P2StatEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 3 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p3_stat_en(&mut self) -> P3StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P3StatEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 4 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p4_stat_en(&mut self) -> P4StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P4StatEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 5 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p5_stat_en(&mut self) -> P5StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P5StatEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 6 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p6_stat_en(&mut self) -> P6StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P6StatEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 7 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p7_stat_en(&mut self) -> P7StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P7StatEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 8 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p8_stat_en(&mut self) -> P8StatEnW<CpswNussVbuspStatPortEnRegSpec> {
        P8StatEnW::new(self, 8)
    }
}
#[doc = "CPSW Statistics Port Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_stat_port_en_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_stat_port_en_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspStatPortEnRegSpec;
impl crate::RegisterSpec for CpswNussVbuspStatPortEnRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_stat_port_en_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspStatPortEnRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_stat_port_en_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspStatPortEnRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_STAT_PORT_EN_REG to value 0"]
impl crate::Resettable for CpswNussVbuspStatPortEnRegSpec {
    const RESET_VALUE: u32 = 0;
}
