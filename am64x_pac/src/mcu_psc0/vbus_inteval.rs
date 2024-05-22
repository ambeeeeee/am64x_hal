#[doc = "Register `VBUS_INTEVAL` reader"]
pub type R = crate::R<VbusIntevalSpec>;
#[doc = "Register `VBUS_INTEVAL` writer"]
pub type W = crate::W<VbusIntevalSpec>;
#[doc = "Field `ALLEV` reader - 0:0\\]
Re_evaluate combined PSC interrupt"]
pub type AllevR = crate::BitReader;
#[doc = "Field `ALLEV` writer - 0:0\\]
Re_evaluate combined PSC interrupt"]
pub type AllevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERREV` reader - 1:1\\]
Re_evaluate Error Interrupt"]
pub type ErrevR = crate::BitReader;
#[doc = "Field `ERREV` writer - 1:1\\]
Re_evaluate Error Interrupt"]
pub type ErrevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPCEV` reader - 2:2\\]
External Power Control Interrupt Set"]
pub type EpcevR = crate::BitReader;
#[doc = "Field `EPCEV` writer - 2:2\\]
External Power Control Interrupt Set"]
pub type EpcevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSET` reader - 17:17\\]
Combined Interrupt Set"]
pub type ErrsetR = crate::BitReader;
#[doc = "Field `ERRSET` writer - 17:17\\]
Combined Interrupt Set"]
pub type ErrsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPCSET` reader - 18:18\\]
External Power Control Interrupt Set"]
pub type EpcsetR = crate::BitReader;
#[doc = "Field `EPCSET` writer - 18:18\\]
External Power Control Interrupt Set"]
pub type EpcsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOSET` reader - 19:19\\]
GOSTAT Interrupt Set"]
pub type GosetR = crate::BitReader;
#[doc = "Field `GOSET` writer - 19:19\\]
GOSTAT Interrupt Set"]
pub type GosetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Re_evaluate combined PSC interrupt"]
    #[inline(always)]
    pub fn allev(&self) -> AllevR {
        AllevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Re_evaluate Error Interrupt"]
    #[inline(always)]
    pub fn errev(&self) -> ErrevR {
        ErrevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
External Power Control Interrupt Set"]
    #[inline(always)]
    pub fn epcev(&self) -> EpcevR {
        EpcevR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Combined Interrupt Set"]
    #[inline(always)]
    pub fn errset(&self) -> ErrsetR {
        ErrsetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
External Power Control Interrupt Set"]
    #[inline(always)]
    pub fn epcset(&self) -> EpcsetR {
        EpcsetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
GOSTAT Interrupt Set"]
    #[inline(always)]
    pub fn goset(&self) -> GosetR {
        GosetR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Re_evaluate combined PSC interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn allev(&mut self) -> AllevW<VbusIntevalSpec> {
        AllevW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Re_evaluate Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errev(&mut self) -> ErrevW<VbusIntevalSpec> {
        ErrevW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
External Power Control Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn epcev(&mut self) -> EpcevW<VbusIntevalSpec> {
        EpcevW::new(self, 2)
    }
    #[doc = "Bit 17 - 17:17\\]
Combined Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn errset(&mut self) -> ErrsetW<VbusIntevalSpec> {
        ErrsetW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
External Power Control Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn epcset(&mut self) -> EpcsetW<VbusIntevalSpec> {
        EpcsetW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
GOSTAT Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn goset(&mut self) -> GosetW<VbusIntevalSpec> {
        GosetW::new(self, 19)
    }
}
#[doc = "This register has no storage. Read from this register returns 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_inteval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_inteval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusIntevalSpec;
impl crate::RegisterSpec for VbusIntevalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_inteval::R`](R) reader structure"]
impl crate::Readable for VbusIntevalSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_inteval::W`](W) writer structure"]
impl crate::Writable for VbusIntevalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_INTEVAL to value 0"]
impl crate::Resettable for VbusIntevalSpec {
    const RESET_VALUE: u32 = 0;
}
