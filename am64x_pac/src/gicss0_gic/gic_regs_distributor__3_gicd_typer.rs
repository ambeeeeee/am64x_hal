#[doc = "Register `GIC_REGS_Distributor__3_GICD_TYPER` reader"]
pub type R = crate::R<GicRegsDistributor_3GicdTyperSpec>;
#[doc = "Register `GIC_REGS_Distributor__3_GICD_TYPER` writer"]
pub type W = crate::W<GicRegsDistributor_3GicdTyperSpec>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__0_5` reader - 4:0\\]
ITLinesNumber"]
pub type Distributor_3GicdTyper_0_5R = crate::FieldReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__0_5` writer - 4:0\\]
ITLinesNumber"]
pub type Distributor_3GicdTyper_0_5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__5_3` reader - 7:5\\]
CPUNumber"]
pub type Distributor_3GicdTyper_5_3R = crate::FieldReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__5_3` writer - 7:5\\]
CPUNumber"]
pub type Distributor_3GicdTyper_5_3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__10_1` reader - 10:10\\]
SecurityExtn"]
pub type Distributor_3GicdTyper_10_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__10_1` writer - 10:10\\]
SecurityExtn"]
pub type Distributor_3GicdTyper_10_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__11_5` reader - 15:11\\]
LSPI"]
pub type Distributor_3GicdTyper_11_5R = crate::FieldReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__11_5` writer - 15:11\\]
LSPI"]
pub type Distributor_3GicdTyper_11_5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__16_1` reader - 16:16\\]
MBIS"]
pub type Distributor_3GicdTyper_16_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__16_1` writer - 16:16\\]
MBIS"]
pub type Distributor_3GicdTyper_16_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__17_1` reader - 17:17\\]
LPIS"]
pub type Distributor_3GicdTyper_17_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__17_1` writer - 17:17\\]
LPIS"]
pub type Distributor_3GicdTyper_17_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__18_1` reader - 18:18\\]
DVIS"]
pub type Distributor_3GicdTyper_18_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__18_1` writer - 18:18\\]
DVIS"]
pub type Distributor_3GicdTyper_18_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__19_5` reader - 23:19\\]
IDbits"]
pub type Distributor_3GicdTyper_19_5R = crate::FieldReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__19_5` writer - 23:19\\]
IDbits"]
pub type Distributor_3GicdTyper_19_5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__24_1` reader - 24:24\\]
A3V"]
pub type Distributor_3GicdTyper_24_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__3_GICD_TYPER__24_1` writer - 24:24\\]
A3V"]
pub type Distributor_3GicdTyper_24_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
ITLinesNumber"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__0_5(&self) -> Distributor_3GicdTyper_0_5R {
        Distributor_3GicdTyper_0_5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
CPUNumber"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__5_3(&self) -> Distributor_3GicdTyper_5_3R {
        Distributor_3GicdTyper_5_3R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
SecurityExtn"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__10_1(&self) -> Distributor_3GicdTyper_10_1R {
        Distributor_3GicdTyper_10_1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
LSPI"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__11_5(&self) -> Distributor_3GicdTyper_11_5R {
        Distributor_3GicdTyper_11_5R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
MBIS"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__16_1(&self) -> Distributor_3GicdTyper_16_1R {
        Distributor_3GicdTyper_16_1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
LPIS"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__17_1(&self) -> Distributor_3GicdTyper_17_1R {
        Distributor_3GicdTyper_17_1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
DVIS"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__18_1(&self) -> Distributor_3GicdTyper_18_1R {
        Distributor_3GicdTyper_18_1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
IDbits"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__19_5(&self) -> Distributor_3GicdTyper_19_5R {
        Distributor_3GicdTyper_19_5R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
A3V"]
    #[inline(always)]
    pub fn distributor__3_gicd_typer__24_1(&self) -> Distributor_3GicdTyper_24_1R {
        Distributor_3GicdTyper_24_1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
ITLinesNumber"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__0_5(
        &mut self,
    ) -> Distributor_3GicdTyper_0_5W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_0_5W::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
CPUNumber"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__5_3(
        &mut self,
    ) -> Distributor_3GicdTyper_5_3W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_5_3W::new(self, 5)
    }
    #[doc = "Bit 10 - 10:10\\]
SecurityExtn"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__10_1(
        &mut self,
    ) -> Distributor_3GicdTyper_10_1W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_10_1W::new(self, 10)
    }
    #[doc = "Bits 11:15 - 15:11\\]
LSPI"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__11_5(
        &mut self,
    ) -> Distributor_3GicdTyper_11_5W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_11_5W::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
MBIS"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__16_1(
        &mut self,
    ) -> Distributor_3GicdTyper_16_1W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_16_1W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
LPIS"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__17_1(
        &mut self,
    ) -> Distributor_3GicdTyper_17_1W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_17_1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
DVIS"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__18_1(
        &mut self,
    ) -> Distributor_3GicdTyper_18_1W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_18_1W::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
IDbits"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__19_5(
        &mut self,
    ) -> Distributor_3GicdTyper_19_5W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_19_5W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
A3V"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__3_gicd_typer__24_1(
        &mut self,
    ) -> Distributor_3GicdTyper_24_1W<GicRegsDistributor_3GicdTyperSpec> {
        Distributor_3GicdTyper_24_1W::new(self, 24)
    }
}
#[doc = "GICD_TYPER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__3_gicd_typer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__3_gicd_typer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_3GicdTyperSpec;
impl crate::RegisterSpec for GicRegsDistributor_3GicdTyperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__3_gicd_typer::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_3GicdTyperSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__3_gicd_typer::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_3GicdTyperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__3_GICD_TYPER to value 0x00ab_0428"]
impl crate::Resettable for GicRegsDistributor_3GicdTyperSpec {
    const RESET_VALUE: u32 = 0x00ab_0428;
}
