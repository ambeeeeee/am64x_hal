#[doc = "Register `VBUS_GBLSTAT` reader"]
pub type R = crate::R<VbusGblstatSpec>;
#[doc = "Register `VBUS_GBLSTAT` writer"]
pub type W = crate::W<VbusGblstatSpec>;
#[doc = "Field `OVRIDE` reader - 0:0\\]
PSC Override Status"]
pub type OvrideR = crate::BitReader;
#[doc = "Field `OVRIDE` writer - 0:0\\]
PSC Override Status"]
pub type OvrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EF_SMRFLEX` reader - 27:16\\]
Smart reflex class0 bits"]
pub type EfSmrflexR = crate::FieldReader<u16>;
#[doc = "Field `EF_SMRFLEX` writer - 27:16\\]
Smart reflex class0 bits"]
pub type EfSmrflexW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PSC Override Status"]
    #[inline(always)]
    pub fn ovride(&self) -> OvrideR {
        OvrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Smart reflex class0 bits"]
    #[inline(always)]
    pub fn ef_smrflex(&self) -> EfSmrflexR {
        EfSmrflexR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PSC Override Status"]
    #[inline(always)]
    #[must_use]
    pub fn ovride(&mut self) -> OvrideW<VbusGblstatSpec> {
        OvrideW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Smart reflex class0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn ef_smrflex(&mut self) -> EfSmrflexW<VbusGblstatSpec> {
        EfSmrflexW::new(self, 16)
    }
}
#[doc = "This register shows the PSC global status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_gblstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_gblstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusGblstatSpec;
impl crate::RegisterSpec for VbusGblstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_gblstat::R`](R) reader structure"]
impl crate::Readable for VbusGblstatSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_gblstat::W`](W) writer structure"]
impl crate::Writable for VbusGblstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_GBLSTAT to value 0"]
impl crate::Resettable for VbusGblstatSpec {
    const RESET_VALUE: u32 = 0;
}
