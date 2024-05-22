#[doc = "Register `CPSW_NUSS_VBUSP_PTYPE_REG` reader"]
pub type R = crate::R<CpswNussVbuspPtypeRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_PTYPE_REG` writer"]
pub type W = crate::W<CpswNussVbuspPtypeRegSpec>;
#[doc = "Field `ESC_PRI_LD_VAL` reader - 4:0\\]
Escalate Priority Load Value"]
pub type EscPriLdValR = crate::FieldReader;
#[doc = "Field `ESC_PRI_LD_VAL` writer - 4:0\\]
Escalate Priority Load Value"]
pub type EscPriLdValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `P0_PTYPE_ESC` reader - 8:8\\]
Port 0 Priority Type Escalate"]
pub type P0PtypeEscR = crate::BitReader;
#[doc = "Field `P0_PTYPE_ESC` writer - 8:8\\]
Port 0 Priority Type Escalate"]
pub type P0PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1_PTYPE_ESC` reader - 9:9\\]
Port 1 Priority Type Escalate"]
pub type P1PtypeEscR = crate::BitReader;
#[doc = "Field `P1_PTYPE_ESC` writer - 9:9\\]
Port 1 Priority Type Escalate"]
pub type P1PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2_PTYPE_ESC` reader - 10:10\\]
Port 2 Priority Type Escalate"]
pub type P2PtypeEscR = crate::BitReader;
#[doc = "Field `P2_PTYPE_ESC` writer - 10:10\\]
Port 2 Priority Type Escalate"]
pub type P2PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3_PTYPE_ESC` reader - 11:11\\]
Port 3 Priority Type Escalate"]
pub type P3PtypeEscR = crate::BitReader;
#[doc = "Field `P3_PTYPE_ESC` writer - 11:11\\]
Port 3 Priority Type Escalate"]
pub type P3PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4_PTYPE_ESC` reader - 12:12\\]
Port 4 Priority Type Escalate"]
pub type P4PtypeEscR = crate::BitReader;
#[doc = "Field `P4_PTYPE_ESC` writer - 12:12\\]
Port 4 Priority Type Escalate"]
pub type P4PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5_PTYPE_ESC` reader - 13:13\\]
Port 5 Priority Type Escalate"]
pub type P5PtypeEscR = crate::BitReader;
#[doc = "Field `P5_PTYPE_ESC` writer - 13:13\\]
Port 5 Priority Type Escalate"]
pub type P5PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6_PTYPE_ESC` reader - 14:14\\]
Port 6 Priority Type Escalate"]
pub type P6PtypeEscR = crate::BitReader;
#[doc = "Field `P6_PTYPE_ESC` writer - 14:14\\]
Port 6 Priority Type Escalate"]
pub type P6PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7_PTYPE_ESC` reader - 15:15\\]
Port 7 Priority Type Escalate"]
pub type P7PtypeEscR = crate::BitReader;
#[doc = "Field `P7_PTYPE_ESC` writer - 15:15\\]
Port 7 Priority Type Escalate"]
pub type P7PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8_PTYPE_ESC` reader - 16:16\\]
Port 8 Priority Type Escalate"]
pub type P8PtypeEscR = crate::BitReader;
#[doc = "Field `P8_PTYPE_ESC` writer - 16:16\\]
Port 8 Priority Type Escalate"]
pub type P8PtypeEscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Escalate Priority Load Value"]
    #[inline(always)]
    pub fn esc_pri_ld_val(&self) -> EscPriLdValR {
        EscPriLdValR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 0 Priority Type Escalate"]
    #[inline(always)]
    pub fn p0_ptype_esc(&self) -> P0PtypeEscR {
        P0PtypeEscR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 1 Priority Type Escalate"]
    #[inline(always)]
    pub fn p1_ptype_esc(&self) -> P1PtypeEscR {
        P1PtypeEscR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 2 Priority Type Escalate"]
    #[inline(always)]
    pub fn p2_ptype_esc(&self) -> P2PtypeEscR {
        P2PtypeEscR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 3 Priority Type Escalate"]
    #[inline(always)]
    pub fn p3_ptype_esc(&self) -> P3PtypeEscR {
        P3PtypeEscR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 4 Priority Type Escalate"]
    #[inline(always)]
    pub fn p4_ptype_esc(&self) -> P4PtypeEscR {
        P4PtypeEscR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 5 Priority Type Escalate"]
    #[inline(always)]
    pub fn p5_ptype_esc(&self) -> P5PtypeEscR {
        P5PtypeEscR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 6 Priority Type Escalate"]
    #[inline(always)]
    pub fn p6_ptype_esc(&self) -> P6PtypeEscR {
        P6PtypeEscR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 7 Priority Type Escalate"]
    #[inline(always)]
    pub fn p7_ptype_esc(&self) -> P7PtypeEscR {
        P7PtypeEscR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Port 8 Priority Type Escalate"]
    #[inline(always)]
    pub fn p8_ptype_esc(&self) -> P8PtypeEscR {
        P8PtypeEscR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Escalate Priority Load Value"]
    #[inline(always)]
    #[must_use]
    pub fn esc_pri_ld_val(&mut self) -> EscPriLdValW<CpswNussVbuspPtypeRegSpec> {
        EscPriLdValW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 0 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p0_ptype_esc(&mut self) -> P0PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P0PtypeEscW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 1 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p1_ptype_esc(&mut self) -> P1PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P1PtypeEscW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 2 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p2_ptype_esc(&mut self) -> P2PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P2PtypeEscW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 3 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p3_ptype_esc(&mut self) -> P3PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P3PtypeEscW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 4 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p4_ptype_esc(&mut self) -> P4PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P4PtypeEscW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 5 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p5_ptype_esc(&mut self) -> P5PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P5PtypeEscW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 6 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p6_ptype_esc(&mut self) -> P6PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P6PtypeEscW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 7 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p7_ptype_esc(&mut self) -> P7PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P7PtypeEscW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Port 8 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn p8_ptype_esc(&mut self) -> P8PtypeEscW<CpswNussVbuspPtypeRegSpec> {
        P8PtypeEscW::new(self, 16)
    }
}
#[doc = "CPSW Transmit Priority Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ptype_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ptype_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspPtypeRegSpec;
impl crate::RegisterSpec for CpswNussVbuspPtypeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_ptype_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspPtypeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_ptype_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspPtypeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_PTYPE_REG to value 0"]
impl crate::Resettable for CpswNussVbuspPtypeRegSpec {
    const RESET_VALUE: u32 = 0;
}
