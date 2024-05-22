#[doc = "Register `APBADDR_ETM_CPU1_TRCITIDATAR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcitidatarSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCITIDATAR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcitidatarSpec>;
#[doc = "Field `ATDATAM_0` reader - 0:0\\]
Drives the ATDATAM\\[0\\]
output"]
pub type Atdatam0R = crate::BitReader;
#[doc = "Field `ATDATAM_0` writer - 0:0\\]
Drives the ATDATAM\\[0\\]
output"]
pub type Atdatam0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM_7` reader - 1:1\\]
Drives the ATDATAM\\[7\\]
output"]
pub type Atdatam7R = crate::BitReader;
#[doc = "Field `ATDATAM_7` writer - 1:1\\]
Drives the ATDATAM\\[7\\]
output"]
pub type Atdatam7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM_15` reader - 2:2\\]
Drives the ATDATAM\\[15\\]
output"]
pub type Atdatam15R = crate::BitReader;
#[doc = "Field `ATDATAM_15` writer - 2:2\\]
Drives the ATDATAM\\[15\\]
output"]
pub type Atdatam15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM_23` reader - 3:3\\]
Drives the ATDATAM\\[23\\]
output"]
pub type Atdatam23R = crate::BitReader;
#[doc = "Field `ATDATAM_23` writer - 3:3\\]
Drives the ATDATAM\\[23\\]
output"]
pub type Atdatam23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM_31` reader - 4:4\\]
Drives the ATDATAM\\[31\\]
output"]
pub type Atdatam31R = crate::BitReader;
#[doc = "Field `ATDATAM_31` writer - 4:4\\]
Drives the ATDATAM\\[31\\]
output"]
pub type Atdatam31W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCITIDATAR_31_5` reader - 31:5\\]
Reserved RES0"]
pub type Res0Trcitidatar31_5R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCITIDATAR_31_5` writer - 31:5\\]
Reserved RES0"]
pub type Res0Trcitidatar31_5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Drives the ATDATAM\\[0\\]
output"]
    #[inline(always)]
    pub fn atdatam_0(&self) -> Atdatam0R {
        Atdatam0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Drives the ATDATAM\\[7\\]
output"]
    #[inline(always)]
    pub fn atdatam_7(&self) -> Atdatam7R {
        Atdatam7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Drives the ATDATAM\\[15\\]
output"]
    #[inline(always)]
    pub fn atdatam_15(&self) -> Atdatam15R {
        Atdatam15R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Drives the ATDATAM\\[23\\]
output"]
    #[inline(always)]
    pub fn atdatam_23(&self) -> Atdatam23R {
        Atdatam23R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Drives the ATDATAM\\[31\\]
output"]
    #[inline(always)]
    pub fn atdatam_31(&self) -> Atdatam31R {
        Atdatam31R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved RES0"]
    #[inline(always)]
    pub fn res0_trcitidatar_31_5(&self) -> Res0Trcitidatar31_5R {
        Res0Trcitidatar31_5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Drives the ATDATAM\\[0\\]
output"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam_0(&mut self) -> Atdatam0W<ApbaddrEtmCpu1TrcitidatarSpec> {
        Atdatam0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Drives the ATDATAM\\[7\\]
output"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam_7(&mut self) -> Atdatam7W<ApbaddrEtmCpu1TrcitidatarSpec> {
        Atdatam7W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Drives the ATDATAM\\[15\\]
output"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam_15(&mut self) -> Atdatam15W<ApbaddrEtmCpu1TrcitidatarSpec> {
        Atdatam15W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Drives the ATDATAM\\[23\\]
output"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam_23(&mut self) -> Atdatam23W<ApbaddrEtmCpu1TrcitidatarSpec> {
        Atdatam23W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Drives the ATDATAM\\[31\\]
output"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam_31(&mut self) -> Atdatam31W<ApbaddrEtmCpu1TrcitidatarSpec> {
        Atdatam31W::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved RES0"]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcitidatar_31_5(&mut self) -> Res0Trcitidatar31_5W<ApbaddrEtmCpu1TrcitidatarSpec> {
        Res0Trcitidatar31_5W::new(self, 5)
    }
}
#[doc = "Integration Instruction ATB Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcitidatar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcitidatar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcitidatarSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcitidatarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcitidatar::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcitidatarSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcitidatar::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcitidatarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCITIDATAR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcitidatarSpec {
    const RESET_VALUE: u32 = 0;
}
